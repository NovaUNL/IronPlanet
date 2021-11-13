use crate::ClientMeta;
use sysinfo::{System, SystemExt};


pub(crate) fn get_client_meta() -> Option<ClientMeta> {
    let mut sys = System::new_all();
    sys.refresh_all();

    Some(ClientMeta {
        system: sys.name().clone(),
        hostname: sys.host_name().clone(),
        release: sys.os_version().clone(),
        version: sys.kernel_version().clone(),
        client: Some(format!("iron_planet {}", env!("CARGO_PKG_VERSION"))),
    })
}
