//! `java.net.InetAddress` natives and the IP stack availability of HotSpot's `net_util.c`.

use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use once_cell::sync::OnceCell;

/// `java.net.InetAddress.init()V`, `java.net.Inet4Address.init()V` and `java.net.Inet6Address.init()V`
pub(crate) fn init() -> Result<()> {
    Ok(())
}

/// `java.net.InetAddress.isIPv4Available()Z`
pub(crate) fn is_ipv4_available() -> Result<bool> {
    Ok(availability()?.ipv4)
}

/// `java.net.InetAddress.isIPv6Supported()Z`
pub(crate) fn is_ipv6_supported() -> Result<bool> {
    ipv6_available()
}

/// Whether IPv6 is used: HotSpot decides it once from the platform support and
/// `java.net.preferIPv4Stack`.
pub(crate) fn ipv6_available() -> Result<bool> {
    Ok(availability()?.ipv6)
}

struct Availability {
    ipv4: bool,
    ipv6: bool,
}

/// HotSpot's net library works this out when it's loaded.
fn availability() -> Result<&'static Availability> {
    static AVAILABILITY: OnceCell<Availability> = OnceCell::new();
    AVAILABILITY.get_or_try_init(|| {
        let property_ref = StringPoolHelper::get_string("java.net.preferIPv4Stack")?;
        let prefer_ipv4_stack = Executor::invoke_static_method(
            "java/lang/Boolean",
            "getBoolean:(Ljava/lang/String;)Z",
            &[property_ref.into()],
        )?[0]
            != 0;
        Ok(Availability {
            ipv4: ipv4_supported(),
            ipv6: ipv6_supported() && !prefer_ipv4_stack,
        })
    })
}

#[cfg(unix)]
fn ipv4_supported() -> bool {
    can_create_stream_socket(nix::libc::AF_INET)
}

#[cfg(unix)]
fn ipv6_supported() -> bool {
    // Linux also needs an interface with an IPv6 address
    can_create_stream_socket(nix::libc::AF_INET6)
        && (cfg!(not(target_os = "linux"))
            || std::fs::read_to_string("/proc/net/if_inet6")
                .is_ok_and(|content| !content.is_empty()))
}

#[cfg(unix)]
fn can_create_stream_socket(family: nix::libc::c_int) -> bool {
    use std::os::fd::{FromRawFd, OwnedFd};
    let fd = unsafe { nix::libc::socket(family, nix::libc::SOCK_STREAM, 0) };
    if fd < 0 {
        return false;
    }
    drop(unsafe { OwnedFd::from_raw_fd(fd) });
    true
}

// HotSpot checks that a socket of the family can be created, which binding one shows too
#[cfg(windows)]
fn ipv4_supported() -> bool {
    std::net::UdpSocket::bind((std::net::Ipv4Addr::LOCALHOST, 0)).is_ok()
}

#[cfg(windows)]
fn ipv6_supported() -> bool {
    std::net::UdpSocket::bind((std::net::Ipv6Addr::LOCALHOST, 0)).is_ok()
}
