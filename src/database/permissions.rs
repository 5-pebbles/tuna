use rocket::serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, EnumIter, EnumString)]
#[serde(crate = "rocket::serde")]
#[strum(serialize_all = "camelCase")]
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
    SessionsDelete, // delete another users sessions

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
}
