use fimi::Migration;

pub struct AddNewKeyToTestStructs;

mod second_migi_structs {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct NewTestStruct {
        pub epic_key: String,
        pub new_key: usize,
    }

    impl From<OldTestStruct> for NewTestStruct {
        fn from(value: OldTestStruct) -> Self {
            Self {
                epic_key: value.epic_key,
                new_key: 24,
            }
        }
    }

    impl From<NewTestStruct> for OldTestStruct {
        fn from(value: NewTestStruct) -> Self {
            Self {
                epic_key: value.epic_key,
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct OldTestStruct {
        pub epic_key: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MyOldTestStructs {
        pub test_structs: Vec<OldTestStruct>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MyNewTestStructs {
        pub test_structs: Vec<NewTestStruct>,
    }

    impl From<MyOldTestStructs> for MyNewTestStructs {
        fn from(value: MyOldTestStructs) -> Self {
            let mut new_test_structs: Vec<NewTestStruct> = vec![];
            for s in value.test_structs {
                new_test_structs.push(s.into())
            }

            Self {
                test_structs: new_test_structs,
            }
        }
    }

    impl From<MyNewTestStructs> for MyOldTestStructs {
        fn from(value: MyNewTestStructs) -> Self {
            let mut new_test_structs: Vec<OldTestStruct> = vec![];
            for s in value.test_structs {
                new_test_structs.push(s.into())
            }

            Self {
                test_structs: new_test_structs,
            }
        }
    }
}

impl Migration for AddNewKeyToTestStructs {
    fn up(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut my_test_structs_path = dirs::data_dir().unwrap_or_default();
        my_test_structs_path.push("fimi_example");
        my_test_structs_path.push("my_test_structs.json");

        let old_test_structs = crate::load_json!(
            my_test_structs_path.clone(),
            second_migi_structs::MyOldTestStructs
        )?;

        let new_test_structs: second_migi_structs::MyNewTestStructs = old_test_structs.into();

        let my_test_structs_json = serde_json::to_string(&new_test_structs)?;
        std::fs::write(&my_test_structs_path, my_test_structs_json)?;

        Ok(())
    }

    fn down(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut my_test_structs_path = dirs::data_dir().unwrap_or_default();
        my_test_structs_path.push("fimi_example");
        my_test_structs_path.push("my_test_structs.json");

        let new_test_structs = crate::load_json!(
            my_test_structs_path.clone(),
            second_migi_structs::MyNewTestStructs
        )?;

        let old_test_structs: second_migi_structs::MyOldTestStructs = new_test_structs.into();

        let my_test_structs_json = serde_json::to_string(&old_test_structs)?;
        std::fs::write(&my_test_structs_path, my_test_structs_json)?;
        Ok(())
    }
}
