# Demo for ag_dubs

## Works

```sh
cd firmware/firmware-demo
cargo build --release --bin hello
```

## Doesn't work

(this fails on macos, only coincedentally works on linux iirc)

```sh
# top level
cargo build --release -p firmware-demo --bin hello
# (wrong target error)

# top level
cargo build --workspace
# (wrong target error)
```

---

## Works

```sh
cd firmware/firmware-demo
cargo build --release --bin hello
cd ../../host/host-demo
cargo build --release
```

## Doesn't work

```sh
# top level
cargo build --release -p firmware-demo --bin hello
# (wrong target error)

# top level
cargo build --workspace
# (wrong target error)
```
