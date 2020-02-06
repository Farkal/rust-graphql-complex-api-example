use super::{
    query_trails::*, CinemaFields, Context, Movie, MovieSearchCriterias, Order, Pagination,
    PlannedMovie, QueryTrail, Walked,
};
use crate::models::{self, movie, movie_to_cinema};
use crate::{AppResult, Error};
use juniper::{Executor, FieldResult, ID};
use juniper_eager_loading::{
    prelude::*, EagerLoading, HasMany, HasManyThrough, LoadChildrenOutput, LoadFrom,
};

#[derive(Clone, EagerLoading)]
#[eager_loading(context= Context, error = Error)]
pub struct Cinema {
    cinema: models::Cinema,

    #[has_many_through(join_model = models::MovieToCinema)]
    movies: HasManyThrough<PlannedMovie>,
}

impl CinemaFields for Cinema {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<ID> {
        Ok(self.cinema.id.to_string().into())
    }

    fn field_name(&self, executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.cinema.name)
    }

    fn field_movies(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, PlannedMovie, Walked>,
        criterias: Option<MovieSearchCriterias>,
        pagination: Pagination,
        order: Option<Order>,
    ) -> FieldResult<&Vec<PlannedMovie>> {
        self.movies.try_unwrap().map_err(From::from)
    }
}

impl juniper_eager_loading::LoadFrom<models::Cinema> for models::MovieToCinema {
    type Error = Error;
    type Context = Context;

    fn load(
        cinemas: &[models::Cinema],
        _field_args: &(),
        ctx: &Self::Context,
    ) -> AppResult<Vec<Self>> {
        let cinema_ids = cinemas.iter().map(|e| e.id).collect::<Vec<_>>();

        movie_to_cinema::get_by_cinemas_ids(&ctx.db, &cinema_ids)
    }
}

impl juniper_eager_loading::LoadFrom<models::MovieToCinema> for models::Movie {
    type Error = Error;
    type Context = Context;

    fn load(
        movies_to_cinemas: &[models::MovieToCinema],
        _field_args: &(),
        ctx: &Self::Context,
    ) -> AppResult<Vec<Self>> {
        let movies_ids = movies_to_cinemas
            .iter()
            .map(|movie_to_cinema| movie_to_cinema.movie_id)
            .collect::<Vec<_>>();

        movie::get_by_ids(&ctx.db, &movies_ids)
    }
}

// struct EagerLoadingContextMovieToCinema;

// impl<'a>
//     EagerLoadChildrenOfType<
//         'a,
//         PlannedMovie,
//         EagerLoadingContextMovieToCinema,
//         models::MovieToCinema,
//     > for Cinema
// {
//     type FieldArguments = ();

//     fn load_children(
//         models: &[Self::Model],
//         field_args: &Self::FieldArguments,
//         ctx: &Self::Context,
//     ) -> Result<LoadChildrenOutput<PlannedMovie, models::MovieToCinema>, Self::Error> {
//         let join_models: Vec<models::MovieToCinema> = LoadFrom::load(&models, field_args, ctx)?;
//         let child_models: Vec<models::Movie> = LoadFrom::load(&join_models, field_args, ctx)?;

//         let mut child_and_join_model_pairs = Vec::new();

//         // // WANT TO CONVERT MOVIE TO PLANNED MOVIE BY MERGING WITH MOVIETOCINEMA HERE BUT CAN'T BECAUSE RETURN TYPE NEED TO IMPL EAGERLOADING

//         Ok(LoadChildrenOutput::ChildAndJoinModels(
//             child_and_join_model_pairs,
//         ))
//     }

//     fn is_child_of(
//         node: &Self,
//         child: &Movie,
//         join_model: &models::MovieToCinema,
//         _field_args: &Self::FieldArguments,
//         _ctx: &Self::Context,
//     ) -> bool {
//         node.cinema.id == join_model.cinema_id && join_model.movie_id == child.movie.id
//     }

//     fn association(node: &mut Self) -> &mut dyn Association<Movie> {
//         &mut node.movies
//     }
// }
