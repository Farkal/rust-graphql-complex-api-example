use super::Movie;
use super::{Context, EExposedFormat};
use crate::models;
use crate::Error;
use juniper_eager_loading::{prelude::*, EagerLoading, HasOne};

#[derive(Clone)]
pub struct PlannedMovie {
    id: i32,
    exposed_format: EExposedFormat,
    pixels_box: Vec<f64>,
    movie: Movie,
}
