use axum::http::StatusCode;
use diesel::{
    ExpressionMethods, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    models::{
        activation_link::{ActivationLink, NewActivationLink},
        response::error::Error,
    },
    schema::activation_links::{self, id},
};

/// Inserts a new activation link into the database
///
/// # Arguments
/// * `new_activation_link` - The activation link record to insert
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(usize)` - Number of rows inserted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn insert(
    new_activation_link: NewActivationLink,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<usize, Error> {
    diesel::insert_into(activation_links::table)
        .values(new_activation_link)
        .execute(conn)
        .await
        .map_err(|e| e.into())
}

/// Search for an activation link by id return it and delete from db
///
/// # Arguments
/// * `find_id` - The id to find
/// * `conn` - Database connection from the pool
///
/// # Returns
/// * `Ok(User)` - The `ActivationLink` object from the database
/// * `Err(Error)` - Database operation error
pub async fn filter_and_delete_by_id(
    find_id: Uuid,
    conn: &mut deadpool::managed::Object<
        diesel_async::pooled_connection::AsyncDieselConnectionManager<
            diesel_async::AsyncPgConnection,
        >,
    >,
) -> Result<ActivationLink, Error> {
    let link = activation_links::table
        .filter(id.eq(find_id))
        .select(ActivationLink::as_select())
        .first(conn)
        .await
        .map_err(|e: diesel::result::Error| -> Error {
            match e {
                diesel::result::Error::NotFound => {
                    (StatusCode::BAD_REQUEST, "Activation link not found.").into()
                }
                _ => e.into(),
            }
        })?;

    diesel::delete(activation_links::table)
        .filter(id.eq(find_id))
        .execute(conn)
        .await?;
    Ok(link)
}
