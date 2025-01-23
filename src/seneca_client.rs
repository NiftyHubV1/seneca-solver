use crate::utils::generate_hex_string;

use chrono::{prelude::*, Duration};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

pub struct SenecaClient<'a> {
    client: Client,
    access_key: &'a str,
    pub user_id: String,
}

impl<'a> SenecaClient<'a> {
    pub async fn new(access_key: &'a str) -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        println!("SenecaClient created");
        let mut self_to_return = Self {
            client,
            access_key,
            user_id: String::new(),
        };

        self_to_return.user_id = Self::get_user_id(&self_to_return).await?;
        Ok(self_to_return)
    }

    async fn get_user_id(&self) -> Result<String, Box<dyn Error>> {
        let url = "https://user-info.app.senecalearning.com/api/user-info/me";

        let headers: HeaderMap = Self::assemble_headers(vec![
            ("Host", "user-info.app.senecalearning.com"),
            (
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0",
            ),
            ("Accept", "*/*"),
            ("Accept-Language", "en-GB,en;q=0.5"),
            ("Accept-Encoding", "gzip, deflate, br, zstd"),
            ("Referer", "https://app.senecalearning.com/"),
            ("access-key", &self.access_key),
            ("Content-Type", "application/json"),
            (
                "correlationId",
                "1737330516472::76115c42-02c9-4d56-0000-000000000000",
            ),
            ("user-region", "GB"),
            ("Origin", "https://app.senecalearning.com"),
            ("DNT", "1"),
            ("Sec-GPC", "1"),
            ("Sec-Fetch-Dest", "empty"),
            ("Sec-Fetch-Mode", "cors"),
            ("Sec-Fetch-Site", "same-site"),
            ("Connection", "keep-alive"),
            ("host", "user-info.app.senecalearning.com"),
        ]);

        let response = self.client.get(url).headers(headers).send().await?;

        if response.status().is_success() {
            let body = response.json::<Value>().await?;
            Ok(body["userId"].as_str().unwrap().to_string())
        } else {
            Err(Box::new(response.error_for_status().unwrap_err()))
        }
    }

    async fn get_signed_url(
        &self,
        course_id: &str,
        section_id: &str,
    ) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "https://course.app.senecalearning.com/api/courses/{}/signed-url?sectionId={}&contentTypes=standard,hardestQuestions",
            course_id, section_id
        );

        let headers: HeaderMap = Self::assemble_headers(vec![
            ("Host", "course.app.senecalearning.com"),
            (
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0",
            ),
            ("Accept", "*/*"),
            ("Accept-Language", "en-GB,en;q=0.5"),
            ("Accept-Encoding", "gzip, deflate, br, zstd"),
            ("Referer", "https://app.senecalearning.com/"),
            ("access-key", &self.access_key),
            ("Content-Type", "application/json"),
            (
                "correlationId",
                "1737330516472::76115c42-02c9-4d56-0000-000000000000",
            ),
            ("user-region", "GB"),
            ("Origin", "https://app.senecalearning.com"),
            ("DNT", "1"),
            ("Sec-GPC", "1"),
            ("Sec-Fetch-Dest", "empty"),
            ("Sec-Fetch-Mode", "cors"),
            ("Sec-Fetch-Site", "same-site"),
            ("Connection", "keep-alive"),
            ("host", "course.app.senecalearning.com"),
        ]);

        let response = self.client.get(&url).headers(headers).send().await?;

        if response.status().is_success() {
            let body = response.json::<Value>().await?;
            Ok(body["url"].as_str().unwrap().to_string())
        } else {
            Err(Box::new(response.error_for_status().unwrap_err()))
        }
    }

    pub async fn get_content(
        &self,
        course_id: &str,
        section_id: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let url = self.get_signed_url(course_id, section_id).await?;

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let body = response.json::<Value>().await?;
            // println!("{:#?}", body);
            Ok(body["contents"].clone())
        } else {
            Err(Box::new(response.error_for_status().unwrap_err()))
        }
    }

    pub async fn run_solver(
        &self,
        course_id: &str,
        section_id: &str,
        content_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let contents = self.get_content(course_id, section_id).await?;
        let content = contents
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["id"].as_str().unwrap() == content_id)
            .unwrap();

        let session_id = format!(
            "{}-{}-{}-{}-{}",
            generate_hex_string(4),
            generate_hex_string(2),
            generate_hex_string(2),
            generate_hex_string(2),
            generate_hex_string(6)
        );
        println!("Session ID: {}", session_id);

        let now = Utc::now();
        let started = now - Duration::minutes(2);
        let started_module = now - Duration::seconds(15);

        // SecondsFormat::Secs used to only show seconds, false used to show timezone (+00:00)
        println!("{}", now.to_rfc3339_opts(SecondsFormat::Secs, false));
        println!("{}", started.to_rfc3339_opts(SecondsFormat::Secs, false));
        println!(
            "{}",
            started_module.to_rfc3339_opts(SecondsFormat::Secs, false)
        );

        let non_question_module_types = vec!["concept", "video", "image", "delve"];

        let content_modules = content["contentModules"].as_array().unwrap();
        let content_modules_len = content_modules.len();

        let mut modules = Vec::<Value>::new();

        let mut data = json!({
            "platform": "seneca",
            "clientVersion": "2.13.81",
            "userId": self.user_id,
            "userLevelFeatureFlagValue": "control",
            "session": {
                "sessionId": session_id,
                "courseId": &course_id,
                "timeStarted": started.to_rfc3339_opts(SecondsFormat::Secs, false),
                "timeFinished": now.to_rfc3339_opts(SecondsFormat::Secs, false),
                "startingProficiency": 0,
                "endingProficiency": 0.5,
                "startingCourseProficiency": 0.003601579633505109,
                "endingCourseProficiency": 0.04580470162748644,
                "endingCourseScore": 0.07210750573582432,
                "sessionScore": 1,
                "completed": true,
                "modulesCorrect": content_modules_len,
                "modulesIncorrect": 0,
                "averageScore": 1,
                "modulesGaveUp": 0,
                "modulesStudied": content_modules_len,
                "modulesTested": content_modules_len,
                "sessionType": "adaptive",
                "sectionIds": [&section_id],
                "contentIds": [],
                "options": {
                    "hasHardestQuestionContent": if let Some(content_type) = content["contentType"].as_str() {
                        content_type == "hardestQuestions"
                    } else {
                        false
                    },
                },
            },
            "modules": [],
        });

        let module_template = json!({
            "sessionId": session_id,
            "moduleOrder": 0,
            "moduleId": "",
            "timeStarted": started_module.to_rfc3339_opts(SecondsFormat::Secs, false),
            "timeFinished": now.to_rfc3339_opts(SecondsFormat::Secs, false),
            "gaveUp": false,
            "submitted": true,
            "completed": true,
            "testingActive": true,
            "content": {},
            "score": 1,
            "moduleScore": {
                "score": 1,
            },
            "userAnswer": {},
            "courseId": course_id,
            "sectionId": section_id,
            "contentId": content_id,
        });

        let mut non_questions: u64 = 0;

        for (content_module_no, content_module) in content_modules.iter().enumerate() {
            let content_module = content_module.as_object().unwrap();

            let mut module = module_template.clone();
            module["moduleOrder"] = json!(content_module_no);
            module["moduleId"] = json!(content_module["id"].as_str().unwrap());
            module["moduleType"] = json!(content_module["moduleType"].as_str().unwrap());

            if non_question_module_types.contains(&content_module["moduleType"].as_str().unwrap()) {
                module["submitted"] = json!(false);
                module["testingActive"] = json!(false);
                module["score"] = json!(0);
                module.as_object_mut().unwrap().remove("moduleScore");
                module.as_object_mut().unwrap().remove("userAnswer");

                // Increment non_questions counter
                non_questions += 1;
            } else if module["moduleType"].as_str().unwrap() == "toggles" {
                module["userAnswer"] = json!([]);
            }

            modules.push(module);
        }

        data["modules"] = json!(modules);

        data["session"]["modulesCorrect"] =
            json!(data["session"]["modulesCorrect"].as_u64().unwrap() - non_questions);
        data["session"]["modulesTested"] =
            json!(data["session"]["modulesTested"].as_u64().unwrap() - non_questions);

        let url = "https://stats.app.senecalearning.com/api/stats/sessions";

        let headers = Self::assemble_headers(vec![
            ("Host", "stats.app.senecalearning.com"),
            (
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0",
            ),
            ("Accept", "*/*"),
            ("Accept-Language", "en-GB,en;q=0.5"),
            ("Accept-Encoding", "gzip, deflate, br, zstd"),
            ("Content-Type", "application/json"),
            ("Referer", "https://app.senecalearning.com/"),
            ("access-key", &self.access_key),
            (
                "correlationId",
                "1737330516472::76115c42-02c9-4d56-0000-000000000000",
            ),
            ("user-region", "GB"),
            ("Origin", "https://app.senecalearning.com"),
            ("DNT", "1"),
            ("Sec-GPC", "1"),
            ("Sec-Fetch-Dest", "empty"),
            ("Sec-Fetch-Mode", "cors"),
            ("Sec-Fetch-Site", "same-site"),
            ("Connection", "keep-alive"),
            ("host", "stats.app.senecalearning.com"),
        ]);

        let response = self
            .client
            .post(url)
            .headers(headers)
            .json(&data)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Box::new(response.error_for_status().unwrap_err()))
        }
    }

    fn assemble_headers(headers: Vec<(&str, &str)>) -> HeaderMap {
        headers
            .iter()
            .map(|(key, value)| (key.parse().unwrap(), HeaderValue::from_str(value).unwrap()))
            .collect()
    }
}
