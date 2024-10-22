use std::{
    env,
    fs::{self, File},
};

use super::MigrationData;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub migrations: Vec<String>,
    pub path: String,
}

fn get_manifest_path() -> std::path::PathBuf {
    let mut manifest_path = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::new());
    manifest_path.push(env::var("APPNAME").unwrap_or_else(|_| "fimi".to_string()));
    if !manifest_path.exists() {
        fs::create_dir_all(&manifest_path).expect("Failed to create directories");
    }

    manifest_path.push("migration_manifest.json");

    if !manifest_path.exists() {
        File::create(&manifest_path).expect("Failed to create file");
        let manifest = Manifest {
            migrations: vec![],
            path: manifest_path.to_str().unwrap().to_string(),
        };
        let manifest_str = serde_json::to_string(&manifest).unwrap();
        std::fs::write(&manifest_path, manifest_str).unwrap();
    }

    manifest_path
}

pub fn get_manifest() -> Manifest {
    let manifest_path = get_manifest_path();
    let manifest = std::fs::read_to_string(&manifest_path).unwrap_or_else(|_| String::new());

    match serde_json::from_str(&manifest) {
        Ok(man) => man,
        Err(_) => {
            let manifest = Manifest {
                migrations: vec![],
                path: manifest_path.to_str().unwrap().to_string(),
            };
            write_manifest(&manifest);
            manifest
        }
    }
}

fn write_manifest(manifest: &Manifest) {
    let manifest_path = get_manifest_path();
    let manifest_str = serde_json::to_string(manifest).unwrap();
    std::fs::write(manifest_path, manifest_str).unwrap();
}

pub fn add_to_manifest(migration: &MigrationData) {
    let mut manifest = get_manifest();

    manifest.migrations.push(migration.full_name.clone());

    write_manifest(&manifest);
}

pub fn remove_from_manifest(migration: &MigrationData) {
    let mut manifest = get_manifest();

    println!("Manifest {}: {:?}", manifest.path, manifest.migrations);

    manifest
        .migrations
        .retain(|mig_full_name| mig_full_name == &migration.full_name);

    write_manifest(&manifest);
}

// #[cfg(test)]
// mod test {
//     use crate::{Migration, MigrationData};
//
//     struct MigrationMock;
//
//     impl Migration for MigrationMock {
//         fn up(&self) -> Result<(), Box<dyn std::error::Error>> {
//             println!("Hello, world!");
//             Ok(())
//         }
//
//         fn down(&self) -> Result<(), Box<dyn std::error::Error>> {
//             println!("Goodbye, world!");
//             Ok(())
//         }
//     }
//
//     #[test]
//     fn test_filter_todo() {
//         let mut migrations = vec![MigrationData {
//             full_name: "_123_jipla".to_string(),
//             id: 123,
//             name: "jipla".to_string(),
//             migration: Box::new(MigrationMock {}),
//         }];
//
//         filter_todo(&mut migrations);
//
//         assert_eq!(migrations.len(), 1);
//     }
// }
