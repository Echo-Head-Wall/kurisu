mod client;

use client::{JiraClient, BasicAuth, Error};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // A simple example of using the Jira client
    println!("Jira API Client Example");
    
    // Typically, you would get these from environment variables
    let jira_url = env::var("JIRA_URL").unwrap_or_else(|_| "https://your-jira-instance.atlassian.net".to_string());
    let jira_user = env::var("JIRA_USER").unwrap_or_else(|_| "your-username".to_string());
    let jira_token = env::var("JIRA_TOKEN").unwrap_or_else(|_| "your-api-token".to_string());
    
    // Create the Jira client
    let client = JiraClient::builder()
        .base_url(jira_url)
        .auth(BasicAuth::new(jira_user, Some(jira_token)))
        .build()?;
    
    // Example 1: Search for issues
    if let Ok(search_results) = client.search()
        .search_issues(
            Some("project = DEMO AND status = 'In Progress'"),
            Some(0),
            Some(10),
            None,
            None,
        ).await 
    {
        println!("Found {} issues", search_results.total.unwrap_or(0));
        if let Some(issues) = search_results.issues {
            for issue in issues {
                if let Some(key) = issue.key {
                    println!("Issue: {}", key);
                }
            }
        }
    }
    
    // Example 2: Get project information
    if let Ok(projects) = client.projects().get_all_projects(None, None, None).await {
        println!("Found {} projects", projects.len());
        for project in projects {
            if let Some(key) = project.key {
                println!("Project: {} ({})", key, project.name.unwrap_or_default());
            }
        }
    }
    
    // Example 3: Get user information
    if let Ok(all_users) = client.users().get_all_users(Some(0), Some(10)).await {
        println!("Found {} users", all_users.len());
        for user in all_users {
            println!("User: {} ({})", 
                user.display_name.unwrap_or_default(), 
                user.email_address.unwrap_or_default());
        }
    }
    
    Ok(())
}