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

2. Set resolver and name server configuration in `.../src/config.rs`. 

   The resolver configuration is as follows:

   - Set the IP and PORT that will host and use the resolver.

   ```
   pub static RESOLVER_IP_PORT: &'static str = "RESOLVER_IP:RESOLVER_PORT";
   ```

   - (Optional) Update ```SBELT_ROOT_IPS``` variable with the addresses of the root server and the host server in the SBelt.
   - (Optional) Update ```QUERIES_FOR_CLIENT_REQUEST``` variable with the number of queries before the resolver panic in a Temporary Error.

   As for the name server:

   - Set the IP and PORT that will host the name server:

   ```
   pub static NAME_SERVER_IP: &'static str = "NAME_SERVER_IP"
   ```

   - Set the Master file name and path, and the origin value if not specified in the Master file.

   ```
   pub static MASTER_FILES: [(&str,&str );1] = [("MASTER_FILE", "ORIGIN")];
   ```

   - (Optional) Update ``RECURSIVE_AVAILABLE`` variable if recursive name server is not available
   - (Optional) Update `CHECK_MASTER_FILES` variable to not check the Master file validity.


3. Set client configuration in `.../src/client/config.rs`.

   - Set the IP and PORT that host the resolver.
   - Set the IP and PORT where the client will run.

``` 
pub static RESOLVER_IP_PORT: &'static str = "RESOLVER_IP:RESOLVER_PORT";
pub static CLIENT_IP_PORT: &'static str = "CLIENT_IP:CLIENT_PORT";
```

## Usage

The library is build and run through `cargo` with the command:

```
cargo run
```

When run it show the following menu:

```
Rustlang library for DNS
Name server compatible with RFC 1034 and RFC 1035 only.
To only check the validity of a Master file, enter MF.
For other services, enter program to run: 
   [C] Client
   [R] Resolver
   [N] Nameserver
   [NR] Nameserver and Resolver
```

Here you can select whether to run a *client*, a *server*, a *nameserver* or a *nameserver* with a *resolver*. Note that if you want to run a resolver and a client in the same machine it is necessaryto run two instances of the library.

In addition, the library allows you to validate a Master file, for this you need to enter the ``MF`` command as it says in the menu and the file specified in ```config.rs``` file will be check.

## Development features

### GitHub Actions

To improve code quality GitHub Actions has been implemented in the repository, in the "main", "testing" and "bugfixes" branches. GitHub Actions has been configured to trigger when a commit is pushed to the project repository, running "cargo build" and "cargo run", and then all tests, finally showing an "x" if any test fails or a "✓" if all tests pass.

### Docker

Coming soon.

## Contact

Javiera B.
- github user @Javi801
- email javi@niclabs.cl
