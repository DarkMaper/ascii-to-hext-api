#[macro_use] extern crate rocket;
extern crate hex;

use core::str;
use hex::FromHex;
use url::form_urlencoded::{parse};
use regex::Regex;


fn ascii_to_hex<'a>(text: String) -> String {
    let hex_text: String = hex::encode(text);
    
    format!("{}", hex_text)
}

fn hex_to_ascii<'a>(hex: String) -> String {
    let hex_to_bytes: String = parse(hex.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    let hex_slice: &str = &hex_to_bytes.to_owned()[..];
    if !re.is_match(hex_slice) { return "Introduce un texto hexadecimal correcto".to_string() }
    let buffer = <Vec<u8>>::from_hex(&hex_to_bytes).unwrap();
    let string = str::from_utf8(&buffer).expect("invalid string");

    format!("{}",string)
    
}


#[get("/to-hex/<text>")]
fn text_encoder(text: String) -> String {
    ascii_to_hex(text)
}

#[get("/to-text/<hex>")]
fn text_decoder(hex: String) -> String {
    hex_to_ascii(hex)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![text_encoder, text_decoder])
}