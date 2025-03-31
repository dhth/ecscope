# Deployments Web UI

Local Development
---

Prequisites

- gleam

```sh
# start local development server
# from project root
cargo run -- deps <PROFILE> -m web

cd src/server/deployments/client
# replace window.location() in ./src/effects.gleam with http://127.0.0.1:<PORT>
gleam run -m lustre/dev start
```

Before committing code
---

```sh
# ensure local changes in src/server/deployments/client/src/effects.gleam are
# reverted
cd src/server/deployments/client

# compile app to js code
gleam run -m lustre/dev build app
```
