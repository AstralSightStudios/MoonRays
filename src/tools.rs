/*
    tools.rs
    提供一些进行类型转换的快捷函数
*/

pub fn veci8_to_string(veci8: Vec<i8>) -> String{
    let vecu8: Box<[u8]> = veci8.into_iter().map(|x| x as u8).collect();
    let string_ret = String::from_utf8_lossy(&vecu8).to_string();
    return string_ret
}