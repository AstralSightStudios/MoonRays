use shaderc;
use std::fs;

pub fn CompileBaseShaders(){
    log::info!("Start compiling base shader...");

    let BaseShadersGetRet = crate::Tools::GetBaseShaders();

    let mut BaseShaders = BaseShadersGetRet.0;

    let mut BaseShadersName = BaseShadersGetRet.1;

    let mut index = 0;
    for _BaseShaderPath in BaseShaders{
        let mut BaseShaderPath = _BaseShaderPath;
        let mut ShaderKindCompile = shaderc::ShaderKind::Vertex;
        // TODO: 支持更多着色器类型
        if(BaseShaderPath.contains("frag")){
            ShaderKindCompile = shaderc::ShaderKind::Fragment;
        }
        log::info!("Compiling base shader {} ...", &BaseShadersName[index]);
        CompileFromGlslFile(BaseShaderPath.to_string(), BaseShadersName[index].to_string(), ShaderKindCompile, false);
        index += 1;
    }
}

pub fn CompileFromGlslFile(GlslPath: String, ShaderInputFileName: String, CompileShaderKind: shaderc::ShaderKind, IsRetryed: bool){
    let GlslPathPtr = Box::into_raw(Box::new(&GlslPath));
    let ShaderInputFileNamePtr = Box::into_raw(Box::new(&ShaderInputFileName));
    let CompileShaderKindPtr = Box::into_raw(Box::new(&CompileShaderKind));

    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();

    let binary_result_match = compiler
        .compile_into_spirv(
            fs::read_to_string(&GlslPath).unwrap().as_str(),
            CompileShaderKind,
            &ShaderInputFileName,
            "main",
            Some(&options),
        );
    
    match(binary_result_match){
        Ok(binary_result) => {
            fs::write(("./cache/shaders/".to_string() + ShaderInputFileName.as_str()).replace(".glsl", ".spirv"), binary_result.as_binary_u8()).unwrap();
        },
        Err(errinfo) => {
            if(!IsRetryed){
                // 或许是运行路径问题，回上一级看看
                CompileFromGlslFile(unsafe { "../".to_string() + (*GlslPathPtr) }, unsafe { (*ShaderInputFileNamePtr).to_string() }, unsafe { **CompileShaderKindPtr }, true);
            }
            else{
                // 还是不行就算了
            }
        },
    }
}