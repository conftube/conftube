use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use diesel::dsl::count;
use diesel::prelude::*;
use diesel::result::Error;
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
    pub published_at: DateTime<Utc>,
    pub thumbnail_url: String,
    pub rating: Option<f64>,
}

impl Video {
    pub fn paginated(
        filter: PaginatedVideosFilter,
        conn: &mut PgConnection,
    ) -> Result<PaginatedVideos, Error> {
        use crate::db_schema::videos::dsl::videos;
        use crate::db_schema::videos::*;

        let mut total_query = videos.into_boxed();
        if let Some(id_filter) = filter.id.clone() {
            total_query = total_query.filter(id.eq(id_filter));
        }

        let total = total_query
            .select(count(id))
            .first::<i64>(conn)
            .unwrap_or(0);

        let mut data_query = videos.into_boxed();
        if let Some(id_filter) = filter.id {
            data_query = data_query.filter(id.eq(id_filter));
        }

        let loaded_videos = data_query
            .offset(filter.offset)
            .limit(filter.first)
            .load::<Video>(conn)?;

        Ok(PaginatedVideos {
            first: filter.first,
            offset: filter.offset,
            total,
            results: loaded_videos,
        })
    }
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
