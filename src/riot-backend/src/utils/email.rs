use lettre::{message::header, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

pub async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .header(header::ContentType::TEXT_HTML)
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    log::info!("!!!sending email");
    mailer.send(email).await?;

    Ok(())
}
