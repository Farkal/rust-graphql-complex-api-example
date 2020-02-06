use crate::schema::movies;
use crate::schema::movies::dsl::{id as col_id, movies as movies_table};
use crate::{AppResult, DbPool};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

#[derive(Debug, Clone, Queryable, Identifiable)]
pub struct Movie {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub pixels_box: Option<Vec<f64>>,
    pub path: Option<String>,
}

pub fn get_by_ids(pool: &DbPool, ids: &[i32]) -> AppResult<Vec<Movie>> {
    Ok(movies_table
        .filter(col_id.eq_any(ids))
        .get_results(&pool.get()?)?)
}
