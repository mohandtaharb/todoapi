# todoapi

A type-safe, memory-safe API for a data-driven TODO app implemented using Rust and Rocket. This app uses PostgreSQL as database and Diesel as an ORM.

## Build instructions
1. Download and install the [Rust toolchain](https://www.rust-lang.org/tools/install)
2. Install the Diesel CLI using `cargo install diesel-cli`
3. Clone the repository and build the app using the following commands
```sh
git clone https://github.com/mohandtaharb/todoapi.git
cd todoapi
cargo build
```
4. Rename `.env.example` to `.env` and set up the variables
5. Execute `diesel migration run` to set up the database
6. Start the web server with `cargo run`

## Authorization
A `POST` request is sent to `/user/login` with the following JSON in the body : `{username: "username", password: "password"}`. The server then sends back a Token that will be used in the Authorization header of HTTP requests to access the secure endpoints.

## Endpoints
`GET /tasks/` - Fetch all tasks
Example response
```json
[
	{
		"id": 1,
		"name": "Buy milk",
		"accomplished": true
	},
	{
		"id": 2,
		"name": "Buy bread",
		"accomplished": false
	}
]
```

`POST /tasks/` - Create a new task

Request body : 
```
{
	name: string
}
```
Example Request
```sh
curl --location 'localhost:8000/tasks' \
--header 'Content-Type: application/json' \
--data '{
"name": "Become the next linus torvalds"
}'
```
Response body 
```
{

"id": int,

"name": string,

"accomplished": bool

}
```

`GET /tasks/<id>` - Fetch a task by id
Request : 
```sh
curl --location 'localhost:8000/tasks/3'
```

Response : 

```
{

"id": int,

"name": string,

"accomplished": bool

}
```

`GET /tasks/pending`  - Fetch all unaccomplished tasks

`GET /tasks/accomplished`  - Fetch all accomplished tasks

`DELETE /tasks/<id>`  - Delete task by id

`PATCH /tasks/<id>` - Change the `id` tasks' title.

Body :
```
{
	name: string
}
```
Response
```
{

"id": int,

"name": string,

"accomplished": bool

}
```
