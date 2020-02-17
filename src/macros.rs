macro_rules! system {
    ($($json:tt)+) => {{
        let command = serde_json::json!({"system":$($json)+});
        let bytes = serde_json::to_vec(&command).expect("invalid json");
        crypto::encrypt(&bytes)
    }};
}
