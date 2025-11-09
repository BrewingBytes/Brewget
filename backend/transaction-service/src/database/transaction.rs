use axum::http::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    response::Error,
    transaction::{CreateTransaction, Transaction, UpdateTransaction},
};

/// Finds all transactions for a specific user
///
/// # Arguments
///
/// * `user_id` - The UUID of the user whose transactions to retrieve
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Vec<Transaction>)` - The user's transactions
/// * `Err(Error)` - Database operation error
pub async fn find_all_by_user(user_id: Uuid, pool: &PgPool) -> Result<Vec<Transaction>, Error> {
    let transactions = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, user_id, wallet_id, amount, transaction_type, category, 
               description, transaction_date, destination_wallet_id, created_at, updated_at
        FROM transactions
        WHERE user_id = $1
        ORDER BY transaction_date DESC, created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(transactions)
}

/// Finds all transactions for a specific wallet
///
/// # Arguments
///
/// * `wallet_id` - The UUID of the wallet
/// * `user_id` - The UUID of the user (for authorization)
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Vec<Transaction>)` - The wallet's transactions
/// * `Err(Error)` - Database operation error
pub async fn find_all_by_wallet(
    wallet_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<Transaction>, Error> {
    let transactions = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, user_id, wallet_id, amount, transaction_type, category, 
               description, transaction_date, destination_wallet_id, created_at, updated_at
        FROM transactions
        WHERE wallet_id = $1 AND user_id = $2
        ORDER BY transaction_date DESC, created_at DESC
        "#,
    )
    .bind(wallet_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(transactions)
}

/// Finds a specific transaction by ID
///
/// # Arguments
///
/// * `transaction_id` - The UUID of the transaction to retrieve
/// * `user_id` - The UUID of the user (for authorization)
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Transaction)` - The transaction
/// * `Err(Error)` - Database operation error or transaction not found
pub async fn find_by_id(
    transaction_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Transaction, Error> {
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, user_id, wallet_id, amount, transaction_type, category, 
               description, transaction_date, destination_wallet_id, created_at, updated_at
        FROM transactions
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(transaction_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}

/// Creates a new transaction and updates wallet balance(s)
///
/// # Arguments
///
/// * `user_id` - The UUID of the user creating the transaction
/// * `create_transaction` - The transaction creation data
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Transaction)` - The newly created transaction
/// * `Err(Error)` - Database operation error
pub async fn create(
    user_id: Uuid,
    create_transaction: CreateTransaction,
    pool: &PgPool,
) -> Result<Transaction, Error> {
    // Start a transaction for atomic operations
    let mut tx = pool.begin().await?;

    // Verify the wallet belongs to the user
    let wallet_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM wallets WHERE id = $1 AND user_id = $2)",
    )
    .bind(create_transaction.wallet_id)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    if !wallet_exists {
        return Err(Error::new(
            StatusCode::NOT_FOUND,
            shared_types::response::TranslationKey::SomethingWentWrong,
        ));
    }

    // If it's a transfer, verify destination wallet belongs to the user
    if let Some(dest_wallet_id) = create_transaction.destination_wallet_id {
        let dest_wallet_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM wallets WHERE id = $1 AND user_id = $2)",
        )
        .bind(dest_wallet_id)
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await?;

        if !dest_wallet_exists {
            return Err(Error::new(
                StatusCode::NOT_FOUND,
                shared_types::response::TranslationKey::SomethingWentWrong,
            ));
        }
    }

    let transaction_date = create_transaction
        .transaction_date
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());

    // Create the transaction
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        INSERT INTO transactions (user_id, wallet_id, amount, transaction_type, category, description, transaction_date, destination_wallet_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, wallet_id, amount, transaction_type, category, description, transaction_date, destination_wallet_id, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(create_transaction.wallet_id)
    .bind(create_transaction.amount)
    .bind(create_transaction.transaction_type.as_str())
    .bind(create_transaction.category.as_str())
    .bind(create_transaction.description)
    .bind(transaction_date)
    .bind(create_transaction.destination_wallet_id)
    .fetch_one(&mut *tx)
    .await?;

    // Update wallet balance based on transaction type
    match create_transaction.transaction_type {
        shared_types::enums::TransactionType::Income => {
            // Add to wallet balance
            sqlx::query(
                r#"
                UPDATE wallets
                SET balance = balance + $1, updated_at = NOW()
                WHERE id = $2 AND user_id = $3
                "#,
            )
            .bind(create_transaction.amount)
            .bind(create_transaction.wallet_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        }
        shared_types::enums::TransactionType::Expense => {
            // Subtract from wallet balance
            sqlx::query(
                r#"
                UPDATE wallets
                SET balance = balance - $1, updated_at = NOW()
                WHERE id = $2 AND user_id = $3
                "#,
            )
            .bind(create_transaction.amount)
            .bind(create_transaction.wallet_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        }
        shared_types::enums::TransactionType::Transfer => {
            // Subtract from source wallet
            sqlx::query(
                r#"
                UPDATE wallets
                SET balance = balance - $1, updated_at = NOW()
                WHERE id = $2 AND user_id = $3
                "#,
            )
            .bind(create_transaction.amount)
            .bind(create_transaction.wallet_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

            // Add to destination wallet
            if let Some(dest_wallet_id) = create_transaction.destination_wallet_id {
                sqlx::query(
                    r#"
                    UPDATE wallets
                    SET balance = balance + $1, updated_at = NOW()
                    WHERE id = $2 AND user_id = $3
                    "#,
                )
                .bind(create_transaction.amount)
                .bind(dest_wallet_id)
                .bind(user_id)
                .execute(&mut *tx)
                .await?;
            }
        }
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(transaction)
}

/// Updates a transaction (does not update wallet balances - transactions are immutable for balance purposes)
///
/// # Arguments
///
/// * `transaction_id` - The UUID of the transaction to update
/// * `user_id` - The UUID of the user (for authorization)
/// * `update_transaction` - The transaction update data
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Transaction)` - The updated transaction
/// * `Err(Error)` - Database operation error
///
/// # Note
///
/// This function only updates metadata fields (description, category, date).
/// It does NOT update the amount or type to prevent balance inconsistencies.
/// To change amount/type, delete and create a new transaction.
pub async fn update(
    transaction_id: Uuid,
    user_id: Uuid,
    update_transaction: UpdateTransaction,
    pool: &PgPool,
) -> Result<Transaction, Error> {
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        UPDATE transactions
        SET 
            amount = COALESCE($1, amount),
            category = COALESCE($2, category),
            description = COALESCE($3, description),
            transaction_date = COALESCE($4, transaction_date),
            updated_at = NOW()
        WHERE id = $5 AND user_id = $6
        RETURNING id, user_id, wallet_id, amount, transaction_type, category, description, transaction_date, destination_wallet_id, created_at, updated_at
        "#,
    )
    .bind(update_transaction.amount)
    .bind(update_transaction.category.map(|c| c.as_str()))
    .bind(update_transaction.description)
    .bind(update_transaction.transaction_date)
    .bind(transaction_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}

/// Deletes a transaction and reverses its effect on wallet balance(s)
///
/// # Arguments
///
/// * `transaction_id` - The UUID of the transaction to delete
/// * `user_id` - The UUID of the user (for authorization)
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(usize)` - Number of rows deleted (1 if successful)
/// * `Err(Error)` - Database operation error
pub async fn delete(
    transaction_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<usize, Error> {
    // Start a transaction for atomic operations
    let mut tx = pool.begin().await?;

    // Get the transaction details first
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, user_id, wallet_id, amount, transaction_type, category, 
               description, transaction_date, destination_wallet_id, created_at, updated_at
        FROM transactions
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(transaction_id)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    // Reverse the wallet balance changes
    match transaction.transaction_type.as_str() {
        "Income" => {
            // Subtract from wallet balance (reverse of income)
            sqlx::query(
                r#"
                UPDATE wallets
                SET balance = balance - $1, updated_at = NOW()
                WHERE id = $2 AND user_id = $3
                "#,
            )
            .bind(transaction.amount)
            .bind(transaction.wallet_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        }
        "Expense" => {
            // Add to wallet balance (reverse of expense)
            sqlx::query(
                r#"
                UPDATE wallets
                SET balance = balance + $1, updated_at = NOW()
                WHERE id = $2 AND user_id = $3
                "#,
            )
            .bind(transaction.amount)
            .bind(transaction.wallet_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        }
        "Transfer" => {
            // Add back to source wallet
            sqlx::query(
                r#"
                UPDATE wallets
                SET balance = balance + $1, updated_at = NOW()
                WHERE id = $2 AND user_id = $3
                "#,
            )
            .bind(transaction.amount)
            .bind(transaction.wallet_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

            // Subtract from destination wallet
            if let Some(dest_wallet_id) = transaction.destination_wallet_id {
                sqlx::query(
                    r#"
                    UPDATE wallets
                    SET balance = balance - $1, updated_at = NOW()
                    WHERE id = $2 AND user_id = $3
                    "#,
                )
                .bind(transaction.amount)
                .bind(dest_wallet_id)
                .bind(user_id)
                .execute(&mut *tx)
                .await?;
            }
        }
        _ => {}
    }

    // Delete the transaction
    let result = sqlx::query(
        r#"
        DELETE FROM transactions
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(transaction_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    // Commit the transaction
    tx.commit().await?;

    Ok(result.rows_affected() as usize)
}
