use std::{ffi::CStr, fs};

use ash::{self, vk::{self, ShaderModule, PipelineShaderStageCreateInfo, ShaderStageFlags}};

pub fn CreateVkShaderModule(VkDevice: &ash::Device, ShaderSpirvBin:Vec<u32>) -> ShaderModule{
    let VK_SHADER_MODULE_CREATE_INFO_DEFAULT = vk::ShaderModuleCreateInfo{
        s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
        code_size: ShaderSpirvBin.len() * 4,
        p_code: ShaderSpirvBin.as_ptr(),
        ..Default::default()
    };

    let VkShaderModule = unsafe { VkDevice.create_shader_module(&VK_SHADER_MODULE_CREATE_INFO_DEFAULT, None).unwrap() };
    return VkShaderModule;
}

pub fn CreateVkPipelineShaderStage(VkShaderModule: ShaderModule, ShaderStageFlag: ShaderStageFlags) -> PipelineShaderStageCreateInfo{
    // 踩坑（已修复）：不要使用CString，而是使用CStr，因为CString会导致生命周期问题
    let p_name_cstr = CStr::from_bytes_with_nul(b"main\0").unwrap();
    let P_NAME = p_name_cstr.as_ptr();
    let VK_PIPLINE_SHADER_STAGE_CREATE_INFO = PipelineShaderStageCreateInfo{
        s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
        stage: ShaderStageFlag,
        module: VkShaderModule,
        p_name: P_NAME,
        ..Default::default()
    };

    return VK_PIPLINE_SHADER_STAGE_CREATE_INFO;
}

pub fn GetBaseShadersPipelineShaderStage(VkDevice: &ash::Device) -> Vec<PipelineShaderStageCreateInfo>{
    let SpirVBaseShadersPathList = crate::Tools::GetBaseCompiledSpirVShaders();

    let mut SpirVBaseShaderStages: Vec<PipelineShaderStageCreateInfo> = vec![];

    for ShaderPath in SpirVBaseShadersPathList{
        let TempShaderBin = crate::Tools::vec_u8_to_vec_u32(fs::read(&ShaderPath).unwrap());
        log::info!("Getting Base Shader {} 's Pipeline Shader, File Size={}", &ShaderPath, &TempShaderBin.len() * 4);
        let TempShaderModule = CreateVkShaderModule(VkDevice, TempShaderBin);
        let mut TempShaderStageFlag = ShaderStageFlags::VERTEX;
        // TODO: 支持更多着色器类型
        if(ShaderPath.contains("frag")){
            TempShaderStageFlag = ShaderStageFlags::FRAGMENT;
        }
        let PipelineShaderStageCreateRet = CreateVkPipelineShaderStage(TempShaderModule, TempShaderStageFlag);
        SpirVBaseShaderStages.push(PipelineShaderStageCreateRet);
    }

    return SpirVBaseShaderStages;
}