use std::time::{Instant, Duration};

pub fn ops(nops: usize, threads: usize, mut f: impl FnMut(usize, usize)) {
    if threads != 1 {
        panic!("support for multiple threads is not supported yet");
    }
    let spread = if nops / threads * threads < nops {
        nops / threads + 1
    } else {
        nops / threads
    };
    let mut i = 0;
    let mut remaining = nops;
    let now = Instant::now();
    for _ in 0..threads {
        let take = if remaining > spread {
            spread
        } else {
            remaining
        };
        remaining -= take;
        for _ in 0..take {
            f(i, 0);
            i += 1;
        }
    }
    println!("{}", res_string(nops, threads, now.elapsed()));
}

fn commaize(num: u64) -> String {
    let s1: String = format!("{}", num);
    let b = s1.as_bytes();
    let mut s2 = String::new();
    let mut i = (s1.len() - 1) as isize;
    let mut j = 0;
    while i >= 0 {
        if j % 3 == 0 && j != 0 {
            s2 = ",".to_owned() + &s2;
        }
        s2 = format!("{}", b[i as usize] as char) + &s2;
        i -= 1;
        j += 1;
    }
    s2
}

pub fn res_string(nops: usize, threads: usize, elapsed: Duration) -> String {
    let nano = elapsed.as_nanos();
    let ms = elapsed.as_millis();
    let secs = nano as f64 / 1e9;
    let ops_sec = ((nops as f64) / secs) as u64;
    let nsop = (nano as f64 / nops as f64) as u64;

    let mut out = String::new();
    out += &format!("{} ops", commaize(nops as u64));
    if threads != 1 {
        out += &format!(" over {} threads", threads);
    }
    out += &format!(" in {}ms, {}/sec, {} ns/op", ms, commaize(ops_sec), nsop);
    out
}