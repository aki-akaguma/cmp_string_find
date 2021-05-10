use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_string_std(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_find::do_find_string_std(texts, pattern)
}

fn process_string_twoway(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_find::do_find_string_twoway(texts, pattern)
}

fn process_string_memchr(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_find::do_find_string_memchr(texts, pattern)
}

fn process_string_memmem(texts: &[&str], pat: &memmem::TwoWaySearcher) -> anyhow::Result<usize> {
    cmp_string_find::do_find_string_memmem(texts, pat)
}

fn process_string_aho(texts: &[&str], pat: &aho_corasick::AhoCorasick) -> anyhow::Result<usize> {
    cmp_string_find::do_find_string_aho(texts, pat)
}

fn process_string_libc(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_string_find::do_find_string_libc(texts, pattern)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    let (v, match_cnt, pat_string_s, _pat_regex_s, _pat_glob_s) = create_data::create_data();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //let pattern = "Error";
    //let pattern = "夏目漱石";
    //
    let pat_aho = aho_corasick::AhoCorasick::new(&[pat_string_s]);
    let pat_memmem = memmem::TwoWaySearcher::new(pat_string_s.as_bytes());
    //
    match process_string_std(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_string_twoway(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_string_memchr(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_string_memmem(criterion::black_box(&vv), criterion::black_box(&pat_memmem)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_string_aho(criterion::black_box(&vv), criterion::black_box(&pat_aho)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_string_libc(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    //
    c.bench_function("cmp-string-std", |b| {
        b.iter(|| {
            let _r = process_string_std(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-string-twoway", |b| {
        b.iter(|| {
            let _r = process_string_twoway(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-string-memchr", |b| {
        b.iter(|| {
            let _r = process_string_memchr(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-string-memmem", |b| {
        b.iter(|| {
            let _r =
                process_string_memmem(criterion::black_box(&vv), criterion::black_box(&pat_memmem));
        })
    });
    c.bench_function("cmp-string-aho", |b| {
        b.iter(|| {
            let _r = process_string_aho(criterion::black_box(&vv), criterion::black_box(&pat_aho));
        })
    });
    c.bench_function("cmp-string-libc", |b| {
        b.iter(|| {
            let _r = process_string_libc(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte)
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(1500));
    targets = criterion_benchmark);
criterion_main!(benches);
