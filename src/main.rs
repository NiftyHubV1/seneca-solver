mod seneca_client;
mod utils;

use utils::{generate_assignment_string, read_clipboard};

use indicatif::ProgressBar;
use inquire::{Password, PasswordDisplayMode, Select, Text};
use seneca_client::SenecaClient;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Prompt for access key
    let entered_key = if let Ok(entered_key) =
        Password::new("Enter your access key (leave empty to use clipboard):")
            .without_confirmation()
            .with_display_mode(PasswordDisplayMode::Hidden)
            .prompt()
    {
        entered_key
    } else {
        println!("");
        return Err("Failed to read access key".into());
    };

    // Read access key from clipboard if none was entered, then trim whitespace
    let access_key = if entered_key.is_empty() {
        read_clipboard()
    } else {
        entered_key
    }
    .trim()
    .to_string();

    let client = SenecaClient::new(&access_key).await?;
    let assignments = client.get_assignments().await?;

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
    assignment_names.push("Custom (advanced)");

    let assignment_name = Select::new("Choose assignment:", assignment_names).prompt()?;

    // Custom assignment
    if assignment_name == "Custom (advanced)" {
        let course_id = Text::new("Enter course ID:").prompt()?;
        let section_id = Text::new("Enter section ID:").prompt()?;

        let contents = client.get_contents(&course_id, &section_id).await;

        // Check if error is 404
        if let Err(e) = &contents {
            if e.is_status() {
                return Err("Course and/or section not found. Double-check that you have entered the correct IDs".into());
            }
        }

        println!("Solving custom assignment");
        
        let contents = contents?;
        for content in contents.as_array().unwrap() {
            client.run_solver(&course_id, &section_id, content).await?;
        }

        return Ok(());
    }

    let assignment = assignments
        .iter()
        .find(|assignment| {
            generate_assignment_string(assignment, longest_assignment_length, longest_status_length)
                == assignment_name
        })
        .unwrap();

    println!(
        "Solving assignment: {}",
        assignment["name"].as_str().unwrap()
    );

    solve_assignments(assignment, client).await
}

async fn solve_assignments<'a>(
    assignment: &Value,
    client: SenecaClient<'_>,
) -> Result<(), Box<dyn Error>> {
    let course_id = assignment["spec"]["courseId"].as_str().unwrap().to_string();
    let section_id_len = assignment["spec"]["sectionIds"].as_array().unwrap().len();

    let progress_bar = ProgressBar::new(section_id_len as u64);

    for section_id in assignment["spec"]["sectionIds"].as_array().unwrap().iter() {
        let section_id = section_id.as_str().unwrap();
        let contents = client.get_contents(&course_id, section_id).await?;

        for content in contents.as_array().unwrap() {
            client.run_solver(&course_id, section_id, content).await?;
        }

        // Increment progress bar
        progress_bar.inc(1);
    }

    progress_bar.finish();
    println!(
        "Assignment solved in {} seconds",
        progress_bar.elapsed().as_secs_f32()
    );

    Ok(())
}
