/*
    sysinfo.rs
    获取系统在软件层面上的一些信息
*/

pub fn get_system_name() -> String {
    return std::env::consts::OS.to_string();
}