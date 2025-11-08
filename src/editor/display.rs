use std::fmt;

use super::Form;

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
r#"# KiCad Database Library Part Manager
# Fill in the fields below, use empty strings ("") for null values.
# Save and exit editor when done.
# See README for library info, conventions and category examples: {}

{}

[about]
category = "{}" # Required field
subcategory = "{}" # Required field
description = "{}" # Required field
value = "{}" # Required field

[component]
{}
[suppliers]
{}
[library]
symbol = "{}" # Required field
footprint = "{}"
status = "{}" # Required field, options: {}
notes = "{}"

{}
"#,
            // Header
            Form::README,

            // Long description
            if let Some(desc) = &self.long_description {
                format!("# Long Description: {}", desc)
            } else {
                String::new()
            },

            &self.about.category,
            &self.about.subcategory,
            &self.about.description,
            &self.about.value,
            toml::ser::to_string_pretty(&self.component).unwrap(),
            toml::ser::to_string_pretty(&self.suppliers).unwrap(),
            &self.library.symbol,
            &self.library.footprint,
            &self.library.status,
            Form::STATUSES,
            &self.library.notes,

            // Category breakdown
            if let Some(cat) = &self.categories {
                let mut s = "# Category and Subcategory options:".to_string();
                for (name, category) in cat {
                    s.push_str(&format!("\n#    {}: {:?}", name, category.subcategories));
                };
                s.replace("\"", "")
            } else {
                "bye".to_string()
            }
        )
    
    }
}