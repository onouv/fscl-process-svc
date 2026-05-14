# fscl-process-svc

Bounded-context service crate for the process view.

Kubernetes naming uses `process-api` for the workload to avoid colliding with the Kubernetes `Service` suffix, but the git repo and crate name stay `fscl-process-svc`.

The crate currently runs HTTP APIs and an in-process JetStream consumer for `ProjectCreatedEvent` (`events.project.created`) using core event-handling support.

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

Runtime values are read from process environment variables (shell or container).

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
- `NATS_PROJECT_CREATED_SUBJECT`: subject consumed for `ProjectCreatedEvent` (default `events.project.created`)
- `NATS_SUBJECT`: fallback subject if `NATS_PROJECT_CREATED_SUBJECT` is not set
- `NATS_JETSTREAM_STREAM`: target JetStream stream name for the bounded context
- `NATS_JETSTREAM_DURABLE_CONSUMER`: durable consumer name for `process-api`
- `NATS_JETSTREAM_ACK_POLICY`: intended acknowledgement policy
- `NATS_JETSTREAM_ACK_WAIT`: intended acknowledgement timeout

Today both DB and NATS consumer settings are used by the running crate.

## Dev Setup

Load local dev secrets from the compose helper:

```sh
source ../compose/load-secrets.sh
```

For host-side API debugging, load process-api runtime variables:

```sh
source ../compose/load-process-api-env.sh
```

Run tests/build locally:

```sh
cd fscl-process-svc
cargo test
cargo run
```

Run the local dev stack:

```sh
source ../compose/load-secrets.sh
docker compose -p fscl -f ../compose/process-stack.yaml up
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
