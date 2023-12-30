#! [allow(non_snake_case)]
#! [feature(stmt_expr_attributes)]

# [path="./output/logger/logger.rs"]
mod logger;
# [path="./base/baseinfolog.rs"]
mod BaseInfoLogger;
# [path="./render/vk/vkmain.rs"]
mod VKRenderMain;

fn main() {
    // 初始化基础组件
    // 初始化日志打印模块
    logger::InitLogger();
    // 打印电脑配置信息
    BaseInfoLogger::Log();
    // 初始化Vulkan渲染引擎
    VKRenderMain::Init();
}
