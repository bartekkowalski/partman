//! DigiKey product data structures.
//! 
//! Defines the types used to deserialize DigiKey API responses
//! for product information.

#![allow(dead_code)]

use serde::Deserialize;

/// Product description containing both brief and detailed descriptions
#[derive(Deserialize, Debug)]
pub struct ProductDescription {
    #[serde(rename = "ProductDescription")]
    pub product_description: String,
    #[serde(rename = "DetailedDescription")]
    pub detailed_description: String,
}

/// Manufacturer information
#[derive(Deserialize, Debug)]
pub struct Manufacturer {
    #[serde(rename = "Name")]
    pub name: String,
}

/// Product category with support for nested subcategories
#[derive(Deserialize, Debug)]
pub struct Category {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ChildCategories")]
    pub child_categories: Vec<Option<Category>>,
}

/// Product variation information including stock and ordering details
#[derive(Deserialize, Debug)]
pub struct ProductVariation {
    #[serde(rename = "DigiKeyProductNumber")]
    pub digikey_product_number: String,
    #[serde(rename = "MarketPlace")]
    pub market_place: bool,
    #[serde(rename = "QuantityAvailableforPackageType")]
    pub qty_available_for_package_type: u32,
    #[serde(rename = "MinimumOrderQuantity")]
    pub minimum_order_qty: u32,
}

/// Technical parameter and its value
#[derive(Deserialize, Debug)]
pub struct Parameter {
    #[serde(rename = "ParameterText")]
    pub parameter_text: String,
    #[serde(rename = "ValueText")]
    pub parameter_value: String,
}

/// Product lifecycle status information
#[derive(Deserialize, Debug)]
pub struct ProductStatus {
    #[serde(rename = "Status")]
    pub status: String,
}

/// Complete product information from DigiKey API
#[derive(Deserialize, Debug)]
pub struct DigikeyProduct {
    // SPN used in search
    pub search_spn: Option<String>,

    #[serde(rename = "Description")]
    pub description: ProductDescription,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Manufacturer,
    #[serde(rename = "ManufacturerProductNumber")]
    pub manufacturer_product_number: String,
    #[serde(rename = "UnitPrice")]
    pub unit_price: f64,
    #[serde(rename = "DatasheetUrl")]
    pub datasheet_url: Option<String>,
    #[serde(rename = "ProductVariations")]
    pub product_variations: Vec<ProductVariation>,
    #[serde(rename = "NormallyStocking")]
    pub normally_stocking: bool,
    #[serde(rename = "Discontinued")]
    pub discontinued: bool,
    #[serde(rename = "EndOfLife")]
    pub end_of_life: bool,
    #[serde(rename = "Parameters")]
    pub parameters: Vec<Parameter>,
    #[serde(rename = "QuantityAvailable")]
    pub quantity_available: u32,
    #[serde(rename = "ProductStatus")]
    pub product_status: ProductStatus,
    #[serde(rename = "Category")]
    pub category: Category,
}

/// Wrapper for DigiKey API responses containing product information
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    #[serde(rename = "Product")]
    pub product: DigikeyProduct,
}