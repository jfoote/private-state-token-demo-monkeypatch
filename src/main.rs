//! private-state-token-demo.glitch.me is currently broken
//! This is a monkeypatch Compute service that fixes it and serves it via https://pst-demo.foote.dev

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

use std::io::Read;

const DEMO_ORIGIN: &str = "pst_demo";
const ISSUER_ORIGIN: &str = "pst_issuer";
const REDEEMER_ORIGIN: &str = "pst_redeemer";

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {

    // Log service version
    
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    // As of 2024-09-05 /js/issuer.js on glitch is broken with some live-debug code in it (try/catch).
    // So we serve a version that matches what is in the github repo (with hostnames monkeypatched)
    if req.get_path()  == "/js/issuer.js" {
        return Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(include_str!("issuer.js")
                .replace("private-state-token-demo.glitch.me", "pst-demo.foote.dev")
                .replace("private-state-token-issuer.glitch.me", "pst-issuer.foote.dev")
                .replace("private-state-token-redeemer.glitch.me", "pst-redeemer.foote.dev")))
                                                                 
    }


    // Switch Host header in request to the one that glitch expects
    // TODO: Do this for all headers, path, and body for completeness?

    println!("original request host: {}", req.get_header_str(header::HOST).unwrap_or_default());

    let mut new_req = req.clone_with_body();
    let host = req.get_header_str(header::HOST)
        .unwrap_or_default()
        .replace("pst-demo.foote.dev", "private-state-token-demo.glitch.me")
        .replace("pst-issuer.foote.dev", "private-state-token-issuer.glitch.me")
        .replace("pst-redeemer.foote.dev", "private-state-token-redeemer.glitch.me")
        .replace("127.0.0.1:7676", "private-state-token-demo.glitch.me");
    new_req.set_header("host", host);


    println!("new request host: {}", new_req.get_header_str(header::HOST).unwrap_or_default());


    // Select the right glitch backend based on the new Host header

    let origin = match(new_req.get_header_str(header::HOST).unwrap_or_default()) {
        "private-state-token-demo.glitch.me" => DEMO_ORIGIN,
        "private-state-token-issuer.glitch.me" => ISSUER_ORIGIN,
        "private-state-token-redeemer.glitch.me" => REDEEMER_ORIGIN,
        "127.0.0.1:7676" => DEMO_ORIGIN,
        _ => panic!("Unexpected host header")
    };


    // Send request and get response
    
    new_req.set_pass(true);
    let mut beresp = new_req.send(origin)?;


    // Switch hostnames in response to the ones that the browser expects
    
    let mut new_body = beresp.take_body()
       .into_string()
       .replace("private-state-token-demo.glitch.me", "pst-demo.foote.dev")
       .replace("private-state-token-issuer.glitch.me", "pst-issuer.foote.dev")
       .replace("private-state-token-redeemer.glitch.me", "pst-redeemer.foote.dev")
       .replace("1715356984440000", "1757082726721000"); // for readme; issuer key commitment good
                                                         // until 2025 Sept 5

   
    // Add missing policy headers and send response
    
    Ok(beresp
        .with_body(new_body)
        .with_header("Cache-Control", "no-store, max-age=0")
        .with_header("Permissions-Policy", "private-state-token-issuance=(self),private-state-token-redemption=(self)"))
}
