use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use async_trait::async_trait;
use rust_p2p_core::pipe::tcp_pipe::{Decoder, Encoder, InitCodec};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Model {
    High,
    #[default]
    Low,
}
#[derive(Clone, Debug, Default)]
pub struct LocalInterface {
    pub index: u32,
    #[cfg(unix)]
    pub name: Option<String>,
}
impl LocalInterface {
    pub fn new(index: u32, #[cfg(unix)] name: Option<String>) -> Self {
        Self {
            index,
            #[cfg(unix)]
            name,
        }
    }
}
pub(crate) const MAX_SYMMETRIC_PIPELINE_NUM: usize = 200;
pub(crate) const MAX_MAIN_PIPELINE_NUM: usize = 10;
pub(crate) const ROUTE_IDLE_TIME: Duration = Duration::from_secs(10);

pub struct PipeConfig {
    pub first_latency: bool,
    pub multi_pipeline: usize,
    pub route_idle_time: Duration,
    pub udp_pipe_config: Option<UdpPipeConfig>,
    pub tcp_pipe_config: Option<TcpPipeConfig>,
    pub enable_extend: bool,
}

impl Default for PipeConfig {
    fn default() -> Self {
        Self {
            first_latency: false,
            multi_pipeline: MULTI_PIPELINE,
            enable_extend: false,
            udp_pipe_config: Some(Default::default()),
            tcp_pipe_config: Some(Default::default()),
            route_idle_time: ROUTE_IDLE_TIME,
        }
    }
}

pub(crate) const MULTI_PIPELINE: usize = 2;
pub(crate) const UDP_SUB_PIPELINE_NUM: usize = 82;

impl PipeConfig {
    pub fn new() -> PipeConfig {
        let udp_pipe_config = Some(UdpPipeConfig::default());
        let tcp_pipe_config = Some(TcpPipeConfig::default());
        Self {
            first_latency: false,
            multi_pipeline: MULTI_PIPELINE,
            enable_extend: false,
            udp_pipe_config,
            tcp_pipe_config,
            route_idle_time: ROUTE_IDLE_TIME,
        }
    }
}
impl PipeConfig {
    pub fn none_tcp(self) -> Self {
        self
    }
}
impl PipeConfig {
    pub fn empty() -> Self {
        Self {
            first_latency: false,
            multi_pipeline: MULTI_PIPELINE,
            enable_extend: false,
            udp_pipe_config: None,
            tcp_pipe_config: None,
            route_idle_time: ROUTE_IDLE_TIME,
        }
    }
    pub fn set_first_latency(mut self, first_latency: bool) -> Self {
        self.first_latency = first_latency;
        self
    }
    pub fn set_main_pipeline_num(mut self, main_pipeline_num: usize) -> Self {
        self.multi_pipeline = main_pipeline_num;
        self
    }
    pub fn set_enable_extend(mut self, enable_extend: bool) -> Self {
        self.enable_extend = enable_extend;
        self
    }
    pub fn set_udp_pipe_config(mut self, udp_pipe_config: UdpPipeConfig) -> Self {
        self.udp_pipe_config.replace(udp_pipe_config);
        self
    }
    pub fn set_tcp_pipe_config(mut self, tcp_pipe_config: TcpPipeConfig) -> Self {
        self.tcp_pipe_config.replace(tcp_pipe_config);
        self
    }
}

pub struct TcpPipeConfig {
    pub route_idle_time: Duration,
    pub tcp_multiplexing_limit: usize,
    pub default_interface: Option<LocalInterface>,
    pub tcp_port: u16,
    pub use_v6: bool,
}

impl Default for TcpPipeConfig {
    fn default() -> Self {
        Self {
            route_idle_time: ROUTE_IDLE_TIME,
            tcp_multiplexing_limit: MULTI_PIPELINE,
            default_interface: None,
            tcp_port: 0,
            use_v6: true,
        }
    }
}

impl TcpPipeConfig {
    pub fn set_tcp_multiplexing_limit(mut self, tcp_multiplexing_limit: usize) -> Self {
        self.tcp_multiplexing_limit = tcp_multiplexing_limit;
        self
    }
    pub fn set_route_idle_time(mut self, route_idle_time: Duration) -> Self {
        self.route_idle_time = route_idle_time;
        self
    }
    pub fn set_default_interface(mut self, default_interface: LocalInterface) -> Self {
        self.default_interface = Some(default_interface.clone());
        self
    }
    pub fn set_tcp_port(mut self, tcp_port: u16) -> Self {
        self.tcp_port = tcp_port;
        self
    }
    pub fn set_use_v6(mut self, use_v6: bool) -> Self {
        self.use_v6 = use_v6;
        self
    }
}

#[derive(Clone)]
pub struct UdpPipeConfig {
    pub main_pipeline_num: usize,
    pub sub_pipeline_num: usize,
    pub model: Model,
    pub default_interface: Option<LocalInterface>,
    pub udp_ports: Vec<u16>,
    pub use_v6: bool,
}

impl Default for UdpPipeConfig {
    fn default() -> Self {
        Self {
            main_pipeline_num: MULTI_PIPELINE,
            sub_pipeline_num: UDP_SUB_PIPELINE_NUM,
            model: Model::Low,
            default_interface: None,
            udp_ports: vec![0, 0],
            use_v6: true,
        }
    }
}

impl UdpPipeConfig {
    pub fn set_main_pipeline_num(mut self, main_pipeline_num: usize) -> Self {
        self.main_pipeline_num = main_pipeline_num;
        self
    }
    pub fn set_sub_pipeline_num(mut self, sub_pipeline_num: usize) -> Self {
        self.sub_pipeline_num = sub_pipeline_num;
        self
    }
    pub fn set_model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }
    pub fn set_default_interface(mut self, default_interface: LocalInterface) -> Self {
        self.default_interface = Some(default_interface.clone());
        self
    }
    pub fn set_udp_ports(mut self, udp_ports: Vec<u16>) -> Self {
        self.udp_ports = udp_ports;
        self
    }
    pub fn set_simple_udp_port(mut self, udp_port: u16) -> Self {
        self.udp_ports = vec![udp_port];
        self
    }
    pub fn set_use_v6(mut self, use_v6: bool) -> Self {
        self.use_v6 = use_v6;
        self
    }
}
impl From<Model> for rust_p2p_core::pipe::udp_pipe::Model {
    fn from(value: Model) -> Self {
        match value {
            Model::High => rust_p2p_core::pipe::udp_pipe::Model::High,
            Model::Low => rust_p2p_core::pipe::udp_pipe::Model::Low,
        }
    }
}
impl From<LocalInterface> for rust_p2p_core::socket::LocalInterface {
    fn from(value: LocalInterface) -> Self {
        #[cfg(unix)]
        return rust_p2p_core::socket::LocalInterface::new(value.index, value.name);
        #[cfg(not(unix))]
        rust_p2p_core::socket::LocalInterface::new(value.index)
    }
}
impl From<PipeConfig> for rust_p2p_core::pipe::config::PipeConfig {
    fn from(value: PipeConfig) -> Self {
        rust_p2p_core::pipe::config::PipeConfig {
            first_latency: value.first_latency,
            multi_pipeline: value.multi_pipeline,
            route_idle_time: value.route_idle_time,
            udp_pipe_config: value.udp_pipe_config.map(|v| v.into()),
            tcp_pipe_config: value.tcp_pipe_config.map(|v| v.into()),
            enable_extend: value.enable_extend,
        }
    }
}
impl From<UdpPipeConfig> for rust_p2p_core::pipe::config::UdpPipeConfig {
    fn from(value: UdpPipeConfig) -> Self {
        rust_p2p_core::pipe::config::UdpPipeConfig {
            main_pipeline_num: value.main_pipeline_num,
            sub_pipeline_num: value.sub_pipeline_num,
            model: value.model.into(),
            default_interface: value.default_interface.map(|v| v.into()),
            udp_ports: value.udp_ports,
            use_v6: value.use_v6,
        }
    }
}
impl From<TcpPipeConfig> for rust_p2p_core::pipe::config::TcpPipeConfig {
    fn from(value: TcpPipeConfig) -> Self {
        rust_p2p_core::pipe::config::TcpPipeConfig {
            route_idle_time: value.route_idle_time,
            tcp_multiplexing_limit: value.tcp_multiplexing_limit,
            default_interface: value.default_interface.map(|v| v.into()),
            tcp_port: value.tcp_port,
            use_v6: value.use_v6,
            init_codec: Box::new(LengthPrefixedInitCodec),
        }
    }
}

/// Fixed-length prefix encoder/decoder.
pub(crate) struct LengthPrefixedCodec;
#[async_trait]
impl Decoder for LengthPrefixedCodec {
    async fn decode(&mut self, read: &mut OwnedReadHalf, src: &mut [u8]) -> io::Result<usize> {
        let mut head = [0; 2];
        read.read_exact(&mut head).await?;
        let len = u16::from_be_bytes(head) as usize;
        read.read_exact(&mut src[..len]).await?;
        Ok(len)
    }
}

#[async_trait]
impl Encoder for LengthPrefixedCodec {
    async fn encode(&mut self, write: &mut OwnedWriteHalf, data: &[u8]) -> io::Result<usize> {
        if data.len() > u16::MAX as usize {
            return Err(io::Error::from(io::ErrorKind::OutOfMemory));
        }
        let head: [u8; 2] = (data.len() as u16).to_be_bytes();
        write.write_all(&head).await?;
        write.write_all(data).await?;
        Ok(data.len())
    }
}
pub(crate) struct LengthPrefixedInitCodec;
impl InitCodec for LengthPrefixedInitCodec {
    fn codec(&self, _addr: SocketAddr) -> io::Result<(Box<dyn Decoder>, Box<dyn Encoder>)> {
        Ok((Box::new(LengthPrefixedCodec), Box::new(LengthPrefixedCodec)))
    }
}