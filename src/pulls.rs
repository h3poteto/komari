use github_rs::client::{Executor, Github};
use serde_json::Value;
use github_rs::errors::Error;
use std::env;

pub enum PullsError {
    GitHubError {
        error: Error,
    },

    JsonError {
        error: String,
    },

    EnvError {
        error: env::VarError,
    }
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
        JsonError {
            error: err,
        }
    }
}


pub fn list(owner: &String, repo: &String) -> Result<Option<Value>, PullsError> {
    let github_token = env::var("GITHUB_TOKEN")?;
    let client = Github::new(github_token).unwrap();

    // https://github.com/github-rs/github-rs/blob/master/src/repos/get.rs#L265
    // Pulls does not provide reference parameter, so we can't provide query parameter.
    let pulls_url = format!("repos/{}/{}/pulls?state=closed&sort=updated&direction=desc", owner, repo);
    let response = client.get().custom_endpoint(&pulls_url).execute::<Value>()?;
    let (headers, status, json) = response;

    if let Some(json) = json {
        return Ok(Some(json))
    }
    println!("{:#?}", headers);
    println!("{}", status);
    let err: JsonError = JsonError::new("json is empty".to_string());
    Err(err.into())
}

pub fn select(array: &Vec<Value>, since: &i64) -> Vec<Value> {
    let res: Vec<Value> = array.iter().filter(|a| a["number"].as_i64().unwrap() > *since).cloned().collect();
    res
}
