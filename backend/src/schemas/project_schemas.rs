use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct Video {
    pub id: String,
    pub platform: String,
    pub title: String,
    pub description: String,
    pub published_at: NaiveDateTime,
    pub thumbnail_url: String,
    pub rating: Option<f64>,
}

#[derive(InputObject)]
pub struct PaginatedVideosFilter {
    pub id: Option<String>,
    #[graphql(default = 25)]
    pub first: i64,
    #[graphql(default = 0)]
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PaginatedVideos {
    pub first: i64,
    pub offset: i64,
    pub total: i64,
    pub results: Vec<Video>,
}
