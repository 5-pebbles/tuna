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
POST {{url}}/genre/indie%20pop
POST {{url}}/genre/indie%20rock
POST {{url}}/artist
{
    "id": "0",
    "name": "Phoebe Bridgers",
    "genres": ["indie pop"],
    "bio": ""
}
HTTP 200
POST {{url}}/artist
{
    "id": "1",
    "name": "Conor Oberst",
    "genres": ["indie rock"],
    "bio": ""
}
HTTP 200
# End Setup

# Create Albums
POST {{url}}/album
{
    "id": "0",
    "name": "the record",
    "artists": ["0"],
    "release": 2023,
    "genres": ["indie pop"]
}
HTTP 200

# We shouldn't not be able to create two albums with the same id
POST {{url}}/album
{
    "id": "0",
    "name": "Better Oblivion Community Center",
    "artists": ["0", "1"],
    "release": 2019,
    "genres": ["indie pop", "indie rock"]
}
HTTP 409

POST {{url}}/album
{
    "id": "1",
    "name": "Better Oblivion Community Center",
    "artists": ["0", "1"],
    "release": 2019,
    "genres": ["indie pop", "indie rock"]
}
HTTP 200
# End Create Albums

# Get Albums
GET {{url}}/album
HTTP 200
[Asserts]
jsonpath "$" count == 2

GET {{url}}/album?id=0
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "0"
jsonpath "$[0].name" == "the record"
jsonpath "$[0].release" == 2023
jsonpath "$[0].artists" count == 1
jsonpath "$[0].artists" includes "0"
jsonpath "$[0].tracks" count == 0
jsonpath "$[0].genres" count == 1
jsonpath "$[0].genres" includes "indie pop"

GET {{url}}/album?name=he%20recor
HTTP 200
[Asserts]
jsonpath "$" count == 1

GET {{url}}/album?maxrelease=2020
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "1"

GET {{url}}/album?minrelease=2020
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "0"

GET {{url}}/album?genres=%5B%22indie%20rock%22%5D
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "1"

GET {{url}}/album?maxcount=1
HTTP 200
[Asserts]
jsonpath "$" count == 2

GET {{url}}/album?mincount=1
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/album?limit=1
HTTP 200
[Asserts]
jsonpath "$" count == 1

GET {{url}}/album?maxcount=1&limit=0
HTTP 200
[Asserts]
jsonpath "$" count == 0
# End Get Albums

# Delete Albums
DELETE {{url}}/album/0
HTTP 200

DELETE {{url}}/album/0
HTTP 404

DELETE {{url}}/album/1
HTTP 200
# End Delete Albums

# Required Permissions
DELETE {{url}}/permissions/SystemTest
[
    "AlbumWrite"
]
HTTP 200

POST {{url}}/album
{
    "id": "0",
    "name": "the record",
    "artists": ["0"],
    "release": 2023,
    "genres": ["indie pop"]
}
HTTP 403
GET {{url}}/album
HTTP 200
[Asserts]
jsonpath "$" count == 0

DELETE {{url}}/permissions/SystemTest
[
    "AlbumRead"
]
HTTP 200

GET {{url}}/album
HTTP 403


DELETE {{url}}/permissions/SystemTest
[
    "AlbumDelete"
]
HTTP 200

DELETE {{url}}/album/0
HTTP 403
# End Required Permissions

# Cleanup
DELETE {{url}}/artist/0
HTTP 200
DELETE {{url}}/artist/1
HTTP 200
DELETE {{url}}/genre/indie%20pop
HTTP 200
DELETE {{url}}/genre/indie%20rock
HTTP 200
DELETE {{url}}/user/SystemTest
HTTP 200
# End Cleanup
