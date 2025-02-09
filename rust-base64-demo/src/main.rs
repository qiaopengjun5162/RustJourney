use base64::engine::{general_purpose, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original = "Hello?_d";

    let b64 = general_purpose::STANDARD.encode(original);
    println!("{b64:<20} (Base64 encoded)");

    let decoded = general_purpose::STANDARD.decode(b64)?;
    let decoded = String::from_utf8(decoded)?;
    println!("{decoded:<20} (Base64 decoded)");

    let b64u = general_purpose::URL_SAFE_NO_PAD.encode(original);
    println!("{b64u:<20} (Base64 URL encoded)");
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(b64u)?;
    let decoded = String::from_utf8(decoded)?;
    println!("{decoded:<20} (Base64 URL decoded)");

    Ok(())
}
