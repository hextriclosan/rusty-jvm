use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use indexmap::IndexMap;
use std::net::IpAddr;

/// `java.net.NetworkInterface.init()V`
pub(crate) fn init() -> Result<()> {
    Ok(()) // todo: implement me
}

/// `java.net.NetworkInterface.getAll()[Ljava/net/NetworkInterface;`
pub(crate) fn get_all() -> Result<i32> {
    let mut grouped = IndexMap::<String, (i32, Vec<i32>)>::new();
    for interface in if_addrs::get_if_addrs()? {
        let next_index = grouped.len() as i32 + 1;
        let index = interface
            .index
            .map(|value| value as i32)
            .unwrap_or(next_index);
        let address = create_inet_address(interface.ip(), index)?;
        grouped
            .entry(interface.name)
            .or_insert_with(|| (index, Vec::new()))
            .1
            .push(address);
    }

    let mut interfaces = Vec::with_capacity(grouped.len());
    for (name, (index, addresses)) in grouped {
        let name_ref = StringPoolHelper::get_string(&name)?;
        let addresses_ref = HEAP.create_array_with_values("[Ljava/net/InetAddress;", &addresses);
        interfaces.push(Executor::invoke_args_constructor(
            "java/net/NetworkInterface",
            "<init>:(Ljava/lang/String;I[Ljava/net/InetAddress;)V",
            &[name_ref.into(), index.into(), addresses_ref.into()],
            Some("network interface creation"),
        )?);
    }

    Ok(HEAP.create_array_with_values("[Ljava/net/NetworkInterface;", &interfaces))
}

fn create_inet_address(address: IpAddr, scope_id: i32) -> Result<i32> {
    let (class_name, signature, octets, scope) = match address {
        IpAddr::V4(address) => (
            "java/net/Inet4Address",
            "<init>:(Ljava/lang/String;[B)V",
            address.octets().to_vec(),
            None,
        ),
        IpAddr::V6(address) => (
            "java/net/Inet6Address",
            "<init>:(Ljava/lang/String;[BI)V",
            address.octets().to_vec(),
            Some(scope_id),
        ),
    };
    let values = octets.into_iter().map(i32::from).collect::<Vec<_>>();
    let bytes_ref = HEAP.create_array_with_values("[B", &values);
    let mut arguments = vec![0.into(), bytes_ref.into()];
    if let Some(scope) = scope {
        arguments.push(scope.into());
    }
    Executor::invoke_args_constructor(
        class_name,
        signature,
        &arguments,
        Some("IP address creation"),
    )
}
