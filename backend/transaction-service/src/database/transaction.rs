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
               description, transaction_date, created_at, updated_at
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
/// * `wallet_id` - The UUID of the wallet whose transactions to retrieve
/// * `user_id` - The UUID of the user (for authorization)
/// * `pool` - Database connection pool
///
/// # Returns
///
/// * `Ok(Vec<Transaction>)` - The wallet's transactions
/// * `Err(Error)` - Database operation error
pub async fn find_by_wallet(
    wallet_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<Transaction>, Error> {
    let transactions = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, user_id, wallet_id, amount, transaction_type, category, 
               description, transaction_date, created_at, updated_at
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
               description, transaction_date, created_at, updated_at
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

/// Creates a new transaction for a user and updates wallet balance
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
    // Start a database transaction to ensure atomicity
    let mut tx = pool.begin().await?;

    let transaction_date = create_transaction
        .transaction_date
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());

    // Create the transaction
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        INSERT INTO transactions (user_id, wallet_id, amount, transaction_type, category, description, transaction_date)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, user_id, wallet_id, amount, transaction_type, category, 
                  description, transaction_date, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(create_transaction.wallet_id)
    .bind(create_transaction.amount)
    .bind(create_transaction.transaction_type.as_str())
    .bind(&create_transaction.category)
    .bind(create_transaction.description)
    .bind(transaction_date)
    .fetch_one(&mut *tx)
    .await?;

    // Update wallet balance based on transaction type
    let balance_change = match create_transaction.transaction_type {
        shared_types::enums::TransactionType::Income => create_transaction.amount,
        shared_types::enums::TransactionType::Expense => -create_transaction.amount,
        shared_types::enums::TransactionType::Transfer => {
            // For transfers, we'll handle this separately if needed
            rust_decimal::Decimal::ZERO
        }
    };

    sqlx::query(
        r#"
        UPDATE wallets
        SET balance = balance + $1, updated_at = NOW()
        WHERE id = $2 AND user_id = $3
        "#,
    )
    .bind(balance_change)
    .bind(create_transaction.wallet_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    // Commit the transaction
    tx.commit().await?;

    Ok(transaction)
}

/// Updates a transaction and adjusts wallet balance
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
pub async fn update(
    transaction_id: Uuid,
    user_id: Uuid,
    update_transaction: UpdateTransaction,
    pool: &PgPool,
) -> Result<Transaction, Error> {
    // Start a database transaction to ensure atomicity
    let mut tx = pool.begin().await?;

    // First, get the current transaction to calculate balance changes
    let current = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, user_id, wallet_id, amount, transaction_type, category, 
               description, transaction_date, created_at, updated_at
        FROM transactions
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(transaction_id)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    // Update the transaction
    let updated = sqlx::query_as::<_, Transaction>(
        r#"
        UPDATE transactions
        SET 
            amount = COALESCE($1, amount),
            transaction_type = COALESCE($2, transaction_type),
            category = COALESCE($3, category),
            description = COALESCE($4, description),
            transaction_date = COALESCE($5, transaction_date),
            updated_at = NOW()
        WHERE id = $6 AND user_id = $7
        RETURNING id, user_id, wallet_id, amount, transaction_type, category, 
                  description, transaction_date, created_at, updated_at
        "#,
    )
    .bind(update_transaction.amount)
    .bind(update_transaction.transaction_type.map(|t| t.as_str()))
    .bind(update_transaction.category.as_ref())
    .bind(update_transaction.description)
    .bind(update_transaction.transaction_date)
    .bind(transaction_id)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    // Reverse the old balance change
    let old_balance_change = match current.transaction_type.as_str() {
        "Income" => current.amount,
        "Expense" => -current.amount,
        _ => rust_decimal::Decimal::ZERO,
    };

    // Apply the new balance change
    let new_balance_change = match updated.transaction_type.as_str() {
        "Income" => updated.amount,
        "Expense" => -updated.amount,
        _ => rust_decimal::Decimal::ZERO,
    };

    let balance_delta = new_balance_change - old_balance_change;

    if balance_delta != rust_decimal::Decimal::ZERO {
        sqlx::query(
            r#"
            UPDATE wallets
            SET balance = balance + $1, updated_at = NOW()
            WHERE id = $2 AND user_id = $3
            "#,
        )
        .bind(balance_delta)
        .bind(current.wallet_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(updated)
}

/// Deletes a transaction and adjusts wallet balance
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
    // Start a database transaction to ensure atomicity
    let mut tx = pool.begin().await?;

    // First, get the transaction to calculate balance reversal
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, user_id, wallet_id, amount, transaction_type, category, 
               description, transaction_date, created_at, updated_at
        FROM transactions
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(transaction_id)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

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

    // Reverse the balance change
    let balance_change = match transaction.transaction_type.as_str() {
        "Income" => -transaction.amount,
        "Expense" => transaction.amount,
        _ => rust_decimal::Decimal::ZERO,
    };

    if balance_change != rust_decimal::Decimal::ZERO {
        sqlx::query(
            r#"
            UPDATE wallets
            SET balance = balance + $1, updated_at = NOW()
            WHERE id = $2 AND user_id = $3
            "#,
        )
        .bind(balance_change)
        .bind(transaction.wallet_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(result.rows_affected() as usize)
}
