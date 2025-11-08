use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    custom_category::{CreateCustomCategory, CustomCategory, UpdateCustomCategory},
    response::Error,
};

/// Finds all custom categories for a specific user
///
/// # Arguments
///
/// * `user_id` - The UUID of the user whose custom categories to retrieve
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Vec<CustomCategory>)` - The user's custom categories
/// * `Err(Error)` - Database operation error
pub async fn find_all_by_user(
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<CustomCategory>, Error> {
    let categories = sqlx::query_as::<_, CustomCategory>(
        r#"
        SELECT id, user_id, name, created_at, updated_at
        FROM custom_categories
        WHERE user_id = $1
        ORDER BY name ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

/// Finds a specific custom category by ID
///
/// # Arguments
///
/// * `category_id` - The UUID of the custom category to retrieve
/// * `user_id` - The UUID of the user (for authorization)
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(CustomCategory)` - The custom category
/// * `Err(Error)` - Database operation error or category not found
pub async fn find_by_id(
    category_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<CustomCategory, Error> {
    let category = sqlx::query_as::<_, CustomCategory>(
        r#"
        SELECT id, user_id, name, created_at, updated_at
        FROM custom_categories
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(category_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

/// Creates a new custom category for a user
///
/// # Arguments
///
/// * `user_id` - The UUID of the user creating the custom category
/// * `create_category` - The custom category creation data
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(CustomCategory)` - The newly created custom category
/// * `Err(Error)` - Database operation error (e.g., duplicate name)
pub async fn create(
    user_id: Uuid,
    create_category: CreateCustomCategory,
    pool: &PgPool,
) -> Result<CustomCategory, Error> {
    let category = sqlx::query_as::<_, CustomCategory>(
        r#"
        INSERT INTO custom_categories (user_id, name)
        VALUES ($1, $2)
        RETURNING id, user_id, name, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(create_category.name)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

/// Updates a custom category
///
/// # Arguments
///
/// * `category_id` - The UUID of the custom category to update
/// * `user_id` - The UUID of the user (for authorization)
/// * `update_category` - The custom category update data
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(CustomCategory)` - The updated custom category
/// * `Err(Error)` - Database operation error
pub async fn update(
    category_id: Uuid,
    user_id: Uuid,
    update_category: UpdateCustomCategory,
    pool: &PgPool,
) -> Result<CustomCategory, Error> {
    let category = sqlx::query_as::<_, CustomCategory>(
        r#"
        UPDATE custom_categories
        SET 
            name = $1,
            updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        RETURNING id, user_id, name, created_at, updated_at
        "#,
    )
    .bind(update_category.name)
    .bind(category_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

/// Deletes a custom category
///
/// # Arguments
///
/// * `category_id` - The UUID of the custom category to delete
/// * `user_id` - The UUID of the user (for authorization)
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(usize)` - Number of rows deleted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn delete(category_id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<usize, Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM custom_categories
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(category_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as usize)
}
