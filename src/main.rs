use warp::{http::StatusCode, Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};

type Result<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Serialize, Deserialize)]
struct HttpResponse {
    message: String,
}

#[tokio::main]
async fn main() {
  let home_route = warp::any().and_then(health_handler);
  let health_route = warp::path!("health").and_then(health_handler);
  
  let routes = home_route.or(health_route).with(warp::cors().allow_any_origin());

  println!("Started server at localhost:8000");
  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

async fn health_handler() -> Result<impl Reply> {
  let message;
  let code;
  code = StatusCode::OK;
  message = "Welcome to RVST!";
  
  let json = warp::reply::json(&HttpResponse {
    message: message.into(),
  });

  Ok(warp::reply::with_status(json, code))
}
