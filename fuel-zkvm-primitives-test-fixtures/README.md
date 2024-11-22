# fuel-zkvm-primitives-test-fixtures

Here we attempt to establish testing coverage over all opcodes and some example contracts written in sway with zkvm targets.

## Test Opcodes

```sh
cargo test --lib opcodes::tests
```

## Test Counter Contract

```shell
forc build --path src/fixtures/counter_contract
cargo test --lib counter_contract::tests
```

## Test Mainnet blocks

```shell
cargo test --lib mainnet_blocks::tests
```