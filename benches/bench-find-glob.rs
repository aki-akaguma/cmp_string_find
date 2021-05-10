use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_glob_glob(texts: &[&str], pat: &glob::Pattern) -> anyhow::Result<usize> {
    cmp_string_find::do_find_glob_glob(texts, pat)
}

fn process_glob_globber(texts: &[&str], pat: &globber::Pattern) -> anyhow::Result<usize> {
    cmp_string_find::do_find_glob_globber(texts, pat)
}

fn process_glob_capturing(texts: &[&str], pat: &capturing_glob::Pattern) -> anyhow::Result<usize> {
    cmp_string_find::do_find_glob_capturing(texts, pat)
}

fn process_glob_globset(texts: &[&str], pat: &globset::GlobMatcher) -> anyhow::Result<usize> {
    cmp_string_find::do_find_glob_globset(texts, pat)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    let (v, match_cnt, _pat_string_s, _pat_regex_s, pat_glob_s) = create_data::create_data();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //let pattern = "**Error**";
    //let pattern = "*Error*";
    //let pattern = "*夏目漱石*";
    //
    let pat_glob = glob::Pattern::new(pat_glob_s).unwrap();
    let pat_globber = globber::Pattern::new(pat_glob_s).unwrap();
    let pat_capturing = capturing_glob::Pattern::new(pat_glob_s).unwrap();
    let pat_globset = globset::GlobBuilder::new(pat_glob_s)
        .build()
        .unwrap()
        .compile_matcher();
    //assert_eq!(pat_globset.glob().regex(), "");
    //
    match process_glob_glob(criterion::black_box(&vv), criterion::black_box(&pat_glob)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_glob_globber(
        criterion::black_box(&vv),
        criterion::black_box(&pat_globber),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_glob_capturing(
        criterion::black_box(&vv),
        criterion::black_box(&pat_capturing),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_glob_globset(
        criterion::black_box(&vv),
        criterion::black_box(&pat_globset),
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
    c.bench_function("cmp-glob-glob", |b| {
        b.iter(|| {
            let _r = process_glob_glob(criterion::black_box(&vv), criterion::black_box(&pat_glob));
        })
    });
    c.bench_function("cmp-glob-globber", |b| {
        b.iter(|| {
            let _r = process_glob_globber(
                criterion::black_box(&vv),
                criterion::black_box(&pat_globber),
            );
        })
    });
    c.bench_function("cmp-glob-capturing", |b| {
        b.iter(|| {
            let _r = process_glob_capturing(
                criterion::black_box(&vv),
                criterion::black_box(&pat_capturing),
            );
        })
    });
    c.bench_function("cmp-glob-globset", |b| {
        b.iter(|| {
            let _r = process_glob_globset(
                criterion::black_box(&vv),
                criterion::black_box(&pat_globset),
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
