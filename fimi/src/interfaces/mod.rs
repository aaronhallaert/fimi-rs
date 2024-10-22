mod executor;
mod manifest;
pub use executor::*;

pub struct MigrationData {
    pub full_name: String,
    pub name: String,
    pub id: usize,
    pub migration: Box<dyn Migration>,
}

impl MigrationData {
    pub fn up(&self) {
        match self.migration.up() {
            Ok(_) => {
                manifest::add_to_manifest(self);
                println!("UP {}: ✅", self.full_name);
            }
            Err(e) => {
                println!("UP {}: ❌", self.full_name);
                println!("\t {}", e);
            }
        };
    }

    pub fn down(&self) {
        match self.migration.down() {
            Ok(_) => {
                manifest::remove_from_manifest(self);
                println!("DOWN {}: ✅", self.full_name);
            }
            Err(e) => {
                println!("DOWN {}: ❌", self.full_name);
                println!("\t {}", e);
            }
        };
    }
}

pub trait Migration {
    fn up(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn down(&self) -> Result<(), Box<dyn std::error::Error>>;
}
