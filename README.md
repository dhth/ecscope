<p align="center">
  <h1 align="center">ecscope</h1>
  <p align="center">
    <a href="https://github.com/dhth/ecscope/actions/workflows/build.yml"><img alt="GitHub release" src="https://img.shields.io/github/actions/workflow/status/dhth/ecscope/build.yml?style=flat-square"></a>
  </p>
</p>

`ecscope` lets you monitor AWS ECS resources from the terminal.

https://github.com/user-attachments/assets/5abea06a-0749-4e5c-9ac8-3811c895a5c9

🤔 Motivation
---

At work, we run multiple ECS services deployed across several accounts and
environments. Accessing each service through the AWS UI — which often requires
logging into separate accounts — is tedious and time-consuming. I needed a more
efficient way to access ECS deployment updates.

💾 Installation
---

**cargo**:

```sh
cargo install --git https://github.com/dhth/ecscope.git
```

⚡️ Usage
---

```text
Usage: ecscope [OPTIONS] <COMMAND>

Commands:
  profiles  Manage ecscope's profiles
  monitor   Open monitoring TUI
  help      Print this message or the help of the given subcommand(s)

Options:
      --debug  Output debug information without doing anything
  -h, --help   Print help
```

📃 Profiles
---

### Adding a profile

The first thing you'll do after installing `ecscope` is to set up a profile. A
profile is simply configuration that groups together ECS resources you want to
monitor in one go. You set up a profile using:

```bash
ecscope profiles add <PROFILE_NAME>
```

This will generate a TOML file in your config directory that looks like this:

```toml
[[clusters]]
keys = ["<KEY>"]
arn = "arn:aws:ecs:eu-central-1:<ACCOUNT_ID>:cluster/<CLUSTER_NAME>"
services = [
  "service-a",
  "service-b"
]
config_source = "env"

# --- #

[[clusters]]
keys = ["<KEY>"]
arn = "arn:aws:ecs:eu-central-1:<ACCOUNT_ID>:cluster/<CLUSTER_NAME>"
services = [
  "service-a",
  "service-b"
]
config_source = "profile:<PROFILE_NAME>"
```

### Listing profiles

You can list configured profiles using `ecscope profiles list`.

🛠 AWS Configuration
---

`ecscope` supports getting AWS configuration from the following sources:

### Environment variables

This is configured by setting `config_source` to `"env"` in the profile config.
The following environment variables need to be set for this option.

- AWS_ACCESS_KEY_ID
- AWS_SECRET_ACCESS_KEY
- AWS_SESSION_TOKEN
- AWS_REGION

Read more about this
[here](https://docs.aws.amazon.com/sdkref/latest/guide/environment-variables.html).

### Shared config

Use this option when you want to leverage profiles set in the [shared AWS
config](https://docs.aws.amazon.com/sdkref/latest/guide/file-format.html) for
authentication and configuration.

`ecscope` can be configured to use this option by setting `config_source` to
`"profile:<PROFILE_NAME>"` in the profile config.

📟 Monitoring TUI
---

![tui-2](https://github.com/user-attachments/assets/c7d7f005-f582-4eff-a685-dc8d3c8e0b61)

Once a profile is configured, you can begin monitoring ECS deployments via
`ecscope`'s TUI.

```bash
ecscope monitor <PROFILE_NAME>
```

The TUI displays running tasks for each configured service, along with their
respective containers. Additionally, details for the currently selected service,
task, and container are shown in dedicated panes to the right.

The TUI also supports refreshing of results — either on a schedule or manually.
Additionally, you can mark specific services to be targeted for the refresh.

### TUI Reference Manual

```text
Keymaps
---

General
    ?                    show/hide help view
    Esc / q              go back/exit
    <ctrl+c>             exit immediately

Main View
    j / ↓                go down in a list
    k / ↑                go up in a list
    H / ←                move to the pane to the left
    J / Tab              move to the pane below
    K / <S-Tab>          move to the pane above
    L / →                move to the pane to the right
    r                    refresh details for current item
    <c-r>                refresh data (either the ones marked, or all)
    R                    toggle auto refresh (for either the ones marked, or all)

Services List
    m                    mark service for auto refresh
```

### Filtering services to be monitored

You can filter services using two kinds of filter queries, one for the cluster
key and the other for the service name.

```bash
# will show all services that match a regex .*-service
ecscope monitor profile -s '.*-service'
# will show all services in clusters where a key matches the regex qa|staging
ecscope monitor profile -k 'qa|staging'
# combine both filters
ecscope monitor profile -s '.*-service' -k 'qa'
```
