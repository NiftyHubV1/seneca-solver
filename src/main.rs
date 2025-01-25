mod seneca_client;
mod utils;

use utils::{generate_assignment_string, input_or_clipboard};

use chrono::{DateTime, Utc};
use indicatif::ProgressBar;
use inquire::{Password, PasswordDisplayMode, Select, Text};
use regex::Regex;
use seneca_client::SenecaClient;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read access key from clipboard if none was entered, then trim whitespace
    let access_key = input_or_clipboard(
        Password::new("Enter your access key (leave blank to use clipboard):")
            .without_confirmation()
            .with_display_mode(PasswordDisplayMode::Hidden)
            .prompt(),
    )?;

    let client = SenecaClient::new(&access_key).await?;
    let assignments = client.get_assignments().await?;

    // Filter out assignments that have not started yet
    let now = Utc::now();
    let assignments: Vec<&Value> = assignments
        .iter()
        .filter(|assignment| {
            DateTime::parse_from_rfc3339(assignment["startDate"].as_str().unwrap()).unwrap() <= now
        })
        .collect();

    // Find longest assignment name and longest status to use for padding
    let longest_assignment_length = assignments
        .iter()
        .map(|assignment| assignment["name"].as_str().unwrap().len())
        .max()
        .unwrap_or(0);
    let longest_status_length = assignments
        .iter()
        .map(|assignment| assignment["status"].as_str().unwrap().len())
        .max()
        .unwrap_or(0);

    let assignment_names: Vec<String> = assignments
        .iter()
        .map(|assignment| {
            generate_assignment_string(assignment, longest_assignment_length, longest_status_length)
        })
        .collect();
    let mut assignment_names: Vec<&str> = assignment_names.iter().map(|s| s.as_str()).collect();

    // Other options
    assignment_names.insert(0, "All assignments");
    assignment_names.insert(1, "Custom (from URL)");

    let assignment_name = Select::new("Choose assignment:", assignment_names).prompt()?;

    match assignment_name {
        "Custom (from URL)" => {
            let course_id = input_or_clipboard(
                Text::new("Enter URL of section (leave blank to use clipboard):").prompt(),
            )?;

            let re = Regex::new(r"(?<course_id>\w{8}-\w{4}-\w{4}-\w{4}-\w{12})/section/(?<section_id>\w{8}-\w{4}-\w{4}-\w{4}-\w{12})").unwrap();

            if let Some(captures) = re.captures(&course_id) {
                let course_id = captures.name("course_id").unwrap().as_str();
                let section_id = captures.name("section_id").unwrap().as_str();

                let contents_tuple = client.get_contents(&course_id, &section_id).await;

                // Check if error is 404
                if let Err(e) = &contents_tuple {
                    if e.is_status() {
                        return Err("Course and/or section not found. Double-check that you have entered a valid Seneca URL".into());
                    }
                }

                let (index, title, contents) = contents_tuple?;
                let contents = contents.as_array().unwrap();

                println!("📝 Solving section: ");
                println!(
                    "🔍 Found {} subsection(s) in section {}: {}",
                    contents.len(),
                    index,
                    title
                );

                if contents.len() > 1 {
                    let content_names: Vec<String> = contents
                        .iter()
                        .map(|content| {
                            content["tags"].as_array().unwrap()[0]
                                .as_str()
                                .unwrap()
                                .to_string()
                        })
                        .collect();

                    let content_name = Select::new("Choose subsection:", content_names).prompt()?;

                    let content = contents
                        .iter()
                        .find(|content| {
                            content["tags"].as_array().unwrap()[0]
                                .as_str()
                                .unwrap()
                                .to_string() == content_name
                        })
                        .unwrap();

                    // for content in contents {
                    client.run_solver(&course_id, &section_id, content).await?;
                    // }
                } else {
                    client
                        .run_solver(&course_id, &section_id, &contents[0])
                        .await?;
                }

                println!("✅ Subsection solved");

                return Ok(());
            } else {
                return Err("Invalid assignment URL. This URL should be in the format https://app.senecalearning.com/classroom/course/<course_id>/section/<section_id>/session".into());
            }
        }
        "All assignments" => {
            let assignments_len = assignments.len();

            for (i, assignment) in assignments.iter().enumerate() {
                println!(
                    "📝 Solving assignment: {}     {}/{assignments_len}",
                    assignment["name"].as_str().unwrap(),
                    i + 1,
                );
                solve_assignments(assignment, &client).await?;
            }

            println!("✅ All assignments solved");

            return Ok(());
        }
        _ => {
            let assignment = assignments
                .iter()
                .find(|assignment| {
                    generate_assignment_string(
                        assignment,
                        longest_assignment_length,
                        longest_status_length,
                    ) == assignment_name
                })
                .unwrap();

            println!(
                "📝 Solving assignment: {}",
                assignment["name"].as_str().unwrap()
            );

            solve_assignments(assignment, &client).await
        }
    }
}

async fn solve_assignments<'a>(
    assignment: &Value,
    client: &SenecaClient<'_>,
) -> Result<(), Box<dyn Error>> {
    let course_id = assignment["spec"]["courseId"].as_str().unwrap().to_string();
    let section_id_len = assignment["spec"]["sectionIds"].as_array().unwrap().len();

    let progress_bar = ProgressBar::new(section_id_len as u64);

    for section_id in assignment["spec"]["sectionIds"].as_array().unwrap() {
        let section_id = section_id.as_str().unwrap();
        let (_, _, contents) = client.get_contents(&course_id, section_id).await?;
        let contents = contents.as_array().unwrap();

        for content in contents {
            if let Err(e) = client.run_solver(&course_id, section_id, content).await {
                eprintln!("Error running solver for section {}: {}", section_id, e);
            }
        }

        // Increment progress bar
        progress_bar.inc(1);
    }

    progress_bar.finish();
    println!(
        "⏱️ Assignment solved in {} seconds",
        progress_bar.elapsed().as_secs_f32()
    );

    Ok(())
}
