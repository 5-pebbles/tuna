use rocket::{
    fairing::AdHoc,
    http::{ContentType, Status},
};

use crate::docs::{generate_docs, DocFormat};
use crate::{
    api::data::{permissions::Permission, users::User},
    error::ApiError,
};

/// Retrieve OpenAPI documentation
///
/// Requires: `DocsRead` permission
#[utoipa::path(
    get,
    responses(
    (
        status = 200,
        description = "Success",
        body = String,
    ),
    (
        status = 403,
        description = "Forbidden requires permission `DocsRead`"
    ),
    (
        status = 418,
        description = "Unsuported documentation format"
    )),
    params(
        ("format" = DocFormat, description = "The requested documentation format"),
    ),
    security(
        ("permissions" = ["DocsRead"])
    )
)]
#[get("/docs/openapi/<format>")]
fn docs_openapi(user: User, format: DocFormat) -> Result<(ContentType, String), ApiError> {
    if !user.permissions.contains(&Permission::DocsRead) {
        Err(Status::Forbidden)?
    }

    let docs = generate_docs(&format)?;

    Ok(match format {
        DocFormat::JSON | DocFormat::PrettyJSON => (ContentType::new("application", "json"), docs),
        DocFormat::YAML => (ContentType::new("application", "x-yaml"), docs),
        DocFormat::Unsupported => unreachable!(),
    })
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("Docs Systems", |rocket| async {
        rocket.mount("/", routes![docs_openapi])
    })
}
