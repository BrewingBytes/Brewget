use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use uuid::Uuid;

/// Represents an activation link stored in the database
///
/// This struct maps to the activation links table
///
/// # Fields
/// * `user_id` - ID of the user this activation link belongs to
#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::activation_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ActivationLink {
    user_id: Uuid,
}

impl ActivationLink {
    /// Get the User ID of the activation link
    ///
    /// # Returns
    /// * `Uuid` - The User ID associated to the activation link
    pub fn get_uuid(&self) -> Uuid {
        self.user_id
    }
}

/// Represents a new activation link to be inserted into the database
///
/// This struct is used for creating new activation link records
///
/// # Fields
/// * `id` - UUIDv4 for the activation link
/// * `user_id` - The user account uuid it is generated for
#[derive(Insertable)]
#[diesel(table_name = crate::schema::activation_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewActivationLink {
    id: Uuid,
    user_id: Uuid,
}

impl NewActivationLink {
    /// Creates a new activation link record
    ///
    /// # Arguments
    /// * `user_id` - The user account uuid it is generated for
    ///
    /// # Returns
    /// A new `NewActivationLink` instance ready for database insertion
    pub fn new(user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
        }
    }

    /// Get the ID of the activation link
    ///
    /// # Returns
    /// * `Uuid` - The ID associated to the activation link
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    // Get the activation link
    ///
    /// # Returns
    /// * `String` - The activation link
    pub fn get_link(&self) -> String {
        format!(
            "https://brewget.brewingbytes.com/activate/{}",
            self.get_id()
        )
    }
}
