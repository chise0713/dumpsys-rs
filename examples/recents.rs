use std::collections::BTreeSet;

use dumpsys_rs::Dumpsys;

fn main() {
    let mut dumpsys = Dumpsys::new().unwrap();
    dumpsys.insert_service("activity").unwrap();
    let dump = dumpsys.dump("activity", &["recents"]).unwrap();
    let packages: BTreeSet<_> = dump
        .split("Visible recent tasks")
        .nth(1)
        .map(|t| {
            t.split("* RecentTaskInfo")
                .skip(1)
                .filter_map(|task| task.split("baseIntent=Intent").nth(1))
                .filter_map(|s| s.split("cmp=").nth(1))
                .filter_map(|s| s.split_once('/').map(|(pkg, _)| pkg.trim()))
                .collect()
        })
        .unwrap_or_default();
    println!("{:?}", packages);
}
