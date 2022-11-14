use std::path::Path;
use std::sync::Arc;

use crate::config::UserConfig;

use anyhow::{anyhow, Result};
use lettre::message::header::ContentType;
use lettre::message::Attachment;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_email<P>(config: Arc<UserConfig>, rr_email: String, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let email_file_name = path
        .file_name()
        .ok_or_else(|| anyhow!("无法获取文件名"))?
        .to_str()
        .ok_or_else(|| anyhow!("无法获取文件名"))?;
    let f_e = format!("TNICL_242 <{}>", config.email());
    let r_e = format!("You <{}>", rr_email);

    let fil_body = std::fs::read(path)?;
    let content_type = ContentType::parse("application/pdf").unwrap();
    let att = Attachment::new(email_file_name.into()).body(fil_body, content_type);

    let email = Message::builder()
        .from(f_e.parse()?)
        .to(r_e.parse()?)
        .subject("TJU_TNICL_PL_DATA")
        .singlepart(att)?;
    let creds = Credentials::new(config.email().to_string(), config.pwd().to_string());

    let mailer = SmtpTransport::relay("smtp.qq.com")
        .unwrap()
        .port(465)
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => log::info!("邮件发送成功: {}", rr_email),
        Err(e) => {
            return Err(anyhow!("邮件发送失败: {}", e));
        }
    }
    Ok(())
}
