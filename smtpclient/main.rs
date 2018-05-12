extern crate lettre;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{SimpleSendableEmail, EmailTransport, SmtpTransport};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;

fn main() {
    let email = SimpleSendableEmail::new(
                    //From:
                    "myaccount@gmail.com".to_string(),
                    //To:
                    &["recipient@gmail.com".to_string()],
                    "message_id".to_string(),
                    "Subject: Sähköpostitilin vahvistaminen\n\nOle hyvä ja clickaa allaolevaa linkkiä vahvistaaksesi sähköpostiosoitteesi:".to_string(),
                ).unwrap();

    // Connect to a remote server on a custom port
    let mut mailer = SmtpTransport::simple_builder("smtp.gmail.com").unwrap()
        // Set the name sent during EHLO/HELO, default is `localhost`
        .hello_name(ClientId::Domain("custom.domain".to_string()))
        // Add credentials for authentication
        .credentials(Credentials::new("myaccount@gmail.com".to_string(), "mypassword123456".to_string()))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        // Configure expected authentication mechanism
        .authentication_mechanism(Mechanism::Plain)
        // Enable connection reuse
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).build();

    let result = mailer.send(&email);
    println!("{:?}", result.is_ok());

    // Explicitly close the SMTP transaction as we enabled connection reuse
    mailer.close();
}
