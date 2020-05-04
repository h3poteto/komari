use github_rs;
use std::env;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PullsError {
    GitHubError { error: github_rs::errors::Error },

    JsonError { error: JsonError },

    EnvError { error: env::VarError },

    LinkError { error: LinkError },
}

impl From<github_rs::errors::Error> for PullsError {
    fn from(error: github_rs::errors::Error) -> Self {
        PullsError::GitHubError { error }
    }
}

impl From<JsonError> for PullsError {
    fn from(err: JsonError) -> Self {
        PullsError::JsonError { error: err }
    }
}

impl From<env::VarError> for PullsError {
    fn from(error: env::VarError) -> Self {
        PullsError::EnvError { error }
    }
}

impl From<LinkError> for PullsError {
    fn from(err: LinkError) -> Self {
        PullsError::LinkError { error: err }
    }
}

impl fmt::Display for PullsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PullsError::GitHubError { error: err } => write!(f, "GitHub error: {}", err),
            PullsError::JsonError { error: err } => write!(f, "Json error: {}", err),
            PullsError::EnvError { error: err } => write!(f, "Env error: {}", err),
            PullsError::LinkError { error: err } => write!(f, "Link error: {}", err),
        }
    }
}

impl Error for PullsError {
    fn description(&self) -> &str {
        match self {
            PullsError::GitHubError { error: err } => err.description(),
            PullsError::JsonError { error: err } => err.description(),
            PullsError::EnvError { error: err } => err.description(),
            PullsError::LinkError { error: err } => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            PullsError::GitHubError { error: err } => Some(err),
            PullsError::JsonError { error: err } => Some(err),
            PullsError::EnvError { error: err } => Some(err),
            PullsError::LinkError { error: err } => Some(err),
        }
    }
}

#[derive(Debug)]
pub struct JsonError {
    error: String,
}

impl JsonError {
    pub fn new(err: String) -> JsonError {
        JsonError { error: err }
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for JsonError {
    fn description(&self) -> &str {
        &self.error
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

#[derive(Debug)]
pub struct LinkError {
    error: String,
}

impl LinkError {
    pub fn new(err: String) -> LinkError {
        LinkError { error: err }
    }
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for LinkError {
    fn description(&self) -> &str {
        &self.error
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
