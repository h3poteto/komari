use chrono::{DateTime, FixedOffset};
use github_rs::client::{Executor, Github};
use github_rs::errors::Error;
use serde_json::Value;
use std::env;

pub enum PullsError {
    GitHubError { error: Error },

    JsonError { error: String },

    EnvError { error: env::VarError },
}

impl From<Error> for PullsError {
    fn from(error: Error) -> Self {
        PullsError::GitHubError { error }
    }
}

impl From<JsonError> for PullsError {
    fn from(err: JsonError) -> Self {
        PullsError::JsonError { error: err.error }
    }
}

impl From<env::VarError> for PullsError {
    fn from(error: env::VarError) -> Self {
        PullsError::EnvError { error }
    }
}

struct JsonError {
    error: String,
}

impl JsonError {
    fn new(err: String) -> JsonError {
        JsonError { error: err }
    }
}

pub struct Pulls {
    owner: String,
    repo: String,
}

impl Pulls {
    pub fn new(owner: &String, repo: &String) -> Self {
        Pulls {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    pub fn list(&self) -> Result<Value, PullsError> {
        let github_token = env::var("GITHUB_TOKEN")?;
        let client = Github::new(github_token)?;

        // https://github.com/github-rs/github-rs/blob/master/src/repos/get.rs#L265
        // Pulls does not provide reference parameter, so we can't provide query parameter.
        let pulls_url = format!(
            "repos/{}/{}/pulls?state=closed&sort=updated&direction=desc",
            self.owner, self.repo
        );
        let response = client
            .get()
            .custom_endpoint(&pulls_url)
            .execute::<Value>()?;
        let (headers, status, json) = response;

        if let Some(json) = json {
            return Ok(json);
        }
        println!("{:#?}", headers);
        println!("{}", status);
        let err: JsonError = JsonError::new("json is empty".to_string());
        Err(err.into())
    }

    pub fn get(&self, number: &i64) -> Result<Value, PullsError> {
        let github_token = env::var("GITHUB_TOKEN")?;
        let client = Github::new(github_token)?;

        let response = client
            .get()
            .repos()
            .owner(&self.owner)
            .repo(&self.repo)
            .pulls()
            .number(&number.to_string())
            .execute::<Value>()?;
        let (headers, status, json) = response;

        if let Some(json) = json {
            return Ok(json);
        }
        println!("{:#?}", headers);
        println!("{}", status);
        let err: JsonError = JsonError::new("json is empty".to_string());
        Err(err.into())
    }

    pub fn select(&self, array: &Vec<Value>, number: &i64) -> Result<Vec<Value>, PullsError> {
        let since = self.get(number)?;
        if let Some(Ok(since_date)) = since["merged_at"].as_str().map(|m| DateTime::parse_from_rfc3339(&m)) {
            let res: Vec<Value> = array
                .iter()
                .filter(|a| {
                    a["merged_at"].is_string()
                        && self.time_diff(since_date, a["merged_at"].as_str().unwrap())
                })
                .cloned()
                .collect();
            return Ok(res);
        }
        let err: JsonError = JsonError::new("merged_at is not contained".to_string());
        Err(err.into())
    }

    fn time_diff(&self, since: DateTime<FixedOffset>, time: &str) -> bool {
        match DateTime::parse_from_rfc3339(time) {
            Ok(t) => {
                since.lt(&t)
            },
            Err(_e) => false
        }
    }
}
