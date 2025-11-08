use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a custom category stored in the database
///
/// This struct maps to the `custom_categories` table and contains user-defined
/// categories for transactions.
///
/// # Fields
///
/// * `id` - Unique identifier of the custom category
/// * `user_id` - Unique identifier of the user who owns this category
/// * `name` - Name of the custom category
/// * `created_at` - Timestamp when the category was created
/// * `updated_at` - Timestamp when the category was last updated
#[derive(FromRow, Clone, Serialize)]
pub struct CustomCategory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Represents a request to create a new custom category
///
/// This struct is used when creating a new custom category.
///
/// # Fields
///
/// * `name` - Name of the custom category
#[derive(Deserialize)]
pub struct CreateCustomCategory {
    pub name: String,
}

/// Represents updates to a custom category
///
/// This struct is used for updates to custom categories.
///
/// # Fields
///
/// * `name` - New name for the custom category
#[derive(Deserialize)]
pub struct UpdateCustomCategory {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_create_custom_category_deserialization() {
        let json = r#"{
            "name": "Subscriptions"
        }"#;

        let create_category: CreateCustomCategory = serde_json::from_str(json).unwrap();
        assert_eq!(create_category.name, "Subscriptions");
    }

    #[test]
    fn test_update_custom_category_deserialization() {
        let json = r#"{
            "name": "Monthly Subscriptions"
        }"#;

        let update: UpdateCustomCategory = serde_json::from_str(json).unwrap();
        assert_eq!(update.name, "Monthly Subscriptions");
    }
}
