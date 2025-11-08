use crate::{config::Config, editor::Form, Result, util::Normalise};

#[derive(Debug, Clone)]
pub struct Part {
    pub id: Option<String>,
    pub category: String,
    pub subcategory: String,
    pub description: String,
    pub value: String,
    pub manufacturer: Option<String>,
    pub mpn: Option<String>,
    pub package: Option<String>,
    pub supplier1: Option<String>,
    pub spn1: Option<String>,
    pub supplier2: Option<String>,
    pub spn2: Option<String>,
    pub symbol: String,
    pub footprint: Option<String>,
    pub datasheet: Option<String>,
    pub status: String,
    pub notes: Option<String>,
}


impl From<Form> for Part {
    fn from(value: Form) -> Self {
        Self {
            id: value.about.id,
            category: value.about.category.trim().to_lowercase(),
            subcategory: value.about.subcategory.trim().to_lowercase(),
            description: value.about.description,
            value: value.about.value,
            manufacturer: value.component.manufacturer.none_if_empty(),
            mpn: value.component.mpn.none_if_empty(),
            package: value.component.package.none_if_empty(),
            supplier1: value.suppliers.supplier1.none_if_empty(),
            spn1: value.suppliers.spn1.none_if_empty(),
            supplier2: value.suppliers.supplier2.none_if_empty(),
            spn2: value.suppliers.spn2.none_if_empty(),
            symbol: value.library.symbol,
            footprint: value.library.footprint.none_if_empty(),
            datasheet: value.component.datasheet.none_if_empty(),
            status: value.library.status,
            notes: None,
        }
    }
}

impl Part {
    pub fn validate(&self, config: &Config) -> Result<()>{

        // Check ID has been assigned
        if self.id.is_none() {
            return Err("Validation error: ID was not assigned to Part".into());
        }

        // Check category is one of the correct categories
        let correct_categoy = config.cat.keys().any(|s| s == &self.category);
        if !correct_categoy {
            return Err("Validation error: Category does not match config file".into());
        }
        


        Ok(())
    }
}