use crate::ClientMeta;
use sysinfo::{System, SystemExt};

pub(crate) fn get_client_meta() -> ClientMeta {
    let mut sys = System::new_all();
    sys.refresh_all();

    ClientMeta {
        system: sys.name(),
        hostname: sys.host_name(),
        release: sys.os_version(),
        version: sys.kernel_version(),
        client: Some(format!("iron_planet {}", env!("CARGO_PKG_VERSION"))),
    }
}
