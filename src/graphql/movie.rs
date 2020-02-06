use super::{Context, MovieFields};
use crate::models;
use crate::{AppResult, Error};
use juniper::{Executor, FieldResult, ID};
use juniper_eager_loading::{prelude::*, EagerLoading, HasOne};

#[derive(Clone, EagerLoading)]
#[eager_loading(context= Context, error = Error)]
pub struct Movie {
    pub movie: models::Movie,
}

impl MovieFields for Movie {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<ID> {
        Ok(self.movie.id.to_string().into())
    }

    fn field_name(&self, executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.movie.name)
    }
    fn field_description(&self, executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.movie.description)
    }
    fn field_pixels_box(&self, executor: &Executor<'_, Context>) -> FieldResult<&Option<Vec<f64>>> {
        Ok(&self.movie.pixels_box)
    }
    fn field_path(&self, executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.movie.path)
    }
}
