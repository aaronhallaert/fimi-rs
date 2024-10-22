use serde_json::Value;

mod migrations;

fn init_app() {
    let executor = migrations::migrations();
    executor.up();
}

fn main() {
    init_app();

    // my epic program (printing out config files)
    let mut my_test_structs_path = dirs::data_dir().unwrap_or_default();
    my_test_structs_path.push("fimi_example");
    my_test_structs_path.push("my_test_structs.json");

    println!("{}", my_test_structs_path.to_str().unwrap());
    let json_content =
        std::fs::read_to_string(&my_test_structs_path).unwrap_or_else(|_| String::new());

    match serde_json::from_str::<Value>(&json_content) {
        Ok(json_value) => {
            let pretty_json = serde_json::to_string_pretty(&json_value).unwrap();
            println!("{}", pretty_json);
        }
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }
}
