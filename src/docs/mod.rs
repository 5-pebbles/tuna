use rocket::{fairing::AdHoc, http::{Status, ContentType}};
use std::{fs::File, io::Write, path::Path};

use utoipa::OpenApi;

use crate::database::{permissions::Permission, users::DangerousUser};

#[derive(OpenApi)]
#[openapi(paths(docs_yaml), components(schemas()))]
struct ApiDoc;

fn generate_docs() -> Result<(), String> {
    let openapi = ApiDoc::openapi();
    let yaml =
        serde_yaml::to_string(&openapi).map_err(|e| format!("Failed to serialize docs: {}", e))?;

    let path = Path::new("docs/openapi.yaml");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    let mut file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(yaml.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/docs/openapi.yaml",
    responses(
    (
        status = 200,
        description = "Success",
        content_type = "application/x-yaml",
        body = String,
    ),
    (
        status = 403,
        description = "Unauthorized requires permission `DocsRead`"
    ))
)]
#[get("/docs/openapi.yaml")]
fn docs_yaml(user: DangerousUser) -> Result<(ContentType, String), Status> {
    if !user.has_permissions(&[Permission::DocsRead]) {
        Err(Status::Forbidden)?
    }

    let yaml = std::fs::read_to_string("docs/openapi.yaml").map_err(|_| Status::InternalServerError)?;
    Ok((ContentType::new("application", "x-yaml"), yaml))
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("Docs Systems", |rocket| async {
        generate_docs().expect("Failed to generate_docs");
        rocket.mount("/", routes![docs_yaml])
    })
}
