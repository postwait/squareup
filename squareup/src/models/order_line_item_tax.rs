//! Model struct for OrderLineItemTax type

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    Money,
    enums::{OrderLineItemTaxScope, OrderLineItemTaxType},
};

/// Represents a tax that applies to one or more line item in the order.
///
/// Fixed-amount, order-scoped taxes are distributed across all non-zero line item totals. The
/// amount distributed to each line item is relative to the amount the item contributes to the order
/// subtotal.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OrderLineItemTax {
    /// A unique ID that identifies the tax only within this order.
    pub uid: Option<String>,
    /// The catalog object ID referencing [CatalogTax].
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this tax references.
    pub catalog_version: Option<i64>,
    /// The tax's name.
    pub name: Option<String>,
    /// Indicates the calculation method used to apply the tax.
    pub r#type: Option<OrderLineItemTaxType>,
    /// The percentage of the tax, as a string representation of a decimal number. For example, a
    /// value of "7.25" corresponds to a percentage of 7.25%.
    pub percentage: Option<String>,
    /// Application-defined data attached to this tax. Metadata fields are intended to store
    /// descriptive references or associations with an entity in another system or store brief
    /// information about the object. Square does not process this field; it only stores and returns
    /// it in relevant API calls. Do not use metadata to store any sensitive information (such as
    /// personally identifiable information or card details).
    ///
    /// Keys written by applications must be 60 characters or less and must be in the character set
    /// `[a-zA-Z0-9_-]`. Entries can also include metadata generated by Square. These keys are
    /// prefixed with a namespace, separated from the key with a ':' character.
    ///
    /// Values have a maximum length of 255 characters.
    ///
    /// An application can have up to 10 entries per metadata field.
    ///
    /// Entries written by applications are private and can only be read or modified by the same
    /// application.
    ///
    /// For more information,
    /// see [Metadata](https://developer.squareup.com/docs/build-basics/metadata).
    pub metadata: Option<HashMap<String, String>>,
    /// The amount of money applied by the tax in the order.
    pub applied_money: Option<Money>,
    /// Indicates the level at which the tax applies. For `ORDER` scoped taxes, Square generates
    /// references in `applied_taxes` on all order line items that do not have them. For `LineItem`
    /// scoped taxes, the tax only applies to line items with references in their `applied_taxes`
    /// field.
    ///
    /// This field is immutable. To change the scope, you must delete the tax and re-add it as a new
    /// tax.
    pub scope: Option<OrderLineItemTaxScope>,
    /// **Read only** Determines whether the tax was automatically applied to the order based on the
    /// catalog configuration. For an example, see [Automatically Apply Taxes to an
    /// Order](https://developer.squareup.com/docs/orders-api/apply-taxes-and-discounts/auto-apply-taxes).
    pub auto_applied: Option<bool>,
}
