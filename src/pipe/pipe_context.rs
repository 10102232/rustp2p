use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Context;
use crossbeam_utils::atomic::AtomicCell;
use dashmap::DashMap;
use parking_lot::RwLock;

use rust_p2p_core::punch::PunchConsultInfo;
use rust_p2p_core::route::Index;
use rust_p2p_core::socket::LocalInterface;

use crate::config::punch_info::NodePunchInfo;
use crate::error::Error;
use crate::extend::dns_query::{dns_query_all, dns_query_txt};
use crate::protocol::node_id::NodeID;

#[derive(Clone)]
pub struct PipeContext {
    self_node_id: Arc<AtomicCell<Option<NodeID>>>,
    direct_node_address_list: Arc<RwLock<Vec<(PeerNodeAddress, Vec<NodeAddress>)>>>,
    reachable_nodes: Arc<DashMap<NodeID, Vec<(NodeID, u8, Instant)>>>,
    punch_info: Arc<RwLock<NodePunchInfo>>,
    default_interface: Option<LocalInterface>,
    dns: Vec<String>,
}

impl PipeContext {
    pub(crate) fn new(
        local_udp_ports: Vec<u16>,
        local_tcp_port: u16,
        default_interface: Option<crate::config::LocalInterface>,
        dns: Option<Vec<String>>,
    ) -> Self {
        let punch_info = NodePunchInfo::new(local_udp_ports, local_tcp_port);
        Self {
            self_node_id: Arc::new(Default::default()),
            direct_node_address_list: Arc::new(Default::default()),
            reachable_nodes: Arc::new(Default::default()),
            punch_info: Arc::new(RwLock::new(punch_info)),
            default_interface: default_interface.map(|v| v.into()),
            dns: dns.unwrap_or(vec![]),
        }
    }
    pub fn store_self_id(&self, node_id: NodeID) -> crate::error::Result<()> {
        if node_id.is_unspecified() || node_id.is_broadcast() {
            return Err(Error::InvalidArgument("invalid node id".into()));
        }
        self.self_node_id.store(Some(node_id));
        Ok(())
    }
    pub fn load_id(&self) -> Option<NodeID> {
        self.self_node_id.load()
    }
    pub fn set_direct_nodes(&self, direct_node: Vec<(PeerNodeAddress, Vec<NodeAddress>)>) {
        *self.direct_node_address_list.write() = direct_node;
    }
    pub fn get_direct_nodes(&self) -> Vec<NodeAddress> {
        let guard = self.direct_node_address_list.read();
        let mut addrs = Vec::new();
        for (_, v) in guard.iter() {
            for x in v {
                addrs.push(x.clone())
            }
        }
        addrs
    }
    pub async fn update_direct_nodes(&self) -> crate::error::Result<()> {
        let mut addrs = self.direct_node_address_list.read().clone();
        for (peer_addr, addr) in &mut addrs {
            *addr = peer_addr
                .to_addr(&self.dns, &self.default_interface)
                .await?;
        }
        self.set_direct_nodes(addrs);
        Ok(())
    }
    pub fn update_reachable_nodes(&self, src_id: NodeID, reachable_id: NodeID, metric: u8) {
        let now = Instant::now();
        self.reachable_nodes
            .entry(reachable_id)
            .and_modify(|v| {
                for (node, m, time) in &mut *v {
                    if node == &src_id {
                        *m = metric;
                        *time = now;
                        v.sort_by_key(|(_, m, _)| *m);
                        return;
                    }
                }
                v.push((src_id, metric, now));
                v.sort_by_key(|(_, m, _)| *m);
            })
            .or_insert_with(|| vec![(src_id, metric, now)]);
    }
    pub(crate) fn clear_timeout_reachable_nodes(&self) {
        let timeout = if let Some(time) = Instant::now().checked_sub(Duration::from_secs(60)) {
            time
        } else {
            return;
        };
        for mut val in self.reachable_nodes.iter_mut() {
            val.value_mut().retain(|(_, _, time)| time > &timeout);
        }
        self.reachable_nodes.retain(|_, v| !v.is_empty());
    }
    pub fn reachable_node(&self, dest_id: &NodeID) -> Option<NodeID> {
        if let Some(v) = self.reachable_nodes.get(dest_id) {
            v.value().get(0).map(|(v, _, _)| *v)
        } else {
            None
        }
    }
    pub fn default_route(&self) -> Option<NodeAddress> {
        let guard = self.direct_node_address_list.read();
        if let Some((_, v)) = guard.get(0) {
            v.get(0).cloned()
        } else {
            None
        }
    }
    pub(crate) fn exists_nat_info(&self) -> bool {
        self.punch_info.read().exists_nat_info()
    }
    pub fn punch_info(&self) -> &Arc<RwLock<NodePunchInfo>> {
        &self.punch_info
    }
    pub(crate) fn gen_punch_info(&self, seq: u32) -> PunchConsultInfo {
        self.punch_info.read().punch_consult_info(seq)
    }
    pub fn set_mapping_addrs(&self, mapping_addrs: Vec<NodeAddress>) {
        let tcp_addr: Vec<SocketAddr> = mapping_addrs
            .iter()
            .filter(|v| v.is_tcp())
            .map(|v| *v.addr())
            .collect();
        let udp_addr: Vec<SocketAddr> = mapping_addrs
            .iter()
            .filter(|v| !v.is_tcp())
            .map(|v| *v.addr())
            .collect();
        let mut guard = self.punch_info.write();
        guard.mapping_tcp_addr = tcp_addr;
        guard.mapping_udp_addr = udp_addr;
    }
    pub fn update_public_addr(&self, index: Index, addr: SocketAddr) {
        self.punch_info.write().update_public_addr(index, addr);
    }
    pub fn update_tcp_public_addr(&self, addr: SocketAddr) {
        self.punch_info.write().update_tcp_public_port(addr);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum NodeAddress {
    Tcp(SocketAddr),
    Udp(SocketAddr),
}

impl NodeAddress {
    pub fn is_tcp(&self) -> bool {
        match self {
            NodeAddress::Tcp(_) => true,
            NodeAddress::Udp(_) => false,
        }
    }
    pub fn addr(&self) -> &SocketAddr {
        match self {
            NodeAddress::Tcp(addr) => addr,
            NodeAddress::Udp(addr) => addr,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PeerNodeAddress {
    Tcp(SocketAddr),
    Udp(SocketAddr),
    TcpDomain(String),
    UdpDomain(String),
    TxtDomain(String),
}
impl PeerNodeAddress {
    pub async fn to_addr(
        &self,
        name_servers: &Vec<String>,
        default_interface: &Option<LocalInterface>,
    ) -> crate::error::Result<Vec<NodeAddress>> {
        let addrs = match self {
            PeerNodeAddress::Tcp(addr) => vec![NodeAddress::Tcp(*addr)],
            PeerNodeAddress::Udp(addr) => vec![NodeAddress::Udp(*addr)],
            PeerNodeAddress::TcpDomain(domain) => {
                let addrs = dns_query_all(domain, &name_servers, default_interface).await?;
                addrs.into_iter().map(|v| NodeAddress::Tcp(v)).collect()
            }
            PeerNodeAddress::UdpDomain(domain) => {
                let addrs = dns_query_all(domain, &name_servers, default_interface).await?;
                addrs.into_iter().map(|v| NodeAddress::Udp(v)).collect()
            }
            PeerNodeAddress::TxtDomain(domain) => {
                let txt = dns_query_txt(domain, name_servers.clone(), default_interface).await?;
                let mut addrs = Vec::with_capacity(txt.len());
                for x in txt {
                    let x = x.to_lowercase();
                    let addr = if let Some(v) = x.strip_prefix("udp://") {
                        NodeAddress::Udp(
                            SocketAddr::from_str(v).context("record type txt is not SocketAddr")?,
                        )
                    } else if let Some(v) = x.strip_prefix("tcp://") {
                        NodeAddress::Tcp(
                            SocketAddr::from_str(v).context("record type txt is not SocketAddr")?,
                        )
                    } else {
                        NodeAddress::Tcp(
                            SocketAddr::from_str(&x)
                                .context("record type txt is not SocketAddr")?,
                        )
                    };
                    addrs.push(addr);
                }
                addrs
            }
        };
        Ok(addrs)
    }
}
impl FromStr for PeerNodeAddress {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let domain = s.to_lowercase();
        let addr = if let Some(v) = domain.strip_prefix("tcp://") {
            match SocketAddr::from_str(v) {
                Ok(addr) => PeerNodeAddress::Tcp(addr),
                Err(_) => PeerNodeAddress::TcpDomain(v.to_string()),
            }
        } else if let Some(v) = domain.strip_prefix("udp://") {
            match SocketAddr::from_str(v) {
                Ok(addr) => PeerNodeAddress::Udp(addr),
                Err(_) => PeerNodeAddress::UdpDomain(v.to_string()),
            }
        } else if let Some(v) = domain.strip_prefix("txt://") {
            PeerNodeAddress::TxtDomain(v.to_string())
        } else {
            match SocketAddr::from_str(&domain) {
                Ok(addr) => PeerNodeAddress::Udp(addr),
                Err(_) => PeerNodeAddress::UdpDomain(domain),
            }
        };
        Ok(addr)
    }
}
