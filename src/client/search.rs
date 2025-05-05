use async_trait::async_trait;
use jira_openapi::{
    apis::{
        issue_search_api,
        jql_api,
    },
    models::{SearchResults, SearchRequestBean, ParsedJqlQueries, JqlQueriesToParse},
};
use crate::client::{JiraClient, Error};

/// Search API interface
#[async_trait]
pub trait SearchApiTrait {
    /// Search for issues using JQL
    async fn search_issues(
        &self, 
        jql: Option<&str>, 
        start_at: Option<i32>, 
        max_results: Option<i32>,
        fields: Option<Vec<String>>,
        expand: Option<Vec<String>>,
    ) -> Result<SearchResults, Error>;
    
    /// Search for issues with a more detailed search request
    async fn search_issues_with_request(&self, search_request: &SearchRequestBean) -> Result<SearchResults, Error>;
    
    /// Parse JQL queries
    async fn parse_jql_queries(&self, queries: Vec<String>, validation: Option<String>) -> Result<ParsedJqlQueries, Error>;
}

/// Search API implementation
#[derive(Clone)]
pub struct SearchApi {
    client: JiraClient,
}

impl SearchApi {
    /// Create a new SearchApi instance
    pub fn new(client: JiraClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SearchApiTrait for SearchApi {
    async fn search_issues(
        &self, 
        jql: Option<&str>, 
        start_at: Option<i32>, 
        max_results: Option<i32>,
        fields: Option<Vec<String>>,
        expand: Option<Vec<String>>,
    ) -> Result<SearchResults, Error> {
        let result = issue_search_api::search_for_issues_using_jql(
            self.client.config(),
            jql.map(|s| s.to_string()),
            start_at,
            max_results,
            None,
            fields,
            expand,
        ).await?;
        
        Ok(result)
    }
    
    async fn search_issues_with_request(&self, search_request: &SearchRequestBean) -> Result<SearchResults, Error> {
        let result = issue_search_api::search_for_issues_using_jql_post(
            self.client.config(),
            search_request.clone(),
        ).await?;
        
        Ok(result)
    }
    
    async fn parse_jql_queries(&self, queries: Vec<String>, validation: Option<String>) -> Result<ParsedJqlQueries, Error> {
        let result = jql_api::parse_jql_queries(
            self.client.config(),
            JqlQueriesToParse {
                queries,
                validation,
            },
        ).await?;
        
        Ok(result)
    }
}