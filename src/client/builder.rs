use crate::client::{JiraClient, Auth, NoAuth, Error};
use jira_openapi::apis::configuration::Configuration;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::Arc;
use anyhow::Result;

/// Builder for creating a configured JiraClient
pub struct JiraClientBuilder {
    base_url: Option<String>,
    auth: Box<dyn Auth>,
    timeout: Option<std::time::Duration>,
    user_agent: Option<String>,
}

impl Default for JiraClientBuilder {
    fn default() -> Self {
        Self {
            base_url: None,
            auth: Box::new(NoAuth),
            timeout: None,
            user_agent: Some(format!("kurisu-jira-client/{}", env!("CARGO_PKG_VERSION"))),
        }
    }
}

impl JiraClientBuilder {
    /// Create a new JiraClientBuilder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL for the Jira instance
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the authentication method to use
    pub fn auth(mut self, auth: impl Auth + 'static) -> Self {
        self.auth = Box::new(auth);
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set a custom user agent string
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Build the JiraClient with the configured settings
    pub fn build(self) -> Result<JiraClient, Error> {
        let base_url = self.base_url.ok_or(Error::MissingBaseUrl)?;

        // Build the reqwest client
        let mut reqwest_builder = Client::builder();
        
        if let Some(timeout) = self.timeout {
            reqwest_builder = reqwest_builder.timeout(timeout);
        }
        
        let reqwest_client = reqwest_builder.build()?;
        let client = ClientBuilder::new(reqwest_client).build();

        // Create the OpenAPI configuration
        let mut config = Configuration::new();
        config.base_path = base_url;
        config.user_agent = self.user_agent;
        
        // Apply authentication
        self.auth.apply_to_config(&mut config)?;
        
        // Create the client
        Ok(JiraClient {
            config,
            client,
        })
    }
}