extern crate lettre;
extern crate lettre_email;
extern crate clap;
extern crate mime;
extern crate native_tls;
extern crate config;

use native_tls::{TlsConnector};
use lettre::{EmailTransport, SmtpTransport, ClientSecurity};
use lettre::smtp::SmtpTransportBuilder;
use lettre::smtp::client::net::ClientTlsParameters;
use lettre_email::EmailBuilder;
use lettre::smtp::authentication::{Credentials};
use clap::{Arg, App};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();

    let conf = settings.deserialize::<HashMap<String, String>>().unwrap();

    let matches = App::new("Send")
                      .version("1.0")
                      .author("Matt")
                      .about("Send an email from your command line")
                      .arg(Arg::with_name("to")
                           .long("to")
                           .value_name("STRING")
                           .help("Email of recipient"))
                      .arg(Arg::with_name("file")
                           .short("f")
                           .long("file")
                           .value_name("STRING")
                           .help("File path"))
                      .arg(Arg::with_name("body")
                           .short("b")
                           .long("body")
                           .value_name("STRING")
                           .help("Email body"))
                      .arg(Arg::with_name("subject")
                           .short("s")
                           .long("subject")
                           .value_name("STRING")
                           .help("Email subject"))
                      .get_matches();

    let to_address = matches.value_of("to").unwrap_or("");
    let text = matches.value_of("body").unwrap_or("Sent with Send");
    let subject = matches.value_of("subject").unwrap_or("See attached");
    let file = matches.value_of("file").unwrap_or("");

    let plain_text: mime::Mime = "text/plain".parse().unwrap();
    let email = EmailBuilder::new()
        .from("")
        .to("")
        .subject("HI")
        .text("Hi")
        .attachment(Path::new("Cargo.toml"), None, &plain_text)
        .unwrap()
        .build();

    let connector = TlsConnector::builder().unwrap().build().unwrap();
    let client_tls_parameters = ClientTlsParameters{ connector: connector, domain: "smtp.gmail.com".to_string() };
    let mut mailer = SmtpTransportBuilder::new(("smtp.gmail.com", 587),ClientSecurity::Opportunistic(client_tls_parameters))
        .unwrap()
        .credentials(Credentials::new("".to_string(), "".to_string())) // email address & password
        .build();

    let result = mailer.send(&email.unwrap());
    match result {
        Ok(i) => println!("Message succesfully sent!\n"),
        Err(err) => println!("Failed sending message\n"),
    }

    mailer.close()
}
