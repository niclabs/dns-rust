# Rust implementation of DNS library

This project aims to implement a DNS library in rustlang based on, and only on, DNS-related RFCs. 
A DNS Client and a DNS resolver can be built using this library.

Implementation in progress.

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

| Argument | Description                                                      |
|----------|------------------------------------------------------------------|
|   `client`   | Execute a client that connects to the server and sends requests. |
|   `resolver`   | Runs a DNS resolver                                              |

#### Client

 - For the client there is three arguments:

   | Argument        | Description               |
   |-----------------|---------------------------|
   | `<SERVER>`      | DNS server ip             |
   | `<DOMAIN_NAME>` | Host name to query for IP |
   | `[OPTIONS]`     | EDNS0 options             |

- Three options:

   | Option              | Description                             |
   |---------------------|-----------------------------------------| 
   | `--qtype <QTYPE>`   | Query type [default: A]                 |
   | `--qclass <QCLASS>` | Query class [default: IN]               |
   | `--noedns`          | Disables the use of EDNS when specified |

- And four EDNS0 options

   | EDNS0 option | Description             |
   |--------------|-------------------------|
   | +nsid        | NSID option code        |
   | +padding     | PADDING option code     |
   | +ede         | EDE option code         |
   | +zoneversion | ZONEVERSION option code |
   

#### Resolver

- For the resolver there are two arguments:

   | Argument          | Description|
   |-------------------|------------| 
   | `<DOMAIN_NAME>`   | Host name to query |
   | `[NAMESERVER]...` | Recursive servers |

- And three options:

   | Option                  | Description|
   |-------------------------|------------| 
   | `--qtype <QTYPE>`       | Query type [default: A] |
   | `--qclass <QCLASS>`     |Query class [default: IN]|
   | `--protocol <PROTOCOL>` | Protocol [default: UDP] |

Additionally, the *client* and *resolver* have the command `-h` or `--help` to print the description of the structure and its usage.

## Examples

### 1. Client examples

#### 1.1 These commands runs a query for `example.com` using a client.

```sh
dns_rust client "1.1.1.1" "example.com"
```

or

```sh
cargo run client "1.1.1.1" "example.com"
```

#### 1.2 This command runs a query for `example.com` using a client with NSID EDNS option.

```sh
dns_rust client "1.1.1.1" "example.com" "+nsid"
```

#### 1.2 This command runs a query for `example.com` using a client multiple EDNS options.

```sh
dns_rust client "74.82.42.42" "example.com" "+nsid" "+padding"
```

#### 1.3 This command runs a query for `example.com` using a client with EDNS disabled.

```sh
dns_rust client --noedns "1.1.1.1" "example.com" 
```

#### 1.4 This command runs a query for `example.com` using a client with qtype = MX.

```sh
dns_rust client --qtype "MX" "1.1.1.1" "example.com" 
```

#### 1.5 This command runs a query for `example.com` using a client with qtype = MX and qclass = CH.

```sh
dns_rust client --qtype "MX" --qclass "CH" "1.1.1.1" "example.com" 
```

### 2. Resolver examples

#### 2.1 These commands runs a query for `example.com` running a resolver using the specified servers.

```sh
dns_rust resolver "example.com" "1.1.1.1" "8.8.8.8" 
```
or

```sh
cargo run  resolver "example.com" "1.1.1.1" "8.8.8.8"
```

#### 2.2 This command runs a query for `example.com` running the system's default resolver.

```sh
dns_rust  resolver "example.com"
```

#### 2.3 This command runs a query for `example.com` using TCP..

```sh
dns_rust resolver --protocol "TCP" "example.com" "1.1.1.1" "8.8.8.8"
```

#### 2.4 This command runs a query for `example.com` with qtype = MX using TCP.

```sh
dns_rust resolver --protocol "TCP" --qtype "MX" "example.com" 
```

## Supported RFCs

* 1034 - Domain names, concepts and facilities.
* 1035 - Domain names, implementation and specification.
* 1123 - Requirements for Internet Hosts -- Application and Support.
* 2181 - Clarifications to the DNS Specification.
* Negative Caching
   * 2308 - Negative Caching of DNS Queries (DNS NCACHE)
   * 9520 - Negative Caching of DNS Resolution Failures
* 3596 - DNS Extensions to Support IP Version 6
* 3597 - Handling of Unknown DNS Resource Record (RR) Types
* Edns0
   * 6891 - Extension Mechanisms for DNS (EDNS(0))
   * 5001 - DNS Name Server Identifier (NSID) Option
   * 7830 - The EDNS(0) Padding Option
   * 8914 - Extended DNS Errors
   * 9660 - The DNS Zone Version (ZONEVERSION) Option
* Tsig
   * 8945 - Secret Key Transaction Authentication for DNS (TSIG)

### In progress:

* DNSSEC is not supported at the moment, but it will be eventually.

## Contact

Javiera Alegria.
- github user @Javi801
- email javi@niclabs.cl
