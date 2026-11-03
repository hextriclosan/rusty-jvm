//! Unix network interfaces, collected like HotSpot's `unix/native/libnet/NetworkInterface.c`:
//! on Linux from `SIOCGIFCONF` and `/proc/net/if_inet6`, elsewhere (BSD) from `getifaddrs`.

use super::{JavaInetAddress, NetAddr, NetIf, SocketError, SocketResult};
use nix::libc;
use std::ffi::{CStr, CString};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

// `_IOWR('i', 17, struct ifreq)` and `_IOWR('i', 51, struct ifreq)`, which libc lacks for BSD
#[cfg(not(target_os = "linux"))]
const SIOCGIFFLAGS: libc::c_ulong = 0xc020_6911;
#[cfg(not(target_os = "linux"))]
const SIOCGIFMTU: libc::c_ulong = 0xc020_6933;
#[cfg(target_os = "linux")]
const SIOCGIFFLAGS: libc::c_ulong = libc::SIOCGIFFLAGS as libc::c_ulong;
#[cfg(target_os = "linux")]
const SIOCGIFMTU: libc::c_ulong = libc::SIOCGIFMTU as libc::c_ulong;

pub(super) fn all(ipv6: bool) -> SocketResult<Vec<NetIf>> {
    enumerate(true, ipv6)
}

pub(super) fn by_name(name: &str, ipv6: bool) -> SocketResult<Option<NetIf>> {
    let interfaces = enumerate(true, ipv6)?;
    // A virtual sub-interface such as `eth0:1` is looked up under its parent
    let (parent_name, is_child) = match name.find(':') {
        Some(colon) => (&name[..colon], true),
        None => (name, false),
    };
    let parent = interfaces
        .into_iter()
        .find(|interface| interface.name == parent_name);
    if is_child {
        Ok(parent.and_then(|parent| parent.children.into_iter().find(|child| child.name == name)))
    } else {
        Ok(parent)
    }
}

pub(super) fn by_index(index: i32, ipv6: bool) -> SocketResult<Option<NetIf>> {
    if index <= 0 {
        return Ok(None);
    }
    let interfaces = enumerate(true, ipv6)?;
    Ok(interfaces
        .into_iter()
        .find(|interface| interface.index == index))
}

pub(super) fn by_inet_address(
    address: JavaInetAddress,
    ipv6: bool,
) -> SocketResult<Option<NetIf>> {
    Ok(find_bound(enumerate(true, ipv6)?, address))
}

pub(super) fn is_bound(address: JavaInetAddress, ipv6: bool) -> SocketResult<bool> {
    let interfaces = match address {
        JavaInetAddress::V4(_) => enumerate(true, false)?,
        JavaInetAddress::V6(..) if ipv6 => enumerate(false, true)?,
        JavaInetAddress::V6(..) => return Ok(false),
    };
    Ok(find_bound(interfaces, address).is_some())
}

pub(super) fn is_up(name: &str, _index: i32) -> SocketResult<bool> {
    let flags = flags_of(name)?;
    Ok(flags & libc::IFF_UP != 0 && flags & libc::IFF_RUNNING != 0)
}

pub(super) fn is_loopback(name: &str, _index: i32) -> SocketResult<bool> {
    Ok(flags_of(name)? & libc::IFF_LOOPBACK != 0)
}

pub(super) fn is_point_to_point(name: &str, _index: i32) -> SocketResult<bool> {
    Ok(flags_of(name)? & libc::IFF_POINTOPOINT != 0)
}

pub(super) fn supports_multicast(name: &str, _index: i32) -> SocketResult<bool> {
    Ok(flags_of(name)? & libc::IFF_MULTICAST != 0)
}

pub(super) fn mtu(name: &str, _index: i32) -> SocketResult<i32> {
    let socket = open_socket_with_fallback()?;
    let mut request = interface_request(name);
    ioctl(&socket, SIOCGIFMTU, &mut request)
        .map_err(|errno| os_error(errno, "ioctl(SIOCGIFMTU) failed"))?;
    Ok(unsafe { request.ifr_ifru.ifru_mtu })
}

#[cfg(target_os = "linux")]
pub(super) fn mac_address(name: &str, _index: i32) -> SocketResult<Option<Vec<u8>>> {
    const IFHWADDRLEN: usize = 6;
    let socket = open_socket_with_fallback()?;
    let mut request = interface_request(name);
    ioctl(&socket, libc::SIOCGIFHWADDR as libc::c_ulong, &mut request)
        .map_err(|errno| os_error(errno, "ioctl(SIOCGIFHWADDR) failed"))?;
    let hardware_address = unsafe { request.ifr_ifru.ifru_hwaddr.sa_data };
    let mac_address = hardware_address[..IFHWADDRLEN]
        .iter()
        .map(|&b| b as u8)
        .collect::<Vec<_>>();
    // all bytes 0 means no hardware address
    Ok(mac_address.iter().any(|&b| b != 0).then_some(mac_address))
}

#[cfg(not(target_os = "linux"))]
pub(super) fn mac_address(name: &str, _index: i32) -> SocketResult<Option<Vec<u8>>> {
    const ETHER_ADDR_LEN: usize = 6;
    // HotSpot treats a failed getifaddrs as no hardware address
    let Ok(addresses) = InterfaceAddresses::new() else {
        return Ok(None);
    };
    for entry in addresses.iter() {
        if entry.ifa_addr.is_null()
            || unsafe { (*entry.ifa_addr).sa_family } as libc::c_int != libc::AF_LINK
            || unsafe { CStr::from_ptr(entry.ifa_name) }.to_bytes() != name.as_bytes()
        {
            continue;
        }
        let link = entry.ifa_addr as *const libc::sockaddr_dl;
        let (name_length, address_length) =
            unsafe { ((*link).sdl_nlen as usize, (*link).sdl_alen as usize) };
        if address_length == ETHER_ADDR_LEN {
            // the address follows the name in `sdl_data`, which may run past its declared size
            let data =
                unsafe { (link as *const u8).add(mem::offset_of!(libc::sockaddr_dl, sdl_data)) };
            let mac_address =
                unsafe { std::slice::from_raw_parts(data.add(name_length), ETHER_ADDR_LEN) };
            return Ok(Some(mac_address.to_vec()));
        }
    }
    Ok(None)
}

/// HotSpot's `enumInterfaces`: an empty list when the IPv6 socket can't be opened, as HotSpot
/// then returns no interfaces
fn enumerate(ipv4: bool, ipv6: bool) -> SocketResult<Vec<NetIf>> {
    let mut interfaces = Vec::new();
    if ipv4 {
        if let Some(socket) = open_socket(libc::AF_INET)? {
            enumerate_ipv4(&socket, &mut interfaces)?;
        }
    }
    if ipv6 {
        let Some(socket) = open_socket(libc::AF_INET6)? else {
            return Ok(Vec::new());
        };
        enumerate_ipv6(&socket, &mut interfaces)?;
    }
    Ok(interfaces)
}

#[cfg(target_os = "linux")]
fn enumerate_ipv4(socket: &OwnedFd, interfaces: &mut Vec<NetIf>) -> SocketResult<()> {
    let request_size = mem::size_of::<libc::ifreq>();
    // a first call without a buffer reports the size needed
    let mut configuration: libc::ifconf = unsafe { mem::zeroed() };
    ioctl(
        socket,
        libc::SIOCGIFCONF as libc::c_ulong,
        &mut configuration,
    )
    .map_err(|errno| os_error(errno, "ioctl(SIOCGIFCONF) failed"))?;
    let mut requests = vec![
        unsafe { mem::zeroed::<libc::ifreq>() };
        configuration.ifc_len as usize / request_size
    ];
    configuration.ifc_len = (requests.len() * request_size) as libc::c_int;
    configuration.ifc_ifcu.ifcu_req = requests.as_mut_ptr();
    ioctl(
        socket,
        libc::SIOCGIFCONF as libc::c_ulong,
        &mut configuration,
    )
    .map_err(|errno| os_error(errno, "ioctl(SIOCGIFCONF) failed"))?;
    requests.truncate(configuration.ifc_len as usize / request_size);

    for request in &requests {
        let address = unsafe { request.ifr_ifru.ifru_addr };
        if address.sa_family as libc::c_int != libc::AF_INET {
            continue;
        }
        let ip = ipv4_of(&address);

        let mut probe = *request;
        let broadcast = match ioctl(socket, SIOCGIFFLAGS, &mut probe) {
            Ok(())
                if unsafe { probe.ifr_ifru.ifru_flags } as libc::c_int & libc::IFF_BROADCAST
                    != 0 =>
            {
                let mut probe = *request;
                ioctl(socket, libc::SIOCGIFBRDADDR as libc::c_ulong, &mut probe)
                    .ok()
                    .map(|()| ipv4_of(unsafe { &probe.ifr_ifru.ifru_broadaddr }))
            }
            _ => None,
        };
        let mut probe = *request;
        let prefix = match ioctl(socket, libc::SIOCGIFNETMASK as libc::c_ulong, &mut probe) {
            Ok(()) => ipv4_prefix(ipv4_of(unsafe { &probe.ifr_ifru.ifru_netmask })),
            Err(_) => 0,
        };

        let name = unsafe { CStr::from_ptr(request.ifr_name.as_ptr()) }.to_string_lossy();
        add_interface(
            interfaces,
            socket,
            &name,
            NetAddr {
                ip: IpAddr::V4(ip),
                scope_id: 0,
                broadcast,
                prefix: Some(prefix),
            },
        );
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn enumerate_ipv6(socket: &OwnedFd, interfaces: &mut Vec<NetIf>) -> SocketResult<()> {
    // Lines such as `fe800000000000000000000000000001 02 40 20 80 eth0`: the address, the
    // interface index, the prefix length, the scope, the DAD status and the interface name
    let Ok(content) = std::fs::read_to_string("/proc/net/if_inet6") else {
        return Ok(());
    };
    for line in content.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let [address, index, prefix, _scope, _dad_status, name] = fields[..] else {
            continue;
        };
        let (Ok(address), Ok(index), Ok(prefix)) = (
            u128::from_str_radix(address, 16),
            u32::from_str_radix(index, 16),
            u32::from_str_radix(prefix, 16),
        ) else {
            continue;
        };
        add_interface(
            interfaces,
            socket,
            name,
            NetAddr {
                ip: IpAddr::V6(Ipv6Addr::from(address)),
                scope_id: index as i32,
                broadcast: None,
                prefix: Some(prefix as i16),
            },
        );
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn enumerate_ipv4(socket: &OwnedFd, interfaces: &mut Vec<NetIf>) -> SocketResult<()> {
    let addresses = InterfaceAddresses::new()?;
    for entry in addresses.iter() {
        let Some(address) = (unsafe { read_sockaddr::<libc::sockaddr_in>(entry.ifa_addr) }) else {
            continue;
        };
        if address.sin_family as libc::c_int != libc::AF_INET {
            continue;
        }
        let flags = entry.ifa_flags as libc::c_int;
        let broadcast = if flags & libc::IFF_POINTOPOINT == 0 && flags & libc::IFF_BROADCAST != 0 {
            unsafe { read_sockaddr::<libc::sockaddr_in>(entry.ifa_dstaddr) }
                .map(|broadcast| Ipv4Addr::from(u32::from_be(broadcast.sin_addr.s_addr)))
        } else {
            None
        };
        let prefix = unsafe { read_sockaddr::<libc::sockaddr_in>(entry.ifa_netmask) }
            .map_or(0, |netmask| {
                ipv4_prefix(Ipv4Addr::from(u32::from_be(netmask.sin_addr.s_addr)))
            });
        let name = unsafe { CStr::from_ptr(entry.ifa_name) }.to_string_lossy();
        add_interface(
            interfaces,
            socket,
            &name,
            NetAddr {
                ip: IpAddr::V4(Ipv4Addr::from(u32::from_be(address.sin_addr.s_addr))),
                scope_id: 0,
                broadcast,
                prefix: Some(prefix),
            },
        );
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn enumerate_ipv6(socket: &OwnedFd, interfaces: &mut Vec<NetIf>) -> SocketResult<()> {
    let addresses = InterfaceAddresses::new()?;
    for entry in addresses.iter() {
        let Some(address) = (unsafe { read_sockaddr::<libc::sockaddr_in6>(entry.ifa_addr) })
        else {
            continue;
        };
        if address.sin6_family as libc::c_int != libc::AF_INET6 {
            continue;
        }
        let name = unsafe { CStr::from_ptr(entry.ifa_name) }.to_string_lossy();
        let prefix = unsafe { read_sockaddr::<libc::sockaddr_in6>(entry.ifa_netmask) }
            .map_or(0, |netmask| ipv6_prefix(&netmask.sin6_addr.s6_addr));
        add_interface(
            interfaces,
            socket,
            &name,
            NetAddr {
                ip: IpAddr::V6(Ipv6Addr::from(address.sin6_addr.s6_addr)),
                // HotSpot replaces the scope with the interface index
                scope_id: index_of(&name),
                broadcast: None,
                prefix: Some(prefix),
            },
        );
    }
    Ok(())
}

/// HotSpot's `addif`: a new interface and a new address go first in their lists, and an
/// address of a virtual sub-interface such as `eth0:1` also goes to its parent.
fn add_interface(interfaces: &mut Vec<NetIf>, socket: &OwnedFd, if_name: &str, address: NetAddr) {
    let mut name = if_name;
    let mut is_virtual = false;
    let mut child_name = None;
    if let Some(colon) = if_name.find(':') {
        if get_flags(socket, &if_name[..colon]).is_ok() {
            name = &if_name[..colon];
            child_name = Some(if_name);
        } else {
            // without access to the parent, it's a virtual interface on its own
            is_virtual = true;
        }
    }

    let position = match interfaces
        .iter()
        .position(|interface| interface.name == name)
    {
        Some(position) => position,
        None => {
            interfaces.insert(0, new_interface(name, is_virtual));
            0
        }
    };
    let parent = &mut interfaces[position];
    parent.addresses.insert(0, address.clone());

    if let Some(child_name) = child_name {
        let position = match parent
            .children
            .iter()
            .position(|child| child.name == child_name)
        {
            Some(position) => position,
            None => {
                parent.children.insert(0, new_interface(child_name, true));
                0
            }
        };
        parent.children[position].addresses.insert(0, address);
    }
}

fn new_interface(name: &str, is_virtual: bool) -> NetIf {
    NetIf {
        name: name.to_string(),
        display_name: None,
        index: index_of(name),
        is_virtual,
        addresses: Vec::new(),
        children: Vec::new(),
    }
}

/// HotSpot's `find_bound_interface`: an IPv6 address with a scope ends the search on an
/// interface at the first address of another scope.
fn find_bound(interfaces: Vec<NetIf>, wanted: JavaInetAddress) -> Option<NetIf> {
    interfaces.into_iter().find(|interface| {
        for address in &interface.addresses {
            match (wanted, address.ip) {
                (JavaInetAddress::V4(wanted), IpAddr::V4(ip)) if wanted == ip => return true,
                (JavaInetAddress::V6(wanted, scope_id), IpAddr::V6(ip)) => {
                    if scope_id != 0 && scope_id != address.scope_id as u32 {
                        return false;
                    }
                    if wanted == ip {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    })
}

fn index_of(name: &str) -> i32 {
    let Ok(name) = CString::new(name) else {
        return -1;
    };
    match unsafe { libc::if_nametoindex(name.as_ptr()) } {
        0 => -1,
        index => index as i32,
    }
}

/// HotSpot's `getFlags0`.
fn flags_of(name: &str) -> SocketResult<libc::c_int> {
    let socket = open_socket_with_fallback()?;
    get_flags(&socket, name).map_err(|errno| os_error(errno, "getFlags() failed"))
}

fn get_flags(socket: &OwnedFd, name: &str) -> Result<libc::c_int, libc::c_int> {
    let mut request = interface_request(name);
    ioctl(socket, SIOCGIFFLAGS, &mut request)?;
    Ok(unsafe { request.ifr_ifru.ifru_flags } as libc::c_int & 0xffff)
}

/// HotSpot's `openSocket`: `None` when the address family isn't supported.
fn open_socket(family: libc::c_int) -> SocketResult<Option<OwnedFd>> {
    let fd = unsafe { libc::socket(family, libc::SOCK_DGRAM, 0) };
    if fd >= 0 {
        return Ok(Some(unsafe { OwnedFd::from_raw_fd(fd) }));
    }
    match errno() {
        libc::EPROTONOSUPPORT | libc::EAFNOSUPPORT => Ok(None),
        errno => Err(os_error(errno, "Socket creation failed")),
    }
}

/// HotSpot's `openSocketWithFallback`: an IPv4 socket, or an IPv6 one without IPv4 support.
fn open_socket_with_fallback() -> SocketResult<OwnedFd> {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };
    if fd >= 0 {
        return Ok(unsafe { OwnedFd::from_raw_fd(fd) });
    }
    match errno() {
        libc::EPROTONOSUPPORT | libc::EAFNOSUPPORT => {
            let fd = unsafe { libc::socket(libc::AF_INET6, libc::SOCK_DGRAM, 0) };
            if fd < 0 {
                return Err(os_error(errno(), "IPV6 Socket creation failed"));
            }
            Ok(unsafe { OwnedFd::from_raw_fd(fd) })
        }
        errno => Err(os_error(errno, "IPV4 Socket creation failed")),
    }
}

fn interface_request(name: &str) -> libc::ifreq {
    let mut request: libc::ifreq = unsafe { mem::zeroed() };
    for (target, &byte) in request
        .ifr_name
        .iter_mut()
        .zip(name.as_bytes().iter().take(libc::IFNAMSIZ - 1))
    {
        *target = byte as libc::c_char;
    }
    request
}

fn ioctl<T>(
    socket: &OwnedFd,
    request_code: libc::c_ulong,
    argument: &mut T,
) -> Result<(), libc::c_int> {
    match unsafe { libc::ioctl(socket.as_raw_fd(), request_code as _, argument as *mut T) } {
        -1 => Err(errno()),
        _ => Ok(()),
    }
}

fn errno() -> libc::c_int {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

/// A `SocketException` message with the description of `errno`, as `strerror` gives it.
fn os_error(errno: libc::c_int, operation: &str) -> SocketError {
    let description = std::io::Error::from_raw_os_error(errno).to_string();
    let description = description
        .strip_suffix(&format!(" (os error {errno})"))
        .map_or(description.clone(), str::to_string);
    SocketError::with_os_error(Some(description), operation)
}

#[cfg(target_os = "linux")]
fn ipv4_of(address: &libc::sockaddr) -> Ipv4Addr {
    let address = unsafe { &*(address as *const libc::sockaddr as *const libc::sockaddr_in) };
    Ipv4Addr::from(u32::from_be(address.sin_addr.s_addr))
}

/// HotSpot's `translateIPv4AddressToPrefix`.
fn ipv4_prefix(netmask: Ipv4Addr) -> i16 {
    match u32::from(netmask) {
        0 => 0,
        mask => (32 - mask.trailing_zeros()) as i16,
    }
}

/// HotSpot's `translateIPv6AddressToPrefix`, which gives 0 for a non-contiguous mask.
#[cfg(not(target_os = "linux"))]
fn ipv6_prefix(netmask: &[u8; 16]) -> i16 {
    let Some(byte) = netmask.iter().position(|&b| b != 0xff) else {
        return 128;
    };
    let mut prefix = 8 * byte as i16;
    let mut bit = 7;
    while bit != 0 && netmask[byte] & (1 << bit) != 0 {
        bit -= 1;
        prefix += 1;
    }
    if (1..=bit).rev().any(|bit| netmask[byte] & (1 << bit) != 0) {
        prefix = 0;
    }
    if prefix > 0 && netmask[byte + 1..].iter().any(|&b| b != 0) {
        prefix = 0;
    }
    prefix
}

/// The result of `getifaddrs`, freed on drop.
#[cfg(not(target_os = "linux"))]
struct InterfaceAddresses(*mut libc::ifaddrs);

#[cfg(not(target_os = "linux"))]
impl InterfaceAddresses {
    fn new() -> SocketResult<Self> {
        let mut addresses = std::ptr::null_mut();
        if unsafe { libc::getifaddrs(&mut addresses) } != 0 {
            return Err(os_error(errno(), "getifaddrs() failed"));
        }
        Ok(Self(addresses))
    }

    fn iter(&self) -> impl Iterator<Item = &libc::ifaddrs> {
        std::iter::successors(unsafe { self.0.as_ref() }, |entry| unsafe {
            entry.ifa_next.as_ref()
        })
    }
}

#[cfg(not(target_os = "linux"))]
impl Drop for InterfaceAddresses {
    fn drop(&mut self) {
        unsafe { libc::freeifaddrs(self.0) };
    }
}

/// Copies a socket address of `sa_len` bytes, which BSD may give shorter than its type (as for
/// netmasks), zero-filling the rest.
#[cfg(not(target_os = "linux"))]
unsafe fn read_sockaddr<T>(address: *const libc::sockaddr) -> Option<T> {
    if address.is_null() {
        return None;
    }
    let mut value = unsafe { mem::zeroed::<T>() };
    let length = (unsafe { (*address).sa_len } as usize).min(mem::size_of::<T>());
    unsafe {
        std::ptr::copy_nonoverlapping(
            address as *const u8,
            &mut value as *mut T as *mut u8,
            length,
        )
    };
    Some(value)
}
