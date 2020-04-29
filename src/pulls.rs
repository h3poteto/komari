use github_rs::client::{Executor, Github};
use serde_json::Value;
use github_rs::errors::Error;

pub fn list(owner: &String, repo: &String) -> Result<Option<Value>, Error> {
    let github_token = env!("GITHUB_TOKEN");
    let client = Github::new(github_token).unwrap();

    // https://github.com/github-rs/github-rs/blob/master/src/repos/get.rs#L265
    // Pulls does not provide reference parameter, so we can't provide query parameter.
    let pulls_url = format!("repos/{}/{}/pulls?state=closed&sort=updated&direction=desc", owner, repo);
    let response = client.get().custom_endpoint(&pulls_url).execute::<Value>();

    match response {
        Ok((headers, status, json)) => {
            if let Some(json) = json {
                return Ok(Some(json))
            }
            println!("{:#?}", headers);
            println!("{}", status);
            Ok(None)
        },
        Err(e) => Err(e),
    }
}

pub fn select(array: &Vec<Value>, since: &i64) -> Vec<Value> {
    let res: Vec<Value> = array.iter().filter(|a| a["number"].as_i64().unwrap() > *since).cloned().collect();
    res
}
