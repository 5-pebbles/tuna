use crate::database::permissions::Permission;
use strum::IntoEnumIterator;

pub fn migration() -> String {
    format!(
        "INSERT OR IGNORE INTO permissions (id) VALUES ('{}');",
        Permission::iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("'), ('")
    )
}
