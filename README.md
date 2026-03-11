# FSCL Process Service — Hexagonal Skeleton 

A web service for managing Components of the FSCL Process View.

## Architecture

Implementig a hexagonal architecture.

### Adapters: 

#### Inbound Web/ REST API

Wrapping Axum in an `HttpServer` type. 

#### Outbound Database

PostgreSQL 12+

`ComponentRepository`

### Ports

`ComponentPort`

### Domain

#### Application Services
`ComponentService` 
- implementing `ComponentPort`
- using `ComponentRepository`


## Setup
### Database Setup

```bash
# from project root folder
docker compose --env-file .env -f ./docker/compose.yaml up
```
Assuming a postgres server is now running on localhost, observe that an empty db has been started: 

```bash
psql -h localhost -p 5432 -U postgres
postgres=# \d
Did not find any relations.
postgres=# 
```

Note: the system will initialize the database by executing a sea-orm migration on startup.

## Run
```bash
# Build and run
cargo build --release
cargo run --release
```
or apply log flags

```bash 
RUST_LOG=trace cargo run
``` 
## Example Usage

### Create a Component
```bash
curl -X POST http://localhost:3100/api/v2/components \
  -H "Content-Type: application/json" \
  -d '{"id": "100", "name": "Door Lock" }'
```


## License

MIT
