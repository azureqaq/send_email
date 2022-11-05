use anyhow::{anyhow, Result};
use clap::{crate_name, crate_version, Arg, ArgAction, Command};
use libs::{config::get_config, send_email::send_email};
use platform_dirs::AppDirs;
use simple_logger::SimpleLogger;
use std::{
    fs::{create_dir_all, remove_dir_all},
    path::Path,
};

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    match mma().await {
        Ok(_) => {}
        Err(e) => log::error!("Error: {}", e),
    }
}

async fn mma() -> Result<()> {
    let mat = Command::new(crate_name!())
        .version(crate_version!())
        .about("发送邮件")
        .arg_required_else_help(true)
        // .allow_external_subcommands(true)
        .arg(
            Arg::new("uninstall")
                .long("uninstall")
                .help("删除所有相关文件")
                .action(ArgAction::SetTrue)
                .exclusive(true)
                .num_args(0),
        )
        .arg(
            Arg::new("email")
                .long("email")
                .short('e')
                .action(ArgAction::Set)
                .help("收件人邮箱")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .action(ArgAction::Set)
                .help("要发送的 zip 文件")
                .required(true)
                .num_args(1),
        )
        .get_matches();
    // 配置文件
    let appdir =
        AppDirs::new(Some(crate_name!()), false).ok_or_else(|| anyhow!("无法创建app目录"))?;

    // 如果是卸载
    if mat.get_flag("uninstall") {
        // 卸载
        remove_dir_all(&appdir.config_dir)?;
        log::info!("删除文件夹: {}", &appdir.config_dir.display());
        log::info!("卸载完成");
        return Ok(());
    }

    // 获取参数
    let (Some(mut email_addr), Some(mut email_file)) = (mat.get_raw("email"), mat.get_raw("file")) else {
        return Err(anyhow!("缺少必须参数"));
    };
    let Some(email_addr) = email_addr.next().unwrap().to_str() else {
        return Err(anyhow!("无法解析邮件地址"));
    };
    let Some(email_file) = email_file.next().unwrap().to_str() else {
        return Err(anyhow!("无法解析文件地址"));
    };

    log::debug!("\n邮件地址: {}\n邮件文件: {}", email_addr, email_file);

    // 是否是zip文件
    if !email_file.ends_with(r#".zip"#) {
        return Err(anyhow!("需要一个 zip 文件"));
    }

    let email_file = Path::new(email_file);
    // 判断文件是否存在
    if !email_file.is_file() {
        return Err(anyhow!("所选择的单文件不存在"));
    }

    // 配置文件
    let config_path = appdir.config_dir.join("config.toml");

    // 配置文件的上级目录
    if !appdir.config_dir.is_dir() {
        create_dir_all(&appdir.config_dir)?;
        log::info!("创建文件夹: {}", appdir.config_dir.display());
    }

    // 获取用户配置
    let config = get_config(config_path)?;
    log::debug!("发件人配置: {}", config);

    // 发送邮件

    send_email(&config, email_addr, &email_file).await?;

    Ok(())
}
