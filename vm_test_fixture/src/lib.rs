use vm_runner::Machine;

pub fn fixture(ports: &[u16], body: impl FnOnce(&Machine)) {
    let boot_image = concat!(env!("OUT_DIR"), "/boot.qcow2");
    vm_runner::run_vm(boot_image, ports, body);
}
