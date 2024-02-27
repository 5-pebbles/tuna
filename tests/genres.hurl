POST {{url}}/init
{
    "username": "SystemTest",
    "password": "BadPass123"
}
HTTP 200
POST {{url}}/login
{
    "username": "SystemTest",
    "password": "BadPass123"
}
HTTP 200
# End Setup

POST {{url}}/genre
{
    "id": 0,
    "name": "goodmusic"
}
HTTP 200
POST {{url}}/genre
{
    "id": 0,
    "name": "goodmusic"
}
HTTP 409

POST {{url}}/genre
{
    "id": 1,
    "name": "badmusic"
}
HTTP 200

GET {{url}}/genre?name=music
HTTP 200
[Asserts]
jsonpath "$" count == 2

GET {{url}}/genre?id=1
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == 1
jsonpath "$[0].name" == "badmusic"

GET {{url}}/genre?name=goodmusic
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == 0
jsonpath "$[0].name" == "goodmusic"


DELETE {{url}}/genre/0
HTTP 200

GET {{url}}/genre
HTTP 200
[Asserts]
jsonpath "$" count == 1

DELETE {{url}}/genre/1
HTTP 200

DELETE {{url}}/permissions/SystemTest
[
    "GenreWrite"
]
HTTP 200

POST {{url}}/genre
{
    "id": 2,
    "name": "fakemusic"
}
HTTP 403

DELETE {{url}}/permissions/SystemTest
[
    "GenreRead"
]
HTTP 200

GET {{url}}/genre
HTTP 403

DELETE {{url}}/permissions/SystemTest
[
    "GenreDelete"
]
HTTP 200

DELETE {{url}}/genre/0
HTTP 403

# Cleanup
DELETE {{url}}/user/SystemTest
HTTP 200
