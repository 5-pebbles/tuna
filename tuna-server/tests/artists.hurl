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
HTTP 200
# End Setup

POST {{url}}/artist
{
    "id": "0",
    "name": "Phoebe Bridgers",
    "genres": ["not a real one"],
    "bio": ""
}
HTTP 400

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
    "name": "boygenius",
    "genres": ["indie pop"],
    "bio": ""
}
HTTP 200

# Same Id
POST {{url}}/artist
{
    "id": "1",
    "name": "Conor Oberst",
    "genres": [],
    "bio": ""
}
HTTP 409

# Same Name
POST {{url}}/artist
{
    "id": "2",
    "name": "boygenius",
    "genres": [],
    "bio": ""
}
HTTP 200


GET {{url}}/artist
HTTP 200
[Asserts]
jsonpath "$" count == 3

GET {{url}}/artist?id=0
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].name" == "Phoebe Bridgers"
jsonpath "$[0].genres" includes "indie pop"

GET {{url}}/artist?name=%20Bridgers
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].name" == "Phoebe Bridgers"
jsonpath "$[0].genres" includes "indie pop"

GET {{url}}/artist?genres=%5B%22indie%20pop%22%5D
HTTP 200
[Asserts]
jsonpath "$" count == 2
jsonpath "$[0].genres" includes "indie pop" 
jsonpath "$[1].genres" includes "indie pop" 

GET {{url}}/artist?limit=2
HTTP 200
[Asserts]
jsonpath "$" count == 2

DELETE {{url}}/artist/2
HTTP 200

DELETE {{url}}/artist/2
HTTP 404

DELETE {{url}}/artist/1
HTTP 200

DELETE {{url}}/artist/0
HTTP 200


DELETE {{url}}/permission/SystemTest
[
    "ArtistWrite"
]
HTTP 200

POST {{url}}/artist
{
    "id": "4",
    "name": "Conor Oberst",
    "genres": [],
    "bio": ""
}
HTTP 403

DELETE {{url}}/permission/SystemTest
[
    "ArtistRead"
]
HTTP 200

GET {{url}}/artist
HTTP 403

DELETE {{url}}/permission/SystemTest
[
    "ArtistDelete"
]
HTTP 200

DELETE {{url}}/artist/0
HTTP 403

# Cleanup
DELETE {{url}}/genre/indie%20pop
HTTP 200
DELETE {{url}}/user/SystemTest
HTTP 200
