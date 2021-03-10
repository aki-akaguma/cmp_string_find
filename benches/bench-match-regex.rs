use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_regex_regex(texts: &[&str], re: &regex::Regex) -> anyhow::Result<usize> {
    cmp_pattern_match::do_match_regex_regex(texts, re)
}

fn process_regex_fancy(texts: &[&str], re: &fancy_regex::Regex) -> anyhow::Result<usize> {
    cmp_pattern_match::do_match_regex_fancy(texts, re)
}

fn process_regex_onig(texts: &[&str], re: &onig::Regex) -> anyhow::Result<usize> {
    cmp_pattern_match::do_match_regex_onig(texts, re)
}

fn process_regex_pcre(texts: &[&str], re: &pcre2::bytes::Regex) -> anyhow::Result<usize> {
    cmp_pattern_match::do_match_regex_pcre(texts, re)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    let (v, _pat_string_s, pat_regex_s, _pat_glob_s) = create_data::create_data();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //let pattern = ".*Error.*";
    //let pattern = "Error";
    //let pattern = "夏目漱石";
    let match_cnt = vv.len() / 2;
    //
    let pat_regex = regex::Regex::new(pat_regex_s).unwrap();
    let pat_fancy = fancy_regex::Regex::new(pat_regex_s).unwrap();
    let pat_onig = onig::Regex::new(pat_regex_s).unwrap();
    let pat_pcre = pcre2::bytes::Regex::new(pat_regex_s).unwrap();
    //
    match process_regex_regex(criterion::black_box(&vv), criterion::black_box(&pat_regex)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_regex_fancy(criterion::black_box(&vv), criterion::black_box(&pat_fancy)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_regex_onig(criterion::black_box(&vv), criterion::black_box(&pat_onig)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_regex_pcre(criterion::black_box(&vv), criterion::black_box(&pat_pcre)) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    //
    c.bench_function("match-regex-regex", |b| {
        b.iter(|| {
            let _r =
                process_regex_regex(criterion::black_box(&vv), criterion::black_box(&pat_regex));
        })
    });
    c.bench_function("match-regex-fancy", |b| {
        b.iter(|| {
            let _r =
                process_regex_fancy(criterion::black_box(&vv), criterion::black_box(&pat_fancy));
        })
    });
    c.bench_function("match-regex-onig", |b| {
        b.iter(|| {
            let _r = process_regex_onig(criterion::black_box(&vv), criterion::black_box(&pat_onig));
        })
    });
    c.bench_function("match-regex-pcre", |b| {
        b.iter(|| {
            let _r = process_regex_pcre(criterion::black_box(&vv), criterion::black_box(&pat_pcre));
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = criterion_benchmark);
criterion_main!(benches);
