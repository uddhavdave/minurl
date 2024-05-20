# MINURL
An in-memory URL shortner with metrics API. Supports 68 billion Unique URls without collision.

# Design
Designed with Scalability and Observability in mind, this application uses Cassandra as its Data Store and has caching enabled by default
to provide blazingly fast redirects to original URL, and has Prometheus and Grafana integration to monitor the granular usage statistics.

## ROADMAP
- [ ] Add Session management
- [ ] Improve cache by using concurrent HashMaps
- [ ] Provide dashboard template for Grafana
