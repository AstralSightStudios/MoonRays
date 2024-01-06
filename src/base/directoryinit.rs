use std::path::Path;
use std::fs;

pub fn Init(){
    let CheckDirectorys = vec![
        "./cache",
        "./cache/shaders"
    ];

    for CheckDirecroty in CheckDirectorys{
        if(!Path::new(CheckDirecroty).exists()){
            log::info!("Path \"{}\" does not exist, creating...", &CheckDirecroty);
            fs::create_dir(CheckDirecroty).unwrap();
        }
    }
}