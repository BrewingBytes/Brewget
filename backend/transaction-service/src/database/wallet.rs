use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    response::Error,
    wallet::{CreateWallet, UpdateWallet, Wallet},
};

/// Finds all wallets for a specific user
///
/// # Arguments
///
/// * `user_id` - The UUID of the user whose wallets to retrieve
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Vec<Wallet>)` - The user's wallets
/// * `Err(Error)` - Database operation error
pub async fn find_all_by_user(user_id: Uuid, pool: &PgPool) -> Result<Vec<Wallet>, Error> {
    let wallets = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, user_id, name, balance, currency, category, created_at, updated_at
        FROM wallets
        WHERE user_id = $1
        ORDER BY category NULLS LAST, created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(wallets)
}

/// Finds a specific wallet by ID
///
/// # Arguments
///
/// * `wallet_id` - The UUID of the wallet to retrieve
/// * `user_id` - The UUID of the user (for authorization)
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Wallet)` - The wallet
/// * `Err(Error)` - Database operation error or wallet not found
pub async fn find_by_id(
    wallet_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Wallet, Error> {
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, user_id, name, balance, currency, category, created_at, updated_at
        FROM wallets
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(wallet_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(wallet)
}

/// Creates a new wallet for a user
///
/// # Arguments
///
/// * `user_id` - The UUID of the user creating the wallet
/// * `create_wallet` - The wallet creation data
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Wallet)` - The newly created wallet
/// * `Err(Error)` - Database operation error
pub async fn create(
    user_id: Uuid,
    create_wallet: CreateWallet,
    pool: &PgPool,
) -> Result<Wallet, Error> {
    let balance = create_wallet.balance.unwrap_or_default();
    
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        INSERT INTO wallets (user_id, name, balance, currency, category)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, name, balance, currency, category, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(create_wallet.name)
    .bind(balance)
    .bind(create_wallet.currency)
    .bind(create_wallet.category)
    .fetch_one(pool)
    .await?;

    Ok(wallet)
}

/// Updates a wallet
///
/// # Arguments
///
/// * `wallet_id` - The UUID of the wallet to update
/// * `user_id` - The UUID of the user (for authorization)
/// * `update_wallet` - The wallet update data
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Wallet)` - The updated wallet
/// * `Err(Error)` - Database operation error
pub async fn update(
    wallet_id: Uuid,
    user_id: Uuid,
    update_wallet: UpdateWallet,
    pool: &PgPool,
) -> Result<Wallet, Error> {
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        UPDATE wallets
        SET 
            name = COALESCE($1, name),
            currency = COALESCE($2, currency),
            category = COALESCE($3, category),
            updated_at = NOW()
        WHERE id = $4 AND user_id = $5
        RETURNING id, user_id, name, balance, currency, category, created_at, updated_at
        "#,
    )
    .bind(update_wallet.name)
    .bind(update_wallet.currency)
    .bind(update_wallet.category)
    .bind(wallet_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(wallet)
}

/// Deletes a wallet
///
/// # Arguments
///
/// * `wallet_id` - The UUID of the wallet to delete
/// * `user_id` - The UUID of the user (for authorization)
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(usize)` - Number of rows deleted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn delete(wallet_id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<usize, Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM wallets
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(wallet_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as usize)
}
