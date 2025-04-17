<!-- @format -->

Assets
none

Components
shared = { path = "../shared", features = ["frontend"] }
routes = { path = "../routes" }

Pages
components = { path = "../components" } # Relative path to your components crate
shared = { path = "../shared" }

Routes:
components = { path = "../components" }

Server
shared = { workspace = true }

Shared

## Notes

shared/ # Only pure data types, no UI
components/ # UI components (depends on shared)
pages/ # Pages (depends on components and shared)
server/ # Backend (depends only on shared)
