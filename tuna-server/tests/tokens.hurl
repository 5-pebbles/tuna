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
[Captures]
rootToken: cookie "token"
POST {{url}}/invite
{
    "code": "peppers",
    "permissions": [],
    "remaining": 1
}
HTTP 200
POST {{url}}/invite/peppers
{
    "username": "SystemTest2",
    "password": "BadPass123"
}
HTTP 200
POST {{url}}/token
{
    "username": "SystemTest2",
    "password": "BadPass123"
}
# End Setup

# delete for self without the tokendelete permission
DELETE {{url}}/token/SystemTest2
HTTP 200

DELETE {{url}}/token/SystemTest2
HTTP 401


# delete for other with tokendelete permission
POST {{url}}/token
{
    "username": "SystemTest2",
    "password": "BadPass123"
}
HTTP 200
[Captures]
subToken: cookie "token"

DELETE {{url}}/token/SystemTest2
Cookie: token={{rootToken}}
HTTP 200

DELETE {{url}}/token/SystemTest2
Cookie: token={{subToken}}
HTTP 401

# Cleanup
DELETE {{url}}/user/SystemTest2
Cookie: token={{rootToken}}
HTTP 200
DELETE {{url}}/user/SystemTest
Cookie: token={{rootToken}}
HTTP 200
