use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::body::Bytes;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use routerify::prelude::*;
use routerify::{RequestInfo, Router, RouterService};
use std::path::PathBuf;
use std::{convert::Infallible, net::SocketAddr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "test-server",
    about = "server for serving a file with different mime types, depending on request"
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    file: PathBuf,
    #[structopt(default_value = "8585")]
    port: u16,
}

struct State {
    file: Bytes,
}

async fn file_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mime_type = req.uri().query().and_then(|query| {
        form_urlencoded::parse(query.as_bytes())
            .find_map(|(k, v)| if k == "type" { Some(v) } else { None })
    });
    let state = req.data::<State>().unwrap();
    let body = Body::from(state.file.clone());
    let mut response_builder = Response::builder();
    if let Some(mime_type) = mime_type {
        response_builder = response_builder.header("content-type", mime_type.as_ref());
    }
    Ok(response_builder.body(body).unwrap())
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    eprintln!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

fn router(file: Vec<u8>) -> Router<Body, Infallible> {
    Router::builder()
        .data(State {
            file: Bytes::from(file),
        })
        .get("/:file_name", file_handler)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let mut file = File::open(opt.file).await.expect("could not read file");
    let mut content = vec![];
    file.read_to_end(&mut content)
        .await
        .expect("could not read file");
    let router = router(content);

    let service = RouterService::new(router).unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], opt.port));

    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
