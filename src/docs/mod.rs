use rocket::{
    fairing::AdHoc,
    http::{ContentType, Status},
};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use utoipa::{
    openapi::{
        schema::Components,
        security::{ApiKey, ApiKeyValue, SecurityScheme},
    },
    Modify, OpenApi,
};

use crate::api::{
    data::{
        permissions::Permission,
        users::{DangerousLogin, User},
    },
    endpoints::{genres, invites, permissions, tokens, users},
};

#[derive(OpenApi)]
#[openapi(paths(
        docs_yaml,
        docs_json,
        tokens::token_write,
        tokens::token_delete,
        permissions::permission_add,
        permissions::permission_delete,
        invites::invite_use,
        invites::invite_write,
        invites::invite_get,
        invites::invite_delete,
        users::user_init,
        users::user_get,
        users::user_delete,
        genres::genre_write,
        genres::genre_get,
        genres::genre_delete,
    ), components(schemas(DangerousLogin, User)), modifiers(&SecurityAddon))]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Components::default);

        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("permissions"))),
        )
    }
}

fn generate_docs() -> Result<(), String> {
    let openapi = ApiDoc::openapi();
    let docs = [
        (
            "docs/openapi.yaml",
            openapi
                .to_yaml()
                .map_err(|e| format!("Failed to serialize yaml docs: {}", e))?,
        ),
        (
            "docs/openapi.json",
            openapi
                .to_pretty_json()
                .map_err(|e| format!("Failed to serialize json docs: {}", e))?,
        ),
    ];

    for (path, content) in docs {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let mut file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;
    }

    Ok(())
}

/// Retrieve yaml OpenAPI documentation
///
/// Requires: `DocsRead` permission
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
        description = "Forbidden requires permission `DocsRead`"
    )),
    security(
        ("permissions" = ["DocsRead"])
    )
)]
#[get("/docs/openapi.yaml")]
fn docs_yaml(user: User) -> Result<(ContentType, String), Status> {
    if !user.permissions.contains(&Permission::DocsRead) {
        Err(Status::Forbidden)?
    }

    let yaml =
        std::fs::read_to_string("docs/openapi.yaml").map_err(|_| Status::InternalServerError)?;
    Ok((ContentType::new("application", "x-yaml"), yaml))
}

/// Retrieve json OpenAPI documentation
///
/// Requires: `DocsRead` permission
#[utoipa::path(
    get,
    path = "/docs/openapi.json",
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
fn docs_json(user: User) -> Result<(ContentType, String), Status> {
    if !user.permissions.contains(&Permission::DocsRead) {
        Err(Status::Forbidden)?
    }

    let json =
        std::fs::read_to_string("docs/openapi.json").map_err(|_| Status::InternalServerError)?;
    Ok((ContentType::new("application", "json"), json))
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("Docs Systems", |rocket| async {
        generate_docs().expect("Failed to generate_docs");
        rocket.mount("/", routes![docs_yaml, docs_json])
    })
}
