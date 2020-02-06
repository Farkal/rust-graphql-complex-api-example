use crate::models;
use crate::DbPool;
use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema_from_file;

mod cinema;
mod movie;
mod movie_to_cinema;

pub use cinema::Cinema;
pub use movie::Movie;
pub use movie_to_cinema::PlannedMovie;

graphql_schema_from_file!("src/graphql/schema.graphql");

pub struct Context {
    db: DbPool,
}

impl juniper::Context for Context {}

pub struct Query;

impl QueryFields for Query {
    fn field_cinemas(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Cinema, Walked>,
        criterias: Option<CinemaSearchCriterias>,
        pagination: Pagination,
        order: Option<Order>,
    ) -> FieldResult<&Vec<Cinema>> {
        let ctx = executor.context();
        let cinema_models = models::cinema::get_all(&ctx.db, criterias, pagination, order)?;
        let mut cinema = Cinema::from_db_models(&cinema_models);
        Cinema::eager_load_all_children_for_each(&mut cinema, &cinema_models, ctx, trail)?;
        Ok(&cinema)
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_noop(&self, _executor: &Executor<'_, Context>) -> FieldResult<&bool> {
        Ok(&true)
    }
}

impl Pagination {
    pub fn new(page: i32, per_page: i32) -> Self {
        Pagination {
            page: Some(page),
            per_page: Some(per_page),
        }
    }
}
