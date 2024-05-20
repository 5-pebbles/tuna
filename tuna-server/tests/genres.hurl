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
# End Setup

# Create Genres
POST {{url}}/genre/goodmusic
HTTP 200

# can't create the same genre twice
POST {{url}}/genre/goodmusic
HTTP 409

POST {{url}}/genre/badmusic
HTTP 200
# End Create Genres

# Get Genres

# should return all
GET {{url}}/genre
HTTP 200
[Asserts]
jsonpath "$" count == 2
jsonpath "$" includes "goodmusic"
jsonpath "$" includes "badmusic"

GET {{url}}/genre?genre=music
HTTP 200
[Asserts]
jsonpath "$" count == 2

GET {{url}}/genre?genre=badmusic
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0]" == "badmusic"

GET {{url}}/genre?genre=goodmusic
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0]" == "goodmusic"
# End Get Genres


# Delete Genres
DELETE {{url}}/genre/goodmusic
HTTP 200

DELETE {{url}}/genre/goodmusic
HTTP 404

DELETE {{url}}/genre/badmusic
HTTP 200
# End Delete Genres

# Required Permissions
DELETE {{url}}/permission/SystemTest
[
    "GenreWrite"
]
HTTP 200

POST {{url}}/genre/fakemusic
HTTP 403

DELETE {{url}}/permission/SystemTest
[
    "GenreRead"
]
HTTP 200

GET {{url}}/genre
HTTP 403

DELETE {{url}}/permission/SystemTest
[
    "GenreDelete"
]
HTTP 200

DELETE {{url}}/genre/fakemusic
HTTP 403
# End Required Permissions

# Cleanup
DELETE {{url}}/user/SystemTest
HTTP 200
# End Cleanup
