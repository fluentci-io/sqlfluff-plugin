use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup() -> FnResult<String> {
    let stdout = dag().call(
        "https://pkg.fluentci.io/rye@v0.1.0?wasm=1",
        "tools",
        vec!["install", "sqlfluff"],
    )?;
    Ok(stdout)
}

#[plugin_fn]
pub fn lint(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["sqlfluff", "lint", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn fix(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["sqlfluff", "fix", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn format(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["sqlfluff", "format", &args])?
        .stdout()?;
    Ok(stdout)
}
