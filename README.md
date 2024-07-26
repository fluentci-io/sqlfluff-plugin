# Sqlfluff Plugin

[![ci](https://github.com/fluentci-io/sqlfluff-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/sqlfluff-plugin/actions/workflows/ci.yml)

This plugin sets up [sqlfluff](https://www.sqlfluff.com/) in your CI/CD pipeline.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm sqlfluff setup
```

## Functions

| Name   | Description              |
| ------ | ------------------------ |
| setup  | Install sqlfluff.        |
| lint   | Lint SQL files.          |
| fix    | Fix SQL files.           |
| format | Format SQL files.        |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/sqlfluff@v0.1.1?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: sqlfluff
    args: |
      setup
- name: Show sqlfluff version
  run: |
    type sqlfluff
    sqlfluff --version
```
