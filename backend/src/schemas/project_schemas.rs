use diesel::prelude::*;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Video {
    pub id: String,
    pub platform: String,
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,
    pub published_at: String,
    pub rating: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PaginatedVideos {
    pub first: usize,
    pub offset: usize,
    pub total: usize,
    pub results: Vec<Video>,
}
