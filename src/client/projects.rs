use std::sync::Arc;
use async_trait::async_trait;
use jira_openapi::{
    apis::projects_api,
    models::{Project, PageBeanProject, ProjectDetails},
};
use crate::client::{JiraClient, Error};

/// Projects API interface
#[async_trait]
pub trait ProjectsApiTrait {
    /// Get all projects
    async fn get_all_projects(&self, expand: Option<Vec<String>>, recent: Option<i32>, properties: Option<Vec<String>>) -> Result<Vec<Project>, Error>;
    
    /// Get projects paginated
    async fn get_projects_paginated(&self, start_at: Option<i64>, max_results: Option<i32>, expand: Option<Vec<String>>, properties: Option<Vec<String>>) -> Result<PageBeanProject, Error>;
    
    /// Get project by ID or key
    async fn get_project(&self, project_id_or_key: &str, expand: Option<Vec<String>>, properties: Option<Vec<String>>) -> Result<Project, Error>;
    
    /// Create project
    async fn create_project(&self, project_details: &ProjectDetails) -> Result<ProjectDetails, Error>;
    
    /// Update project
    async fn update_project(&self, project_id_or_key: &str, project_details: &ProjectDetails) -> Result<Project, Error>;
    
    /// Delete project
    async fn delete_project(&self, project_id_or_key: &str) -> Result<(), Error>;
}

/// Projects API implementation
#[derive(Clone)]
pub struct ProjectsApi {
    client: JiraClient,
}

impl ProjectsApi {
    /// Create a new ProjectsApi instance
    pub fn new(client: JiraClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ProjectsApiTrait for ProjectsApi {
    async fn get_all_projects(&self, expand: Option<Vec<String>>, recent: Option<i32>, properties: Option<Vec<String>>) -> Result<Vec<Project>, Error> {
        let result = projects_api::get_all_projects(
            self.client.config(),
            expand,
            recent,
            properties,
            None,
            None,
            None,
            None,
        ).await?;
        
        Ok(result)
    }
    
    async fn get_projects_paginated(&self, start_at: Option<i64>, max_results: Option<i32>, expand: Option<Vec<String>>, properties: Option<Vec<String>>) -> Result<PageBeanProject, Error> {
        let result = projects_api::search_projects(
            self.client.config(),
            None,
            None,
            None,
            None,
            start_at,
            max_results,
            expand,
            None,
            properties,
            None,
        ).await?;
        
        Ok(result)
    }
    
    async fn get_project(&self, project_id_or_key: &str, expand: Option<Vec<String>>, properties: Option<Vec<String>>) -> Result<Project, Error> {
        let result = projects_api::get_project(
            self.client.config(),
            project_id_or_key,
            expand,
            properties,
        ).await?;
        
        Ok(result)
    }
    
    async fn create_project(&self, project_details: &ProjectDetails) -> Result<ProjectDetails, Error> {
        let result = projects_api::create_project(
            self.client.config(),
            project_details.clone(),
        ).await?;
        
        Ok(result)
    }
    
    async fn update_project(&self, project_id_or_key: &str, project_details: &ProjectDetails) -> Result<Project, Error> {
        let result = projects_api::update_project(
            self.client.config(),
            project_id_or_key,
            project_details.clone(),
            None,
        ).await?;
        
        Ok(result)
    }
    
    async fn delete_project(&self, project_id_or_key: &str) -> Result<(), Error> {
        projects_api::delete_project(
            self.client.config(),
            project_id_or_key,
            None,
        ).await?;
        
        Ok(())
    }
}