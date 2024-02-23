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
[Captures]
rootSession: cookie "session"
# End Setup


# create
POST {{url}}/invite
{
    "code": "peppers",
    "permissions": ["InviteDelete", "InviteWrite"],
    "remaining": 2
}
HTTP 200
[Asserts]
jsonpath "$.code" == "peppers"
jsonpath "$.permissions" includes "InviteDelete"
jsonpath "$.permissions" includes "InviteWrite"
jsonpath "$.remaining" == 2
jsonpath "$.creator" == "SystemTest"


# queries
GET {{url}}/invite?code=badcode
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/invite?code=peppers
HTTP 200
[Asserts]
jsonpath "$" count == 1

GET {{url}}/invite?permissions=%5B%22InviteRead%22%5D
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/invite?permissions=%5B%22InviteDelete%22%5D
HTTP 200
[Asserts]
jsonpath "$" count == 1

GET {{url}}/invite?maxremaining=1
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/invite?maxremaining=2
HTTP 200
[Asserts]
jsonpath "$" count == 1

GET {{url}}/invite?minremaining=3
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/invite?minremaining=2
HTTP 200
[Asserts]
jsonpath "$" count == 1

GET {{url}}/invite?creator=BadUsername
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/invite?creator=SystemT
HTTP 200
[Asserts]
jsonpath "$" count == 1

GET {{url}}/invite?code=peppers&limit=0
HTTP 200
[Asserts]
jsonpath "$" count == 0

# use
POST {{url}}/invite/peppers
{
    "username": "SystemTest2",
    "password": "BadPass123"
}
HTTP 200

GET {{url}}/invite?code=peppers
HTTP 200
[Asserts]
jsonpath "$[0].remaining" == 1

# last use should delete it
POST {{url}}/invite/peppers
{
    "username": "SystemTest3",
    "password": "BadPass123"
}
HTTP 200

GET {{url}}/invite?code=peppers
Cookie: session={{rootSession}}
HTTP 200
[Asserts]
jsonpath "$" count == 0


# only create invites for which you have permissions
POST {{url}}/login
{
    "username": "SystemTest2",
    "password": "BadPass123"
}
HTTP 200

POST {{url}}/invite
{
    "code": "peppers2",
    "permissions": ["InviteRead"],
    "remaining": 1
}
HTTP 403


# delete
POST {{url}}/invite
{
    "code": "peppers2",
    "permissions": ["InviteDelete"],
    "remaining": 1
}
HTTP 200

DELETE {{url}}/invite/peppers2
HTTP 200

GET {{url}}/invite?code=peppers2
Cookie: session={{rootSession}}
HTTP 200
[Asserts]
jsonpath "$" count == 0

# Cleanup
DELETE {{url}}/user/SystemTest3
Cookie: session={{rootSession}}
HTTP 200
DELETE {{url}}/user/SystemTest2
Cookie: session={{rootSession}}
HTTP 200
DELETE {{url}}/user/SystemTest
Cookie: session={{rootSession}}
HTTP 200
