use std::collections::HashMap;
use std::path::Path;
use csv::{ReaderBuilder, WriterBuilder};
use std::fs::OpenOptions;

use crate::config::{Cat, Config};
use crate::{part::Part, Result, util::Normalise};

/// The CSV representation of a component. This maps *exactly* to CSV columns.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CsvRow {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Subcategory")]
    pub subcategory: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Value")]
    pub value: String,

    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,

    #[serde(rename = "MPN")]
    pub mpn: String,

    #[serde(rename = "Package")]
    pub package: String,

    #[serde(rename = "Supplier1")]
    pub supplier1: String,

    #[serde(rename = "SPN1")]
    pub spn1: String,

    #[serde(rename = "Supplier2")]
    pub supplier2: String,

    #[serde(rename = "SPN2")]
    pub spn2: String,

    #[serde(rename = "Symbol")]
    pub symbol: String,

    #[serde(rename = "Footprint")]
    pub footprint: String,

    #[serde(rename = "Datasheet")]
    pub datasheet: String,

    #[serde(rename = "Status")]
    pub status: String,

    #[serde(rename = "Notes")]
    pub notes: String,
}

impl From<CsvRow> for Part {
    fn from(r: CsvRow) -> Self {
        Part {
            id: Some(r.id),
            category: String::new(),
            subcategory: r.subcategory,
            description: r.description,
            value: r.value,
            manufacturer: r.manufacturer.none_if_empty(),
            mpn: r.mpn.none_if_empty(),
            package: r.package.none_if_empty(),
            supplier1: r.supplier1.none_if_empty(),
            spn1: r.spn1.none_if_empty(),
            supplier2: r.supplier2.none_if_empty(),
            spn2: r.spn2.none_if_empty(),
            symbol: r.symbol,
            footprint: r.footprint.none_if_empty(),
            datasheet: r.datasheet.none_if_empty(),
            status: r.status,
            notes: r.notes.none_if_empty(),
        }
    }
}

impl From<Part> for CsvRow {
    fn from(p: Part) -> Self {
        CsvRow {
            id: p.id.unwrap_or_default(),
            subcategory: p.subcategory,
            description: p.description,
            value: p.value,
            manufacturer: p.manufacturer.unwrap_or_default(),
            mpn: p.mpn.unwrap_or_default(),
            package: p.package.unwrap_or_default(),
            supplier1: p.supplier1.unwrap_or_default(),
            spn1: p.spn1.unwrap_or_default(),
            supplier2: p.supplier2.unwrap_or_default(),
            spn2: p.spn2.unwrap_or_default(),
            symbol: p.symbol,
            footprint: p.footprint.unwrap_or_default(),
            datasheet: p.datasheet.unwrap_or_default(),
            status: p.status,
            notes: p.notes.unwrap_or_default(),
        }
    }
}



pub fn load_all_parts(categories: &HashMap<String, Cat>, csv_dir: &Path) -> Result<Vec<Part>> {
    let mut parts = Vec::new();

    for (category, cat_cfg) in categories {
        let path = csv_dir.join(&cat_cfg.filename);
        if !path.exists() {
            continue;
        }

        let mut rdr = ReaderBuilder::new().from_path(path)?;
        for row in rdr.deserialize::<CsvRow>() {
            let mut part: Part = row?.into();
            part.category = category.to_string();
            parts.push(part);
        }
    }

    Ok(parts)
}


pub fn insert_part(part: &mut Part, config: &Config, library_path: &Path) -> Result<()> {
    let category_config = config.cat.get(&part.category)
        .ok_or_else(|| format!("Invalid category '{}'", part.category))?;

    let csv_path = library_path.join(&category_config.filename);

    let row: CsvRow = part.clone().into();

    let file_exists = csv_path.exists();
    let file = OpenOptions::new().append(true).create(true).open(csv_path)?;
    let mut wtr = WriterBuilder::new().has_headers(!file_exists).from_writer(file);
    wtr.serialize(row)?;
    wtr.flush()?;

    Ok(())
}

pub fn get_next_id(parts: &Vec<Part>, category_code: &str) -> Result<String> {
    let max = parts
        .iter()
        .filter_map(|part| part.id.as_deref())
        .filter_map(|id| {
            let mut splits = id.split('-');
            let cat = splits.next()?;
            if cat == category_code {
                splits.next()?.parse::<u32>().ok()
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0);
    Ok(format!("{category_code}-{:04}", max + 1))
}    
