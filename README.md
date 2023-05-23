# Rust implementation of DNS library

This project aims to implement a DNS library in rustlang based on, and only on, DNS-related RFCs. 
With this library it can be build a DNS Client, and a DNS resolver.

Implementation in progress.

## Supported RFCs 

* 1034 - Domain names, concepts and facilities. 
* 1035 - Domain names, implementation and specification. 

DNSSEC is not supported at the moment, but it will be eventually.

## Getting started

### Prerequisites

As this library is build it in rustlang, is mandatory to have [**Rust**](https://www.rust-lang.org/learn/get-started) installed.

### Installation


1. Clone the repository.

```
git clone https://github.com/niclabs/dns-rust.git
```

1. Set resolver configuration in `.../src/config.rs`. 

   The resolver configuration is as follows:

   - Set the IP and PORT that will host and use the resolver.

   ```Rust
   pub static RESOLVER_IP_PORT: &'static str = "RESOLVER_IP:RESOLVER_PORT";
   ```

   - (Optional) Update ```SBELT_ROOT_IPS``` variable with the addresses of the root server and the host server in the SBelt.
   - (Optional) Update ```QUERIES_FOR_CLIENT_REQUEST``` variable with the number of queries before the resolver panic in a Temporary Error.

2. Set client configuration in `.../src/client/config.rs`.

   - Set the IP and PORT that host the resolver.
   - Set the IP and PORT where the client will run.

   ```Rust
   pub static RESOLVER_IP_PORT: &'static str = "RESOLVER_IP:RESOLVER_PORT";
   pub static CLIENT_IP_PORT: &'static str = "CLIENT_IP:CLIENT_PORT";
   ```

## Usage

The library is built and run through `cargo` with the run the command `cargo run` followed by any necessary option. 

```sh 
cargo run -- [options]
```

Here you can specify whether to run: *client* or a *server*. Note that if you want to run a resolver and a client in the same machine it is necessary to run two instances of the library.

### Options:

| Argument | Description |
|----------|-------------|
|   `-c`   | Exceute a client that connects to the server and sends requests. |
|   `-r`   | Runs a DNS resolver |

### Example:
For example, to execute a new client:

```sh
cargo run -- -c
```
## Development features

### GitHub Actions

To improve code quality GitHub Actions has been implemented in the repository, in the "main", "testing" and "bugfixes" branches. GitHub Actions has been configured to trigger when a commit is pushed to the project repository, running "cargo build" and "cargo run", and then all tests, finally showing an "x" if any test fails or a "✓" if all tests pass.

### Docker

Coming soon.

## Contact

Javiera Alegria.
- github user @Javi801
- email javi@niclabs.cl
