<!-- @format -->

dx serve --platform desktop

# DioxusPostgres2

The other file was all a makerfile. Re-doing

```
npm install tailwindcss @tailwindcss/cli
```

```
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Docker compose

```
docker compose up
```

<!-- @format -->

### tailwind

npx tailwindcss -i ./input.css -o ./ui/assets/tailwind.css --watch

### Using

1. Dioxus -- Axum and Tokio
2. Sqlx -- dont think this needs a generic connection pool like r2d2
   a. built in connection pool: sqlx::Pool
   sqlx = { version = "0.8", features = [ "runtime-tokio" ] }

## Create the database with sqlx-cli

sqlx database create
sqlx database drop

sqlx migrate add <name>

mkdir -p migrations/20250420000000_create_posts
touch migrations/20250420000000_create_posts/{up.sql,down.sql}

sqlx migrate run
sqlx migrate revert
sqlx migrate run

## Using PSQL

Terminal that can help run SQL commands. Comes when we download PG Admin

#### Connects to the DB?

```
psql -h localhost -p 5432 -d tester - U tester
```

#### SQLX database create

```
sqlx database create --database-url postgres://tester:tester@localhost:5432/tester
```

```
sqlx migrate add profile_table
```

```
sqlx migrate add message_table
```

```
sqlx migrate run --database-url postgres://tester:tester@localhost:5432/tester
```

-- Add migration script here

CREATE TABLE posts (
id SERIAL PRIMARY KEY,
title TEXT NOT NULL,
body TEXT NOT NULL
);

sqlx migrate run --database-url="postgres://myuser:mypassword@localhost:5435/mydatabase"

CREATE TABLE posts (
id SERIAL PRIMARY KEY,
title TEXT NOT NULL,
body TEXT NOT NULL
);

dx bundle --platform desktop \
 --package-types "macos" \
 --package-types "dmg"

## Possible email

Rust Crates to Consider

lettre: SMTP email sending
async-imap: IMAP client for receiving
mail-builder: Email composition
mail-parser: Email parsing
reqwest: For webhook handling

pg_app/app
pg_app/assets
pg_app/components
pg_app/migrations
pg_app/node_modules
pg_app/pages
pg_app/server
pg_app/shared

shared → (pure types only, no dependencies)
components → shared
pages → shared + components
server → shared
app → shared + components + pages
