use rocket::{request::FromParam, serde::Deserialize};
use std::{convert::Infallible, str::FromStr};
use strum::EnumString;
use utoipa::{
    openapi::{
        schema::Components,
        security::{ApiKey, ApiKeyValue, SecurityScheme},
    },
    Modify, OpenApi, ToSchema,
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

/// Enum representing documentation formats.
#[allow(dead_code)]
#[derive(EnumString, ToSchema, Deserialize)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
#[serde(rename_all = "lowercase", crate = "rocket::serde")]
pub enum DocFormat {
    #[serde(skip)]
    Unsupported,
    JSON,
    PrettyJSON,
    YAML,
}

impl<'r> FromParam<'r> for DocFormat {
    type Error = Infallible;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(Self::from_str(param).unwrap_or(Self::Unsupported))
    }
}

/// Generates the documentation in the specified format.
pub fn generate_docs(format: &DocFormat) -> Result<String, ApiError> {
    let openapi = ApiDoc::openapi();
    Ok(match format {
        DocFormat::JSON => openapi.to_json()?,
        DocFormat::PrettyJSON => openapi.to_pretty_json()?,
        DocFormat::YAML => openapi.to_yaml()?,
        _ => Err(ApiError::Unsupported(
            "Error: Format Unsupported".to_string(),
        ))?,
    })
}

#[derive(OpenApi)]
#[openapi(paths(
        docs::docs_openapi,
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
    ), components(schemas(Permission, DangerousLogin, User, DocFormat)), modifiers(&SecurityAddon))]
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
