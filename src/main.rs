mod prelude;
mod result;
mod service;

use actix_web::{web::Data, App, HttpServer};
use anyhow::Context;
use serde_json::json;

async fn main_anyhow() -> anyhow::Result<()> {
    fern::Dispatch::new()
    .format(|out, message, record| {
        let thread_handle = std::thread::current();

        out.finish(format_args!("{}",
            json!({
                "thread_handle": thread_handle.name().map(|s| s.to_string()).unwrap_or(format!("{:?}", thread_handle.id())),
                "level": record.level().to_string(),
                "timestamp": chrono::Local::now().timestamp_millis(),
                "message": message
            })
        ));
    })
    .level(log::LevelFilter::Info)
    .chain(std::io::stdout())
    .apply()?;

    log::info!("Logger initialized.");

    let address = std::env::var("SERVICE_ADDRESS").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("SERVICE_PORT")
        .map(|source| source.parse::<u16>())
        .unwrap_or(Ok(8080))
        .context("Invalid port.")?;

    log::info!("Binding to {{{address}:{port}}}");

    HttpServer::new(|| {
        let initial = App::new().app_data(Data::new(service::ServiceContext::new()));
        vec![service::info_service()]
            .into_iter()
            .flatten()
            .fold(initial, |accum, next| {
                let prelude::InformativeResource { endpoint, resource } = next;
                log::debug!("Creating router for endpoint {endpoint}");
                accum.service(resource)
            })
    })
    .bind((address, port))
    .context("Failed to bind to port.")?
    .run()
    .await
    .context("A fatal exception occured while running the tos archive.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    main_anyhow()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{e}")))
}
