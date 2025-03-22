use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use colored::Colorize;
use figlet_rs::FIGfont;
use local_ip_address::local_ip;
use std::time::{Duration, UNIX_EPOCH};
use sysinfo::{Disks, System};

extern crate term_size;
fn main() {
    let left_column = vec![
        "Hostname",
        "System Name",
        "Distribution",
        "Kernel version",
        "OS version",
        "RAM",
        "Swap",
        "UpTime",
        "Resolution",
        "IP",
        "CPUs",
        "Processes",
    ];
    let right_column = get_sys_info();

    println!("\n");
    display_banner();
    for i in 0..right_column.len() {
        println!(
            "{:<20}: {:<30}",
            left_column[i].red().bold(),
            right_column[i]
        );
    }
    println!("\n");
}

fn get_sys_info() -> Vec<String> {
    let mut column = vec![];
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();
    let hostname = System::host_name().unwrap_or_else(|| "<unknown>".to_owned());
    column.push(hostname);
    let dname = System::name().unwrap_or_else(|| "<unknown>".to_owned());
    column.push(dname);
    let did = System::distribution_id();
    column.push(did);
    let kv = System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned());
    column.push(kv);
    let osv = System::os_version().unwrap_or_else(|| "<unknown>".to_owned());
    column.push(osv);
    let um = sys.used_memory();
    let ums = ByteSize::b(um).display().si().to_string();
    let tm = sys.total_memory();
    let tms = ByteSize::b(tm).display().si().to_string();
    let mems = format!("{ums}/{tms}");
    column.push(mems);
    let sm = sys.used_swap();
    let sms = ByteSize::b(sm).display().si().to_string();
    let tsm = sys.total_swap();
    let tsms = ByteSize::b(tsm).display().si().to_string();
    let swaps = format!("{sms}/{tsms}");
    column.push(swaps);
    let timestamp_u64 = System::boot_time();
    let d = UNIX_EPOCH + Duration::from_secs(timestamp_u64);
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    column.push(timestamp_str);
    let (w, h) = term_size::dimensions().expect("Failed to get terminal dimensions.");
    let resolution = format!("{w}x{h}");
    column.push(resolution);
    let my_local_ip = local_ip().unwrap();
    let ip = format!("{my_local_ip}");
    column.push(ip);
    let bcpus = sys.cpus().len().to_string();
    let numcpus = bcpus.as_str();
    let scpus = format!("{numcpus}");
    column.push(scpus);
    let bp = sys.processes().len().to_string();
    let ps = bp.as_str();
    let pnum = format!("{ps}");
    column.push(pnum);

    return column;
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
