use std::fs;

const PROC_CPUINFO_PATH: &str = "/proc/cpuinfo";

pub struct Cpu {
    pub vendor: String,
    pub model: String,
    pub core_count: usize,
}

impl Cpu {
    pub fn query() -> Vec<Self> {
        let mut cpus = Vec::new();
        let cpuinfo = fs::read_to_string(PROC_CPUINFO_PATH)
            .expect(format!("unable to find/read file {}", PROC_CPUINFO_PATH).as_str());
        let mut lines = cpuinfo.lines().filter(|line| {
            line.starts_with("processor")
                || line.starts_with("vendor_id")
                || line.starts_with("model name")
                || line.starts_with("physical id")
                || line.starts_with("cpu cores")
        });

        let mut cpu;
        for line in lines {
            if line.starts_with("processor") {
                cpu = Vec::new();
            } else if line.starts_with("vendor_id") {
            }
        }
    }
}

fn get_cpuinfo_value(line: &str) -> &str {
    let parts = line.split_once(':').unwrap_or_else(|| (line, line));
    parts.1
}
