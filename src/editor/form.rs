use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::config::Cat;
use crate::api;


#[derive(Debug, Serialize, Deserialize)]
pub struct About {
    pub id: Option<String>,
    pub category: String,
    pub subcategory: String,
    pub description: String,
    pub value: String,
}

impl Default for About {
    fn default() -> Self {
        Self {
            id: None,
            category: String::from("resistor"),
            subcategory: String::from("chip"),
            description: String::from("RES 120K OHM 1% 1/16W 0402"),
            value: String::from("120K"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub manufacturer: String,
    pub mpn: String,
    pub package: String,
    pub datasheet: String,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            manufacturer: String::from("YAGEO"),
            mpn: String::from("RC0402FR-07120KL"),
            package: String::from("0402"),
            datasheet: String::from("https://www.yageo.com/upload/media/product/products/datasheet/rchip/PYu-RC_Group_51_RoHS_L_12.pdf"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suppliers {
    pub supplier1: String,
    pub spn1: String,
    pub supplier2: String,
    pub spn2: String,
}

impl Default for Suppliers {
    fn default() -> Self {
        Self {
            supplier1: String::from("Digi-Key"),
            spn1: String::from(""),
            supplier2: String::from("LCSC"),
            spn2: String::from(""),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub symbol: String,
    pub footprint: String,
    pub status: String,
    pub notes: String,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            symbol: String::from("Generic:R_US"),
            footprint: String::from("RES:RESC100x05x04N"),
            status: String::from("New"),
            notes: String::from(""),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Form {
    pub about: About,
    pub component: Component,
    pub suppliers: Suppliers,
    pub library: Library,
    pub categories: Option<HashMap<String, Cat>>,
    pub long_description: Option<String>,
}


impl Form {
    pub(super) const README: &str = "http://github.com/bartekkowalski/partman/README.md";

    // TODO: Move statuses to config
    pub(super) const STATUSES: &str = "[new, verified, issues, obsolete]";

}


impl From<api::digikey::DigikeyProduct> for Form {
    fn from(value: api::digikey::DigikeyProduct) -> Self {
        Self {
            about: About {
                // TODO: Pull out more info for category and subcategory
                category: value.category.name,
                description: value.description.product_description,
                ..Default::default()
            },
            component: Component {
                manufacturer: value.manufacturer.name,
                mpn: value.manufacturer_product_number,
                // package: value,
                datasheet: value.datasheet_url.unwrap_or("".to_string()),
                ..Default::default()
            },
            suppliers: Suppliers {
                // supplier1: value,
                spn1: value.search_spn.unwrap_or("".to_string()),
                // supplier2: value,
                // spn2: value,
                ..Default::default()
            },
            library: Library {
                // symbol: value,
                // footprint: value,
                // status: value,
                // notes: value,
                ..Default::default()
            },
            long_description: Some(value.description.detailed_description),
            ..Default::default()
        }
    }
}