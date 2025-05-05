use async_trait::async_trait;
use jira_openapi::{
    apis::users_api,
    models::{User, PageBeanUser, NewUserDetails, UserDetails},
};
use crate::client::{JiraClient, Error};

/// Users API interface
#[async_trait]
pub trait UsersApiTrait {
    /// Get all users
    async fn get_all_users(&self, start_at: Option<i32>, max_results: Option<i32>) -> Result<Vec<User>, Error>;
    
    /// Get users paginated
    async fn get_users_paginated(&self, start_at: Option<i64>, max_results: Option<i32>) -> Result<PageBeanUser, Error>;
    
    /// Get user by account ID
    async fn get_user(&self, account_id: &str, expand: Option<Vec<String>>) -> Result<User, Error>;
    
    /// Create user
    async fn create_user(&self, user_details: &NewUserDetails) -> Result<User, Error>;
    
    /// Update user
    async fn update_user(&self, account_id: &str, user_details: &UserDetails) -> Result<User, Error>;
    
    /// Delete user
    async fn delete_user(&self, account_id: &str) -> Result<(), Error>;
    
    /// Find users
    async fn find_users(&self, query: &str, max_results: Option<i32>) -> Result<Vec<User>, Error>;
}

/// Users API implementation
#[derive(Clone)]
pub struct UsersApi {
    client: JiraClient,
}

impl UsersApi {
    /// Create a new UsersApi instance
    pub fn new(client: JiraClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl UsersApiTrait for UsersApi {
    async fn get_all_users(&self, start_at: Option<i32>, max_results: Option<i32>) -> Result<Vec<User>, Error> {
        let result = users_api::get_all_users(
            self.client.config(),
            start_at,
            max_results,
        ).await?;
        
        Ok(result)
    }
    
    async fn get_users_paginated(&self, start_at: Option<i64>, max_results: Option<i32>) -> Result<PageBeanUser, Error> {
        let result = users_api::search_users(
            self.client.config(),
            start_at,
            max_results,
            None,
        ).await?;
        
        Ok(result)
    }
    
    async fn get_user(&self, account_id: &str, expand: Option<Vec<String>>) -> Result<User, Error> {
        let result = users_api::get_user(
            self.client.config(),
            account_id,
            expand,
        ).await?;
        
        Ok(result)
    }
    
    async fn create_user(&self, user_details: &NewUserDetails) -> Result<User, Error> {
        let result = users_api::create_user(
            self.client.config(),
            user_details.clone(),
        ).await?;
        
        Ok(result)
    }
    
    async fn update_user(&self, account_id: &str, user_details: &UserDetails) -> Result<User, Error> {
        let result = users_api::update_user(
            self.client.config(),
            account_id,
            user_details.clone(),
        ).await?;
        
        Ok(result)
    }
    
    async fn delete_user(&self, account_id: &str) -> Result<(), Error> {
        users_api::delete_user(
            self.client.config(),
            account_id,
        ).await?;
        
        Ok(())
    }
    
    async fn find_users(&self, query: &str, max_results: Option<i32>) -> Result<Vec<User>, Error> {
        let result = users_api::find_users(
            self.client.config(),
            Some(query.to_string()),
            max_results,
            None,
            None,
            None,
            None,
        ).await?;
        
        Ok(result)
    }
}