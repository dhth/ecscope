<p align="center">
  <h1 align="center">ecscope</h1>
  <p align="center">
    <a href="https://github.com/dhth/ecscope/actions/workflows/build.yml"><img alt="GitHub release" src="https://img.shields.io/github/actions/workflow/status/dhth/ecscope/build.yml?style=flat-square"></a>
    <a href="https://crates.io/crates/ecscope"><img alt="GitHub release" src="https://img.shields.io/crates/v/ecscope?style=flat-square"></a>
    <a href="https://github.com/dhth/ecscope/releases/latest"><img alt="Latest release" src="https://img.shields.io/github/release/dhth/ecscope.svg?style=flat-square"></a>
    <a href="https://github.com/dhth/ecscope/releases"><img alt="Commits since latest release" src="https://img.shields.io/github/commits-since/dhth/ecscope/latest?style=flat-square"></a>
  </p>
</p>

`ecscope` lets you monitor AWS ECS resources from the terminal.

![tui](https://github.com/user-attachments/assets/c7d7f005-f582-4eff-a685-dc8d3c8e0b61)

It does so by offering a TUI which shows services, tasks and containers in a
single view. Instead of having to log into several accounts (or change regions)
via the AWS website, you're able to view relevant information for ECS
deployments in one place. You can group services by configuring them via a
"profile", and have `ecscope` load it up.

💾 Installation
---

**homebrew**:

```sh
brew install dhth/tap/ecscope
```

**cargo**:

```sh
cargo install ecscope
```

Or get the binaries directly from a Github [release][1]. Read more about
verifying the authenticity of released artifacts
[here](#-verifying-release-artifacts).

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

https://github.com/user-attachments/assets/5abea06a-0749-4e5c-9ac8-3811c895a5c9

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

🔐 Verifying release artifacts
---

In case you get the `ecscope` binary directly from a [release][1], you may want
to verify its authenticity. Checksums are applied to all released artifacts, and
the resulting checksum file is attested using [Github Attestations][2].

Steps to verify (replace `A.B.C` in the commands below with the version you
want):

1. Download the sha256 checksum file for your platform from the release:

   ```shell
   curl -sSLO https://github.com/dhth/ecscope/releases/download/vA.B.C/ecscope-x86_64-unknown-linux-gnu.tar.xz.sha256
   ```

2. Verify the integrity of the checksum file using [gh][3].

   ```shell
   gh attestation verify ecscope-x86_64-unknown-linux-gnu.tar.xz.sha256 --repo dhth/ecscope
   ```

3. Download the compressed archive you want, and validate its checksum:

   ```shell
   curl -sSLO https://github.com/dhth/ecscope/releases/download/vA.B.C/ecscope-x86_64-unknown-linux-gnu.tar.xz
   sha256sum --ignore-missing -c ecscope-x86_64-unknown-linux-gnu.tar.xz.sha256
   ```

3. If checksum validation goes through, uncompress the archive:

   ```shell
   tar -xzf ecscope-x86_64-unknown-linux-gnu.tar.xz
   cd ecscope-x86_64-unknown-linux-gnu
   ./ecscope
   # profit!
   ```

[1]: https://github.com/dhth/ecscope/releases
[2]: https://github.blog/news-insights/product-news/introducing-artifact-attestations-now-in-public-beta/
[3]: https://github.com/cli/cli
