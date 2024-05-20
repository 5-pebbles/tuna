use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::rusqlite::{Error, Row};
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use utoipa::ToSchema;

/// The permissions available in the server.
#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    IntoStaticStr,
    ToSchema,
)]
#[serde(crate = "rocket::serde")]
#[strum(serialize_all = "PascalCase", ascii_case_insensitive)]
pub enum Permission {
    //Docs
    DocsRead,

    // Invites
    InviteWrite, // you can only create an invite with permissions you already have...
    InviteRead,
    InviteDelete,
    // Users
    UserRead,
    UserDelete, // Only on users who's permissions are the same or a subset of their own
    // Permissions
    PermissionAdd, // grant permissions (you still need to have the permissions you grant)
    PermissionDelete, // Only on users who's permissions are the same or a subset of their own
    // Sessions
    TokenDelete, // delete another users sessions

    // Music Stuff
    GenreWrite,
    GenreRead,
    GenreDelete,

    ArtistWrite,
    ArtistRead,
    ArtistDelete,

    AlbumWrite,
    AlbumRead,
    AlbumDelete,

    TrackWrite,
    TrackRead,
    TrackDelete,

    // Content
    AudioWrite,
    AudioRead,
    AudioDelete,
}

/// Extracts permissions from a rusqlite row and converts them into a `Vec<Permission>`.
///
/// This function retrieves the permissions string from the `permissions` row, splits it by commas,
/// and attempts to convert each segment into a `Permission` enum variant. If the conversion
/// is successful, the permission is added to the resulting vector. If the conversion fails
/// for any segment, it is silently ignored. If the permissions field is missing or empty,
/// an empty vector is returned.
///
/// # Arguments
///
/// * `row` - A reference to a database row from which permissions are to be extracted.
///
/// # Returns
///
/// * `Result<Vec<Permission>, Error>` - A result containing a vector of permissions if successful,
///   or an error if the permissions could not be extracted or converted.
///
pub fn permissions_from_row(row: &Row) -> Result<Vec<Permission>, Error> {
    Ok(row
        .get::<&str, Option<String>>("permissions")?
        .map_or_else(Vec::new, |v| {
            v.split(',')
                .filter_map(|s| Permission::from_str(s).ok())
                .collect()
        }))
}
