use std::{collections::HashMap, path::Path};
use rusqlite::{params, Connection};
use std::fs;
use crate::{config::Cat, part::Part, Result};


pub fn build_kicad_db(kicad_db_path: &Path, parts: Vec<Part>, categories: &HashMap<String, Cat> ) -> Result<()>{

    if kicad_db_path.exists() {
        fs::remove_file(kicad_db_path)?;
    } else if let Some(parent_dir) = kicad_db_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }


    let mut conn = Connection::open(kicad_db_path)?;
    conn.execute_batch(
        "
        PRAGMA journal_mode = OFF;
        PRAGMA synchronous = OFF;
        PRAGMA cache_size = 1000000;
        PRAGMA locking_mode = EXCLUSIVE;
        PRAGMA temp_store = MEMORY;
        "
    )?;

    for category_name in categories.keys() {

        let create_sql = format!(
            "CREATE TABLE {} (
                id TEXT PRIMARY KEY,
                Subcategory TEXT,
                Description TEXT,
                Value TEXT,
                Manufacturer TEXT,
                MPN TEXT,
                Package TEXT,
                Supplier1 TEXT,
                SPN1 TEXT,
                Supplier2 TEXT,
                SPN2 TEXT,
                Symbol TEXT,
                Footprint TEXT,
                Datasheet TEXT,
                Status TEXT,
                Notes TEXT
            )",
            category_name
        );
        conn.execute(&create_sql, [])?;
    }

    let tx = conn.transaction()?;
    {
        let mut insert_statements = HashMap::new();
        for category_name in categories.keys() {
            let insert_sql = format!(
                "INSERT INTO {} (\r
                    id, Subcategory, Description, Value, Manufacturer, MPN, Package,\r
                    Supplier1, SPN1, Supplier2, SPN2,\r
                    Symbol, Footprint, Datasheet, Status, Notes\r
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
                category_name
            );
            insert_statements.insert(category_name, tx.prepare(&insert_sql)?);
        }
        for part in parts {
            if let Some(stmt) = insert_statements.get_mut(&part.category) {
                stmt.execute(params![
                    part.id.as_deref().unwrap_or(""),
                    part.subcategory,
                    part.description,
                    part.value,
                    part.manufacturer.unwrap_or_default(),
                    part.mpn.unwrap_or_default(),
                    part.package.unwrap_or_default(),
                    part.supplier1.unwrap_or_default(),
                    part.spn1.unwrap_or_default(),
                    part.supplier2.unwrap_or_default(),
                    part.spn2.unwrap_or_default(),
                    part.symbol,
                    part.footprint.unwrap_or_default(),
                    part.datasheet.unwrap_or_default(),
                    part.status,
                    part.notes.unwrap_or_default(),
                ])?;
            }
        }
    }
    tx.commit()?;

    println!("Successfully built KiCad DB: {}", kicad_db_path.display());

    Ok(())
}