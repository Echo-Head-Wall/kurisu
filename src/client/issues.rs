use std::sync::Arc;
use async_trait::async_trait;
use jira_openapi::{
    apis::issues_api,
    models::{IssueBean, IssueCreateMetadata, IssueUpdateDetails, CreatedIssues, IssuesUpdateBean, BulkChangelogRequestBean, BulkChangelogResponseBean},
};
use crate::client::{JiraClient, Error};

/// Issues API interface
#[async_trait]
pub trait IssuesApiTrait {
    /// Get issue by ID or key
    async fn get_issue(&self, issue_id_or_key: &str, fields: Option<Vec<String>>, expand: Option<Vec<String>>) -> Result<IssueBean, Error>;
    
    /// Create a new issue
    async fn create_issue(&self, issue: &IssueUpdateDetails) -> Result<CreatedIssues, Error>;
    
    /// Update an issue
    async fn update_issue(&self, issue_id_or_key: &str, issue: &IssueUpdateDetails) -> Result<(), Error>;
    
    /// Delete an issue
    async fn delete_issue(&self, issue_id_or_key: &str) -> Result<(), Error>;
    
    /// Get issue metadata for creating issues
    async fn get_create_issue_metadata(&self, project_ids: Option<Vec<String>>, project_keys: Option<Vec<String>>, issue_type_ids: Option<Vec<String>>, issue_type_names: Option<Vec<String>>, expand: Option<Vec<String>>) -> Result<IssueCreateMetadata, Error>;
    
    /// Get bulk changelogs for multiple issues
    async fn get_bulk_changelogs(&self, request: &BulkChangelogRequestBean) -> Result<BulkChangelogResponseBean, Error>;
}

/// Issues API implementation
#[derive(Clone)]
pub struct IssuesApi {
    client: JiraClient,
}

impl IssuesApi {
    /// Create a new IssuesApi instance
    pub fn new(client: JiraClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl IssuesApiTrait for IssuesApi {
    async fn get_issue(&self, issue_id_or_key: &str, fields: Option<Vec<String>>, expand: Option<Vec<String>>) -> Result<IssueBean, Error> {
        let result = issues_api::get_issue(
            self.client.config(),
            issue_id_or_key,
            fields,
            expand,
            None,
            None,
            None,
        ).await?;
        
        Ok(result)
    }
    
    async fn create_issue(&self, issue: &IssueUpdateDetails) -> Result<CreatedIssues, Error> {
        let result = issues_api::create_issues(
            self.client.config(),
            Some(IssuesUpdateBean {
                issue_updates: Some(vec![issue.clone()]),
                update_history: None,
            }),
        ).await?;
        
        Ok(result)
    }
    
    async fn update_issue(&self, issue_id_or_key: &str, issue: &IssueUpdateDetails) -> Result<(), Error> {
        issues_api::update_issue(
            self.client.config(),
            issue_id_or_key,
            Some(issue.clone()),
            None,
            None,
            None,
            None,
        ).await?;
        
        Ok(())
    }
    
    async fn delete_issue(&self, issue_id_or_key: &str) -> Result<(), Error> {
        issues_api::delete_issue(
            self.client.config(),
            issue_id_or_key,
            None,
        ).await?;
        
        Ok(())
    }
    
    async fn get_create_issue_metadata(&self, project_ids: Option<Vec<String>>, project_keys: Option<Vec<String>>, issue_type_ids: Option<Vec<String>>, issue_type_names: Option<Vec<String>>, expand: Option<Vec<String>>) -> Result<IssueCreateMetadata, Error> {
        let result = issues_api::get_create_issue_meta(
            self.client.config(),
            project_ids,
            project_keys,
            issue_type_ids,
            issue_type_names,
            expand,
        ).await?;
        
        Ok(result)
    }
    
    async fn get_bulk_changelogs(&self, request: &BulkChangelogRequestBean) -> Result<BulkChangelogResponseBean, Error> {
        let result = issues_api::get_bulk_changelogs(
            self.client.config(),
            request.clone(),
        ).await?;
        
        Ok(result)
    }
}