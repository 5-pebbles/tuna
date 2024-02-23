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
POST {{url}}/login
{
    "username": "SystemTest2",
    "password": "BadPass123"
}
# End Setup

# delete for self without the sessiondelete permission
DELETE {{url}}/session/SystemTest2
HTTP 200

DELETE {{url}}/session/SystemTest2
HTTP 401


# delete for other with sessiondelete permission
POST {{url}}/login
{
    "username": "SystemTest2",
    "password": "BadPass123"
}
HTTP 200
[Captures]
subSession: cookie "session"

DELETE {{url}}/session/SystemTest2
Cookie: session={{rootSession}}
HTTP 200

DELETE {{url}}/session/SystemTest2
Cookie: session={{subSession}}
HTTP 401

# Cleanup
DELETE {{url}}/user/SystemTest2
Cookie: session={{rootSession}}
HTTP 200
DELETE {{url}}/user/SystemTest
Cookie: session={{rootSession}}
HTTP 200
