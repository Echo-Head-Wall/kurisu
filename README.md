# Kurisu - A Builder-style Jira API Client for Rust

Kurisu is a modern, builder-style Jira API client for Rust with async/tokio support. It provides a convenient and type-safe way to interact with the Jira REST API.

## Features

- Builder pattern for easy configuration
- Async/tokio support for efficient I/O
- Type-safe API using Rust's strong type system
- Comprehensive error handling
- Support for different authentication methods (Basic Auth, Bearer Token)
- Modular design with separate API modules

## Installation

Add Kurisu to your `Cargo.toml`:

```toml
[dependencies]
kurisu = { git = "https://github.com/Echo-Head-Wall/kurisu.git" }
```

## Usage

Here's a simple example of using Kurisu to interact with the Jira API:

```rust
use kurisu::{JiraClient, BasicAuth, Error};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get Jira credentials from environment variables
    let jira_url = env::var("JIRA_URL").expect("JIRA_URL must be set");
    let jira_user = env::var("JIRA_USER").expect("JIRA_USER must be set");
    let jira_token = env::var("JIRA_TOKEN").expect("JIRA_TOKEN must be set");
    
    // Create a configured Jira client
    let client = JiraClient::builder()
        .base_url(jira_url)
        .auth(BasicAuth::new(jira_user, Some(jira_token)))
        .build()?;
    
    // Search for issues
    let search_results = client.search()
        .search_issues(
            Some("project = DEMO AND status = 'In Progress'"),
            Some(0),
            Some(10),
            None,
            None,
        ).await?;
    
    println!("Found {} issues", search_results.total.unwrap_or(0));
    
    if let Some(issues) = search_results.issues {
        for issue in issues {
            println!("Issue: {}", issue.key.unwrap_or_default());
        }
    }
    
    Ok(())
}
```

## API Modules

Kurisu provides several API modules to interact with different aspects of the Jira API:

- `IssuesApi`: Work with Jira issues (create, update, delete, search)
- `ProjectsApi`: Manage Jira projects
- `UsersApi`: Handle Jira users
- `DashboardApi`: Work with Jira dashboards
- `SearchApi`: Advanced search capabilities

Each module can be accessed through the main `JiraClient` instance:

```rust
// Access Issues API
let issues_api = client.issues();

// Access Projects API
let projects_api = client.projects();

// Access Users API
let users_api = client.users();

// Access Dashboard API
let dashboard_api = client.dashboard();

// Access Search API
let search_api = client.search();
```

## Authentication

Kurisu supports multiple authentication methods:

### Basic Auth

```rust
let client = JiraClient::builder()
    .base_url("https://your-instance.atlassian.net")
    .auth(BasicAuth::new("your-email@example.com", Some("your-api-token")))
    .build()?;
```

### Bearer Token

```rust
let client = JiraClient::builder()
    .base_url("https://your-instance.atlassian.net")
    .auth(BearerAuth::new("your-bearer-token"))
    .build()?;
```

### No Auth (for public instances)

```rust
let client = JiraClient::builder()
    .base_url("https://your-instance.atlassian.net")
    .auth(NoAuth)
    .build()?;
```

## Error Handling

Kurisu provides a comprehensive error type `kurisu::Error` that wraps various error types from dependencies:

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing base URL for Jira instance")]
    MissingBaseUrl,

    #[error("HTTP client error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Middleware client error: {0}")]
    ReqwestMiddlewareError(#[from] reqwest_middleware::Error),

    #[error("API request error: {0}")]
    ApiError(String),

    #[error("JSON parsing error: {0}")]
    SerdeError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("{0}")]
    Other(String),
}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.