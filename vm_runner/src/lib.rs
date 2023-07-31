// copyright 2023 Remi Bernotavicius

use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::Command;

struct Qmp(TcpStream);

impl Qmp {
    fn new(stream: TcpStream) -> Self {
        let self_ = Self(stream);
        self_.qmp().handshake().unwrap();
        self_
    }

    fn qmp(&self) -> qapi::Qmp<qapi::Stream<std::io::BufReader<&TcpStream>, &TcpStream>> {
        qapi::Qmp::from_stream(&self.0)
    }

    fn execute<C: qapi::Command>(&self, command: &C) -> qapi::ExecuteResult<C> {
        self.qmp().execute(command)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ForwardedPort {
    pub host: u16,
    pub guest: u16,
}

impl ForwardedPort {
    fn parse_many(input: &str) -> Vec<Self> {
        let mut lines = input.lines();

        // hub header
        lines.next().unwrap();

        // column header
        lines.next().unwrap();

        // 0: Protocol[State] 1: FD 2: Source Address 3: Port 4: Dest. Address 5: Port
        let mut output = vec![];
        for line in lines {
            let columns = Vec::from_iter(line.split_ascii_whitespace());
            if columns[0] != "TCP[HOST_FORWARD]" {
                continue;
            }
            output.push(Self {
                host: columns[3].parse().unwrap(),
                guest: columns[5].parse().unwrap(),
            });
        }
        output
    }
}

#[test]
fn parse_forwarded_ports_test() {
    let input = "Hub -1 (network0):\r\n\
     Protocol[State]    FD  Source Address  Port   Dest. Address  Port RecvQ SendQ\r\n\
     TCP[HOST_FORWARD]  13               *  8080       10.0.2.15   111     0     0\r\n\
     TCP[HOST_FORWARD]  14               *    80       10.0.2.15    77     0     0\r\n";
    let parsed = ForwardedPort::parse_many(input);
    assert_eq!(
        parsed,
        vec![
            ForwardedPort {
                host: 8080,
                guest: 111
            },
            ForwardedPort {
                host: 80,
                guest: 77
            }
        ]
    );
}

pub struct Machine {
    proc: rexpect::session::PtySession,
    forwarded_ports: Vec<ForwardedPort>,
    #[allow(dead_code)]
    work_dir: tempdir::TempDir,
}

impl Machine {
    fn new(cd_rom_image: Option<&str>, overlay: bool, boot_img: &str) -> Self {
        let work_dir = tempdir::TempDir::new("vm_runner").unwrap();

        let hda = if overlay {
            let overlay_boot_img_path = work_dir.path().join("boot_overlay.qcow2");
            let overlay_boot_img = overlay_boot_img_path.to_str().unwrap();
            sh([
                "qemu-img",
                "create",
                "-b",
                std::fs::canonicalize(boot_img).unwrap().to_str().unwrap(),
                "-F",
                "qcow2",
                "-f",
                "qcow2",
                overlay_boot_img,
            ]);
            format!("-hda {overlay_boot_img}")
        } else {
            format!("-hda {boot_img}")
        };

        let monitor_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let monitor_addr = monitor_listener.local_addr().unwrap();

        let mut args = vec![
            "qemu-system-x86_64".into(),
            "-enable-kvm".into(),
            "-m 4G".into(),
            "-cpu host".into(),
            "-smp 12".into(),
            "-nographic".into(),
            format!("-qmp tcp:{monitor_addr}"),
            "-netdev user,hostfwd=tcp::0-:111,id=network0".into(),
            "-device e1000,netdev=network0".into(),
            hda,
        ];

        if let Some(img) = cd_rom_image {
            args.push(format!("-cdrom {img}"));
        }

        let qemu_cmd = args.join(" ");
        let proc = rexpect::spawn(&qemu_cmd, Some(30_000)).unwrap();

        let (monitor_socket, _) = monitor_listener.accept().unwrap();
        let qmp = Qmp::new(monitor_socket);

        let res = qmp
            .execute(&qapi::qmp::human_monitor_command {
                cpu_index: None,
                command_line: "info usernet".into(),
            })
            .unwrap();
        let forwarded_ports: Vec<ForwardedPort> = ForwardedPort::parse_many(&res);

        Self {
            proc,
            forwarded_ports,
            work_dir,
        }
    }

    fn log_in(&mut self) {
        self.proc.exp_string("alpine login: ").unwrap();
        self.proc.send_line("root").unwrap();

        self.proc.exp_string("Password: ").unwrap();
        self.proc.send_line("a").unwrap();

        log::info!("logged in");
    }

    fn run_command(&mut self, cmd: &str) {
        self.proc.exp_string("alpine:~# ").unwrap();
        self.proc.send_line(cmd).unwrap();
    }

    pub fn forwarded_ports(&self) -> &[ForwardedPort] {
        &self.forwarded_ports[..]
    }
}

impl Drop for Machine {
    fn drop(&mut self) {
        log::info!("powering off");
        self.run_command("poweroff");
        self.proc.process.wait().unwrap();
    }
}

fn answers() -> HashMap<&'static str, &'static str> {
    let mut d = HashMap::new();

    // Example answer file for setup-alpine script
    // If you don't want to use a certain option, then comment it out
    //
    // Use US layout with US variant
    // KEYMAPOPTS="us us"
    d.insert("KEYMAPOPTS", "none");

    // Set hostname to 'alpine'
    d.insert("HOSTNAMEOPTS", "alpine");

    // Set device manager to mdev
    d.insert("DEVDOPTS", "mdev");

    // Contents of /etc/network/interfaces
    d.insert(
        "INTERFACESOPTS",
        "auto lo\n\
         iface lo inet loopback\n\
         \n\
         auto eth0\n\
         iface eth0 inet dhcp\n\
         hostname alpine-test\
        ",
    );

    // Search domain of example.com, Google public nameserver
    // DNSOPTS="-d example.com 8.8.8.8"

    // Set timezone to UTC
    // TIMEZONEOPTS="UTC"
    d.insert("TIMEZONEOPTS", "none");

    // set http/ftp proxy
    // PROXYOPTS="http://webproxy:8080"
    d.insert("PROXYOPTS", "none");

    // Add first mirror (CDN)
    d.insert("APKREPOSOPTS", "-1");

    // Create admin user
    d.insert("USEROPTS", "-a -u -g audio,video,netdev juser");
    // USERSSHKEY="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOIiHcbg/7ytfLFHUNLRgEAubFz/13SwXBOM/05GNZe4 juser@example.com"
    // USERSSHKEY="https://example.com/juser.keys"

    // Install Openssh
    d.insert("SSHDOPTS", "openssh");
    // ROOTSSHKEY="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOIiHcbg/7ytfLFHUNLRgEAubFz/13SwXBOM/05GNZe4 juser@example.com"
    // ROOTSSHKEY="https://example.com/juser.keys"

    // Use openntpd
    // NTPOPTS="openntpd"
    d.insert("NTPOPTS", "none");

    // Use /dev/sda as a sys disk
    d.insert("DISKOPTS", "-m sys /dev/sda");

    // Setup storage with label APKOVL for config storage
    // LBUOPTS="LABEL=APKOVL"
    d.insert("LBUOPTS", "none");

    // APKCACHEOPTS="/media/LABEL=APKOVL/cache"
    d.insert("APKCACHEOPTS", "none");

    d
}

fn sh<'a>(cmd: impl IntoIterator<Item = &'a str>) {
    let cmd: Vec<&str> = cmd.into_iter().collect();

    let status = Command::new(cmd[0]).args(&cmd[1..]).status().unwrap();

    assert!(status.success(), "{cmd:?} failed with status: {status:?}");
}

const ALPINE: &'static str =
    "https://dl-cdn.alpinelinux.org/alpine/v3.18/releases/x86_64/alpine-standard-3.18.2-x86_64.iso";

fn download<Url>(url: Url, dst: impl AsRef<Path>)
where
    Url: TryInto<http_io::url::Url>,
    <Url as TryInto<http_io::url::Url>>::Error: std::fmt::Display,
{
    let mut body = http_io::client::get(url).unwrap();
    let mut file = std::fs::File::create(dst).unwrap();
    std::io::copy(&mut body, &mut file).unwrap();
}

fn create_disk(path: &str) {
    sh(["qemu-img", "create", "-f", "qcow2", path, "1G"]);
}

fn install_alpine(m: &mut Machine) {
    m.proc.exp_string("localhost login: ").unwrap();
    m.proc.send_line("root").unwrap();
    log::info!("logged in");

    m.proc.exp_string("localhost:~# ").unwrap();

    m.proc.send_line("tee answers.txt <<EOF").unwrap();
    for (k, v) in answers() {
        m.proc.send_line(&format!("{k}=\"{v}\"")).unwrap();
    }
    m.proc.send_line("EOF").unwrap();

    log::info!("uploaded answers.txt");

    m.proc.exp_string("localhost:~# ").unwrap();
    m.proc.send_line("setup-alpine -f answers.txt").unwrap();
    log::info!("starting set-up");

    m.proc.exp_string("New password: ").unwrap();
    m.proc.send_line("a").unwrap();

    m.proc.exp_string("Retype password: ").unwrap();
    m.proc.send_line("a").unwrap();

    log::info!("erasing disk");
    m.proc
        .exp_string("WARNING: Erase the above disk(s) and continue? (y/n) [n] ")
        .unwrap();
    m.proc.send_line("y").unwrap();
}

fn install_packages(m: &mut Machine) {
    m.run_command("apk add nfs-utils");
    m.run_command("rc-update add nfs");
    m.run_command("rc-service nfs start");

    log::info!("NFS installed");
}

pub fn create_image(boot_image: impl AsRef<Path>) {
    let boot_image = boot_image.as_ref().to_str().unwrap();

    let install_image = format!("{boot_image}.installer.iso");
    download(ALPINE, &install_image);

    create_disk(boot_image);

    let mut m = Machine::new(Some(&install_image), false /* overlay */, boot_image);
    log::info!("booting VM");

    install_alpine(&mut m);
    drop(m);

    let mut m = Machine::new(None, false /* overlay */, boot_image);
    m.log_in();

    install_packages(&mut m);

    std::fs::remove_file(install_image).unwrap();
}

pub fn run_vm(boot_image: impl AsRef<Path>, body: impl FnOnce(&Machine)) {
    let boot_image = boot_image.as_ref().to_str().unwrap();
    let mut m = Machine::new(None, true /* overlay */, boot_image);
    m.log_in();
    body(&m)
}
