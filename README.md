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

Assuming a postgres server is running on localhost: 

```bash
>> psql -H localhost -P 5432 -U postgres
postgres=# create database process_svc
...
postgres=# \c process_svc
```

Paste the following into the psql commandline:
```sql
-- Create tables (manual or via migrations)
CREATE TABLE functions (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  parent_id TEXT REFERENCES functions(id) ON DELETE CASCADE,
  version INT NOT NULL DEFAULT 1,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE components (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  parent_id TEXT REFERENCES components(id) ON DELETE CASCADE,
  version INT NOT NULL DEFAULT 1,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE component_implements_function (
  id TEXT PRIMARY KEY,
  component_id TEXT NOT NULL REFERENCES components(id) ON DELETE CASCADE,
  function_id TEXT NOT NULL REFERENCES functions(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL,
  UNIQUE(component_id, function_id)
);

CREATE INDEX idx_functions_parent_created ON functions(parent_id, created_at);
CREATE INDEX idx_components_parent_created ON components(parent_id, created_at);
CREATE INDEX idx_impl_component ON component_implements_function(component_id);
```

## Run
```bash
# Set database URL
Modify entries in `.env`


# Build and run
cargo build --release
cargo run --release

# Server listens on http://0.0.0.0:8080
```

## Example Usage

### Create a Component
```bash
curl -X POST http://localhost:8080/api/v1/process/components \
  -H "Content-Type: application/json" \
  -d '{"id": "C100", "name": "Door Lock" }'
```


## License

MIT
