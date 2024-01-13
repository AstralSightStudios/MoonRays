/*
    tools.rs
    提供一些进行类型转换的快捷函数
*/

use std::{ffi::CStr, fs};

pub fn veci8_to_string(veci8: Vec<i8>) -> String{
    let vecu8: Box<[u8]> = veci8.into_iter().map(|x| x as u8).collect();
    let string_ret = String::from_utf8_lossy(&vecu8).to_string();
    return string_ret
}

pub fn vecstaticcstr_to_constconsti8(in_vec:Vec<&'static CStr>) -> *const *const i8{
    let p = in_vec.into_iter().map(CStr::as_ptr).collect::<Vec<_>>().as_ptr();
    return p
}

// 定义一个函数，接受一个Vec<u8>作为参数，返回一个Vec<u32>
pub fn vec_u8_to_vec_u32(v: Vec<u8>) -> Vec<u32> {
    // 判断输入的向量的长度是否是4的倍数，如果不是，返回错误
    if v.len() % 4 != 0 {
        log::error!("Error at Parse Vecu8 to Vecu32");
        panic!("Error at Parse Vecu8 to Vecu32");
    }
    // 使用chunks_exact方法，把向量分割成每4个元素一组的切片
    // 使用map方法，把每个切片转换成一个数组
    // 使用try_into方法，尝试把数组转换成u32类型，如果失败，返回错误
    // 使用from_le_bytes方法，把字节数组按照小端序解析成u32数字
    // 使用collect方法，把迭代器收集成一个向量
    v.chunks_exact(4)
        .map(|s| s.try_into().unwrap())
        .map(|a| u32::from_le_bytes(a))
        .collect()
}

pub fn GetBaseShaders() -> (Vec<String>, Vec<String>){
    let mut BaseShaders = vec![];

    let mut BaseShadersName = vec![];

    // 读取shader目录下的所有文件，并遍历
    for entry in fs::read_dir("shaders").unwrap() {
        // 获取文件的路径
        let path = entry.unwrap().path();
        // 判断文件是否以base开头，并且是.glsl文件
        if path.file_name().unwrap().to_str().unwrap().starts_with("base") && path.extension().unwrap() == "glsl" {
            // 把相对路径放进BaseShaders里
            BaseShaders.push(path.to_str().unwrap().to_string());
            // 去掉shaders/，把文件名放进BaseShadersName里
            BaseShadersName.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }

    return (BaseShaders, BaseShadersName)
}

pub fn GetBaseCompiledSpirVShaders() -> Vec<String>{
    let mut BaseSpirVShaders = vec![];

    // 读取shader目录下的所有文件，并遍历
    for entry in fs::read_dir("cache/shaders").unwrap() {
        // 获取文件的路径
        let path = entry.unwrap().path();
        // 判断文件是否以base开头，并且是.glsl文件
        if path.file_name().unwrap().to_str().unwrap().starts_with("base") && path.extension().unwrap() == "spirv" {
            // 把相对路径放进BaseShaders里
            BaseSpirVShaders.push(path.to_str().unwrap().to_string());
        }
    }

    return BaseSpirVShaders
}