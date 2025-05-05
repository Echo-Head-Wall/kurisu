use anyhow::Result;
use jira_openapi::apis::configuration::{Configuration, BasicAuth as OpenApiBasicAuth};
use crate::client::Error;

/// Authentication trait for the Jira client
pub trait Auth: Send + Sync {
    /// Apply authentication to the OpenAPI configuration
    fn apply_to_config(&self, config: &mut Configuration) -> Result<(), Error>;
}

/// No authentication
#[derive(Debug, Clone)]
pub struct NoAuth;

impl Auth for NoAuth {
    fn apply_to_config(&self, _config: &mut Configuration) -> Result<(), Error> {
        // No authentication to apply
        Ok(())
    }
}

/// Basic authentication
#[derive(Debug, Clone)]
pub struct BasicAuth {
    username: String,
    password: Option<String>,
}

impl BasicAuth {
    /// Create a new BasicAuth with username and password
    pub fn new(username: impl Into<String>, password: Option<impl Into<String>>) -> Self {
        Self {
            username: username.into(),
            password: password.map(Into::into),
        }
    }
}

impl Auth for BasicAuth {
    fn apply_to_config(&self, config: &mut Configuration) -> Result<(), Error> {
        config.basic_auth = Some((self.username.clone(), self.password.clone()));
        Ok(())
    }
}

/// Bearer token authentication
#[derive(Debug, Clone)]
pub struct BearerAuth {
    token: String,
}

impl BearerAuth {
    /// Create a new BearerAuth with the given token
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

impl Auth for BearerAuth {
    fn apply_to_config(&self, config: &mut Configuration) -> Result<(), Error> {
        config.bearer_access_token = Some(self.token.clone());
        Ok(())
    }
}