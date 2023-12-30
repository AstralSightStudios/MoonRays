/*
    baseinfolog.rs
    在启动引擎时输出一些信息
*/

use log;
# [path = "../hardwaretools/hwinfoget.rs"]
mod HardwareInfoGetTools;

pub fn Log(){
    // 输出基本硬件信息到log
    // 这样在收到用户的错误报告后可以第一时间判断是否是硬件导致的
    log::info!("Computer hardware information: CPU: {} Memory Size: {}", HardwareInfoGetTools::get_cpu_name(), HardwareInfoGetTools::get_memory_size());
}