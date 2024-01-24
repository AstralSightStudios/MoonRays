// 阻止产生无意义警告，并进行一个乐子行为的喷

// 傻逼Rust要求snake_case使我怒气冲天 看看这有多糟糕：suck_your_dick_plz 你喜欢吗？
#! [allow(non_snake_case)]
// 傻逼Rust还要管我创建的mutable值有没有被修改简直侵犯我隐私权 上辈子你早该默认mutable的不是吗？
#! [allow(unused_mut)]
// 傻逼Rust还要管我写过的函数有没有用上 你女朋友把你甩了后有没有被破处怎么不去嗅一嗅？
#! [allow(dead_code)]
// 傻逼Rust还要管我创建的变量有没有用上真他妈脑残 你爹爆金币了要不去舔一舔？
#! [allow(unused_variables)]
// 傻逼Rust还要管我if带不带括号真唐氏综合症 我觉得这么写漂亮你狗叫你妈逼呢？
#! [allow(unused_parens)]
// 傻逼Rust还要管我有没有读取我的变量实在是太多管闲事了 我读你妈祭日了？
#! [allow(unused_assignments)]
// 傻逼Rust在return后写代码怎么你了 你不知道能跑就行是行业基准？
#! [allow(unreachable_code)]

#! [feature(stmt_expr_attributes)]
#! [feature(strict_provenance)]
#! [feature(ptr_metadata)]
#! [feature(fs_try_exists)]

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