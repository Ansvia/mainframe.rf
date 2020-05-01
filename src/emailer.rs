//! Module ini berisi fungsi yang berhubungan dengan pembuatan dan pengiriman email.

extern crate chrono_tz;

use chrono::{Datelike, NaiveDate, NaiveDateTime, TimeZone, Timelike};
use chrono_tz::Asia::Jakarta;
use chrono_tz::UTC;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{smtp, ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre_email::{Email, EmailBuilder, Mailbox};
use native_tls::{Protocol, TlsConnector};
use std::env;

use crate::prelude::*;

/// Kirim email
pub fn send_email(builder: EmailBuilder) {
    if let Ok(port) = env::var("EMAIL_PORT")
        .unwrap_or_else(|_| "25".to_string())
        .parse::<u16>()
    {
        debug!("SMTP port {}", port);
        if port != 0 {
            let login_name = env::var("SMTP_LOGIN_NAME").expect("SMTP_LOGIN_NAME not set");
            let login_password = env::var("SMTP_LOGIN_PASSWORD").expect("SMTP_LOGIN_PASSWORD not set");
            debug!("SMTP credential: {} : {}...", login_name, &login_password[0..5]);

            let creds = Credentials::new(login_name, login_password);
            let mail_server = env::var("EMAIL_SERVER").unwrap_or_else(|_| "localhost".to_string());
            debug!("mail_server: {}", mail_server);
            // Open connection
            let mut mailer = match port {
                smtp::SMTP_PORT => {
                    SmtpClient::new((mail_server.as_str(), smtp::SMTP_PORT), ClientSecurity::None)
                        .unwrap()
                        .credentials(creds)
                        .transport()
                }
                smtp::SUBMISSIONS_PORT => {
                    let mut tls_builder = TlsConnector::builder();
                    tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
                    let tls_parameters =
                        ClientTlsParameters::new(mail_server.to_string(), tls_builder.build().unwrap());

                    SmtpClient::new(
                        (mail_server.as_str(), smtp::SUBMISSIONS_PORT),
                        ClientSecurity::Wrapper(tls_parameters),
                    )
                    .unwrap()
                    .credentials(creds)
                    .transport()
                }
                _ => panic!("EMAIL PORT not set, eg: 25 or 465"),
            };

            // Send the email
            let email = builder.build().expect("Cannot build email");
            let result = mailer.send(email.into());

            if result.is_ok() {
                debug!("Email sent");
            } else {
                warn!("Could not send email: {:?}", result);
            }

            mailer.close()
        }
    }
}

/// Convert time from UTC to local timezone Asia/Jakarta.
fn convert_to_local_time(date: NaiveDateTime) -> String {
    let date_time = UTC
        .ymd(date.year(), date.month(), date.day())
        .and_hms(date.hour(), date.minute(), 0);

    date_time.with_timezone(&Jakarta).to_string()
}

/// Get domain base url from file .env
fn get_base_url() -> String {
    // @TODO(you): Implement your url builder to your base web here
    env::var("WEB_BASE_URL").unwrap_or_else(|_| "localhost".to_string())
}

fn gen_email_sender(name: &str, name_address: &str) -> Mailbox {
    format!(
        "{} <{}@{}>",
        name,
        name_address,
        env::var("EMAIL_DOMAIN").unwrap_or_else(|_| "localhost.net".to_string())
    )
    .parse()
    .unwrap()
}

/// Email builder ketika user melakukan self register.
pub fn register_$param.service_name_snake_case$(email_target: String, name: String, code: String, expiration: NaiveDateTime) {
    let date = convert_to_local_time(expiration);
    let body_html = format!(
        "<p>Hi, {}</p><p>Terima kasih telah melakukan pendaftaran akun pada aplikasi $name$.</p>
            <p>Masukkan kode verifikasi berikut pada aplikasi $name$.</p><p>{}</p>
            <p>Kode verifikasi akun Anda akan kadaluarsa pada {}.</p>",
        name, code, date
    );

    let builder = Email::builder()
        .to(email_target.to_owned())
        .from(gen_email_sender("Verifikasi", "verify"))
        .subject("ACCOUNT VERIFICATION")
        .html(body_html);

    send_email(builder)
}

/// Email builder ketika user melakukan reset password.
pub fn reset_password(email_target: String, name: String, code: String, expiration: NaiveDateTime) {
    let date = convert_to_local_time(expiration);
    let body_html = format!(
        "<p>Hi, {}</p><p>Kami telah menerima permintaan Anda untuk reset kata sandi akun $name$ Anda.</p>
            <p>Masukkan kode verifikasi berikut pada aplikasi $name$.</p><p>{}</p>
            <p>Kode verifikasi untuk atur ulang kata sandi Anda akan kadaluarsa pada {}.</p>",
        name, code, date
    );

    dbg!(gen_email_sender("Verifikasi", "verify"));

    let builder = Email::builder()
        .to(email_target.to_owned())
        .from(gen_email_sender("Verifikasi", "verify"))
        .subject("RESET PASSWORD")
        .html(body_html);

    send_email(builder)
}

/// Email builder ketika user melakukan reset password dan mengirimkan link.
pub fn reset_password_link(
    email_target: String,
    name: String,
    token: String,
    expiration: NaiveDateTime,
) {
    let base_url = get_base_url();
    let date = convert_to_local_time(expiration);
    let body_html = format!(
        "<p>Hi, {}</p><p>Kami telah menerima permintaan Anda untuk reset kata sandi akun $name$ Anda.</p>
            <p>Silakan klik link di bawah ini untuk melanjutkan proses reset kata sandi $name$ \
         Anda.</p><p>{}/reset/{}</p>
            <p>Link verifikasi untuk atur ulang kata sandi Anda akan kadaluarsa pada {}.</p>.",
        name, base_url, token, date
    );

    debug!(
        "{} Request for password reset, link {}/reset/{}",
        name, base_url, token
    );

    dbg!(gen_email_sender("Verifikasi", "verify"));

    let builder = Email::builder()
        .to(email_target.to_owned())
        .from(gen_email_sender("Verifikasi", "verify"))
        .subject("RESET PASSWORD")
        .html(body_html);

    dbg!(&builder);

    send_email(builder)
}

/// Email builder ketika mendaftarkan $param.service_name_snake_case$ berdasarkan request (merchant dan admin).
pub fn request_register_$param.service_name_snake_case$(
    email_target: String,
    name: String,
    code: String,
    expiration: NaiveDateTime,
) {
    let base_url = get_base_url();
    let date = convert_to_local_time(expiration);
    let body_html = format!(
        "<p>Hi, {}</p><p>Terima kasih telah melakukan pendaftaran akun pada aplikasi $name$.</p>
            <p>Silakan klik link di bawah ini untuk mengonfirmasi akun $name$ Anda.</p>
            <p>{}/verification/{}</p>
            <p>Link konfirmasi akun Anda akan kadaluarsa pada {}.</p>.",
        name, base_url, code, date
    );

    debug!("email body: {:?}", body_html);

    let builder = Email::builder()
        .to(email_target.to_owned())
        .from(gen_email_sender("Konfirmasi", "confirm"))
        .subject("$param.service_name_upper_case$ INVITATION")
        .html(body_html);

    send_email(builder)
}
