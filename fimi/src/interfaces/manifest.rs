use super::MigrationData;
use chrono::NaiveDateTime;
use mockall::automock;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
};

#[automock]
pub trait ManifestIOTrait {
    fn get_manifest_path(&self) -> std::path::PathBuf;
    fn read_manifest(&self) -> Manifest;
    fn write_manifest(&self, manifest: &Manifest);
}

struct ManifestIO;

impl ManifestIOTrait for ManifestIO {
    fn get_manifest_path(&self) -> std::path::PathBuf {
        let mut manifest_path = dirs::data_dir().unwrap_or_default();
        manifest_path.push(env::var("APPNAME").unwrap_or_else(|_| "fimi".to_string()));
        if !manifest_path.exists() {
            fs::create_dir_all(&manifest_path).expect("Failed to create manifest directories");
        }

        manifest_path.push("migration_manifest.json");

        if !manifest_path.exists() {
            File::create(&manifest_path).expect("Failed to create file");
            let manifest = Manifest {
                migrations: HashSet::new(),
                path: manifest_path.to_str().unwrap().to_string(),
            };
            let manifest_str = serde_json::to_string(&manifest).unwrap();
            std::fs::write(&manifest_path, manifest_str).unwrap();
        }

        manifest_path
    }

    fn read_manifest(&self) -> Manifest {
        let manifest_path = self.get_manifest_path();
        let manifest = std::fs::read_to_string(&manifest_path).unwrap_or_else(|_| String::new());

        match serde_json::from_str(&manifest) {
            Ok(man) => man,
            Err(_) => {
                let manifest = Manifest {
                    migrations: HashSet::new(),
                    path: manifest_path.to_str().unwrap().to_string(),
                };
                self.write_manifest(&manifest);
                manifest
            }
        }
    }

    fn write_manifest(&self, manifest: &Manifest) {
        let manifest_path = self.get_manifest_path();
        let manifest_str = serde_json::to_string(manifest).unwrap();
        std::fs::write(manifest_path, manifest_str).unwrap();
    }
}

pub trait ManifestManagerTrait {
    fn get_manifest(&self) -> Manifest;
    fn add_to_manifest(&self, migration: &MigrationData);
    fn remove_from_manifest(&self, migration: &MigrationData);
}

pub struct ManifestManager {
    io: Box<dyn ManifestIOTrait>,
}

impl ManifestManager {
    pub fn new() -> Self {
        Self {
            io: Box::new(ManifestIO {}),
        }
    }
}

impl ManifestManagerTrait for ManifestManager {
    fn get_manifest(&self) -> Manifest {
        self.io.read_manifest()
    }
    fn add_to_manifest(&self, migration: &MigrationData) {
        let mut manifest = self.io.read_manifest();

        manifest.migrations.insert(migration.into());

        self.io.write_manifest(&manifest);
    }

    fn remove_from_manifest(&self, migration: &MigrationData) {
        let mut manifest = self.io.read_manifest();

        manifest
            .migrations
            .retain(|mig| mig.full_name != migration.full_name);

        self.io.write_manifest(&manifest);
    }
}

#[derive(Serialize, Deserialize, Eq, Debug)]
struct MaterializedMigration {
    pub full_name: String,
    pub executed: NaiveDateTime,
}

impl PartialEq<MaterializedMigration> for MaterializedMigration {
    fn eq(&self, other: &MaterializedMigration) -> bool {
        self.full_name == other.full_name
    }
}
impl std::hash::Hash for MaterializedMigration {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.full_name.hash(state);
    }
}

impl From<&MigrationData> for MaterializedMigration {
    fn from(migration: &MigrationData) -> Self {
        Self {
            full_name: migration.full_name.clone(),
            executed: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize, Deserialize, Eq, Debug)]
pub struct Manifest {
    migrations: HashSet<MaterializedMigration>,
    pub path: String,
}

impl Manifest {
    pub fn contains(&self, migration: &MigrationData) -> bool {
        self.migrations.contains(&migration.into())
    }
}

impl PartialEq<Manifest> for Manifest {
    fn eq(&self, other: &Manifest) -> bool {
        self.migrations == other.migrations
    }
}

#[cfg(test)]
mod test {
    use crate::interfaces::MockMigration;

    use super::*;

    #[test]
    fn test_add_to_manifest() {
        let migration = MigrationData {
            full_name: "_1234_init_migration".to_string(),
            name: "init_migration".to_string(),
            id: 1234,
            migration: Box::new(MockMigration::new()),
        };

        let mut mock_io = MockManifestIOTrait::new();
        mock_io
            .expect_read_manifest()
            .times(1)
            .returning(|| Manifest {
                migrations: HashSet::new(),
                path: "".to_string(),
            });

        let mut expected_write_set = HashSet::new();
        expected_write_set.insert(MaterializedMigration {
            full_name: "_1234_init_migration".to_string(),
            executed: chrono::Utc::now().naive_utc(),
        });

        mock_io
            .expect_write_manifest()
            .times(1)
            .with(mockall::predicate::eq(Manifest {
                migrations: expected_write_set,
                path: "".to_string(),
            }))
            .returning(|_| ());

        let manifest_manager = ManifestManager {
            io: Box::new(mock_io),
        };

        manifest_manager.add_to_manifest(&migration);
    }

    #[test]
    fn test_remove_from_manifest() {
        let migration = MigrationData {
            full_name: "_1234_init_migration".to_string(),
            name: "init_migration".to_string(),
            id: 1234,
            migration: Box::new(MockMigration::new()),
        };

        let mut mock_io = MockManifestIOTrait::new();
        mock_io.expect_read_manifest().times(1).returning(|| {
            let mut expected_read_set = HashSet::new();
            expected_read_set.insert(MaterializedMigration {
                full_name: "_1234_init_migration".to_string(),
                executed: chrono::Utc::now().naive_utc(),
            });
            Manifest {
                migrations: expected_read_set,
                path: "".to_string(),
            }
        });

        mock_io
            .expect_write_manifest()
            .times(1)
            .with(mockall::predicate::eq(Manifest {
                migrations: HashSet::new(),
                path: "".to_string(),
            }))
            .returning(|_| ());

        let manifest_manager = ManifestManager {
            io: Box::new(mock_io),
        };

        manifest_manager.remove_from_manifest(&migration);
    }
}
