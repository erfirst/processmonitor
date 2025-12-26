use std::collections::VecDeque;

pub struct GraphDataPoint {
    pub cpu_usage: f32,
    pub mem_usage: f64,
}

pub struct GraphDataList {
    pub graph_data: VecDeque<GraphDataPoint>,
}

impl GraphDataList {
    pub fn new() -> Self {
        GraphDataList {
            graph_data: VecDeque::new(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &GraphDataPoint> {
        self.graph_data.iter()
    }

    /*   pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut GraphDataPoint> {
        self.graph_data.iter_mut()
    } */

    pub fn refresh_from_sysinfo(&mut self, sys: &mut sysinfo::System) {
        let newdatpoint = GraphDataPoint {
            cpu_usage: sys.global_cpu_usage(),
            mem_usage: (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0,
        };
        if self.graph_data.len() > 60 {
            self.graph_data.pop_front();
        }
        self.graph_data.push_back(newdatpoint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_new_is_empty() {
        let g = GraphDataList::new();
        assert_eq!(g.graph_data.len(), 0);
    }

    #[test]
    fn test_refresh_from_sysinfo_appends() {
        let mut g = GraphDataList::new();
        let mut sys = System::new_all();
        sys.refresh_all();
        g.refresh_from_sysinfo(&mut sys);
        assert_eq!(g.graph_data.len(), 1);
        let p = g.graph_data.front().unwrap();
        assert!(p.cpu_usage >= 0.0 && p.cpu_usage <= 100.0);
        assert!(p.mem_usage >= 0.0 && p.mem_usage <= 100.0);
    }

    #[test]
    fn test_capping_len() {
        let mut g = GraphDataList::new();
        let mut sys = System::new_all();
        // Call refresh > 61 times to trigger the cap behavior in refresh_from_sysinfo
        for _ in 0..62 {
            sys.refresh_all();
            g.refresh_from_sysinfo(&mut sys);
        }
        assert_eq!(g.graph_data.len(), 61);
    }
}
