<h1 align="center">Tuna 🍣</h1>

Tuna is an open source music api, designed to allow client side automation & contributions.

> Warning: this is currently in a work in progress...

## Testing

**Application Testing:**

```
cargo run
```

Then:

```
./tests/tests.sh
```

**Unit Testing:**

```
I don't have any unit tests yet...
```

<details><summary><h2>Api End Points</h2></summary>

<details><summary><code>POST /init</code></summary>

---

**Variables:**

- username: `String`.

- password: `String`

```hurl
# create the first account in the database
POST {{url}}/init
{
    "username": {username},
    "password": {password}
}
HTTP 200
```

</details>

<details><summary><code>POST /login</code></summary>

---

**Variables:**

- username: `String`.

- password: `String`

```hurl
POST {{url}}/login
{
    "username": {{username}},
    "password": {{password}}
}
HTTP 200
[Asserts]
cookie "session" exists
```

</details>

<details><summary><code>GET /user?username={{username}}&permissions={{permissions}}&limit={{limit}}</code></summary>

---

**Permissions**: `UserRead`

**Variables:**

- username: `String`.

- permissions: `URL encoded Json<Vec<String>>`.

- limit: `u16`.

```hurl
# partially matches for the given variables, all are optional.
GET {{url}}/user?username={{username}}&permissions={{permissions}}&limit={{limit}}
HTTP 200
[Asserts]
jsonpath "$" count <= {{limit}}
jsonpath "$[0].username" contains {{username}}
jsonpath "$[0].permissions" exists
```

**Response Example:**
```json
[
    {
        "username": "Owen",
        "permissions": ["InviteRead", "InviteDelete"]
    }
]
```

</details>

<details><summary><code>DELETE /user/{{username}}</code></summary>

---

**Permissions**: `None` If deleting your own account || `UserDelete` & All permissions of the target user

**Variables:**

- username: `String`.

```hurl
DELETE {{url}}/user/{{username}}
HTTP 200
```

</details>

</details>
