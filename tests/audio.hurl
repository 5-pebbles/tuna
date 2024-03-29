# Setup
POST {{url}}/init
{
    "username": "SystemTest",
    "password": "BadPass123"
}
HTTP 200
POST {{url}}/token
{
    "username": "SystemTest",
    "password": "BadPass123"
}
HTTP 200
POST {{url}}/genre/instrumental
POST {{url}}/artist
{
    "id": "0",
    "name": "5-pebbles",
    "genres": ["instrumental"],
    "bio": ""
}
HTTP 200
POST {{url}}/album
{
    "id": "0",
    "name": "arrangements",
    "artists": ["0"],
    "release": 2023,
    "genres": ["instrumental"]
}
HTTP 200
# I am using my arrangement as a test track so I don't get sued...
POST {{url}}/track
{
    "id": "0",
    "name": "The Last of Us (piano arrangement)",
    "release": 2019,
    "albums": ["0"],
    "lyrics": "",
    "genres": ["instrumental"]
}
HTTP 200
# End Setup

PUT {{url}}/audio/1
HTTP 404

PUT {{url}}/audio/0
content-type: audio/mpeg
file, the_last_of_us_main_theme.mp3;
HTTP 200

GET {{url}}/audio/0
HTTP 200
[Asserts]
file, the_last_of_us_main_theme.mp3;


DELETE {{url}}/audio/0
HTTP 200

DELETE {{url}}/audio/0
HTTP 404

# Cleanup
DELETE {{url}}/track/0
HTTP 200
DELETE {{url}}/album/0
HTTP 200
DELETE {{url}}/artist/0
HTTP 200
DELETE {{url}}/genre/instrumental
HTTP 200
DELETE {{url}}/user/SystemTest
HTTP 200
# End Cleanup
