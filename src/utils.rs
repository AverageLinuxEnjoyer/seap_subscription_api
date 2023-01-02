use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub start_index: u32,
    pub count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}

impl Email {
    /// Checks if the email is valid.
    ///
    /// # Examples
    /// ```rust
    /// use seap_subscription_api::utils::Email;
    ///
    /// // Valid email
    /// let email = Email {
    ///     email: "test@gmail.com".to_string(),
    /// };
    /// assert_eq!(email.is_valid(), true);
    ///
    /// // Invalid email
    /// let email = Email {
    ///    email: "test@gmail".to_string(),
    /// };
    /// assert_eq!(email.is_valid(), false);

    pub fn is_valid(&self) -> bool {
        let email = self.email.trim();

        let email_regex = regex::Regex::new(r"^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}$").unwrap();

        email_regex.is_match(email)
    }
}

impl TryFrom<Email> for String {
    type Error = Error;

    fn try_from(value: Email) -> Result<Self, Self::Error> {
        match value.is_valid() {
            true => Ok(value.email),
            false => Err(Error::msg("Invalid email value.")),
        }
    }
}
