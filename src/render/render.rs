use winit::{event::{WindowEvent, Event, VirtualKeyCode},platform::run_return::EventLoopExtRunReturn, event_loop::ControlFlow};
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
            let mut VkRenderEngine = VKRenderMain::RenderEngineVK::LoadVK();
            //创建用于进行帧等待的Fence
            let FenceCreate = vk::FenceCreateInfo{
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                flags: vk::FenceCreateFlags::SIGNALED,
                ..Default::default()
            };
            let VkFence = vec![
                unsafe { VkRenderEngine.VkDevice.create_fence(&FenceCreate, None).unwrap() }
            ];
            let GraphicsQueue = unsafe { VkRenderEngine.VkDevice.get_device_queue(VkRenderEngine.VkQueueFamilyIndicesGraphicsAndPresent.GraphicsQueueIndex, 0) };
            let PresentQueue = unsafe{ VkRenderEngine.VkDevice.get_device_queue(VkRenderEngine.VkQueueFamilyIndicesGraphicsAndPresent.PresentQueueIndex, 0) };
            log::info!("Got Graphics and Present Queue");
            VkRenderEngine.WinitWindow.1.run_return(move |event, _, control_flow| match event {
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
                Event::MainEventsCleared => {
                    VkRenderEngine.WinitWindow.0.request_redraw();
                }
                Event::RedrawRequested(_) => {
                        // Vulkan帧绘制
                        // 主循环

                        // 等待上一帧完成
                        //log::info!("wait");
                        unsafe { VkRenderEngine.VkDevice.wait_for_fences(&VkFence, true, 18446744073709551615).unwrap() };
                        //log::info!("wait finish");
                        unsafe { VkRenderEngine.VkDevice.reset_fences(&VkFence).unwrap() };
                        //log::info!("reset finish");

                        //log::info!("loop");
                        let ImageIndex = unsafe { VkRenderEngine.VkSwapChain.1.acquire_next_image(VkRenderEngine.VkSwapChain.0, 2100000000, VkRenderEngine.VkSemaphoreImageAvailable, vk::Fence::null()).unwrap() };

                        unsafe { VkRenderEngine.VkDevice.reset_command_buffer(VkRenderEngine.VkCommandBuffers[ImageIndex.0 as usize], vk::CommandBufferResetFlags::empty()).unwrap() };
                        VKRenderMain::VkDrawer::DoDrawTask(&VkRenderEngine.VkDevice, &VkRenderEngine.VkPipeline, &VkRenderEngine.VkViewport, &VkRenderEngine.VkRect2DRenderArea, &VkRenderEngine.VkRenderPass, &VkRenderEngine.VkCommandBuffers, &VkRenderEngine.VkFrameBuffers, &VkRenderEngine.VkSwapChainSettings, ImageIndex.0 as usize);

                        //log::info!("ImageIndex: u32={} bool={}", ImageIndex.0, ImageIndex.1);

                        // 提交指令缓冲
                        let WaitSemaphores = vec![VkRenderEngine.VkSemaphoreImageAvailable];
                        let SignalSemaphores = vec![VkRenderEngine.VkSemaphoreRenderFinished];
                        let WaitStages = vec![vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
                        let SubmitInfo = vec![
                            vk::SubmitInfo{
                                s_type: vk::StructureType::SUBMIT_INFO,
                                wait_semaphore_count: WaitSemaphores.len() as u32,
                                p_wait_semaphores: WaitSemaphores.as_ptr(),
                                p_wait_dst_stage_mask: WaitStages.as_ptr(),
                                command_buffer_count: 1,
                                p_command_buffers: VkRenderEngine.VkCommandBuffers.as_ptr(),
                                signal_semaphore_count: SignalSemaphores.len() as u32,
                                p_signal_semaphores: SignalSemaphores.as_ptr(),
                                ..Default::default()
                            }
                        ];
                
                        unsafe { VkRenderEngine.VkDevice.queue_submit(GraphicsQueue, &SubmitInfo, VkFence[0]).unwrap() };

                        let SwapChains = vec![
                            VkRenderEngine.VkSwapChain.0
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
                
                        unsafe { VkRenderEngine.VkSwapChain.1.queue_present(PresentQueue, &DrawPresentInfo).unwrap() };
                }
                Event::LoopDestroyed => {
                    //unsafe { &LoadedTuples.4.1.destroy_surface(LoadedTuples.4.0, None) };
                }
                _ => {
                        
                }
            });
        }
        else{
            log::error!("Unsupported Render Engine");
        }
    }
}