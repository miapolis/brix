### Brix
Brix is a CLI tool written in Rust for scaffolding and code generation.

#### Attributions
Special thanks to [Caleb Cushing](https://github.com/xenoterracide) for the original Java version, early interface design and internal architecture.
#### Running
Usage:
```
brix [LANGUAGE] [CONFIG NAME] [PROJECT] [MODULE]
brix [OPTIONS] --config-dir | -d [CONFIG DIRECTORY]
brix [OPTIONS] --workdir | -w [WORKING DIRECTORY]
```

#### Installing locally
##### Requirements
- Cargo and a minimum Rust version of **1.43.1**
##### Running
- Run `cargo build`
- Run `cargo run`

##### Testing
Run `cargo test --all` to test the entire workspace.

##### Examples
There are a few examples located in `./config/brix/rust`.

- **copy** `cargo run -- rust copy brix foo`
- **mkdir** `cargo run -- rust mkdir brix foo`
- **search_replace** `cargo run -- rust search_replace brix foo`
- **template** `cargo run -- rust template brix foo`
