use fimi::Migration;
use std::fs::{self, File};

mod init_migration_structs {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct TestStruct {
        pub epic_key: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MyTestStructs {
        pub test_structs: Vec<TestStruct>,
    }
}

pub struct InitMigration;

impl Migration for InitMigration {
    fn up(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut my_test_structs_path = dirs::data_dir().unwrap_or_default();
        my_test_structs_path.push("fimi_example");
        if !my_test_structs_path.exists() {
            fs::create_dir_all(&my_test_structs_path).expect("Failed to create directories");
        }
        my_test_structs_path.push("my_test_structs.json");

        if !my_test_structs_path.exists() {
            File::create(&my_test_structs_path).expect("Failed to create file");
            let my_test_structs = init_migration_structs::MyTestStructs {
                test_structs: vec![
                    init_migration_structs::TestStruct {
                        epic_key: "epic_value".to_string(),
                    },
                    init_migration_structs::TestStruct {
                        epic_key: "epic_value".to_string(),
                    },
                    init_migration_structs::TestStruct {
                        epic_key: "epic_value".to_string(),
                    },
                ],
            };
            let my_test_structs_json = serde_json::to_string(&my_test_structs).unwrap();
            std::fs::write(&my_test_structs_path, my_test_structs_json).unwrap();
        }

        Ok(())
    }

    fn down(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
