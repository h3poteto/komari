use crate::errors::{JsonError, LinkError, PullsError};
use chrono::{DateTime, FixedOffset};
use github_rs::client::{Executor, Github};
use github_rs::HeaderMap;
use regex::Regex;
use serde_json::Value;
use std::env;

pub struct Pulls {
    owner: String,
    repo: String,
    client: Github,
}

impl Pulls {
    pub fn new(owner: &String, repo: &String) -> Result<Self, PullsError> {
        let github_token = env::var("GITHUB_TOKEN")?;
        let client = Github::new(github_token)?;
        Ok(Pulls {
            owner: owner.to_string(),
            repo: repo.to_string(),
            client: client,
        })
    }

    pub fn list_pulls_since(&self, since: &i64) -> Result<Vec<Value>, PullsError> {
        // https://github.com/github-rs/github-rs/blob/master/src/repos/get.rs#L265
        // Pulls does not provide reference parameter, so we can't provide query parameter.
        let pulls_url = format!(
            "repos/{}/{}/pulls?state=closed&sort=updated&direction=desc",
            self.owner, self.repo
        );
        self.list(&pulls_url, since)
    }

    fn list(&self, url: &String, since: &i64) -> Result<Vec<Value>, PullsError> {
        let response = self.client.get().custom_endpoint(&url).execute::<Value>()?;
        let (headers, status, json) = response;
        if !status.is_success() {
            let err: JsonError = JsonError::new(format!("Status code is {}", status.as_u16()));
            return Err(err.into());
        }

        if let Some(json) = json {
            if let Some(array) = json.as_array() {
                if self.include_target(array, since) {
                    return Ok(array.to_vec());
                } else {
                    let next_url = self.get_next_link(headers)?;
                    let mut child = self.list(&next_url, since)?;
                    let mut res = array.clone();
                    res.append(&mut child);
                    return Ok(res.to_vec());
                }
            }
        }
        let empty: Vec<Value> = vec![];
        Ok(empty)
    }

    fn include_target(&self, array: &Vec<Value>, since: &i64) -> bool {
        if let Some(_find) = array
            .iter()
            .find(|a| a["number"].as_i64().unwrap() == since.to_owned())
        {
            return true;
        }
        false
    }

    fn get_next_link(&self, headers: HeaderMap) -> Result<String, LinkError> {
        if let Ok(link) = headers
            .get::<&str>("link")
            .ok_or("link does not exist".to_owned())
            .and_then(|l| l.to_str().map_err(|e| e.to_string()))
            .and_then(|l| parse_link_header::parse(l).map_err(|_e| "failed to parse".to_owned()))
        {
            let next: Option<String> = Some("next".to_string());
            if let Some(value) = link.get(&next) {
                let uri = value.uri.to_string().clone();
                let re = Regex::new(r"^https://api\.github\.com/(.*)$").unwrap();
                return re
                    .captures(&uri)
                    .ok_or(LinkError::new("could not found url".to_string()))
                    .and_then(|r| {
                        r.get(1)
                            .ok_or(LinkError::new("failed to parse url".to_string()))
                    })
                    .map(|p| p.as_str().to_string());
            }
        }
        let err: LinkError = LinkError::new("link not found".to_string());
        Err(err)
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
        if let Ok(since_date) = since["merged_at"]
            .as_str()
            .ok_or("merged_at does not exist".to_owned())
            .and_then(|m| DateTime::parse_from_rfc3339(&m).map_err(|e| e.to_string()))
        {
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
            Ok(t) => since.lt(&t),
            Err(_e) => false,
        }
    }
}
