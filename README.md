edgerouter-exporter
===================

[![][workflow-badge]][workflow-link]

A Prometheus exporter for EdgeRouter BGP, DDNS, Load Balancers, and PPPoE sessions

## Requirements

For development, the following dependencies are required:

- [rustup](https://rustup.rs/)
- [cross](https://github.com/cross-rs/cross)

In order to build the artifact, use `cross build`:

```sh
$ export RUSTFLAGS='-C strip=symbols'
$ cross build --release --target=mipsel-unknown-linux-gnu
```

## Installation

Download the latest version of edgerouter-exporter and move it to the directory
where it persists after the EdgeOS upgrades. Note that the root permission is
required because it internally calls `ubnt_vtysh`.

```console
$ curl -fLO https://github.com/chitoku-k/edgerouter-exporter/releases/latest/download/edgerouter-exporter
$ chmod +x edgerouter-exporter
$ mv edgerouter-exporter /config/scripts
```

```sh
# Port number (required)
export PORT=8080

# TLS certificate and private key (optional; if not specified, exporter is served over HTTP)
export TLS_CERT=/path/to/tls/cert
export TLS_KEY=/path/to/tls/key

# Op command (optional)
export IP_COMMAND=/bin/ip
export OP_COMMAND=/opt/vyatta/bin/vyatta-op-cmd-wrapper
export OP_DDNS_COMMAND=/opt/vyatta/bin/sudo-users/vyatta-op-dynamic-dns.pl
export VTYSH_COMMAND=/opt/vyatta/sbin/ubnt_vtysh
```

## Usage

To simply run edgerouter-exporter:

```sh
$ sudo /config/scripts/edgerouter-exporter
```

To daemonize edgerouter-exporter:

```sh
$ cat <<EOF | sudo tee /etc/systemd/system/prometheus-edgerouter-exporter.service
[Unit]
Description=Prometheus exporter for EdgeRouter metrics
Wants=network-online.target
After=network-online.target

[Service]
Environment=PORT=8080
ExecStart=/config/scripts/edgerouter-exporter

[Install]
WantedBy=multi-user.target
EOF

$ sudo systemctl daemon-reload
$ sudo systemctl enable --now prometheus-edgerouter-exporter.service
```

## Prometheus Metrics

### Version

```
# HELP edgerouter_info Version info
# TYPE edgerouter_info gauge
edgerouter_info{version="v2.0.6",build_id="5208541",model="EdgeRouter X 5-Port"} 1
```

### BGP

```
# HELP edgerouter_bgp_message_in_queue Number of BGP messages in incoming queue
# TYPE edgerouter_bgp_message_in_queue gauge
edgerouter_bgp_message_in_queue{as="64497",neighbor="192.0.2.2",table_version="128"} 0
edgerouter_bgp_message_in_queue{as="64497",neighbor="192.0.2.3",table_version="128"} 0
edgerouter_bgp_message_in_queue{as="64497",neighbor="2001:db8::2",table_version="128"} 0
edgerouter_bgp_message_in_queue{as="64497",neighbor="2001:db8::3",table_version="128"} 0
# HELP edgerouter_bgp_message_out_queue Number of BGP messages in outgoing queue
# TYPE edgerouter_bgp_message_out_queue gauge
edgerouter_bgp_message_out_queue{as="64497",neighbor="192.0.2.2",table_version="128"} 0
edgerouter_bgp_message_out_queue{as="64497",neighbor="192.0.2.3",table_version="128"} 0
edgerouter_bgp_message_out_queue{as="64497",neighbor="2001:db8::2",table_version="128"} 0
edgerouter_bgp_message_out_queue{as="64497",neighbor="2001:db8::3",table_version="128"} 0
# HELP edgerouter_bgp_message_received_total Total number of BGP messages received
# TYPE edgerouter_bgp_message_received_total gauge
edgerouter_bgp_message_received_total{as="64497",neighbor="192.0.2.2",table_version="128"} 1000
edgerouter_bgp_message_received_total{as="64497",neighbor="192.0.2.3",table_version="128"} 2000
edgerouter_bgp_message_received_total{as="64497",neighbor="2001:db8::2",table_version="128"} 3000
edgerouter_bgp_message_received_total{as="64497",neighbor="2001:db8::3",table_version="128"} 4000
# HELP edgerouter_bgp_message_sent_total Total number of BGP messages sent
# TYPE edgerouter_bgp_message_sent_total gauge
edgerouter_bgp_message_sent_total{as="64497",neighbor="192.0.2.2",table_version="128"} 5000
edgerouter_bgp_message_sent_total{as="64497",neighbor="192.0.2.3",table_version="128"} 6000
edgerouter_bgp_message_sent_total{as="64497",neighbor="2001:db8::2",table_version="128"} 7000
edgerouter_bgp_message_sent_total{as="64497",neighbor="2001:db8::3",table_version="128"} 8000
# HELP edgerouter_bgp_prefix_received_total Total number of BGP prefixes received
# TYPE edgerouter_bgp_prefix_received_total gauge
edgerouter_bgp_prefix_received_total{as="64497",neighbor="192.0.2.2",table_version="128"} 9
edgerouter_bgp_prefix_received_total{as="64497",neighbor="192.0.2.3",table_version="128"} 10
edgerouter_bgp_prefix_received_total{as="64497",neighbor="2001:db8::2",table_version="128"} 11
edgerouter_bgp_prefix_received_total{as="64497",neighbor="2001:db8::3",table_version="128"} 12
# HELP edgerouter_bgp_session_seconds_total Total seconds for established BGP session
# TYPE edgerouter_bgp_session_seconds_total gauge
edgerouter_bgp_session_seconds_total{as="64497",neighbor="192.0.2.2",table_version="128"} 100
edgerouter_bgp_session_seconds_total{as="64497",neighbor="192.0.2.3",table_version="128"} 200
edgerouter_bgp_session_seconds_total{as="64497",neighbor="2001:db8::2",table_version="128"} 300
edgerouter_bgp_session_seconds_total{as="64497",neighbor="2001:db8::3",table_version="128"} 400
```

### Dynamic DNS

```
# HELP edgerouter_dynamic_dns_status Result of DDNS update
# TYPE edgerouter_dynamic_dns_status gauge
edgerouter_dynamic_dns_status{hostname="1.example.com",interface_name="eth0",ip_address="192.0.2.1"} 1
edgerouter_dynamic_dns_status{hostname="2.example.com",interface_name="eth1",ip_address="192.0.2.2"} 0
```

### Load Balancers

```
# HELP edgerouter_load_balancer_health Result of watchdog
# TYPE edgerouter_load_balancer_health gauge
edgerouter_load_balancer_health{group_name="WAN_FAILOVER",interface_name="eth0"} 1
edgerouter_load_balancer_health{group_name="WAN_FAILOVER",interface_name="eth1"} 0
# HELP edgerouter_load_balancer_ping_fail_total Total number of ping failures
# TYPE edgerouter_load_balancer_ping_fail_total gauge
edgerouter_load_balancer_ping_fail_total{group_name="WAN_FAILOVER",interface_name="eth0"} 125
edgerouter_load_balancer_ping_fail_total{group_name="WAN_FAILOVER",interface_name="eth1"} 2
# HELP edgerouter_load_balancer_ping_health Result of ping
# TYPE edgerouter_load_balancer_ping_health gauge
edgerouter_load_balancer_ping_health{gateway="8.8.8.8",group_name="WAN_FAILOVER",interface_name="eth0"} 1
edgerouter_load_balancer_ping_health{gateway="8.8.8.8",group_name="WAN_FAILOVER",interface_name="eth1"} 0
# HELP edgerouter_load_balancer_ping_total Total number of pings
# TYPE edgerouter_load_balancer_ping_total gauge
edgerouter_load_balancer_ping_total{group_name="WAN_FAILOVER",interface_name="eth0"} 1000
edgerouter_load_balancer_ping_total{group_name="WAN_FAILOVER",interface_name="eth1"} 1000
# HELP edgerouter_load_balancer_route_drop_total Total number of route drops
# TYPE edgerouter_load_balancer_route_drop_total gauge
edgerouter_load_balancer_route_drop_total{group_name="WAN_FAILOVER",interface_name="eth0"} 5
edgerouter_load_balancer_route_drop_total{group_name="WAN_FAILOVER",interface_name="eth1"} 0
# HELP edgerouter_load_balancer_run_fail_total Total number of run failures
# TYPE edgerouter_load_balancer_run_fail_total gauge
edgerouter_load_balancer_run_fail_total{group_name="WAN_FAILOVER",interface_name="eth0"} 0
edgerouter_load_balancer_run_fail_total{group_name="WAN_FAILOVER",interface_name="eth1"} 0
```

### PPPoE Client Sessions

```
# HELP edgerouter_pppoe_client_session_receive_bytes_total Total receive bytes for PPPoE client session
# TYPE edgerouter_pppoe_client_session_receive_bytes_total gauge
edgerouter_pppoe_client_session_receive_bytes_total{interface_name="pppoe0",ip_address="192.0.2.255",local_ip_address="203.0.113.1",protocol="PPPoE",user="user01"} 79360
# HELP edgerouter_pppoe_client_session_receive_packets_total Total receive packets for PPPoE client session
# TYPE edgerouter_pppoe_client_session_receive_packets_total gauge
edgerouter_pppoe_client_session_receive_packets_total{interface_name="pppoe0",ip_address="192.0.2.255",local_ip_address="203.0.113.1",protocol="PPPoE",user="user01"} 1638
# HELP edgerouter_pppoe_client_session_seconds_total Total seconds for PPPoE client session
# TYPE edgerouter_pppoe_client_session_seconds_total gauge
edgerouter_pppoe_client_session_seconds_total{interface_name="pppoe0",ip_address="192.0.2.255",local_ip_address="203.0.113.1",protocol="PPPoE",user="user01"} 18975
# HELP edgerouter_pppoe_client_session_transmit_bytes_total Total transmit bytes for PPPoE client session
# TYPE edgerouter_pppoe_client_session_transmit_bytes_total gauge
edgerouter_pppoe_client_session_transmit_bytes_total{interface_name="pppoe0",ip_address="192.0.2.255",local_ip_address="203.0.113.1",protocol="PPPoE",user="user01"} 39116
# HELP edgerouter_pppoe_client_session_transmit_packets_total Total transmit packets for PPPoE client session
# TYPE edgerouter_pppoe_client_session_transmit_packets_total gauge
edgerouter_pppoe_client_session_transmit_packets_total{interface_name="pppoe0",ip_address="192.0.2.255",local_ip_address="203.0.113.1",protocol="PPPoE",user="user01"} 412
```

### Spec

| Status | Condition                           |
|--------|-------------------------------------|
| 200    | Success.                            |
| 500    | Unexpected error calling a command. |

[workflow-link]:    https://github.com/chitoku-k/edgerouter-exporter/actions?query=branch:master
[workflow-badge]:   https://img.shields.io/github/workflow/status/chitoku-k/edgerouter-exporter/CI%20Workflow/master.svg?style=flat-square
