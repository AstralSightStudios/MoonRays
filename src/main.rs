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

use winit::{event::{WindowEvent, Event, VirtualKeyCode}, event_loop::ControlFlow};

# [path="./output/logger/logger.rs"]
mod Logger;
# [path="./base/baseinfolog.rs"]
mod BaseInfoLogger;
# [path="./render/vk/vkmain.rs"]
mod VKRenderMain;

const GAME_NAME: &str = "MoonRaysEngine SampleGame";
const GAME_VERSION: u32 = 1;
const ENGINE_VERSION: u32 = 1;

fn main() {
    // 初始化基础组件
    // 初始化日志打印模块
    Logger::InitLogger();
    // 打印电脑配置信息
    BaseInfoLogger::Log();
    // 初始化Vulkan渲染引擎
    let LoadedTuple = VKRenderMain::LoadVK();

    LoadedTuple.0.1.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent {
            event:
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                },
            window_id: _,
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::LoopDestroyed => {
            unsafe { LoadedTuple.4.1.destroy_surface(LoadedTuple.4.0, None) };
        }
        _ => {}
    })
}