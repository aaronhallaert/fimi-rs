use super::manifest::{ManifestManager, ManifestManagerTrait};
use super::MigrationData;

pub struct MigrationExecutor {
    pub migrations: Vec<MigrationData>,
    manifest_manager: Box<dyn ManifestManagerTrait>,
}

impl MigrationExecutor {
    pub fn new(migrations: Vec<MigrationData>) -> Self {
        Self {
            migrations,
            manifest_manager: Box::new(ManifestManager::new()),
        }
    }
    fn up_migration(&self, migration: &MigrationData) {
        match migration.up() {
            Ok(_) => {
                println!("`up` - Migration done: {}", migration.full_name);
                self.manifest_manager.add_to_manifest(migration);
            }
            Err(e) => {
                eprintln!(
                    "Error while running `up` migration: {} - {}",
                    migration.full_name, e
                );
            }
        }
    }

    fn down_migration(&self, migration: &MigrationData) {
        match migration.down() {
            Ok(_) => {
                println!("`down` - Migration done: {}", migration.full_name);
                self.manifest_manager.remove_from_manifest(migration);
            }
            Err(e) => {
                eprintln!(
                    "Error while running `down` migration: {} - {}",
                    migration.full_name, e
                );
            }
        }
    }

    fn get_todo_migrations(&self) -> Vec<&MigrationData> {
        let manifest = self.manifest_manager.get_manifest();

        self.migrations
            .iter()
            .filter(|migration| !manifest.contains(migration))
            .collect()
    }

    fn get_done_migrations(&self) -> Vec<&MigrationData> {
        let manifest = self.manifest_manager.get_manifest();

        self.migrations
            .iter()
            .filter(|migration| manifest.contains(migration))
            .collect()
    }

    pub fn up(&self) {
        self.get_todo_migrations().iter().for_each(|migration| {
            self.up_migration(migration);
        })
    }

    pub fn up_with_ids(&self, migration_ids: Vec<usize>) {
        if migration_ids.is_empty() {
            self.up();
            return;
        }

        self.migrations
            .iter()
            .filter(|mig| migration_ids.contains(&mig.id))
            .for_each(|migration| {
                self.up_migration(migration);
            });
    }

    pub fn down(&self) {
        self.get_done_migrations().iter().for_each(|migration| {
            self.down_migration(migration);
        })
    }

    pub fn down_with_ids(&self, migration_ids: Vec<usize>) {
        if migration_ids.is_empty() {
            self.down();
            return;
        }

        self.migrations
            .iter()
            .filter(|mig| migration_ids.contains(&mig.id))
            .for_each(|migration| {
                self.down_migration(migration);
            });
    }
}
