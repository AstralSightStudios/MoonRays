pub fn get_system_name() -> String {
    return std::env::consts::OS.to_string();
}