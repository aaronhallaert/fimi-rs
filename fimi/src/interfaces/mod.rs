mod executor;
mod manifest;
pub use executor::*;
use mockall::automock;

pub struct MigrationData {
    pub full_name: String,
    pub name: String,
    pub id: usize,
    pub migration: Box<dyn Migration>,
}

impl MigrationData {
    pub fn up(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.migration.up()
    }

    pub fn down(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.migration.down()
    }
}

#[automock()]
pub trait Migration {
    fn up(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn down(&self) -> Result<(), Box<dyn std::error::Error>>;
}
