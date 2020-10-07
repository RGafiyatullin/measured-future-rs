
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Report {
    pub time: Duration,
    pub polls: usize,
    pub children: HashMap<&'static str, Report>,
}

impl Report {
    pub(crate) fn dump(&self) -> () {
        dump(self, 0);
    }
}


fn dump(report: &Report, ident: usize) {
    let ident_s = String::from_utf8(vec![b' '; ident]).unwrap();
    println!("{}time:  {:?}", ident_s, report.time);
    println!("{}polls: {:?}", ident_s, report.polls);
    for (key, value) in &report.children {
        println!("{}\\_ {}", ident_s, key);
        dump(value, ident + 3);
    }
}
