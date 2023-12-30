// 阻止产生无意义警告
#! [allow(non_snake_case)]
#! [allow(unused_mut)]
#! [allow(dead_code)]
#! [allow(unused_variables)]
#! [allow(unused_parens)]
#! [allow(unused_assignments)]

#! [feature(stmt_expr_attributes)]

# [path="./output/logger/logger.rs"]
mod Logger;
# [path="./base/baseinfolog.rs"]
mod BaseInfoLogger;
# [path="./render/vk/vkmain.rs"]
mod VKRenderMain;

fn main() {
    // 初始化基础组件
    // 初始化日志打印模块
    Logger::InitLogger();
    // 打印电脑配置信息
    BaseInfoLogger::Log();
    // 初始化Vulkan渲染引擎
    let VkReturn = VKRenderMain::CreateInstance();
    VKRenderMain::GetPhysicalDevice(VkReturn.1);
}
