# MINURL
An on-memory URl shortner with metrics API. Supports 68 billion Unique URls without collision. 

# Design
The program runs independently with a cache which is mapped to each instance. This cache can later be replaced by redis and made into a shared cache so that all instances spawned can share the same state. Program tracks all the shortened URls and keeps track of the hit rate of each redirect, which is done by middleware logic for all requests.

## TODO
- Dockerize the binary
- Add Cassandra support
- Add CRUD apis for Url management
