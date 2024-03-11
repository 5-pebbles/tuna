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
POST {{url}}/album
{
    "id": "0",
    "name": "the record",
    "artists": ["0"],
    "release": 2023,
    "genres": ["indie pop"]
}
HTTP 200
POST {{url}}/album
{
    "id": "1",
    "name": "Better Oblivion Community Center",
    "artists": ["0", "1"],
    "release": 2019,
    "genres": ["indie pop", "indie rock"]
}
HTTP 200
# End Setup

# Create Track
POST {{url}}/track
{
    "id": "0",
    "name": "Service Road",
    "release": 2019,
    "albums": ["1"],
    "lyrics": "You should really call your brother\nSomeone put up a picture where he can't stand\nHe's holding onto the table\nProbably throwing darts or playing cards\nSomething that he lost made him so mad",
    "genres": ["indie rock"]
}
HTTP 200

# Same Id
POST {{url}}/track
{
    "id": "0",
    "name": "Anti-Curse",
    "release": 2023,
    "albums": ["0"],
    "lyrics": "Gettin' deep\nI'm out of my depth at a public beach\nI never listened, I had to see for myself\nIt's comin' in waves\nShoots through my mind like a pinball strayed\nFriendly fire, point-blank\nSalt in my lungs\nHoldin' my breath\nMakin' peace with my inevitable death",
    "genres": ["indie pop"]
}
HTTP 409

POST {{url}}/track
{
    "id": "1",
    "name": "Anti-Curse",
    "release": 2023,
    "albums": ["0"],
    "lyrics": "Gettin' deep\nI'm out of my depth at a public beach\nI never listened, I had to see for myself\nIt's comin' in waves\nShoots through my mind like a pinball strayed\nFriendly fire, point-blank\nSalt in my lungs\nHoldin' my breath\nMakin' peace with my inevitable death",
    "genres": ["indie pop"]
}
HTTP 200
# End Create Track

# Get Tracks
GET {{url}}/track
HTTP 200
[Asserts]
jsonpath "$" count == 2

GET {{url}}/track?id=1
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "1"
jsonpath "$[0].name" == "Anti-Curse"
jsonpath "$[0].release" == 2023
jsonpath "$[0].albums" count == 1
jsonpath "$[0].albums" includes "0"
jsonpath "$[0].artists" count == 1
jsonpath "$[0].albums" includes "0"
jsonpath "$[0].genres" count == 1 
jsonpath "$[0].genres" includes "indie pop"
jsonpath "$[0].lyrics" contains "through my mind like a pinball stray"
jsonpath "$[0].duration" == 0

GET {{url}}/track?id=4
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/track?name=Servic
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "0"

GET {{url}}/track?maxrelease=2022
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "0"

GET {{url}}/track?minrelease=2022
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "1"

GET {{url}}/track?genres=%5B%22indie%20pop%22%5D
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "1"

GET {{url}}/track?albums=%5B%220%22%5D # ["0"]
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "1"

GET {{url}}/track?artists=%5B%221%22%5D # ["0"]
HTTP 200
[Asserts]
jsonpath "$" count == 1

GET {{url}}/track?lyrics=holding%20onto%20the%20table
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "0"

GET {{url}}/track?limit=0
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/track?artists=%5B%220%22%5D&limit=1 # artists = ["0"]
HTTP 200
[Asserts]
jsonpath "$" count == 1
# End Get Tracks

# Delete Track
DELETE {{url}}/track/0
HTTP 200

DELETE {{url}}/track/0
HTTP 404

DELETE {{url}}/track/1
HTTP 200
# End Delete Track

# Required Permissions
DELETE {{url}}/permissions/SystemTest
[
    "TrackWrite"
]
HTTP 200

POST {{url}}/track
{
    "id": "0",
    "name": "Service Road",
    "release": 2019,
    "albums": ["1"],
    "lyrics": "You should really call your brother\nSomeone put up a picture where he can't stand\nHe's holding onto the table\nProbably throwing darts or playing cards\nSomething that he lost made him so mad",
    "genres": ["indie rock"]
}
HTTP 403
GET {{url}}/track
HTTP 200
[Asserts]
jsonpath "$" count == 0

DELETE {{url}}/permissions/SystemTest
[
    "TrackRead"
]
HTTP 200

GET {{url}}/track
HTTP 403


DELETE {{url}}/permissions/SystemTest
[
    "TrackDelete"
]
HTTP 200

DELETE {{url}}/track/0
HTTP 403
# End Required Permissions

# Cleanup
DELETE {{url}}/album/0
HTTP 200
DELETE {{url}}/album/1
HTTP 200
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
