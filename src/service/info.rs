use crate::prelude::*;
use actix_web::web;
use tera::Context as TeraContext;

async fn index_handler(data: ActixData) -> TosDirectoryResult {
    super::tera_response("index.html", data, &TeraContext::new())
}

async fn about_handler(data: ActixData) -> TosDirectoryResult {
    super::tera_response("about.html", data, &TeraContext::new())
}

pub(super) fn create_info_service() -> Vec<InformativeResource> {
    vec![
        super::resource("/", |router| router.route(web::get().to(index_handler))),
        super::resource("/about", |router| {
            router.route(web::get().to(about_handler))
        }),
    ]
}
