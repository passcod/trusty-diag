Running:

```
cargo run 8.8.8.8:53 google.com.
```

Says:

```
<> Starting tokio runtime
<> Parsing server address
<> Connecting to server
<> Parsing name to query
<> Querying with raw client
DEBUG	[trust_dns_proto::xfer::dns_handle] querying: google.com. A
DEBUG	[trust_dns_proto::xfer] enqueueing message: [Query { name: Name { is_fqdn: true, labels: [google, com] }, query_type: A, query_class: IN }]
DEBUG	[trust_dns_proto::udp::udp_stream] created socket successfully
DEBUG	[trust_dns_proto::rr::record_data] reading A
DEBUG	[trust_dns_proto::udp::udp_client_stream] received message id: 20819
<> Got 1 records
A(216.58.196.142)
<> Configuring resolver
 > Nameserver: 8.8.8.8:53
 > Cache size: 32
 > EDNS (larger records): false
 > DNSSEC Validation: true
 > Use hosts file: false
 > Concurrent requests: false
 > IPv6: false
<> Starting resolver
DEBUG	[trust_dns_resolver::async_resolver] trust-dns resolver running
<> Resolving IP record
DEBUG	[trust_dns_proto::xfer::dns_handle] querying: google.com. A
DEBUG	[trust_dns_resolver::name_server::name_server_pool] sending request: [Query { name: Name { is_fqdn: true, labels: [google, com] }, query_type: A, query_class: IN }]
DEBUG	[trust_dns_resolver::name_server::name_server] reconnecting: NameServerConfig { socket_addr: V4(8.8.8.8:53), protocol: Udp, tls_dns_name: None }
DEBUG	[trust_dns_proto::xfer] enqueueing message: [Query { name: Name { is_fqdn: true, labels: [google, com] }, query_type: A, query_class: IN }]
DEBUG	[trust_dns_proto::udp::udp_stream] created socket successfully
DEBUG	[trust_dns_proto::rr::record_data] reading A
DEBUG	[trust_dns_proto::udp::udp_client_stream] received message id: 34939
DEBUG	[trust_dns_resolver::name_server::name_server_pool] mDNS responsed for query: NoError
DEBUG	[trust_dns_proto::xfer::dnssec_dns_handle] validating message_response: 34939
DEBUG	[trust_dns_proto::xfer::dnssec_dns_handle] verifying: google.com., record_type: A, rrsigs: 0
DEBUG	[trust_dns_proto::xfer::dnssec_dns_handle] default validation google.com., record_type: A
DEBUG	[trust_dns_proto::xfer::dnssec_dns_handle] an rrset failed to verify: ProtoError { kind: RrsigsNotPresent { name: Name { is_fqdn: true, labels: [google, com] }, record_type: A }, backtrack: None }
Error: ResolveError { kind: Proto(ProtoError { kind: RrsigsNotPresent { name: Name { is_fqdn: true, labels: [google, com] }, record_type: A }, backtrack: None }), backtrack: None }
```
