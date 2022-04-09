mod info;

use crate::prelude::*;
use actix_web::Resource;
use anyhow::Context;

#[derive(Clone)]
pub struct ServiceContext {
    pub tera: tera::Tera,
}

impl ServiceContext {
    pub(super) fn new() -> Self {
        let tera = tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/html/*"))
            .context("Failed to load html templates.")
            .unwrap();
        Self { tera }
    }
}

pub struct InformativeResource {
    pub endpoint: &'static str,
    pub resource: Resource,
}

pub fn resource<F: Fn(Resource) -> Resource>(
    endpoint: &'static str,
    resource_handler: F,
) -> InformativeResource {
    let resource = resource_handler(actix_web::web::resource(endpoint));
    InformativeResource { endpoint, resource }
}

pub type ActixData = actix_web::web::Data<ServiceContext>;

fn tera_response(
    source: &'static str,
    data: ActixData,
    tera_context: &tera::Context,
) -> TosDirectoryResult {
    Ok(data
        .tera
        .render(source, tera_context)
        .context("Tera render failed.")
        .map(|rendered| {
            actix_web::HttpResponse::Ok()
                .content_type("text/html")
                .body(rendered)
        })?)
}

pub fn info_service() -> Vec<InformativeResource> {
    info::create_info_service()
}
