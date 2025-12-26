use sysinfo::DiskUsage;

/// Represents one process entry in your table.
#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: String,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
    pub disk_usage: DiskUsage,
}

/// Contains a collection of processes + helper methods.

#[derive(Clone)]
pub struct ProcessList {
    pub items: Vec<ProcessInfo>,
    pub sortdir: SortField,
}

#[derive(Clone)]
pub enum SortField {
    Default,
    Cpu,
    Mem,
    Name,
    Pid,
    Read,
    Write,
}

impl ProcessList {
    pub fn iter(&self) -> impl Iterator<Item = &ProcessInfo> {
        self.items.iter()
    }

    /*    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ProcessInfo> {
            self.items.iter_mut()
        }
    */
    
    pub fn new() -> Self {
        ProcessList {
            items: Vec::new(),
            sortdir: SortField::Default,
        }
    }

    pub fn refresh_from_sysinfo(&mut self, sys: &mut sysinfo::System) {
        self.items.clear();
        for (&pid, process) in sys.processes() {
            self.items.push(ProcessInfo {
                pid: pid.to_string(),
                name: process.name().to_string_lossy().into_owned(),
                cpu: process.cpu_usage(),
                mem: process.memory(),
                disk_usage: process.disk_usage(),
            });
        }
        match self.sortdir {
            SortField::Mem => self.sort_by_mem(),
            SortField::Cpu => self.sort_by_cpu(),
            SortField::Name => self.sort_by_name(),
            SortField::Pid => self.sort_by_pid(),
            SortField::Write => self.sort_by_disk_write(),
            SortField::Read => self.sort_by_disk_write(),
            SortField::Default => self.sort_by_pid(),
        }
    }

    pub fn sort_by_cpu(&mut self) {
        self.sortdir = SortField::Cpu;
        self.items.sort_by(|a, b| {
            b.cpu
                .partial_cmp(&a.cpu)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn sort_by_mem(&mut self) {
        self.sortdir = SortField::Mem;
        self.items.sort_by(|a, b| b.mem.cmp(&a.mem));
    }

    pub fn sort_by_pid(&mut self) {
        self.sortdir = SortField::Pid;
        self.items.sort_by(|a, b| a.pid.cmp(&b.pid));
    }

    pub fn sort_by_name(&mut self) {
        self.sortdir = SortField::Name;
        self.items.sort_by(|a, b| a.name.cmp(&b.name));
    }
    pub fn sort_by_disk_read(&mut self) {
        self.sortdir = SortField::Read;
        self.items
            .sort_by(|a, b| a.disk_usage.read_bytes.cmp(&b.disk_usage.read_bytes));
    }
    pub fn sort_by_disk_write(&mut self) {
        self.sortdir = SortField::Write;
        self.items
            .sort_by(|a, b| a.disk_usage.written_bytes.cmp(&b.disk_usage.written_bytes));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::DiskUsage;

    fn make_proc(pid: &str, name: &str, cpu: f32, mem: u64, read: u64, write: u64) -> ProcessInfo {
        let mut du = DiskUsage::default();
        du.read_bytes = read;
        du.written_bytes = write;
        ProcessInfo {
            pid: pid.to_string(),
            name: name.to_string(),
            cpu,
            mem,
            disk_usage: du,
        }
    }

    #[test]
    fn test_sort_by_cpu() {
        let mut list = ProcessList::new();
        list.items.push(make_proc("1", "one", 0.1, 10, 0, 0));
        list.items.push(make_proc("2", "two", 0.5, 20, 0, 0));
        list.items.push(make_proc("3", "three", 0.3, 30, 0, 0));
        list.sort_by_cpu();
        let cpus: Vec<f32> = list.items.iter().map(|p| p.cpu).collect();
        assert_eq!(cpus, vec![0.5, 0.3, 0.1]);
    }

    #[test]
    fn test_sort_by_mem() {
        let mut list = ProcessList::new();
        list.items.push(make_proc("1", "one", 0.1, 100, 0, 0));
        list.items.push(make_proc("2", "two", 0.2, 50, 0, 0));
        list.items.push(make_proc("3", "three", 0.3, 150, 0, 0));
        list.sort_by_mem();
        let mems: Vec<u64> = list.items.iter().map(|p| p.mem).collect();
        assert_eq!(mems, vec![150, 100, 50]);
    }

    #[test]
    fn test_sort_by_pid() {
        let mut list = ProcessList::new();
        list.items.push(make_proc("2", "two", 0.1, 10, 0, 0));
        list.items.push(make_proc("1", "one", 0.2, 20, 0, 0));
        list.items.push(make_proc("3", "three", 0.3, 30, 0, 0));
        list.sort_by_pid();
        let pids: Vec<&str> = list.items.iter().map(|p| p.pid.as_str()).collect();
        assert_eq!(pids, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_sort_by_name() {
        let mut list = ProcessList::new();
        list.items.push(make_proc("1", "b", 0.1, 10, 0, 0));
        list.items.push(make_proc("2", "a", 0.2, 20, 0, 0));
        list.items.push(make_proc("3", "c", 0.3, 30, 0, 0));
        list.sort_by_name();
        let names: Vec<&str> = list.items.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_sort_by_disk_read_write() {
        let mut list = ProcessList::new();
        list.items.push(make_proc("1", "a", 0.1, 10, 500, 100));
        list.items.push(make_proc("2", "b", 0.2, 20, 100, 700));
        list.items.push(make_proc("3", "c", 0.3, 30, 900, 200));
        list.sort_by_disk_read();
        let reads: Vec<u64> = list.items.iter().map(|p| p.disk_usage.read_bytes).collect();
        assert_eq!(reads, vec![100, 500, 900]);
        // After sorting by read, verify order is ascending by read_bytes
        assert!(list.items[0].disk_usage.read_bytes <= list.items[1].disk_usage.read_bytes);
        list.sort_by_disk_write();
        let writes: Vec<u64> = list.items.iter().map(|p| p.disk_usage.written_bytes).collect();
        assert_eq!(writes, vec![100, 200, 700]);
    }
}
