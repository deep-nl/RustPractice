use rand::Rng;
use std::net::SocketAddr;
use std::error::Error;
use hyper::body::Buf;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

// https://www.becomebetterprogrammer.com/rust-generic-error-types/

const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

async fn create_car(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error>> {
    // get the buffer from the request body
    let buffer = hyper::body::aggregate(req).await?;

    // add an id to the new_car
    let mut new_car: serde_json::Value = serde_json::from_reader(buffer.reader())?;

    let mut random = rand::thread_rng();

    let car_id: u8 = random.gen();
    new_car["id"] = serde_json::Value::from(car_id.to_string());

    let res = match serde_json::to_string(&new_car) {
        Ok(json) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };

    Ok(res)
}

async fn cars_handler(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error>> {
    let path = req.uri().path().to_owned();
    let path_segments = path.split("/").collect::<Vec<&str>>();
    let base_path = path_segments[1];

    match (req.method(), base_path) {
        (&Method::POST, "cars") => create_car(req).await,

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(cars_handler))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}