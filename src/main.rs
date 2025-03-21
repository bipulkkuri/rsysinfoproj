use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use colored::Colorize;
use figlet_rs::FIGfont;
use local_ip_address::local_ip;
use std::time::{Duration, UNIX_EPOCH};
use sysinfo::{Components, Disks, Networks, System};

extern crate term_size;
fn main() {
    display_banner();
    display_sys_info();
    display_resolution();
    display_cpu();
    display_proceess();
    display_disks();
    display_ip();
    //display_components_temperature();
}

fn display_banner() {
    let font = FIGfont::standard().expect("Failed to load default font");
    let word = System::distribution_id();
    // Generate and print ASCII art
    if let Some(art) = font.convert(&word) {
        println!("\n{}", art);
    } else {
        println!("Failed to generate ASCII art");
    }
}
fn display_sys_info() {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // Display system information:
    print_title("System Information");
    let hostname = System::host_name().unwrap_or_else(|| "<unknown>".to_owned());

    print_formatted("Hostname", &hostname);
    let did = System::distribution_id();
    print_formatted("Distribution", &did);

    let dname = System::name().unwrap_or_else(|| "<unknown>".to_owned());
    print_formatted("System Name", &dname);

    let kv = System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned());

    print_formatted("Kernel version", &kv);
    let osv = System::os_version().unwrap_or_else(|| "<unknown>".to_owned());

    print_formatted("OS version", &osv);
    // RAM and swap information:

    let um = sys.used_memory();
    let ums = ByteSize::b(um).display().si().to_string();

    let tm = sys.total_memory();
    let tms = ByteSize::b(tm).display().si().to_string();

    let mems = format!("{ums}/{tms}");

    print_formatted("RAM", &mems);

    let sm = sys.used_swap();
    let sms = ByteSize::b(sm).display().si().to_string();
    let tsm = sys.total_swap();
    let tsms = ByteSize::b(tsm).display().si().to_string();
    let swaps = format!("{sms}/{tsms}");
    print_formatted("Swap", &swaps);

    let timestamp_u64 = System::boot_time();
    let d = UNIX_EPOCH + Duration::from_secs(timestamp_u64);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    print_formatted("UpTime", &timestamp_str);
}
fn print_title(title: &str) {
    println!("{}", title.blue().bold());
    let len = title.len();
    println!("{}", "-".repeat(len).blue().bold());
}
fn print_formatted(title: &str, value: &str) {
    println!("{}: {}", title.red().bold(), value);
}
fn display_resolution() {
    let (w, h) = term_size::dimensions().expect("Failed to get terminal dimensions.");
    let resolution = format!("{w}x{h}");
    print_formatted("Resolution", &resolution);
}
fn display_ip() {
    let my_local_ip = local_ip().unwrap();
    let ip = format!("{my_local_ip}");
    print_formatted("IP", &ip);
}
fn display_proceess() {
    let mut sys = System::new_all();
    sys.refresh_all();
    //println!("Processes: {}", sys.processes().len());
    let bp = sys.processes().len().to_string();
    let ps = bp.as_str();
    let pnum = format!("{ps}");

    print_formatted("Processes", &pnum);
}

fn display_cpu() {
    let mut sys = System::new_all();
    sys.refresh_all();
    // CPU information:
    print_title("CPUs:");
    let bcpus = sys.cpus().len().to_string();
    let numcpus = bcpus.as_str();
    let scpus = format!("{numcpus}");
    print_formatted("CPUs", &scpus);
    /*
    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("CPU{}: {:?}", i, cpu);
    }*/
    // If you want the CPU temperature, use `cpu.temperature()`.
    // If you want the CPU frequency, use `cpu.frequency()`.
    // If you want the CPU usage, use `cpu.cpu_usage()`.
    // If you want the CPU user, system and idle time, use `cpu.times()`.
}

fn display_disks() {
    print_title("Disks:");
    // Disks information:
    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {
        let das = disk.available_space();
        let dass = ByteSize::b(das).display().si().to_string();
        let dts = disk.total_space();
        let dtss = ByteSize::b(dts).display().si().to_string();
        let dus = format!("{dass}/{dtss}");
        let dname = disk.name().to_str().unwrap();

        print_formatted(dname, &dus);
        // If you want the amount of data read/written since last call
        // to `Disks::refresh`, use `read_bytes`/`written_bytes`.
    }
}

fn display_networks() {
    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
        // If you want the amount of data received/transmitted since last call
        // to `Networks::refresh`, use `received`/`transmitted`.
    }
}
fn display_components_temperature() {
    // Components temperature:
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }
}
