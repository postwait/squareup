//! Model struct for RetrieveCardResponse type

use serde::Deserialize;

use super::{Card, errors::Error};

/// This is a model struct for RetrieveCardResponse type
#[derive(Clone, Debug, Deserialize, Default, Eq, PartialEq)]
pub struct RetrieveCardResponse {
    /// Information on errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// Represents the payment details of a card to be used for payments. These details are
    /// determined by the payment token generated by Web Payments SDK.
    pub card: Card,
}
