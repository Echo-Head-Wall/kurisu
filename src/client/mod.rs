mod error;
mod auth;
mod builder;

pub use error::Error;
pub use auth::{Auth, BasicAuth, BearerAuth, NoAuth};
pub use builder::JiraClientBuilder;

use std::sync::Arc;
use reqwest_middleware::ClientWithMiddleware;
use jira_openapi::apis::configuration::Configuration;

/// The main Jira client struct that provides access to all Jira APIs
#[derive(Clone)]
pub struct JiraClient {
    config: Configuration,
    client: ClientWithMiddleware,
}

impl JiraClient {
    /// Create a new instance of the JiraClient builder
    pub fn builder() -> JiraClientBuilder {
        JiraClientBuilder::new()
    }

    /// Get the underlying OpenAPI configuration
    pub fn config(&self) -> &Configuration {
        &self.config
    }

    /// Get the underlying HTTP client
    pub fn http_client(&self) -> &ClientWithMiddleware {
        &self.client
    }
}

// API modules
mod issues;
mod projects;
mod users;
mod dashboard;
mod search;

// Re-export the API modules
pub use issues::IssuesApi;
pub use projects::ProjectsApi;
pub use users::UsersApi;
pub use dashboard::DashboardApi;
pub use search::SearchApi;

// Implementation of the API modules for JiraClient
impl JiraClient {
    /// Access the Issues API
    pub fn issues(&self) -> IssuesApi {
        IssuesApi::new(self.clone())
    }

    /// Access the Projects API
    pub fn projects(&self) -> ProjectsApi {
        ProjectsApi::new(self.clone())
    }

    /// Access the Users API
    pub fn users(&self) -> UsersApi {
        UsersApi::new(self.clone())
    }

    /// Access the Dashboard API
    pub fn dashboard(&self) -> DashboardApi {
        DashboardApi::new(self.clone())
    }

    /// Access the Search API
    pub fn search(&self) -> SearchApi {
        SearchApi::new(self.clone())
    }
}