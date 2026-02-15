# FSCL Process Service — DB-First Skeleton

A scalable, database-backed web service for managing Functions and Components of the FSCL Process View.

## Architecture

This implementation uses a **DB-first, repository-based approach**:

- **Models**: SeaORM entities (`Function`, `Component`, `ComponentImplementsFunction`)
  - Plain domain types with no concurrency primitives
  - Persist to PostgreSQL with optimistic locking (version column)
  - Support parent-child hierarchies

- **Repository Layer** (`src/repository.rs`):
  - Single-responsibility methods for CRUD, relations, and operations
  - Errors typed with `thiserror`
  - Pagination support for list endpoints
  - Optimistic locking for concurrent updates

- **Handlers** (`src/handlers.rs`):
  - Actix-web async endpoints for REST API
  - JSON request/response DTOs
  - Proper HTTP status codes and error handling
  - Request validation and pagination

- **Server** (`src/main.rs`):
  - Tokio async runtime + Actix-web
  - PostgreSQL connection via SeaORM
  - Logging via `env_logger`

## API Endpoints

### Functions
- `POST /api/functions` — Create function
- `GET /api/functions` — List functions (paginated)
- `GET /api/functions/{id}` — Get function by ID
- `POST /api/functions/{id}/subs` — Add sub-function

### Components
- `POST /api/components` — Create component
- `GET /api/components` — List components (paginated)
- `GET /api/components/{id}` — Get component by ID
- `POST /api/components/{id}/subs` — Add sub-component
- `POST /api/components/{id}/implements` — Implement a function

### Health Check
- `GET /health` — Service health check

## Design Decisions

### Why Not Actors?
- Actors are great for single-threaded state machines, but with tens of thousands of entities, maintaining one actor per entity is wasteful.
- Database (PostgreSQL) is the canonical store; entities are loaded on-demand.
- Future: add sharded actor cache for hot-read optimization if needed.

### Concurrency & Consistency
- **Optimistic locking**: version field in entity prevents write conflicts
- **Pagination**: efficient list queries with limit/offset
- **Foreign keys**: database enforces referential integrity
- **Transactions**: SeaORM session can wrap multi-step operations

### Scalability
- Stateless handlers → horizontally scalable
- Connection pooling via SeaORM
- Database indices on `(parent_id, created_at)`
- Read replicas for analytics/reporting

## Setup

### Prerequisites
- Rust 1.70+
- PostgreSQL 12+

### Manual Database Setup

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

### Run
```bash
# Set database URL
Modify entries in `.env`


# Build and run
cargo build --release
cargo run --release

# Server listens on http://0.0.0.0:8080
```

## Example Usage

### Create a Function
```bash
curl -X POST http://localhost:8080/api/functions \
  -H "Content-Type: application/json" \
  -d '{"id": "=100", "name": "Protect PAX", "description": "Main safety function"}'
```

### Create a Component
```bash
curl -X POST http://localhost:8080/api/components \
  -H "Content-Type: application/json" \
  -d '{"id": "C100", "name": "Door Lock", "description": "Component that implements =100"}'
```

### Implement Function on Component
```bash
curl -X POST http://localhost:8080/api/components/C100/implements \
  -H "Content-Type: application/json" \
  -d '{"function_id": "=100"}'
```

### List with Pagination
```bash
curl "http://localhost:8080/api/functions?limit=20&offset=0"
```

## Future Enhancements

1. **Migrations**: Use `sea-orm-migration` CLI to generate and manage schema versions
2. **Read Models**: Denormalized views for reporting (CQRS pattern)
3. **Caching**: Redis or in-process LRU for hot reads (hierarchies, implementations)
4. **Sharded Actor Cache**: Load-on-demand actor cache for complex multi-entity operations
5. **GraphQL**: Query complex relationships efficiently
6. **Audit Trail**: Immutable event log for compliance
7. **Soft Deletes**: Timestamp-based soft deletes for data recovery
8. **WebSocket**: Real-time updates via actor broadcasts

## Dependencies

- `sea-orm`: ORM with async, typed queries
- `actix-web`: Web framework
- `tokio`: Async runtime
- `serde` / `serde_json`: Serialization
- `chrono`: Timestamps
- `uuid`: ID generation
- `thiserror`: Error handling
- `log` / `env_logger`: Logging

## License

MIT
