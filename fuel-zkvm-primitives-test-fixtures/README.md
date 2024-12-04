# fuel-zkvm-primitives-test-fixtures

Here we attempt to establish testing coverage over all opcodes and some example contracts written in sway with zkvm targets.

## Test Opcodes

```shell
cargo test --lib opcodes::tests
```

## Test Counter Contract

```shell
cargo test --lib counter_contract::tests
```

## Test Mainnet blocks

```shell
cargo test --lib mainnet_blocks::tests
```

## Refreshing serialized proof inputs

If you update a dependency and don't seem to be detecting changes in behaviour in the tests, you have to refresh the build.

```shell
export REFRESH_BUILD=true
# now any command ran will trigger the rebuild if necessary,
cargo test --lib opcodes::tests
```
