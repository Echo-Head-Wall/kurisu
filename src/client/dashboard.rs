use async_trait::async_trait;
use jira_openapi::{
    apis::dashboards_api,
    models::{Dashboard, PageOfDashboards, DashboardDetails},
};
use crate::client::{JiraClient, Error};

/// Dashboard API interface
#[async_trait]
pub trait DashboardApiTrait {
    /// Get all dashboards
    async fn get_all_dashboards(&self, filter: Option<&str>, start_at: Option<i64>, max_results: Option<i32>) -> Result<PageOfDashboards, Error>;
    
    /// Get dashboard by ID
    async fn get_dashboard(&self, id: &str) -> Result<Dashboard, Error>;
    
    /// Create dashboard
    async fn create_dashboard(&self, dashboard_details: &DashboardDetails) -> Result<Dashboard, Error>;
    
    /// Update dashboard
    async fn update_dashboard(&self, id: &str, dashboard_details: &DashboardDetails) -> Result<Dashboard, Error>;
    
    /// Delete dashboard
    async fn delete_dashboard(&self, id: &str) -> Result<(), Error>;
}

/// Dashboard API implementation
#[derive(Clone)]
pub struct DashboardApi {
    client: JiraClient,
}

impl DashboardApi {
    /// Create a new DashboardApi instance
    pub fn new(client: JiraClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl DashboardApiTrait for DashboardApi {
    async fn get_all_dashboards(&self, filter: Option<&str>, start_at: Option<i64>, max_results: Option<i32>) -> Result<PageOfDashboards, Error> {
        let result = dashboards_api::get_all_dashboards(
            self.client.config(),
            filter.map(|s| s.to_string()),
            start_at,
            max_results,
        ).await?;
        
        Ok(result)
    }
    
    async fn get_dashboard(&self, id: &str) -> Result<Dashboard, Error> {
        let result = dashboards_api::get_dashboard(
            self.client.config(),
            id,
        ).await?;
        
        Ok(result)
    }
    
    async fn create_dashboard(&self, dashboard_details: &DashboardDetails) -> Result<Dashboard, Error> {
        let result = dashboards_api::create_dashboard(
            self.client.config(),
            dashboard_details.clone(),
        ).await?;
        
        Ok(result)
    }
    
    async fn update_dashboard(&self, id: &str, dashboard_details: &DashboardDetails) -> Result<Dashboard, Error> {
        let result = dashboards_api::update_dashboard(
            self.client.config(),
            id,
            dashboard_details.clone(),
        ).await?;
        
        Ok(result)
    }
    
    async fn delete_dashboard(&self, id: &str) -> Result<(), Error> {
        dashboards_api::delete_dashboard(
            self.client.config(),
            id,
        ).await?;
        
        Ok(())
    }
}