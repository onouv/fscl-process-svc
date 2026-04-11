# fscl-process-svc

Bounded-context service crate for the process view.

Kubernetes naming uses `process-api` for the workload to avoid colliding with the Kubernetes `Service` suffix, but the git repo and crate name stay `fscl-process-svc`.

The crate currently compiles in an earlier design state. It is not yet migrated to use `ComponentLifecycleUow` end to end, and its future NATS-consumer role is scaffolded at the deployment layer rather than fully implemented in-process.

## Dependencies

```text
fscl-process-svc
    |
    +--> fscl_core
    |
    +--> migration

dev / k8s runtime
    |
    +--> sidecar fscl-outbox-publisher
```

And the deployment-level relationship is:

```text
process-api container  --> PostgreSQL
process-api container  --> NATS consumer role (scaffolded)
outbox-publisher sidecar --> PostgreSQL outbox
outbox-publisher sidecar --> NATS publisher role
```

## Split

```text
fscl-process-svc        -> HTTP API, process view persistence, current app logic
fscl_core               -> shared domain/application contracts
fscl-messaging          -> shared outbox/wire contract, used indirectly today
fscl-outbox-publisher   -> sidecar publisher runtime
```

## Config

Shared/local dev loading order:

1. `../.env.shared`
2. local `.env`
3. container or shell overrides

Current local config points:

- `DB_TYPE`: database scheme, currently `postgres`
- `DB_HOST`: database host
- `DB_PORT`: database port
- `DB_USER`: database user
- `DB_PASSWORD`: database password
- `DB_NAME`: process bounded-context database name
- `APP_HOST`: bind address for the HTTP server
- `APP_PORT`: bind port for the HTTP server
- `NATS_URL`: reserved for the process-api consumer role and shared local setup
- `NATS_JETSTREAM_STREAM`: target JetStream stream name for the bounded context
- `NATS_JETSTREAM_DURABLE_CONSUMER`: durable consumer name for `process-api`
- `NATS_JETSTREAM_ACK_POLICY`: intended acknowledgement policy
- `NATS_JETSTREAM_ACK_WAIT`: intended acknowledgement timeout

Today the DB settings are used by the running crate. The NATS consumer settings are scaffolding for the target architecture.

## Dev Setup

Create the env files:

```sh
cp ../.env.shared.example ../.env.shared
cp .env.example .env
```

Run tests/build locally:

```sh
cd fscl-process-svc
cargo test
cargo run
```

Run the local dev stack:

```sh
docker compose -f ../compose/infra.yaml -f ../compose/process-stack.yaml up
```

The service currently applies its own SeaORM migration on startup.

## K8s Setup

Scaffold only for now.

Relevant manifests live under [fscl/doc/rust/fscl-k8s](../fscl/doc/rust/fscl-k8s):

- `15-process-messaging.yaml`: shared messaging runtime values for the bounded context
- `20-process-api.yaml`: `process-api` deployment and sidecar wiring
- `22-outbox-publisher.yaml`: publisher-local settings
- `30-postgres.yaml`: bounded-context database

Target workload split:

```text
process-api pod
  |- process-api container       -> serves HTTP, will consume NATS events
  |- outbox-publisher sidecar    -> relays DB outbox rows to NATS
```

The database-owning service remains responsible for applying both its own schema and the shared outbox schema.
