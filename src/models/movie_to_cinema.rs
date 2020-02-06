use super::ExposedFormat;
use crate::schema::cinemas_movies;
use crate::schema::cinemas_movies::dsl::{
    cinema_id as col_cinema_id, cinemas_movies as cinemas_movies_table,
};
use crate::{AppResult, DbPool, Error};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

#[derive(Clone, Debug, Queryable, Identifiable)]
#[table_name = "cinemas_movies"]
pub struct MovieToCinema {
    pub id: i32,
    pub exposed_format: Option<ExposedFormat>,
    pub pixels_box: Option<Vec<f64>>,
    pub cinema_id: i32,
    pub movie_id: i32,
}

pub fn get_by_cinemas_ids(pool: &DbPool, cinema_ids: &[i32]) -> AppResult<Vec<MovieToCinema>> {
    Ok(cinemas_movies_table
        .filter(col_cinema_id.eq_any(cinema_ids))
        .get_results(&pool.get()?)?)
}
