use {
    hyper::{
        // Following functions are used by Hyper to handle a `Request`
        // and returning a `Response` in an asynchronous manner by using a Future
        service::{make_service_fn, service_fn},
        // Miscellaneous types from Hyper for working with HTTP.
        Body,
        Client,
        Request,
        Response,
        Server,
        Uri,
    },
    std::net::SocketAddr,
};

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let url_str = "http://www.rust-lang.org/en-US/";
    let url = url_str.parse::<Uri>().expect("failed to parse URL");
    let res = Client::new().get(url).await?;
    // Return the result of the request directly to the user
    println!("request finished-- returning response");
    Ok(res)
}

async fn run_serve(addr: SocketAddr) {
    println!("Server running at http://{}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| {
        async {
            {
                Ok::<_, hyper::Error>(service_fn(serve_req))
            }
        }
    }));

    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run_serve(addr).await;
}
