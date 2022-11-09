# Rust implementation of DNS library

This project aims to implement a DNS library in rustlang based on, and only on, DNS-related RFCs. 
With this library it can be validate a masterfile and it can be build a DNS Client, DNS Nameserver and a DNS resolver.

Implementation in progress.

## Supported RFCs 

* 1034 - Domain names, concepts and facilities. 
* 1035 - Domain names, implementation and specification. 

DNSSEC is not supported at the moment, but it will be eventually.

## Getting started

### Prerequisites

As this library is build it in rustlang, is mandatory to have [**Rust**](https://www.rust-lang.org/learn/get-started) installed.

### Installation


1. Clone the repo

```
git clone https://github.com/niclabs/dns-rust.git
```

2. Set resolver configuration in `.../src/config.rs`. 

   - Set the IP and PORT that will host and use the resolver.

```
pub static RESOLVER_IP_PORT: &'static str = "RESOLVER_IP:RESOLVER_PORT";
```

3. Set client configuration in `.../src/client/config.rs`.

   - Set the IP and PORT that host the resolver.
   - Set the IP and PORT where the client will run.

``` 
pub static RESOLVER_IP_PORT: &'static str = "RESOLVER_IP:RESOLVER_PORT";
pub static CLIENT_IP_PORT: &'static str = "CLIENT_IP:CLIENT_PORT";
```

## Usage

The library run through `cargo` with the following command:

```
cargo run
```

It will show a menu to select if run a *client*, a *server*, a *nameserver* or a *nameserver* with a *resolver*. If you want to run a resolver and a client in the same machine it is necesarry running two instance of the library.


## Contact

Javiera B.
- github user @Javi801
- email javi@niclabs.cl
