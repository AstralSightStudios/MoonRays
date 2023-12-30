// 获取CPU名称
pub fn get_cpu_name() -> String {
    // 使用sysinfo库
    use sysinfo::System;
    // 创建一个System实例
    let mut sys = System::new();
    // 刷新CPU信息
    sys.refresh_cpu();
    // 返回CPU名称
    sys.global_cpu_info().name().to_string()
}

// 获取内存大小
pub fn get_memory_size() -> String {
    // 使用sysinfo库
    use sysinfo::System;
    // 创建一个System实例
    let mut sys = System::new();
    // 刷新内存信息
    sys.refresh_memory();
    // 返回内存大小，单位为MB
    format!("{} MB", sys.total_memory() / 1024 / 1024)
}