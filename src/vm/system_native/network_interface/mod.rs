//! `java.net.NetworkInterface` natives, following HotSpot's `NetworkInterface.c`.
//!
//! The platform modules collect the interfaces the way HotSpot does on that platform, and this
//! module turns them into Java objects.

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
use unix as platform;
#[cfg(windows)]
use windows as platform;

use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::{
    set_pending_null_pointer_exception_with_message, set_pending_socket_exception,
};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::system_native::inet_address::ipv6_available;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

const NETWORK_INTERFACE: &str = "java/net/NetworkInterface";
const INTERFACE_ADDRESS: &str = "java/net/InterfaceAddress";
const INET_ADDRESS: &str = "java/net/InetAddress";
const INET_ADDRESS_HOLDER: &str = "java/net/InetAddress$InetAddressHolder";
const INET4_ADDRESS: &str = "java/net/Inet4Address";
const INET6_ADDRESS: &str = "java/net/Inet6Address";
const INET6_ADDRESS_HOLDER: &str = "java/net/Inet6Address$Inet6AddressHolder";

/// `java.net.InetAddress.IPv4`
const IPV4_FAMILY: i32 = 1;
/// `java.net.InetAddress.IPv6`
const IPV6_FAMILY: i32 = 2;

/// A network interface as HotSpot collects it before creating the `NetworkInterface`.
#[derive(Debug, Clone)]
pub(crate) struct NetIf {
    name: String,
    /// `None` uses the name, as HotSpot does on Unix
    display_name: Option<String>,
    index: i32,
    is_virtual: bool,
    addresses: Vec<NetAddr>,
    children: Vec<NetIf>,
}

/// An address of a network interface, which becomes an `InetAddress` and an `InterfaceAddress`.
#[derive(Debug, Clone)]
pub(crate) struct NetAddr {
    ip: IpAddr,
    /// IPv6 only: zero leaves the scope of the `Inet6Address` unset
    scope_id: i32,
    broadcast: Option<Ipv4Addr>,
    /// `None` leaves `InterfaceAddress.maskLength` at its default
    prefix: Option<i16>,
}

/// The address a `java.net.InetAddress` holds.
#[derive(Debug, Clone, Copy)]
pub(crate) enum JavaInetAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr, u32),
}

/// A failure HotSpot reports as a `SocketException` with this message.
#[derive(Debug)]
pub(crate) struct SocketError(String);

impl SocketError {
    /// The message of HotSpot's `JNU_ThrowByNameWithMessageAndLastError`: the OS error
    /// description followed by the failed operation, or the operation alone.
    fn with_os_error(os_error: Option<String>, operation: &str) -> Self {
        match os_error {
            Some(os_error) => Self(format!("{os_error} ({operation})")),
            None => Self(operation.to_string()),
        }
    }
}

pub(crate) type SocketResult<T> = std::result::Result<T, SocketError>;

/// `java.net.NetworkInterface.init()V`
pub(crate) fn init() -> Result<()> {
    Ok(())
}

/// `java.net.NetworkInterface.getAll()[Ljava/net/NetworkInterface;`
pub(crate) fn get_all() -> Result<i32> {
    let Some(interfaces) = or_throw(platform::all(ipv6_available()?))? else {
        return Ok(0);
    };
    let interface_refs = interfaces
        .iter()
        .map(create_network_interface)
        .collect::<Result<Vec<_>>>()?;
    Ok(HEAP.create_array_with_values("[Ljava/net/NetworkInterface;", &interface_refs))
}

/// `java.net.NetworkInterface.getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;`
pub(crate) fn get_by_name0(name_ref: i32) -> Result<i32> {
    let Some(name) = interface_name(name_ref)? else {
        return Ok(0);
    };
    let interface = or_throw(platform::by_name(&name, ipv6_available()?))?;
    create_optional_network_interface(interface.flatten())
}

/// `java.net.NetworkInterface.getByIndex0(I)Ljava/net/NetworkInterface;`
pub(crate) fn get_by_index0(index: i32) -> Result<i32> {
    let interface = or_throw(platform::by_index(index, ipv6_available()?))?;
    create_optional_network_interface(interface.flatten())
}

/// `java.net.NetworkInterface.getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;`
pub(crate) fn get_by_inet_address0(address_ref: i32) -> Result<i32> {
    let Some(address) = java_inet_address(address_ref)? else {
        return Ok(0);
    };
    let interface = or_throw(platform::by_inet_address(address, ipv6_available()?))?;
    create_optional_network_interface(interface.flatten())
}

/// `java.net.NetworkInterface.boundInetAddress0(Ljava/net/InetAddress;)Z`
pub(crate) fn bound_inet_address0(address_ref: i32) -> Result<bool> {
    let Some(address) = java_inet_address(address_ref)? else {
        return Ok(false);
    };
    let bound = or_throw(platform::is_bound(address, ipv6_available()?))?;
    Ok(bound.unwrap_or(false))
}

/// `java.net.NetworkInterface.isUp0(Ljava/lang/String;I)Z`
pub(crate) fn is_up0(name_ref: i32, index: i32) -> Result<bool> {
    flag(name_ref, index, platform::is_up)
}

/// `java.net.NetworkInterface.isLoopback0(Ljava/lang/String;I)Z`
pub(crate) fn is_loopback0(name_ref: i32, index: i32) -> Result<bool> {
    flag(name_ref, index, platform::is_loopback)
}

/// `java.net.NetworkInterface.isP2P0(Ljava/lang/String;I)Z`
pub(crate) fn is_p2p0(name_ref: i32, index: i32) -> Result<bool> {
    flag(name_ref, index, platform::is_point_to_point)
}

/// `java.net.NetworkInterface.supportsMulticast0(Ljava/lang/String;I)Z`
pub(crate) fn supports_multicast0(name_ref: i32, index: i32) -> Result<bool> {
    flag(name_ref, index, platform::supports_multicast)
}

/// `java.net.NetworkInterface.getMacAddr0([BLjava/lang/String;I)[B`
pub(crate) fn get_mac_addr0(_address_ref: i32, name_ref: i32, index: i32) -> Result<i32> {
    let Some(name) = interface_name(name_ref)? else {
        return Ok(0);
    };
    let Some(Some(mac_address)) = or_throw(platform::mac_address(&name, index))? else {
        return Ok(0);
    };
    let bytes = mac_address
        .iter()
        .map(|&b| b as i8 as i32)
        .collect::<Vec<_>>();
    Ok(HEAP.create_array_with_values("[B", &bytes))
}

/// `java.net.NetworkInterface.getMTU0(Ljava/lang/String;I)I`
pub(crate) fn get_mtu0(name_ref: i32, index: i32) -> Result<i32> {
    let Some(name) = interface_name(name_ref)? else {
        return Ok(-1);
    };
    Ok(or_throw(platform::mtu(&name, index))?.unwrap_or(-1))
}

fn flag(name_ref: i32, index: i32, query: fn(&str, i32) -> SocketResult<bool>) -> Result<bool> {
    let Some(name) = interface_name(name_ref)? else {
        return Ok(false);
    };
    Ok(or_throw(query(&name, index))?.unwrap_or(false))
}

/// Sets a pending `SocketException` for a failed platform call, which then yields `None`.
fn or_throw<T>(result: SocketResult<T>) -> Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(SocketError(message)) => {
            set_pending_socket_exception(&message)?;
            Ok(None)
        }
    }
}

/// The name passed to a native, or a pending `NullPointerException` for `null` like HotSpot.
fn interface_name(name_ref: i32) -> Result<Option<String>> {
    if name_ref == 0 {
        set_pending_null_pointer_exception_with_message("network interface name is NULL")?;
        return Ok(None);
    }
    get_utf8_string_by_ref(name_ref).map(Some)
}

/// Reads the address of a `java.net.InetAddress`, or `None` for an unknown family.
fn java_inet_address(address_ref: i32) -> Result<Option<JavaInetAddress>> {
    let holder_ref = HEAP.get_object_field_value(address_ref, INET_ADDRESS, "holder")?[0];
    let family = HEAP.get_object_field_value(holder_ref, INET_ADDRESS_HOLDER, "family")?[0];
    let address = match family {
        IPV4_FAMILY => {
            let address =
                HEAP.get_object_field_value(holder_ref, INET_ADDRESS_HOLDER, "address")?[0];
            JavaInetAddress::V4(Ipv4Addr::from(address as u32))
        }
        IPV6_FAMILY => {
            let holder6_ref =
                HEAP.get_object_field_value(address_ref, INET6_ADDRESS, "holder6")?[0];
            let ipaddress_ref =
                HEAP.get_object_field_value(holder6_ref, INET6_ADDRESS_HOLDER, "ipaddress")?[0];
            let octets: [u8; 16] = HEAP.get_entire_array(ipaddress_ref)?.raw_data()[..16]
                .try_into()
                .expect("a slice of 16 bytes");
            let scope_id =
                HEAP.get_object_field_value(holder6_ref, INET6_ADDRESS_HOLDER, "scope_id")?[0];
            JavaInetAddress::V6(Ipv6Addr::from(octets), scope_id as u32)
        }
        _ => return Ok(None),
    };
    Ok(Some(address))
}

fn create_optional_network_interface(interface: Option<NetIf>) -> Result<i32> {
    match interface {
        Some(interface) => create_network_interface(&interface),
        None => Ok(0),
    }
}

fn create_network_interface(interface: &NetIf) -> Result<i32> {
    let interface_ref = Executor::invoke_default_constructor(NETWORK_INTERFACE)?;
    let name_ref = StringPoolHelper::create_string(&interface.name)?;
    let display_name_ref = match &interface.display_name {
        Some(display_name) => StringPoolHelper::create_string(display_name)?,
        None => name_ref,
    };
    set_field(interface_ref, NETWORK_INTERFACE, "name", name_ref)?;
    set_field(
        interface_ref,
        NETWORK_INTERFACE,
        "displayName",
        display_name_ref,
    )?;
    set_field(interface_ref, NETWORK_INTERFACE, "index", interface.index)?;
    set_field(
        interface_ref,
        NETWORK_INTERFACE,
        "virtual",
        interface.is_virtual as i32,
    )?;

    let mut address_refs = Vec::with_capacity(interface.addresses.len());
    let mut binding_refs = Vec::with_capacity(interface.addresses.len());
    for address in &interface.addresses {
        let address_ref = create_inet_address(address, interface_ref)?;
        binding_refs.push(create_interface_address(address, address_ref)?);
        address_refs.push(address_ref);
    }

    let mut child_refs = Vec::with_capacity(interface.children.len());
    for child in &interface.children {
        let child_ref = create_network_interface(child)?;
        set_field(child_ref, NETWORK_INTERFACE, "parent", interface_ref)?;
        child_refs.push(child_ref);
    }

    let addrs_ref = HEAP.create_array_with_values("[Ljava/net/InetAddress;", &address_refs);
    let bindings_ref =
        HEAP.create_array_with_values("[Ljava/net/InterfaceAddress;", &binding_refs);
    let childs_ref = HEAP.create_array_with_values("[Ljava/net/NetworkInterface;", &child_refs);
    set_field(interface_ref, NETWORK_INTERFACE, "addrs", addrs_ref)?;
    set_field(interface_ref, NETWORK_INTERFACE, "bindings", bindings_ref)?;
    set_field(interface_ref, NETWORK_INTERFACE, "childs", childs_ref)?;
    Ok(interface_ref)
}

fn create_inet_address(address: &NetAddr, interface_ref: i32) -> Result<i32> {
    match address.ip {
        IpAddr::V4(ip) => create_inet4_address(ip),
        IpAddr::V6(ip) => {
            let address_ref = Executor::invoke_default_constructor(INET6_ADDRESS)?;
            let holder6_ref =
                HEAP.get_object_field_value(address_ref, INET6_ADDRESS, "holder6")?[0];
            let octets = ip
                .octets()
                .iter()
                .map(|&b| b as i8 as i32)
                .collect::<Vec<_>>();
            let ipaddress_ref = HEAP.create_array_with_values("[B", &octets);
            set_field(
                holder6_ref,
                INET6_ADDRESS_HOLDER,
                "ipaddress",
                ipaddress_ref,
            )?;
            if address.scope_id != 0 {
                set_field(
                    holder6_ref,
                    INET6_ADDRESS_HOLDER,
                    "scope_id",
                    address.scope_id,
                )?;
                if address.scope_id > 0 {
                    set_field(holder6_ref, INET6_ADDRESS_HOLDER, "scope_id_set", 1)?;
                }
                set_field(
                    holder6_ref,
                    INET6_ADDRESS_HOLDER,
                    "scope_ifname",
                    interface_ref,
                )?;
            }
            Ok(address_ref)
        }
    }
}

fn create_inet4_address(ip: Ipv4Addr) -> Result<i32> {
    let address_ref = Executor::invoke_default_constructor(INET4_ADDRESS)?;
    let holder_ref = HEAP.get_object_field_value(address_ref, INET_ADDRESS, "holder")?[0];
    set_field(
        holder_ref,
        INET_ADDRESS_HOLDER,
        "address",
        u32::from(ip) as i32,
    )?;
    Ok(address_ref)
}

fn create_interface_address(address: &NetAddr, address_ref: i32) -> Result<i32> {
    let binding_ref = Executor::invoke_default_constructor(INTERFACE_ADDRESS)?;
    set_field(binding_ref, INTERFACE_ADDRESS, "address", address_ref)?;
    if let Some(broadcast) = address.broadcast {
        let broadcast_ref = create_inet4_address(broadcast)?;
        set_field(binding_ref, INTERFACE_ADDRESS, "broadcast", broadcast_ref)?;
    }
    if let Some(prefix) = address.prefix {
        set_field(binding_ref, INTERFACE_ADDRESS, "maskLength", prefix as i32)?;
    }
    Ok(binding_ref)
}

fn set_field(object_ref: i32, class_name: &str, field_name: &str, value: i32) -> Result<()> {
    HEAP.set_object_field_value(object_ref, class_name, field_name, vec![value])
}
