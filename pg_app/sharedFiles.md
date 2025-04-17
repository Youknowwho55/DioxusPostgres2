<!-- @format -->

App:
pg_app/app/Cargo.toml
main entry into application: pg_app/src/main.rs

Assets:
pg_app/assets/Cargo.toml
Dependencies: none

Components:
pg_app/components/Cargo.toml
Dependencies:
shared = { path = "../shared", features = ["frontend"] }
server = { path = "../server" }
routes = { path = "../routes" }

Pages:
pg_app/pages/Cargo.toml
Dependencies:
components = { path = "../components" } # Relative path to your components crate
shared = { path = "../shared" }

Server:
pg_app/server/Cargo.toml
Dependencies:
shared = { workspace = true }

Shared:
pg_app/shared/Cargo.toml

## Notes

shared/ # Only pure data types, no UI
components/ # UI components (depends on shared)
pages/ # Pages (depends on components and shared)
server/ # Backend (depends only on shared)

.
├── Cargo.toml
├── app/
│ ├── Cargo.toml
│ └── src/
│ └── main.rs
├── server/
│ └── Cargo.toml
├── shared/
│ └── Cargo.toml
├── components/
│ └── Cargo.toml
└── pages/
└── Cargo.toml
