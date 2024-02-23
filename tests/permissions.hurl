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
POST {{url}}/invite
{
    "code": "peppers",
    "permissions": ["InviteDelete", "InviteWrite"],
    "remaining": 1
}
HTTP 200
POST {{url}}/invite/peppers
{
    "username": "SystemTest2",
    "password": "BadPass123"
}
HTTP 200
# End Setup


POST {{url}}/permissions/SystemTest2
[
    "InviteRead"
]
HTTP 200

GET {{url}}/user?username=SystemTest2
HTTP 200
[Asserts]
jsonpath "$[0].permissions" includes "InviteRead"

DELETE {{url}}/permissions/SystemTest2
[
    "InviteRead"
]
HTTP 200

GET {{url}}/user?username=SystemTest2
HTTP 200
[Asserts]
jsonpath "$[0].permissions" not includes "InviteRead"

# Cleanup
DELETE {{url}}/user/SystemTest2
HTTP 200
DELETE {{url}}/user/SystemTest
HTTP 200
