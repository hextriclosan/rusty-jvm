//! Windows network interfaces, collected like HotSpot's `windows/native/libnet/NetworkInterface.c`
//! from the IP Helper interface and address tables.

use super::{JavaInetAddress, NetAddr, NetIf, SocketError, SocketResult};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ptr;
use winapi::shared::ifdef::{
    IfOperStatusUp, NET_IF_ACCESS_POINT_TO_POINT, NET_IF_ADMIN_STATUS_UP, NET_LUID,
};
use winapi::shared::ipifcons::IF_TYPE_SOFTWARE_LOOPBACK;
use winapi::shared::netioapi::{
    ConvertInterfaceLuidToNameW, ConvertInterfaceNameToLuidW, FreeMibTable,
    GetAnycastIpAddressTable, GetIfEntry2, GetIfTable2, GetUnicastIpAddressTable,
    MIB_ANYCASTIPADDRESS_ROW, MIB_ANYCASTIPADDRESS_TABLE, MIB_IF_ROW2, MIB_IF_TABLE2,
    MIB_UNICASTIPADDRESS_ROW, MIB_UNICASTIPADDRESS_TABLE,
};
use winapi::shared::nldef::{IpDadStateDeprecated, IpDadStatePreferred};
use winapi::shared::ntddndis::NDIS_IF_MAX_STRING_SIZE;
use winapi::shared::winerror::{ERROR_FILE_NOT_FOUND, ERROR_INVALID_NAME, NO_ERROR};
use winapi::shared::ws2def::{ADDRESS_FAMILY, AF_INET, AF_UNSPEC};
use winapi::shared::ws2ipdef::SOCKADDR_INET;
use winapi::um::winbase::{
    FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
};

/// The prefix length HotSpot gives anycast addresses, which leaves the mask unset
const NO_PREFIX: u8 = 255;

pub(super) fn all(ipv6: bool) -> SocketResult<Vec<NetIf>> {
    let mut table = ptr::null_mut::<MIB_IF_TABLE2>();
    check(unsafe { GetIfTable2(&mut table) }, "GetIfTable2")?;
    let table = MibTable(table);
    let addresses = AddressTables::new(ipv6)?;
    let rows = unsafe {
        rows(
            ptr::addr_of!((*table.0).Table).cast(),
            (*table.0).NumEntries,
        )
    };
    rows.iter().map(|row| create(row, &addresses)).collect()
}

pub(super) fn by_name(name: &str, ipv6: bool) -> SocketResult<Option<NetIf>> {
    let mut row = unsafe { mem::zeroed::<MIB_IF_ROW2>() };
    let name = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
    match unsafe { ConvertInterfaceNameToLuidW(name.as_ptr(), &mut row.InterfaceLuid) } {
        NO_ERROR => {}
        ERROR_INVALID_NAME => return Ok(None),
        error => return Err(api_error(error, "ConvertInterfaceNameToLuidW")),
    }
    create_for_single_row(row, &AddressTables::new(ipv6)?)
}

pub(super) fn by_index(index: i32, ipv6: bool) -> SocketResult<Option<NetIf>> {
    if index == 0 {
        // never a valid index, and it would leave the row unset for GetIfEntry2
        return Ok(None);
    }
    let mut row = unsafe { mem::zeroed::<MIB_IF_ROW2>() };
    row.InterfaceIndex = index as u32;
    create_for_single_row(row, &AddressTables::new(ipv6)?)
}

pub(super) fn by_inet_address(
    address: JavaInetAddress,
    ipv6: bool,
) -> SocketResult<Option<NetIf>> {
    let addresses = AddressTables::new(ipv6)?;
    let Some(luid) = addresses.luid_of(address) else {
        return Ok(None);
    };
    let mut row = unsafe { mem::zeroed::<MIB_IF_ROW2>() };
    row.InterfaceLuid = luid;
    create_for_single_row(row, &addresses)
}

pub(super) fn is_bound(address: JavaInetAddress, ipv6: bool) -> SocketResult<bool> {
    Ok(AddressTables::new(ipv6)?.luid_of(address).is_some())
}

pub(super) fn is_up(_name: &str, index: i32) -> SocketResult<bool> {
    let row = row_of(index)?;
    Ok(row.AdminStatus == NET_IF_ADMIN_STATUS_UP && row.OperStatus == IfOperStatusUp)
}

pub(super) fn is_loopback(_name: &str, index: i32) -> SocketResult<bool> {
    Ok(row_of(index)?.Type == IF_TYPE_SOFTWARE_LOOPBACK)
}

pub(super) fn is_point_to_point(_name: &str, index: i32) -> SocketResult<bool> {
    Ok(row_of(index)?.AccessType == NET_IF_ACCESS_POINT_TO_POINT)
}

pub(super) fn supports_multicast(_name: &str, _index: i32) -> SocketResult<bool> {
    // HotSpot assumes multicast, as no API tells it reliably
    Ok(true)
}

pub(super) fn mac_address(_name: &str, index: i32) -> SocketResult<Option<Vec<u8>>> {
    let row = row_of(index)?;
    let length = row.PhysicalAddressLength as usize;
    Ok((length != 0).then(|| row.PhysicalAddress[..length].to_vec()))
}

pub(super) fn mtu(_name: &str, index: i32) -> SocketResult<i32> {
    Ok(row_of(index)?.Mtu as i32)
}

fn row_of(index: i32) -> SocketResult<MIB_IF_ROW2> {
    let mut row = unsafe { mem::zeroed::<MIB_IF_ROW2>() };
    row.InterfaceIndex = index as u32;
    check(unsafe { GetIfEntry2(&mut row) }, "GetIfEntry2")?;
    Ok(row)
}

/// HotSpot's `createNetworkInterfaceForSingleRowWithTables`: `None` for an unknown interface.
fn create_for_single_row(
    mut row: MIB_IF_ROW2,
    addresses: &AddressTables,
) -> SocketResult<Option<NetIf>> {
    match unsafe { GetIfEntry2(&mut row) } {
        NO_ERROR => create(&row, addresses).map(Some),
        ERROR_FILE_NOT_FOUND => Ok(None),
        error => Err(api_error(error, "GetIfEntry2")),
    }
}

/// HotSpot's `createNetworkInterface`.
fn create(row: &MIB_IF_ROW2, addresses: &AddressTables) -> SocketResult<NetIf> {
    let mut name = [0u16; NDIS_IF_MAX_STRING_SIZE + 1];
    check(
        unsafe { ConvertInterfaceLuidToNameW(&row.InterfaceLuid, name.as_mut_ptr(), name.len()) },
        "ConvertInterfaceLuidToNameW",
    )?;

    // HotSpot gathers the addresses into a list by adding each to its head
    let mut interface_addresses = Vec::new();
    for unicast in addresses.unicast() {
        if unicast.InterfaceLuid.Value == row.InterfaceLuid.Value
            && (unicast.DadState == IpDadStatePreferred
                || unicast.DadState == IpDadStateDeprecated)
        {
            interface_addresses.insert(0, address(&unicast.Address, unicast.OnLinkPrefixLength));
        }
    }
    for anycast in addresses.anycast() {
        if anycast.InterfaceLuid.Value == row.InterfaceLuid.Value {
            interface_addresses.insert(0, address(&anycast.Address, NO_PREFIX));
        }
    }

    Ok(NetIf {
        name: from_wide(&name),
        display_name: Some(from_wide(&row.Description)),
        index: row.InterfaceIndex as i32,
        // Windows has no virtual interfaces
        is_virtual: false,
        addresses: interface_addresses,
        children: Vec::new(),
    })
}

fn address(address: &SOCKADDR_INET, prefix_length: u8) -> NetAddr {
    let prefix = (prefix_length != NO_PREFIX).then_some(prefix_length);
    match ip_of(address) {
        IpAddr::V4(ip) => NetAddr {
            ip: IpAddr::V4(ip),
            scope_id: 0,
            broadcast: prefix.map(|prefix| {
                let mask = u32::MAX.checked_shl(32 - prefix as u32).unwrap_or(0);
                Ipv4Addr::from(u32::from(ip) | !mask)
            }),
            prefix: prefix.map(i16::from),
        },
        IpAddr::V6(ip) => NetAddr {
            ip: IpAddr::V6(ip),
            scope_id: unsafe { *address.Ipv6().u.sin6_scope_id() } as i32,
            broadcast: None,
            prefix: prefix.map(i16::from),
        },
    }
}

fn ip_of(address: &SOCKADDR_INET) -> IpAddr {
    unsafe {
        if *address.si_family() as i32 == AF_INET {
            IpAddr::V4(Ipv4Addr::from(u32::from_be(
                *address.Ipv4().sin_addr.S_un.S_addr(),
            )))
        } else {
            IpAddr::V6(Ipv6Addr::from(*address.Ipv6().sin6_addr.u.Byte()))
        }
    }
}

/// HotSpot's `NET_SockaddrEqualsInetAddress`.
fn equals(address: &SOCKADDR_INET, wanted: JavaInetAddress) -> bool {
    match (ip_of(address), wanted) {
        (IpAddr::V6(ip), JavaInetAddress::V4(wanted)) => ip.to_ipv4_mapped() == Some(wanted),
        (IpAddr::V6(ip), JavaInetAddress::V6(wanted, scope_id)) => {
            ip.to_ipv4_mapped().is_none()
                && ip == wanted
                && unsafe { *address.Ipv6().u.sin6_scope_id() } == scope_id
        }
        (IpAddr::V4(ip), JavaInetAddress::V4(wanted)) => ip == wanted,
        (IpAddr::V4(_), JavaInetAddress::V6(..)) => false,
    }
}

/// The unicast and anycast address tables, as HotSpot's `getAddressTables` gets them.
struct AddressTables {
    unicast: MibTable<MIB_UNICASTIPADDRESS_TABLE>,
    anycast: MibTable<MIB_ANYCASTIPADDRESS_TABLE>,
}

impl AddressTables {
    fn new(ipv6: bool) -> SocketResult<Self> {
        let family = if ipv6 { AF_UNSPEC } else { AF_INET } as ADDRESS_FAMILY;
        let mut unicast = ptr::null_mut();
        check(
            unsafe { GetUnicastIpAddressTable(family, &mut unicast) },
            "GetUnicastIpAddressTable",
        )?;
        let unicast = MibTable(unicast);
        let mut anycast = ptr::null_mut();
        check(
            unsafe { GetAnycastIpAddressTable(family, &mut anycast) },
            "GetAnycastIpAddressTable",
        )?;
        Ok(Self {
            unicast,
            anycast: MibTable(anycast),
        })
    }

    fn unicast(&self) -> &[MIB_UNICASTIPADDRESS_ROW] {
        unsafe {
            rows(
                ptr::addr_of!((*self.unicast.0).Table).cast(),
                (*self.unicast.0).NumEntries,
            )
        }
    }

    fn anycast(&self) -> &[MIB_ANYCASTIPADDRESS_ROW] {
        unsafe {
            rows(
                ptr::addr_of!((*self.anycast.0).Table).cast(),
                (*self.anycast.0).NumEntries,
            )
        }
    }

    /// The interface of the first usable unicast, or else anycast, address equal to `wanted`.
    fn luid_of(&self, wanted: JavaInetAddress) -> Option<NET_LUID> {
        self.unicast()
            .iter()
            .find(|unicast| {
                equals(&unicast.Address, wanted)
                    && (unicast.DadState == IpDadStatePreferred
                        || unicast.DadState == IpDadStateDeprecated)
            })
            .map(|unicast| unicast.InterfaceLuid)
            .or_else(|| {
                self.anycast()
                    .iter()
                    .find(|anycast| equals(&anycast.Address, wanted))
                    .map(|anycast| anycast.InterfaceLuid)
            })
    }
}

/// A table allocated by the IP Helper API, freed on drop.
struct MibTable<T>(*mut T);

impl<T> Drop for MibTable<T> {
    fn drop(&mut self) {
        unsafe { FreeMibTable(self.0.cast()) };
    }
}

/// The `count` rows of a table, which declares them as a one-row array.
unsafe fn rows<'a, T>(first: *const T, count: u32) -> &'a [T] {
    unsafe { std::slice::from_raw_parts(first, count as usize) }
}

fn check(status: u32, operation: &str) -> SocketResult<()> {
    match status {
        NO_ERROR => Ok(()),
        error => Err(api_error(error, operation)),
    }
}

/// HotSpot's `NET_ThrowByNameWithLastError` message: the system description of the error
/// without its final period and line break, followed by the failed operation.
fn api_error(error: u32, operation: &str) -> SocketError {
    let mut buffer = [0u16; 256];
    let length = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null(),
            error,
            0,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            ptr::null_mut(),
        )
    } as usize;
    let mut description = &buffer[..length];
    if description.len() > 3 {
        for suffix in ['\n', '\r', '.'] {
            if description.last() == Some(&(suffix as u16)) {
                description = &description[..description.len() - 1];
            }
        }
    }
    SocketError::with_os_error(Some(String::from_utf16_lossy(description)), operation)
}

fn from_wide(wide: &[u16]) -> String {
    let length = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..length])
}
