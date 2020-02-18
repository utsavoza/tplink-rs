macro_rules! system {
    ($($json:tt)+) => (serde_json::json!({"system":$($json)+}))
}
