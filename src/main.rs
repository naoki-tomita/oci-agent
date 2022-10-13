use std::convert::Infallible;
use std::fs;
use std::io::{self, Write};
use std::net::SocketAddr;
use dirs::home_dir;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::process::Command;
use once_cell::sync::OnceCell;


fn exec(command: String) {
  println!("execute: '{}'", command);
  let output = Command::new("bash").arg("-c").arg(command).output().expect("failed to execute");
  io::stdout().write_all(&output.stdout).unwrap();
  io::stderr().write_all(&output.stderr).unwrap();
}

static ACCESS_TOKEN: OnceCell<String> = OnceCell::new();

async fn command(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  let token = ACCESS_TOKEN.get().unwrap();
  if let Some(authorization_header) = req.headers().get("authorization") {
    if let Ok(header_value) = authorization_header.to_str() {
      if header_value.replace("Bearer", "").trim() == token {
        exec("docker compose pull".into());
        exec("docker compose up -d".into());
        return Ok(Response::new("{}".into()));
      } else {
        println!("Authorization token is not valid. TOKEN: {}", header_value);
      }
    }
  } else {
    println!("Authorization header is not appeared.");
  }

  Ok(Response::new("{}".into()))
}

#[tokio::main]
async fn main() {
  let dir = home_dir().unwrap();
  let readed = fs::read_to_string(format!("{}/.oci-config", dir.to_str().unwrap()))
        .expect("Should have been able to read the file");

  ACCESS_TOKEN.set(readed.trim().into()).unwrap();

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let make_svc = make_service_fn(|_conn| async {
    Ok::<_, Infallible>(service_fn(command))
  });

  let server = Server::bind(&addr).serve(make_svc);

  println!("Starting server on 0.0.0.0:3000");
  // Run this server for... forever!
  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }
}
