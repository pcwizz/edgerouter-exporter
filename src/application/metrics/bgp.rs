use prometheus_client::{
    encoding::text::Encode,
    metrics::family::Family,
    registry::Registry,
};

use crate::{
    application::metrics::{Collector, Gauge},
    domain::bgp::{BGPIterator, BGPNeighbor},
    service::bgp::BGPStatusResult,
};

#[derive(Clone, Encode, Eq, Hash, PartialEq)]
pub struct BGPNeighborLabel {
    neighbor: String,
    r#as: String,
    table_version: String,
}

impl From<BGPNeighbor> for BGPNeighborLabel {
    fn from(n: BGPNeighbor) -> Self {
        let neighbor = n.neighbor.to_string();
        let r#as = n.remote_as.to_string();
        let table_version = n.table_version.to_string();
        Self {
            neighbor,
            r#as,
            table_version,
        }
    }
}

impl Collector for (BGPStatusResult, BGPStatusResult) {
    fn collect(self, registry: &mut Registry) {
        let bgp_msg_rcv = Family::<BGPNeighborLabel, Gauge>::default();
        registry.register(
            "edgerouter_bgp_message_received_total",
            "Total number of BGP messages received",
            Box::new(bgp_msg_rcv.clone()),
        );

        let bgp_msg_sen = Family::<BGPNeighborLabel, Gauge>::default();
        registry.register(
            "edgerouter_bgp_message_sent_total",
            "Total number of BGP messages sent",
            Box::new(bgp_msg_sen.clone()),
        );

        let bgp_in_q = Family::<BGPNeighborLabel, Gauge>::default();
        registry.register(
            "edgerouter_bgp_message_in_queue",
            "Number of BGP messages in incoming queue",
            Box::new(bgp_in_q.clone()),
        );

        let bgp_out_q = Family::<BGPNeighborLabel, Gauge>::default();
        registry.register(
            "edgerouter_bgp_message_out_queue",
            "Number of BGP messages in outgoing queue",
            Box::new(bgp_out_q.clone()),
        );

        let bgp_session_seconds_total = Family::<BGPNeighborLabel, Gauge>::default();
        registry.register(
            "edgerouter_bgp_session_seconds_total",
            "Total seconds for established BGP session",
            Box::new(bgp_session_seconds_total.clone()),
        );

        let bgp_pfx_rcd = Family::<BGPNeighborLabel, Gauge>::default();
        registry.register(
            "edgerouter_bgp_prefix_received_total",
            "Total number of BGP prefixes received",
            Box::new(bgp_pfx_rcd.clone()),
        );

        for neighbor in BGPIterator::from(self) {
            let (
                messages_received,
                messages_sent,
                in_queue,
                out_queue,
                uptime,
                prefixes_received,
            ) = (
                neighbor.messages_received,
                neighbor.messages_sent,
                neighbor.in_queue,
                neighbor.out_queue,
                neighbor.uptime.map(|d| d.as_secs()).unwrap_or_default(),
                neighbor.prefixes_received.unwrap_or_default(),
            );
            let labels = neighbor.into();

            bgp_msg_rcv
                .get_or_create(&labels)
                .set(messages_received);

            bgp_msg_sen
                .get_or_create(&labels)
                .set(messages_sent);

            bgp_in_q
                .get_or_create(&labels)
                .set(in_queue);

            bgp_out_q
                .get_or_create(&labels)
                .set(out_queue);

            bgp_session_seconds_total
                .get_or_create(&labels)
                .set(uptime);

            bgp_pfx_rcd
                .get_or_create(&labels)
                .set(prefixes_received);
        }
    }
}
