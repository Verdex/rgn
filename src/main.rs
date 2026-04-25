
use std::process::Command;

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("usage: rgn <pattern> <pattern>");
        return;
    }
    args.remove(0);

    let result = Command::new("rg").arg("-l").arg(args.remove(0)).output().expect("rg failed");
    let result_str = String::from_utf8(result.stdout).expect("failure converting output");
    let files = result_str.lines();

    let pattern2 = args.remove(0);
    for file in files {
        let result = Command::new("rg").arg(&pattern2).arg(file).output().expect("rg failed");
        let result_str = String::from_utf8(result.stdout).expect("failure converting output");
        println!("{}\n{}", file, result_str);
    }
}


