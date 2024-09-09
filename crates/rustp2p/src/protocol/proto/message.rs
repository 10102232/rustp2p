// This file is generated by rust-protobuf 3.5.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `message.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:PunchInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PunchInfo {
    // message fields
    // @@protoc_insertion_point(field:PunchInfo.reply)
    pub reply: bool,
    // @@protoc_insertion_point(field:PunchInfo.nat_type)
    pub nat_type: ::protobuf::EnumOrUnknown<PunchNatType>,
    // @@protoc_insertion_point(field:PunchInfo.public_ip_list)
    pub public_ip_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PunchInfo.public_port_range)
    pub public_port_range: u32,
    // @@protoc_insertion_point(field:PunchInfo.local_ip)
    pub local_ip: u32,
    // @@protoc_insertion_point(field:PunchInfo.ipv6)
    pub ipv6: ::bytes::Bytes,
    // @@protoc_insertion_point(field:PunchInfo.local_udp_ports)
    pub local_udp_ports: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PunchInfo.public_ports)
    pub public_ports: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PunchInfo.local_tcp_port)
    pub local_tcp_port: u32,
    // @@protoc_insertion_point(field:PunchInfo.public_tcp_port)
    pub public_tcp_port: u32,
    // @@protoc_insertion_point(field:PunchInfo.punch_model)
    pub punch_model: ::protobuf::EnumOrUnknown<PunchNatModel>,
    // @@protoc_insertion_point(field:PunchInfo.mapping_tcp_addr)
    pub mapping_tcp_addr: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:PunchInfo.mapping_udp_addr)
    pub mapping_udp_addr: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:PunchInfo.seq)
    pub seq: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PunchInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PunchInfo {
    fn default() -> &'a PunchInfo {
        <PunchInfo as ::protobuf::Message>::default_instance()
    }
}

impl PunchInfo {
    pub fn new() -> PunchInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(14);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reply",
            |m: &PunchInfo| { &m.reply },
            |m: &mut PunchInfo| { &mut m.reply },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "nat_type",
            |m: &PunchInfo| { &m.nat_type },
            |m: &mut PunchInfo| { &mut m.nat_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "public_ip_list",
            |m: &PunchInfo| { &m.public_ip_list },
            |m: &mut PunchInfo| { &mut m.public_ip_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "public_port_range",
            |m: &PunchInfo| { &m.public_port_range },
            |m: &mut PunchInfo| { &mut m.public_port_range },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "local_ip",
            |m: &PunchInfo| { &m.local_ip },
            |m: &mut PunchInfo| { &mut m.local_ip },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ipv6",
            |m: &PunchInfo| { &m.ipv6 },
            |m: &mut PunchInfo| { &mut m.ipv6 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "local_udp_ports",
            |m: &PunchInfo| { &m.local_udp_ports },
            |m: &mut PunchInfo| { &mut m.local_udp_ports },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "public_ports",
            |m: &PunchInfo| { &m.public_ports },
            |m: &mut PunchInfo| { &mut m.public_ports },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "local_tcp_port",
            |m: &PunchInfo| { &m.local_tcp_port },
            |m: &mut PunchInfo| { &mut m.local_tcp_port },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "public_tcp_port",
            |m: &PunchInfo| { &m.public_tcp_port },
            |m: &mut PunchInfo| { &mut m.public_tcp_port },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "punch_model",
            |m: &PunchInfo| { &m.punch_model },
            |m: &mut PunchInfo| { &mut m.punch_model },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "mapping_tcp_addr",
            |m: &PunchInfo| { &m.mapping_tcp_addr },
            |m: &mut PunchInfo| { &mut m.mapping_tcp_addr },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "mapping_udp_addr",
            |m: &PunchInfo| { &m.mapping_udp_addr },
            |m: &mut PunchInfo| { &mut m.mapping_udp_addr },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "seq",
            |m: &PunchInfo| { &m.seq },
            |m: &mut PunchInfo| { &mut m.seq },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PunchInfo>(
            "PunchInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PunchInfo {
    const NAME: &'static str = "PunchInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.reply = is.read_bool()?;
                },
                16 => {
                    self.nat_type = is.read_enum_or_unknown()?;
                },
                26 => {
                    is.read_repeated_packed_fixed32_into(&mut self.public_ip_list)?;
                },
                29 => {
                    self.public_ip_list.push(is.read_fixed32()?);
                },
                32 => {
                    self.public_port_range = is.read_uint32()?;
                },
                45 => {
                    self.local_ip = is.read_fixed32()?;
                },
                50 => {
                    self.ipv6 = is.read_tokio_bytes()?;
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.local_udp_ports)?;
                },
                56 => {
                    self.local_udp_ports.push(is.read_uint32()?);
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.public_ports)?;
                },
                64 => {
                    self.public_ports.push(is.read_uint32()?);
                },
                72 => {
                    self.local_tcp_port = is.read_uint32()?;
                },
                80 => {
                    self.public_tcp_port = is.read_uint32()?;
                },
                88 => {
                    self.punch_model = is.read_enum_or_unknown()?;
                },
                98 => {
                    self.mapping_tcp_addr.push(is.read_string()?);
                },
                106 => {
                    self.mapping_udp_addr.push(is.read_string()?);
                },
                112 => {
                    self.seq = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.reply != false {
            my_size += 1 + 1;
        }
        if self.nat_type != ::protobuf::EnumOrUnknown::new(PunchNatType::Symmetric) {
            my_size += ::protobuf::rt::int32_size(2, self.nat_type.value());
        }
        my_size += ::protobuf::rt::vec_packed_fixed32_size(3, &self.public_ip_list);
        if self.public_port_range != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.public_port_range);
        }
        if self.local_ip != 0 {
            my_size += 1 + 4;
        }
        if !self.ipv6.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.ipv6);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(7, &self.local_udp_ports);
        my_size += ::protobuf::rt::vec_packed_uint32_size(8, &self.public_ports);
        if self.local_tcp_port != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.local_tcp_port);
        }
        if self.public_tcp_port != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.public_tcp_port);
        }
        if self.punch_model != ::protobuf::EnumOrUnknown::new(PunchNatModel::All) {
            my_size += ::protobuf::rt::int32_size(11, self.punch_model.value());
        }
        for value in &self.mapping_tcp_addr {
            my_size += ::protobuf::rt::string_size(12, &value);
        };
        for value in &self.mapping_udp_addr {
            my_size += ::protobuf::rt::string_size(13, &value);
        };
        if self.seq != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.seq);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.reply != false {
            os.write_bool(1, self.reply)?;
        }
        if self.nat_type != ::protobuf::EnumOrUnknown::new(PunchNatType::Symmetric) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.nat_type))?;
        }
        os.write_repeated_packed_fixed32(3, &self.public_ip_list)?;
        if self.public_port_range != 0 {
            os.write_uint32(4, self.public_port_range)?;
        }
        if self.local_ip != 0 {
            os.write_fixed32(5, self.local_ip)?;
        }
        if !self.ipv6.is_empty() {
            os.write_bytes(6, &self.ipv6)?;
        }
        os.write_repeated_packed_uint32(7, &self.local_udp_ports)?;
        os.write_repeated_packed_uint32(8, &self.public_ports)?;
        if self.local_tcp_port != 0 {
            os.write_uint32(9, self.local_tcp_port)?;
        }
        if self.public_tcp_port != 0 {
            os.write_uint32(10, self.public_tcp_port)?;
        }
        if self.punch_model != ::protobuf::EnumOrUnknown::new(PunchNatModel::All) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.punch_model))?;
        }
        for v in &self.mapping_tcp_addr {
            os.write_string(12, &v)?;
        };
        for v in &self.mapping_udp_addr {
            os.write_string(13, &v)?;
        };
        if self.seq != 0 {
            os.write_uint32(14, self.seq)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PunchInfo {
        PunchInfo::new()
    }

    fn clear(&mut self) {
        self.reply = false;
        self.nat_type = ::protobuf::EnumOrUnknown::new(PunchNatType::Symmetric);
        self.public_ip_list.clear();
        self.public_port_range = 0;
        self.local_ip = 0;
        self.ipv6.clear();
        self.local_udp_ports.clear();
        self.public_ports.clear();
        self.local_tcp_port = 0;
        self.public_tcp_port = 0;
        self.punch_model = ::protobuf::EnumOrUnknown::new(PunchNatModel::All);
        self.mapping_tcp_addr.clear();
        self.mapping_udp_addr.clear();
        self.seq = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PunchInfo {
        static instance: PunchInfo = PunchInfo {
            reply: false,
            nat_type: ::protobuf::EnumOrUnknown::from_i32(0),
            public_ip_list: ::std::vec::Vec::new(),
            public_port_range: 0,
            local_ip: 0,
            ipv6: ::bytes::Bytes::new(),
            local_udp_ports: ::std::vec::Vec::new(),
            public_ports: ::std::vec::Vec::new(),
            local_tcp_port: 0,
            public_tcp_port: 0,
            punch_model: ::protobuf::EnumOrUnknown::from_i32(0),
            mapping_tcp_addr: ::std::vec::Vec::new(),
            mapping_udp_addr: ::std::vec::Vec::new(),
            seq: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PunchInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PunchInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PunchInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PunchInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:PunchNatType)
pub enum PunchNatType {
    // @@protoc_insertion_point(enum_value:PunchNatType.Symmetric)
    Symmetric = 0,
    // @@protoc_insertion_point(enum_value:PunchNatType.Cone)
    Cone = 1,
}

impl ::protobuf::Enum for PunchNatType {
    const NAME: &'static str = "PunchNatType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PunchNatType> {
        match value {
            0 => ::std::option::Option::Some(PunchNatType::Symmetric),
            1 => ::std::option::Option::Some(PunchNatType::Cone),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<PunchNatType> {
        match str {
            "Symmetric" => ::std::option::Option::Some(PunchNatType::Symmetric),
            "Cone" => ::std::option::Option::Some(PunchNatType::Cone),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [PunchNatType] = &[
        PunchNatType::Symmetric,
        PunchNatType::Cone,
    ];
}

impl ::protobuf::EnumFull for PunchNatType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("PunchNatType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for PunchNatType {
    fn default() -> Self {
        PunchNatType::Symmetric
    }
}

impl PunchNatType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PunchNatType>("PunchNatType")
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:PunchNatModel)
pub enum PunchNatModel {
    // @@protoc_insertion_point(enum_value:PunchNatModel.All)
    All = 0,
    // @@protoc_insertion_point(enum_value:PunchNatModel.IPv4)
    IPv4 = 1,
    // @@protoc_insertion_point(enum_value:PunchNatModel.IPv6)
    IPv6 = 2,
    // @@protoc_insertion_point(enum_value:PunchNatModel.IPv4Tcp)
    IPv4Tcp = 3,
    // @@protoc_insertion_point(enum_value:PunchNatModel.IPv4Udp)
    IPv4Udp = 4,
    // @@protoc_insertion_point(enum_value:PunchNatModel.IPv6Tcp)
    IPv6Tcp = 5,
    // @@protoc_insertion_point(enum_value:PunchNatModel.IPv6Udp)
    IPv6Udp = 6,
}

impl ::protobuf::Enum for PunchNatModel {
    const NAME: &'static str = "PunchNatModel";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PunchNatModel> {
        match value {
            0 => ::std::option::Option::Some(PunchNatModel::All),
            1 => ::std::option::Option::Some(PunchNatModel::IPv4),
            2 => ::std::option::Option::Some(PunchNatModel::IPv6),
            3 => ::std::option::Option::Some(PunchNatModel::IPv4Tcp),
            4 => ::std::option::Option::Some(PunchNatModel::IPv4Udp),
            5 => ::std::option::Option::Some(PunchNatModel::IPv6Tcp),
            6 => ::std::option::Option::Some(PunchNatModel::IPv6Udp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<PunchNatModel> {
        match str {
            "All" => ::std::option::Option::Some(PunchNatModel::All),
            "IPv4" => ::std::option::Option::Some(PunchNatModel::IPv4),
            "IPv6" => ::std::option::Option::Some(PunchNatModel::IPv6),
            "IPv4Tcp" => ::std::option::Option::Some(PunchNatModel::IPv4Tcp),
            "IPv4Udp" => ::std::option::Option::Some(PunchNatModel::IPv4Udp),
            "IPv6Tcp" => ::std::option::Option::Some(PunchNatModel::IPv6Tcp),
            "IPv6Udp" => ::std::option::Option::Some(PunchNatModel::IPv6Udp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [PunchNatModel] = &[
        PunchNatModel::All,
        PunchNatModel::IPv4,
        PunchNatModel::IPv6,
        PunchNatModel::IPv4Tcp,
        PunchNatModel::IPv4Udp,
        PunchNatModel::IPv6Tcp,
        PunchNatModel::IPv6Udp,
    ];
}

impl ::protobuf::EnumFull for PunchNatModel {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("PunchNatModel").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for PunchNatModel {
    fn default() -> Self {
        PunchNatModel::All
    }
}

impl PunchNatModel {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PunchNatModel>("PunchNatModel")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmessage.proto\"\xfc\x03\n\tPunchInfo\x12\x14\n\x05reply\x18\x01\x20\
    \x01(\x08R\x05reply\x12(\n\x08nat_type\x18\x02\x20\x01(\x0e2\r.PunchNatT\
    ypeR\x07natType\x12$\n\x0epublic_ip_list\x18\x03\x20\x03(\x07R\x0cpublic\
    IpList\x12*\n\x11public_port_range\x18\x04\x20\x01(\rR\x0fpublicPortRang\
    e\x12\x19\n\x08local_ip\x18\x05\x20\x01(\x07R\x07localIp\x12\x12\n\x04ip\
    v6\x18\x06\x20\x01(\x0cR\x04ipv6\x12&\n\x0flocal_udp_ports\x18\x07\x20\
    \x03(\rR\rlocalUdpPorts\x12!\n\x0cpublic_ports\x18\x08\x20\x03(\rR\x0bpu\
    blicPorts\x12$\n\x0elocal_tcp_port\x18\t\x20\x01(\rR\x0clocalTcpPort\x12\
    &\n\x0fpublic_tcp_port\x18\n\x20\x01(\rR\rpublicTcpPort\x12/\n\x0bpunch_\
    model\x18\x0b\x20\x01(\x0e2\x0e.PunchNatModelR\npunchModel\x12(\n\x10map\
    ping_tcp_addr\x18\x0c\x20\x03(\tR\x0emappingTcpAddr\x12(\n\x10mapping_ud\
    p_addr\x18\r\x20\x03(\tR\x0emappingUdpAddr\x12\x10\n\x03seq\x18\x0e\x20\
    \x01(\rR\x03seq*'\n\x0cPunchNatType\x12\r\n\tSymmetric\x10\0\x12\x08\n\
    \x04Cone\x10\x01*`\n\rPunchNatModel\x12\x07\n\x03All\x10\0\x12\x08\n\x04\
    IPv4\x10\x01\x12\x08\n\x04IPv6\x10\x02\x12\x0b\n\x07IPv4Tcp\x10\x03\x12\
    \x0b\n\x07IPv4Udp\x10\x04\x12\x0b\n\x07IPv6Tcp\x10\x05\x12\x0b\n\x07IPv6\
    Udp\x10\x06b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PunchInfo::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(2);
            enums.push(PunchNatType::generated_enum_descriptor_data());
            enums.push(PunchNatModel::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
