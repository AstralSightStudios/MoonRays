/*
    tools.rs
    提供一些进行类型转换的快捷函数
*/

use std::ffi::CStr;

pub fn veci8_to_string(veci8: Vec<i8>) -> String{
    let vecu8: Box<[u8]> = veci8.into_iter().map(|x| x as u8).collect();
    let string_ret = String::from_utf8_lossy(&vecu8).to_string();
    return string_ret
}

pub fn vecstaticcstr_to_constconsti8(in_vec:Vec<&'static CStr>) -> *const *const i8{
    let p = in_vec.into_iter().map(CStr::as_ptr).collect::<Vec<_>>().as_ptr();
    return p
}