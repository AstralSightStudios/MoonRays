// 阻止产生无意义警告
#! [allow(non_snake_case)]
#! [allow(unused_mut)]
#! [allow(dead_code)]
#! [allow(unused_variables)]
#! [allow(unused_parens)]
#! [allow(unused_assignments)]
#! [allow(unreachable_code)]

#! [feature(stmt_expr_attributes)]
#! [feature(strict_provenance)]
#! [feature(ptr_metadata)]

# [path="./output/logger/logger.rs"]
mod Logger;
# [path="./base/baseinfolog.rs"]
mod BaseInfoLogger;
# [path="./render/render.rs"]
mod RenderMod;
# [path="./base/directoryinit.rs"]
mod DirectoryInit;
#[path="./tools.rs"]
mod Tools;

use RenderMod::RenderTools;

const GAME_NAME: &str = "MoonRaysEngine SampleGame";
const GAME_VERSION: u32 = 1;
const ENGINE_VERSION: u32 = 1;
const RENDER_ENGINE: RenderMod::RenderEngines = RenderMod::RenderEngines::Vulkan;
const RENDER_VK_CLEAR_COLOR: ash::vk::ClearValue = ash::vk::ClearValue{
    color: ash::vk::ClearColorValue{
        float32: [0.0,0.0,0.0,0.0]
    }
};

fn main() {
    // 初始化基础组件
    // 初始化日志打印模块
    Logger::InitLogger();
    // 打印电脑配置信息
    BaseInfoLogger::Log();
    // 初始化引擎路径
    DirectoryInit::Init();
    // 初始化渲染引擎
    let Render = RenderTools::new(RENDER_ENGINE);
    Render.DoDraw();
}