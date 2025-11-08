use chrono::Utc;
use shared_types::WalletType;
use sqlx::PgPool;
use uuid::Uuid;

use crate::wallet_model::{CreateWallet, UpdateWallet, Wallet};
use shared_types::Error;

pub async fn create(
    user_id: Uuid,
    create_wallet: CreateWallet,
    pool: &PgPool,
) -> Result<Wallet, Error> {
    let now = Utc::now();
    let balance = create_wallet.balance.unwrap_or(0.0);
    let wallet_type = create_wallet.wallet_type.unwrap_or(WalletType::General);

    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        INSERT INTO wallets (user_id, name, balance, currency, wallet_type, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, user_id, name, balance, currency, wallet_type, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(&create_wallet.name)
    .bind(balance)
    .bind(&create_wallet.currency)
    .bind(&wallet_type)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(wallet)
}

pub async fn find_all_by_user(user_id: Uuid, pool: &PgPool) -> Result<Vec<Wallet>, Error> {
    let wallets = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, user_id, name, balance, currency, wallet_type, created_at, updated_at
        FROM wallets
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(wallets)
}

pub async fn find_by_id(
    wallet_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Wallet, Error> {
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, user_id, name, balance, currency, wallet_type, created_at, updated_at
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

pub async fn update(
    wallet_id: Uuid,
    user_id: Uuid,
    update_wallet: UpdateWallet,
    pool: &PgPool,
) -> Result<Wallet, Error> {
    let now = Utc::now();

    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        UPDATE wallets
        SET 
            name = COALESCE($1, name),
            balance = COALESCE($2, balance),
            currency = COALESCE($3, currency),
            wallet_type = COALESCE($4, wallet_type),
            updated_at = $5
        WHERE id = $6 AND user_id = $7
        RETURNING id, user_id, name, balance, currency, wallet_type, created_at, updated_at
        "#,
    )
    .bind(update_wallet.name)
    .bind(update_wallet.balance)
    .bind(update_wallet.currency)
    .bind(update_wallet.wallet_type)
    .bind(now)
    .bind(wallet_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(wallet)
}

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
