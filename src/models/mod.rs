pub mod cinema;
pub mod movie;
pub mod movie_to_cinema;

pub use cinema::Cinema;
pub use movie::Movie;
pub use movie_to_cinema::MovieToCinema;

use crate::graphql::EExposedFormat;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql};
use diesel::sql_types::SmallInt;
use std::io::Write;

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "SmallInt"]
pub enum ExposedFormat {
    FULLHD = 1,
    F4K = 2,
    F3D = 3,
    F4D = 4,
}

impl From<EExposedFormat> for ExposedFormat {
    fn from(p: EExposedFormat) -> Self {
        match p {
            EExposedFormat::Fullhd => ExposedFormat::FULLHD,
            EExposedFormat::F4k => ExposedFormat::F4K,
            EExposedFormat::F3d => ExposedFormat::F3D,
            EExposedFormat::F4d => ExposedFormat::F4D,
        }
    }
}

impl<DB> ToSql<SmallInt, DB> for ExposedFormat
where
    DB: Backend,
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<'_, W, DB>) -> serialize::Result {
        (*self as i16).to_sql(out)
    }
}

impl<DB> FromSql<SmallInt, DB> for ExposedFormat
where
    DB: Backend,
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let value = i16::from_sql(bytes)?;
        Ok(match value {
            1 => ExposedFormat::FULLHD,
            2 => ExposedFormat::F4K,
            3 => ExposedFormat::F3D,
            4 => ExposedFormat::F4D,
            _ => unreachable!(),
        })
    }
}
