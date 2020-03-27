use {
    actix_web::{get, App, HttpServer, Responder},
    std::env,
};

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = server_port();

    println!("Starting server on http://0.0.0.0:{}", port);

    HttpServer::new(move || App::new().service(index))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

fn server_port() -> u16 {
    env::var("PORT")
        .map(|port| port.parse().expect("PORT must be a number"))
        .unwrap_or(3000)
}
