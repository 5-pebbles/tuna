POST {{url}}/init
{
    "username": "SystemTest",
    "password": "BadPass123"
}
HTTP 200

POST {{url}}/init
{
    "username": "SystemTest",
    "password": "BadPass123"
}
HTTP 409


POST {{url}}/token
{
    "username": "SystemTest",
    "password": "ReallyBadPass123"
}
HTTP 403
[Asserts]
cookie "token" not exists

POST {{url}}/token
{
    "username": "SystemTest",
    "password": "BadPass123"
}
HTTP 200
[Asserts]
jsonpath "$" exists
cookie "token" exists


GET {{url}}/user?username=notauser
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/user?limit=0
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET {{url}}/user?permissions=[InviteWrite]
HTTP 200
[Asserts]
jsonpath "$" count == 1

GET {{url}}/user?username=ystemT
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].username" == "SystemTest"
jsonpath "$[0].permissions[0]" exists
jsonpath "$[0].hash" not exists
jsonpath "$[0].password" not exists
jsonpath "$[0].tokens" not exists

DELETE {{url}}/user/SystemTest
HTTP 200
