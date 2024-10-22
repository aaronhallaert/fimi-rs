use super::manifest;
use super::MigrationData;

pub struct MigrationExecutor {
    pub migrations: Vec<MigrationData>,
}

impl MigrationExecutor {
    fn get_todo_migrations(&self) -> Vec<&MigrationData> {
        let manifest = manifest::get_manifest();

        self.migrations
            .iter()
            .filter(|migration| !manifest.migrations.contains(&migration.full_name))
            .collect()
    }

    fn get_done_migrations(&self) -> Vec<&MigrationData> {
        let manifest = manifest::get_manifest();

        self.migrations
            .iter()
            .filter(|migration| manifest.migrations.contains(&migration.full_name))
            .collect()
    }

    pub fn up(&self) {
        self.get_todo_migrations().iter().for_each(|migration| {
            migration.up();
        })
    }

    pub fn up_with_ids(&self, migration_ids: Vec<usize>) {
        if migration_ids.is_empty() {
            self.migrations.iter().for_each(|mig| mig.up());
            return;
        }

        self.migrations
            .iter()
            .filter(|mig| migration_ids.contains(&mig.id))
            .for_each(|migration| {
                migration.up();
            });
    }

    pub fn down(&self) {
        self.get_done_migrations().iter().for_each(|migration| {
            migration.down();
        })
    }

    pub fn down_with_ids(&self, migration_ids: Vec<usize>) {
        if migration_ids.is_empty() {
            self.migrations.iter().for_each(|mig| mig.down());
            return;
        }

        self.migrations
            .iter()
            .filter(|mig| migration_ids.contains(&mig.id))
            .for_each(|migration| {
                migration.down();
            });
    }
}
