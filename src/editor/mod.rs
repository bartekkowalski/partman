use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
use crate::{Result, config::Config, data, part::Part};

mod form;
mod display;

pub use form::Form;

/// Removes comments and whitespace for comparing edited content
/// 
/// Used to determine if meaningful changes were made during editing
fn clean(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(str::trim_end)
        .collect::<Vec<_>>()
}

pub fn edit(editor: &str, history_path: &Path, form: Form) -> Result<Option<Form>> {
    // Create temporary file with timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let mut temp_file: PathBuf = history_path.into();
    temp_file.push(timestamp);
    temp_file.set_extension("toml");

    // Make sure the directories exist or else the write will fail
    if let Some(parent_dir) = temp_file.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    fs::write(&temp_file, form.to_string())?;

    edit_file(editor, &temp_file)
}

pub fn edit_file(editor: &str, file: &Path) -> Result<Option<Form>> {
     // Save original for comparison later
     let original_content = fs::read_to_string(file)?;

     // First element is editor command, rest are the arguments
    let mut editor_parts = editor.split_whitespace();
    let editor_cmd = editor_parts.next().expect("Error: No editor provided in config");
    
    let status = Command::new(editor_cmd)
        .args(editor_parts)
        .arg(file)
        .status()?;

    if !status.success() {
        return Err(
            format!("Editor returned a non-zero exit code::\n{status}").into())
    }

    let new_content = fs::read_to_string(file)?;

    if clean(&new_content) == clean(&original_content) {
        return Ok(None);
    }

    // Return the edited form if content has changed
    Ok(toml::from_str(&new_content)?)
}

pub fn process_form(edited_form: Form, config: &Config) -> Result<()> {
    let mut new_part: Part = edited_form.into();
    let mut parts = data::load_all_parts(&config.cat, &config.csv_dir_path)?;
    let code = &config.cat
        .get(&new_part.category)
        .ok_or("Error: Part category incorrect")?
        .code;
    new_part.id = Some(data::get_next_id(&parts, code)?);

    // TODO: Split different verifications out such as id etc.
    new_part.validate(config)?;

    data::insert_part(&mut new_part, config, &config.csv_dir_path)?;
    parts.push(new_part);
    data::build_kicad_db(&config.db_file_path, parts, &config.cat)?;    

    println!("Successfully added new part");
    Ok(())
}