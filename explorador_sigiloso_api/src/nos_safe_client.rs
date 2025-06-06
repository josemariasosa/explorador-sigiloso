use serde_json::json;
use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;

pub fn sign_tx(path: &str, psbt_base64: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:7600")?;
    let req = json!({
        "method": "sign_btc_tx",
        "params": { "path": path, "psbt_base64": psbt_base64 }
    });
    writeln!(stream, "{}", req.to_string())?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    let res_json: serde_json::Value = serde_json::from_str(&response)?;
    Ok(res_json["result"].as_str().unwrap_or("").to_string())
}
