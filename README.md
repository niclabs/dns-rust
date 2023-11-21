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

### Library installation

First to clone the repository:
```sh
git clone https://github.com/niclabs/dns-rust.git
```

Then to use the library there are three options:

1. Installing the library with `cargo install`. This option let us run the library with the command `dns_rust ...`, Install the library with the following command:
   ```sh
   cargo install --path <PATH>
   ```

   With the library installed it can be run it with `dns_rust` followed by any necessary option.

   ```sh
   dns_rust [options]
   ```

2. Using the library through `cargo` with cargo `cargo run`, accompanied by any neccessary option.
   ```sh
   cargo run [options]
   ```
3. Using the library's code in your new project. \
   Fisrt you need to add the dependency to your Cargo.toml file:

   ```toml
   [dependencies]
   dns_rust = { path = <PATH> }
   ```
   Then to use the Library in your Rust code Import the library at the beginning of your Rust file.
   
   
   ```rust
   use std::net::IpAddr;
   use dns_rust::async_resolver::AsyncResolver;
   use dns_rust::async_resolver::config::ResolverConfig;


   async fn resolver()-> Vec<IpAddr> {
      let config = ResolverConfig::default();
      let domain_name = "example.com";
      let transport_protocol = "TCP";
      let mut resolver = AsyncResolver::new(config);
      let ip_addresses = resolver.lookup_ip(domain_name, transport_protocol).await.unwrap();

      ip_addresses
   }
   ```

### Supported options configurations
Here it can be specified whether to run a *client* or a *resolver* :

| Argument | Description |
|----------|-------------|
|   `client`   | Execute a client that connects to the server and sends requests. |
|   `resolver`   | Runs a DNS resolver |

#### Client

- For the client there is one argument:
   | Argument | Description |
   |----------|-------------|
   |   `<HOST_NAME>`   | Host name to query for IP |

- And three options:
   | Option | Description|
   |--------|------------| 
   |   `--server <SERVER>`   | DNS server ip |
   |   `--qtype <QTYPE>`    | Query type [default: A] |
   |   `--qclass <QCLASS>`   | Query class [default: IN] |

#### Resolver

- For the resolver there are two arguments:
   | Argument | Description|
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

### Examples

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
