use utoipa::{
    openapi::{
        schema::Components,
        security::{ApiKey, ApiKeyValue, SecurityScheme},
    },
    Modify, OpenApi,
};

use crate::{
    api::{
        data::{
            permissions::Permission,
            users::{DangerousLogin, User},
        },
        endpoints::{audio, docs, genres, invites, permissions, tokens, users},
    },
    error::ApiError,
};

#[allow(dead_code)]
pub enum DocFormat {
    JSON,
    PrettyJSON,
    YAML,
}

pub fn generate_docs(format: DocFormat) -> Result<String, ApiError> {
    let openapi = ApiDoc::openapi();
    Ok(match format {
        DocFormat::JSON => openapi.to_json()?,
        DocFormat::PrettyJSON => openapi.to_pretty_json()?,
        DocFormat::YAML => openapi.to_yaml()?,
    })
}

#[derive(OpenApi)]
#[openapi(paths(
        docs::docs_yaml,
        docs::docs_json,
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
        audio::audio_upload,
        audio::audio_get,
        audio::audio_delete,
    ), components(schemas(Permission, DangerousLogin, User)), modifiers(&SecurityAddon))]
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
