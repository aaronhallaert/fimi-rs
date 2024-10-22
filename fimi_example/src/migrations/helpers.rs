#[macro_export]
macro_rules! load_json {
    ($path:expr, $struct_type:ty) => {{
        let mut path = dirs::data_dir().unwrap_or_default();
        path.push($path);
        let file_content = std::fs::read_to_string(&path).unwrap_or_else(|_| String::new());
        let result: Result<$struct_type, serde_json::Error> = serde_json::from_str(&file_content);

        result
    }};
}
