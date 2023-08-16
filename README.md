# Creating an Axum API to learn rust

**` Warning `**
Code is often `convoluted`, `cluttered`, or `plain ugly` because this project was more of a playground for me than a serious endeavor, I apologize for any `headaches` i cause ahead of time

### In Code Examples Of
- https configuration using axum-server
- using axum extractors (Path, Query, State)
- layering an axum route with a resource
- sql queries and templating with tokio_postgres (sqlx to follow, no ORM's)
- custom serde deserializer (no seperate db model and response structs)
- async tests with tokio
- configuring pooling with deadpool_postgres
- simple client/connection configuration with tokio_postgres
- using tokio async tasks, blocking tasks and channels to handle an interupt signal
- derive procedural macros (badly written)

To Start

1. Set environment variables 
```dotenv
POSTGRES_HOST=hostname
POSTGRES_PORT=port
POSTGRES_USER=username
POSTGRES_DBNAME=datebase name
CERT_FOLDER=path to folder containing certificate and private key under project folder e.g /src/mykeyshere
CERT_NAME=name of ssl certificate
KEY_NAME=name of ssl private key
```
2. From Terminal in bin folder

Cargo run

## GraphQL api to follow
