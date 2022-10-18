# iptables-rs

`iptables-rs` is a library in Rust to abstract direct interacting with `iptables`. The tool provides support for `iptable` and `ip6table`. Currently, it provides basic support for `iptables` data types that are translated into Rust.

## Project Structure

The overall structure of the project is as follows:

```bash
❯ tree -d -L 1
.
├── iptables-rs
├── iptables-rs-lib
└── target

3 directories
```

### `iptables-rs-lib`

This is the project where the library is stored and it contains the core code of the `iptables` code that can be utitlized by the client. This project also contains the unit tests for the code.


### `iptables-rs`

This project is the Rust client that uses the `iptables-rs-lib` project which can be considered as an example for using the library.

## Usage

Due to the reliance on `iptables`, this tool only supports Linux and based on that the project can be cloned and used to run and test the library.

The provided `makefile` simplify the usage of the tool with the different tasks.

### Build
```
❯ make build
```

### Run
```
❯ make run
```

### Build and Run
```
❯ make build_and_run
```

### Test

```
❯ make test
running 2 tests
test tests::delete_chain_test ... ok
test tests::is_builtin_chain_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests iptables-rs-lib

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```