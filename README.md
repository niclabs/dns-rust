# Rust implementation of DNS library

(In progress) : implementation of DNS library in rustlang. 
DNSSEC will be supported ... eventually 


## Supported RFCs 

* 1034 - Domain names, concepts and facilities. 
* 1035 - Domain names, implementation and specification. 

## GitHub Actions

GitHub Actions has been deployed to the repository. In the "main", "testing" and "bugfixes" branches. They are triggered when any of the aforementioned branches is pushed.

When a change is made to the project and it is pushed, "cargo build" and "cargo run" are executed, causing it to be notified if all the tests were done.
