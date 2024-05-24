use rocket::{
    fairing::AdHoc,
    http::{ContentType, Status},
};

use crate::docs::{generate_docs, DocFormat};
use crate::{
    api::data::{permissions::Permission, users::User},
    error::ApiError,
};

/// Retrieve yaml OpenAPI documentation
///
/// Requires: `DocsRead` permission
#[utoipa::path(
    get,
    responses(
    (
        status = 200,
        description = "Success",
        content_type = "application/x-yaml",
        body = String,
    ),
    (
        status = 403,
        description = "Forbidden requires permission `DocsRead`"
    )),
    security(
        ("permissions" = ["DocsRead"])
    )
)]
#[get("/docs/openapi.yaml")]
fn docs_yaml(user: User) -> Result<(ContentType, String), ApiError> {
    if !user.permissions.contains(&Permission::DocsRead) {
        Err(Status::Forbidden)?
    }

    let yaml = generate_docs(DocFormat::YAML)?;
    Ok((ContentType::new("application", "x-yaml"), yaml))
}

/// Retrieve json OpenAPI documentation
///
/// Requires: `DocsRead` permission
#[utoipa::path(
    get,
    responses(
    (
        status = 200,
        description = "Success",
        content_type = "application/json",
        body = String,
    ),
    (
        status = 403,
        description = "Forbidden requires permission `DocsRead`"
    )),
    security(
        ("permissions" = ["DocsRead"])
    )
)]
#[get("/docs/openapi.json")]
fn docs_json(user: User) -> Result<(ContentType, String), ApiError> {
    if !user.permissions.contains(&Permission::DocsRead) {
        Err(Status::Forbidden)?
    }

    let json = generate_docs(DocFormat::PrettyJSON)?;
    Ok((ContentType::new("application", "json"), json))
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("Docs Systems", |rocket| async {
        rocket.mount("/", routes![docs_yaml, docs_json])
    })
}
