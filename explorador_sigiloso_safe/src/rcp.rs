use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write};
use serde::{Deserialize, Serialize};
use crate::signer::sign_btc_psbt;

#[derive(Deserialize)]
struct SignRequest {
    method: String,
    params: serde_json::Value,
}

#[derive(Serialize)]
struct SignResponse {
    result: String,
    error: Option<String>,
}

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7600").unwrap();
    println!("🚪 Safe is listening on 127.0.0.1:7600");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();

        let req: SignRequest = serde_json::from_str(&buffer).unwrap();

        let response = match req.method.as_str() {
            "sign_btc_tx" => {
                let path = req.params["path"].as_str().unwrap();
                let psbt = req.params["psbt_base64"].as_str().unwrap();
                match sign_btc_psbt(path, psbt) {
                    Ok(sig) => SignResponse { result: sig, error: None },
                    Err(e) => SignResponse { result: "".into(), error: Some(e.to_string()) },
                }
            }
            _ => SignResponse { result: "".into(), error: Some("Unknown method".into()) }
        };

        let res_json = serde_json::to_string(&response).unwrap();
        writeln!(stream, "{}", res_json).unwrap();
    }
}
