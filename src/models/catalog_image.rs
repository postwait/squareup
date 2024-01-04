//! Model struct for CatalogImage type.

use serde::{Deserialize, Serialize};

/// An image file to use in Square catalogs.
///
/// It can be associated with `CatalogItem`, `CatalogItemVariation`, `CatalogCategory`, and
/// `CatalogModifierList` objects. Only the images on items and item variations are exposed in
/// Dashboard. Only the first image on an item is displayed in Square Point of Sale (SPOS). Images
/// on items and variations are displayed through Square Online Store. Images on other object types
/// are for use by 3rd party application developers.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogImage {
    /// The internal name to identify this image in calls to the Square API. This is a searchable
    /// attribute for use in applicable query filters using the
    /// [SearchCatalogObjects](https://developer.squareup.com/reference/square/catalog-api/search-catalog-objects).
    /// It is not unique and should not be shown in a buyer facing context.
    pub name: Option<String>,
    /// The URL of this image, generated by Square after an image is uploaded using the
    /// [CreateCatalogImage](https://developer.squareup.com/reference/square/catalog-api/create-catalog-image)
    /// endpoint. To modify the image, use the UpdateCatalogImage endpoint. Do not change the URL
    /// field.
    pub url: Option<String>,
    /// A caption that describes what is shown in the image. Displayed in the Square Online Store.
    /// This is a searchable attribute for use in applicable query filters using the
    /// [SearchCatalogObjects](https://developer.squareup.com/reference/square/catalog-api/search-catalog-objects).
    pub caption: Option<String>,
    /// The immutable order ID for this image object created by the Photo Studio service in Square
    /// Online Store.
    pub photo_studio_order_id: Option<String>,
}
