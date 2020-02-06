use crate::db::Paginate;
use crate::diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl};
use crate::graphql::{CinemaSearchCriterias, Order, Pagination};
use crate::schema::cinemas;
use crate::schema::cinemas::dsl::{cinemas as cinemas_table, id as col_id, name as col_name};
use crate::{AppResult, DbPool, Error};

const PAGINATION_DEFAULT: i32 = 50;
const PAGINATION_MAX: i32 = 100;

#[derive(Debug, Clone, Queryable, AsChangeset, Identifiable)]
pub struct Cinema {
    pub id: i32,
    pub name: String,
}

pub fn get_all(
    pool: &DbPool,
    criterias: Option<CinemaSearchCriterias>,
    mut pagination: Pagination,
    order: Option<Order>,
) -> AppResult<Vec<Cinema>> {
    let mut request = cinemas_table.into_boxed();
    if let Some(criterias) = criterias {
        if let Some(name) = criterias.name {
            request = request.filter(col_name.eq(name));
        }
        if let Some(name) = criterias.name_contains {
            request = request.filter(col_name.ilike(format!("%{}%", name)));
        }
    }
    // let p = pagination.unwrap_or(Pagination::new(1, PAGINATION_DEFAULT));
    let p = pagination;
    let mut query = request.paginate(p.page.unwrap_or(1).into());
    if let Some(size) = p.per_page {
        query = query.per_page(size.into());
    };
    let (cinemas, _total_pages) = query.load_and_count_pages(&pool.get()?)?;
    Ok(cinemas)
}
