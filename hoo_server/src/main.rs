mod hue_client;
mod options;

use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use structopt::StructOpt;

use hue_client::HueClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let options = options::Options::from_args();

    let addr = "127.0.0.1:3000".parse().unwrap();

    let client = HueClient::new(&options.hue_base_uri, &options.hue_user_id);

    let new_service = make_service_fn(move |_| {
        let client = client.clone();
        async {
            Ok::<_, anyhow::Error>(service_fn(move |req| {
                handle(req, client.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

async fn handle(req: Request<Body>, client: HueClient) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/lights") => get_all_lights(&client).await,
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Not Found".into())
                .unwrap())
        }
    }
}

async fn get_all_lights(client: &HueClient) -> Result<Response<Body>> {
    client.get("lights").await
}

// async fn client_request_response(client: &Client<HttpConnector>) -> Result<Response<Body>> {
//     let req = Request::builder()
//         .method(Method::POST)
//         .uri(URL)
//         .header(header::CONTENT_TYPE, "application/json")
//         .body(POST_DATA.into())
//         .unwrap();

//     let web_res = client.request(req).await?;
//     // Compare the JSON we sent (before) with what we received (after):
//     let before = stream::once(async {
//         Ok(format!(
//             "<b>POST request body</b>: {}<br><b>Response</b>: ",
//             POST_DATA,
//         )
//         .into())
//     });
//     let after = web_res.into_body();
//     let body = Body::wrap_stream(before.chain(after));

//     Ok(Response::new(body))
// }

// async fn api_get_response() -> Result<Response<Body>> {
//     let data = vec!["foo", "bar"];
//     let res = match serde_json::to_string(&data) {
//         Ok(json) => Response::builder()
//             .header(header::CONTENT_TYPE, "application/json")
//             .body(Body::from(json))
//             .unwrap(),
//         Err(_) => Response::builder()
//             .status(StatusCode::INTERNAL_SERVER_ERROR)
//             .body(INTERNAL_SERVER_ERROR.into())
//             .unwrap(),
//     };
//     Ok(res)
// }

// async fn response_examples(
//     req: Request<Body>,
//     client: Client<HttpConnector>,
// ) -> Result<Response<Body>> {
//     match (req.method(), req.uri().path()) {
//         (&Method::GET, "/") | (&Method::GET, "/index.html") => Ok(Response::new(INDEX.into())),
//         (&Method::GET, "/test.html") => client_request_response(&client).await,
//         (&Method::POST, "/json_api") => api_post_response(req).await,
//         (&Method::GET, "/json_api") => api_get_response().await,
//         _ => {
//             // Return 404 not found response.
//             Ok(Response::builder()
//                 .status(StatusCode::NOT_FOUND)
//                 .body(NOTFOUND.into())
//                 .unwrap())
//         }
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     pretty_env_logger::init();

//     let addr = "127.0.0.1:1337".parse().unwrap();

//     // Share a `Client` with all `Service`s
//     let client = Client::new();

//     let new_service = make_service_fn(move |_| {
//         // Move a clone of `client` into the `service_fn`.
//         let client = client.clone();
//         async {
//             Ok::<_, GenericError>(service_fn(move |req| {
//                 // Clone again to ensure that client outlives this closure.
//                 response_examples(req, client.to_owned())
//             }))
//         }
//     });

//     let server = Server::bind(&addr).serve(new_service);

//     println!("Listening on http://{}", addr);

//     server.await?;

//     Ok(())
// }