# Rust implementation of DNS library

This project aims to implement a DNS library in rustlang based on, and only on, DNS-related RFCs. 
With this library it can be build a DNS Client, and a DNS resolver.

Implementation in progress.

## Supported RFCs 

* 1034 - Domain names, concepts and facilities. 
* 1035 - Domain names, implementation and specification. 

### In progress:

* 1123 - Requirements for Internet Hosts -- Application and Support

* DNSSEC is not supported at the moment, but it will be eventually.

## Getting started

### Prerequisites

As this library is build it in rustlang, is mandatory to have [**Rust**](https://www.rust-lang.org/learn/get-started) installed.

### Installation


1. Clone the repository.

```
git clone https://github.com/niclabs/dns-rust.git
```

## Options of Usage

We have two options:

1. Installing the library with `cargo install`.

2. Using the library through `cargo`.

### Cargo install option:

This option let us run the library with the command `dns_rust ...`

#### Installing using cargo install:

1. Install the library with the following command:
```
cargo install --path <PATH OF THE REPOSITORY>
```

### Usage

With the library installed we can run it with `dns_rust` followed by any necessary option.
```sh
dns_rust [options]
```

Or else, you can run the library through `cargo` with `cargo run`.

```sh
cargo run [options]
```

#### Options:
Here ot can be specified whether to run: *client* or *resolver* :

| Argument | Description |
|----------|-------------|
|   `client`   | Execute a client that connects to the server and sends requests. |
|   `resolver`   | Runs a DNS resolver |

#### Examples:

```sh
dns_rust resolver "example.com" "1.1.1.1" "8.8.8.8" 
```
or

```sh
cargo run  resolver "example.com" "1.1.1.1" "8.8.8.8"
```

These commands runs a query for `example.com` running a resolver.

## Development features

### GitHub Actions

To improve code quality GitHub Actions has been implemented in the repository, in the "main", "testing" and "bugfixes" branches. GitHub Actions has been configured to trigger when a commit is pushed to the project repository, running "cargo build" and "cargo run", and then all tests, finally showing an "x" if any test fails or a "✓" if all tests pass.

### Docker

Coming soon.

## Contact

Javiera Alegria.
- github user @Javi801
- email javi@niclabs.cl
