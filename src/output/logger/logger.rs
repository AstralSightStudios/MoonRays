/*
    logger.rs
    日志工具，这点想必不用多说
*/

use std::fs;

use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub fn InitLogger() {
    // 为了防止log文件太大塞满硬盘（尽管这可能需要很长时间）
    // 在每次启动程序时都要删除上一次启动生成的log文件并重新生成

    if (fs::try_exists("./latest_log.log").unwrap()){
        fs::remove_file("./latest_log.log").unwrap();
    }

    // 定义一个带颜色的info warn error + time输出的格式
    let encoder = PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {h({l})} {M}:{L} - {m}{n}");

    // 定义一个控制台输出器，使用上面的格式
    let console = ConsoleAppender::builder()
        .encoder(Box::new(encoder.clone()))
        .target(Target::Stdout)
        .build();

    // 定义一个文件输出器，使用相同的格式，保存日志到latest_log.log
    let file = FileAppender::builder()
        .encoder(Box::new(encoder))
        .build("latest_log.log")
        .unwrap();

    // 定义一个配置，使用上面的两个输出器，设置日志级别为info
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(Root::builder().appender("console").appender("file").build(LevelFilter::Info))
        .unwrap();

    // 初始化log4rs配置
    log4rs::init_config(config).unwrap();

    // 输出日志系统初始化完毕提示
    log::info!("Engine Logger initialization complete");
}