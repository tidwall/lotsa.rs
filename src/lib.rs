pub fn ops(nops: u32, threads: u32, f: impl Fn(u32, u32)) {
    if threads != 1 {
        panic!("support for multiple threads is not supported yet");
    }
    let spread = if nops / threads * threads < nops {
        nops / threads + 1
    } else {
        nops / threads
    };
    let mut remaining = nops;
    let now = std::time::Instant::now();
    for _ in 0..threads {
        let take = if remaining > spread {
            spread
        } else {
            remaining
        };
        remaining -= take;
        for _ in 0..take {
            f(0, 0);
        }
    }
    let ms = now.elapsed().as_millis();
    let secs = (ms as f64) / 1000.0;
    let ops_sec = ((nops as f64) / secs) as u64;

    print!("{} ops", nops);
    if threads != 1 {
        print!(" over {} threads", threads);
    }
    println!(" in {} secs ({} ops/sec)", secs, commaize(ops_sec));
    println!("{}", ops_sec);
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