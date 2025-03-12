# fuel-zkvm-primitives-test-fixtures

Here we attempt to establish testing coverage over all opcodes and some example contracts written in sway with zkvm targets.

## Test Block execution game fixtures

```shell
cargo test --lib block_execution_fixtures
```

## Refreshing serialized proof inputs

If you update a dependency and don't seem to be detecting changes in behaviour in the tests, you have to refresh the build.

```shell
export REFRESH_BUILD=true
# now any command ran will trigger the rebuild if necessary,
cargo test --lib block_execution_fixtures
```
