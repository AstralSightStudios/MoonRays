use winit::{event::{WindowEvent, Event, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop}, window::Window, platform::run_return::EventLoopExtRunReturn};
use ash::{self, vk};

#[path="./vk/vkmain.rs"]
mod VKRenderMain;

#[derive(PartialEq, Clone, Copy)]
pub enum RenderEngines{
    Vulkan,
    Null
}

#[derive(Clone, Copy)]
pub struct RenderTools{
    RenderEngine: RenderEngines
}

impl RenderTools{
    pub fn new(RenderEngine: RenderEngines) -> RenderTools{
        RenderTools{
            RenderEngine
        }
    }

    pub fn DoDraw(self){
        if(self.RenderEngine == RenderEngines::Vulkan){
            let mut VkLoadTuple: ((Window, EventLoop<()>), (ash::Entry, ash::Instance), vk::PhysicalDevice, (ash::Device, (u32, u32)), (vk::SurfaceKHR, ash::extensions::khr::Surface), (vk::SurfaceCapabilitiesKHR, vk::Extent2D, vk::SurfaceFormatKHR, vk::PresentModeKHR), (vk::SwapchainKHR, ash::extensions::khr::Swapchain), Vec<ash::vk::Image>, Vec<vk::ImageView>, Vec<vk::PipelineShaderStageCreateInfo>, (Vec<vk::Pipeline>, vk::RenderPass, Vec<vk::Viewport>, Vec<vk::Rect2D>), Vec<vk::Framebuffer>, vk::CommandPool, Vec<vk::CommandBuffer>, (vk::Semaphore, vk::Semaphore)) = VKRenderMain::LoadVK();
            let GraphicsQueue = unsafe { VkLoadTuple.3.0.get_device_queue(VkLoadTuple.3.1.0, 0) };
            let PresentQueue = unsafe{ VkLoadTuple.3.0.get_device_queue(VkLoadTuple.3.1.1, 0) };
            log::info!("Got Graphics and Present Queue");
            VkLoadTuple.0.1.run_return(move |event, _, control_flow| match event {
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
                }
                 => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::RedrawRequested(_) => {
                        // Vulkan帧绘制
                        // 主循环
                        //log::info!("loop");
                        let ImageIndex = unsafe { VkLoadTuple.6.1.acquire_next_image(VkLoadTuple.6.0, 2100000000, VkLoadTuple.14.0, vk::Fence::null()).unwrap() };

                        //log::info!("ImageIndex: u32={} bool={}", ImageIndex.0, ImageIndex.1);

                        // 提交指令缓冲
                        let WaitSemaphores = vec![VkLoadTuple.14.0];
                        let SignalSemaphores = vec![VkLoadTuple.14.1];
                        let WaitStages = vec![vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
                        let SubmitInfo = vec![
                            vk::SubmitInfo{
                                s_type: vk::StructureType::SUBMIT_INFO,
                                wait_semaphore_count: WaitSemaphores.len() as u32,
                                p_wait_semaphores: WaitSemaphores.as_ptr(),
                                p_wait_dst_stage_mask: WaitStages.as_ptr(),
                                command_buffer_count: 1,
                                p_command_buffers: VkLoadTuple.13.as_ptr(),
                                signal_semaphore_count: SignalSemaphores.len() as u32,
                                p_signal_semaphores: SignalSemaphores.as_ptr(),
                                ..Default::default()
                            }
                        ];
                
                        unsafe { VkLoadTuple.3.0.queue_submit(GraphicsQueue, &SubmitInfo, vk::Fence::null()).unwrap() };

                        let SwapChains = vec![
                            VkLoadTuple.6.0
                        ];

                        let DrawPresentInfo = vk::PresentInfoKHR{
                            s_type: vk::StructureType::PRESENT_INFO_KHR,
                            wait_semaphore_count: SignalSemaphores.len() as u32,
                            p_wait_semaphores: SignalSemaphores.as_ptr(),
                            swapchain_count: SwapChains.len() as u32,
                            p_swapchains: SwapChains.as_ptr(),
                            p_image_indices: &ImageIndex.0,
                            ..Default::default()
                        };

                        //log::info!("Infos was created");

                        //log::info!("SwapChain={:?}", &DrawPresentInfo.p_swapchains.wrapping_add(0));
                
                        unsafe { VkLoadTuple.6.1.queue_present(PresentQueue, &DrawPresentInfo).unwrap() };
                }
                Event::LoopDestroyed => {
                    //unsafe { &LoadedTuples.4.1.destroy_surface(LoadedTuples.4.0, None) };
                }
                _ => {
                        
                }
            });
        }
    }
}