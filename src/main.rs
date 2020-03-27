use {
    actix_web::{get, App, HttpServer, Responder},
    listenfd::ListenFd,
    std::env,
};

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || App::new().service(index));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => {
            let port = listener.local_addr()?.port();

            println!("Restarting server on http://0.0.0.0:{}", port);
            server.listen(listener)?
        }
        None => {
            let port = server_port();

            println!("Starting server on http://0.0.0.0:{}", port);
            server.bind(("0.0.0.0", port))?
        }
    };

    server.run().await
}

fn server_port() -> u16 {
    env::var("PORT")
        .map(|port| port.parse().expect("PORT must be a number"))
        .unwrap_or(3000)
}
