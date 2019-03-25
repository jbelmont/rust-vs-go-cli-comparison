extern crate base64;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::env;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    typ: String,
    alg: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    header: Header,
    claims: Claims,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iss: String,
    aud: String,
    exp: u32,
    nbf: u32,
    iat: u32,
    jti: String,
}

impl Claims {
    fn new() -> Self {
        Default::default()
    }
}

impl Header {
    fn new() -> Self {
        Header {
            typ: String::from(""),
            alg: String::from(""),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut header = Header::new();
    let mut claims = Claims::new();

    let jwt_string: Vec<_> = args[1].split(".").collect();

    let mut counter = 0;
    for part in jwt_string {
        if counter == 0 {
            let decoded_header = base64::decode(part).unwrap();
            let decoded_header = String::from_utf8(decoded_header).unwrap();
            header = serde_json::from_str(&decoded_header).unwrap();
        } else if counter == 1 {
            let decoded_claims = base64::decode(part).unwrap();
            let decoded_claims = String::from_utf8(decoded_claims).unwrap();
            claims = serde_json::from_str(&decoded_claims).unwrap();
        }
        counter += 1;
    }

    let token = Token {
        header: Header {
            typ: header.typ,
            alg: header.alg,
        },
        claims: Claims {
            sub: claims.sub,
            iss: claims.iss,
            aud: claims.aud,
            exp: claims.exp,
            nbf: claims.nbf,
            iat: claims.iat,
            jti: claims.jti,
        }
    };
    // Serialize it to a JSON string.
    let serialize_jwt = serde_json::to_string_pretty(&token).unwrap();

    // Print, write to a file, or send to an HTTP server.
    println!("{}", serialize_jwt);
}
