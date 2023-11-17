# Rust implementation of DNS library

This project aims to implement a DNS library in rustlang based on, and only on, DNS-related RFCs. 
A DNS Client and a DNS resolver can be built using this library.

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

After clone the repository, there are two options:

1. Installing the library with `cargo install`.

2. Using the library through `cargo`.

#### Cargo install option:
 
This option let us run the library with the command `dns_rust ...`

Install the library with the following command:
```sh
cargo install --path <PATH>
```

### Options of Usage:

With the library installed we can run it with `dns_rust` followed by any necessary option.
```sh
dns_rust [options]
```
Or else, it can be run the library through `cargo` with `cargo run`.

```sh
cargo run [options]
```

#### Supported options configurations:
Here it can be specified whether to run: *client* or *resolver* :

| Argument | Description |
|----------|-------------|
|   `client`   | Execute a client that connects to the server and sends requests. |
|   `resolver`   | Runs a DNS resolver |

##### Client:

- For the client we have one argument:
   | Argument | Description |
   |----------|-------------|
   |   `<HOST_NAME>`   | Host name to query for IP |

- And three options:
   | Option | Description|
   |--------|------------| 
   |   `--server <SERVER>`   | DNS server ip |
   |   `--qtype <QTYPE>`    | Query type [default: A] |
   |   `--qclass <QCLASS>`   | Query class [default: IN] |

##### Resolver

- For the resolver we have two arguments:
   | Option | Description|
   |--------|------------| 
   |   `<HOST NAME>`   | Host name to query |
   |   `[NAMESERVER]...`    | Recursive servers |

- And three options:
   | Option | Description|
   |--------|------------| 
   |   `--bind-addr <BIND_ADDR>`   | Resolver bind address |
   |   `--qtype <QTYPE>`    | Query type [default: A] |
   |   `--protocol <PROTOCOL>`   | Protocol [default: UDP] |

Additionally the *client* and *resolver* have the command `-h` or `--help` to print the description of the structure and its usage.

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
