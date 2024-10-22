extern crate proc_macro;
use proc_macro::TokenStream;

struct ParsedMigration {
    value: String,
    name: String,
    pascal_name: String,
    id: usize,
}

fn parse_migration_string(migration_string: &str) -> ParsedMigration {
    let migration = migration_string.trim();
    let full_name = migration.to_string();
    let parts: Vec<&str> = full_name.split('_').collect();
    let id = parts[1];
    let name = parts[2..].join("_");

    let pascal_name = name
        .split('_')
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect::<String>();

    ParsedMigration {
        value: migration.to_string(),
        name,
        pascal_name,
        id: id.parse().unwrap(),
    }
}

#[proc_macro]
pub fn migrations(migration_token: TokenStream) -> TokenStream {
    let migration_token_str = migration_token.to_string();
    let all_migrations = migration_token_str.split(',').collect::<Vec<&str>>();

    let mut parsed_migrations = all_migrations
        .iter()
        .map(|mig| parse_migration_string(mig))
        .collect::<Vec<ParsedMigration>>();
    parsed_migrations.sort_by(|a, b| a.id.cmp(&b.id));

    let mut return_value = String::new();
    return_value += r#"
        use fimi::{Migration, MigrationData, MigrationExecutor};
    "#;

    for mig in &parsed_migrations {
        return_value += &format!(
            "
            mod {};
            use self::{}::{};
            ",
            mig.value, mig.value, mig.pascal_name
        );
    }

    return_value += r#"
        pub fn migrations() -> MigrationExecutor { 
            MigrationExecutor {
                migrations: vec![
    "#;

    for mig in &parsed_migrations {
        return_value += &format!(
            r#"
            MigrationData {{
                full_name: "{}".to_string(),
                name: "{}".to_string(),
                id: {},
                migration: Box::new({}{{}}),
            }}
            "#,
            mig.value, mig.name, mig.id, mig.pascal_name
        );

        return_value += ",";
    }

    return_value += r#"
        ]
    }}"#;

    return_value.parse().unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_migration_string() {
        let migration_string = "_20241021212814_init_migration";

        let parsed_migration = super::parse_migration_string(migration_string);

        assert_eq!(parsed_migration.value, "_20241021212814_init_migration");
        assert_eq!(parsed_migration.name, "init_migration");
        assert_eq!(parsed_migration.pascal_name, "InitMigration");
        assert_eq!(parsed_migration.id, 20241021212814);
    }
}
