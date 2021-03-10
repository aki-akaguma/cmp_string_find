pub fn do_match_string_std(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        match line.find(pattern) {
            Some(_n) => found += 1,
            None => (),
        }
    }
    Ok(found)
}

pub fn do_match_string_twoway(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        match twoway::find_str(line, pattern) {
            Some(_n) => found += 1,
            None => (),
        }
    }
    Ok(found)
}

pub fn do_match_string_memchr(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        match memchr_find_str(line, pattern) {
            Some(_n) => found += 1,
            None => (),
        }
    }
    Ok(found)
}

fn memchr_find_str(haystack: &str, needle: &str) -> Option<usize> {
    let hay_bytes = haystack.as_bytes();
    let pat_bytes = needle.as_bytes();
    let hay_len = hay_bytes.len();
    let pat_len = pat_bytes.len();
    //
    // utf8 byte sequence:
    //     1 bytes: 7F
    //     2 bytes: DF BF
    //     3 bytes: EF BF BF
    //     4 bytes: F4 8F BF BF
    // 1st byte (of 2..4 bytes seq) is likely to be repeated.
    // I think it is stochastically effective to use the last byte.
    //
    let last_idx = pat_len - 1;
    let last_byte = pat_bytes[last_idx];
    if hay_len <= last_idx {
        return None;
    }
    for m in memchr::memchr_iter(last_byte, &hay_bytes[last_idx..]) {
        let st = m;
        let ed = st + pat_len;
        if ed >= hay_len {
            break;
        }
        if pat_bytes == &hay_bytes[st..ed] {
            return Some(st);
        }
    }
    None
}

pub fn do_match_string_memmem(
    texts: &[&str],
    pat: &memmem::TwoWaySearcher,
) -> anyhow::Result<usize> {
    use memmem::Searcher;
    let mut found: usize = 0;
    for line in texts {
        match pat.search_in(line.as_bytes()) {
            Some(_n) => found += 1,
            None => (),
        }
    }
    Ok(found)
}

pub fn do_match_string_aho(
    texts: &[&str],
    pat: &aho_corasick::AhoCorasick,
) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        match pat.find(line) {
            Some(_n) => found += 1,
            None => (),
        }
    }
    Ok(found)
}

pub fn do_match_regex_regex(texts: &[&str], re: &regex::Regex) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        /*
        if re.is_match(line) {
            found += 1;
        }
        */
        match re.find(line) {
            Some(_m) => found += 1,
            None => (),
        }
    }
    Ok(found)
}

pub fn do_match_regex_fancy(texts: &[&str], re: &fancy_regex::Regex) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        /*
        if re.is_match(line).unwrap() {
            found += 1;
        }
        */
        match re.find(line).unwrap() {
            Some(_m) => found += 1,
            None => (),
        }
    }
    Ok(found)
}

pub fn do_match_regex_onig(texts: &[&str], re: &onig::Regex) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        /*
        if re.is_match(line) {
            found += 1;
        }
        */
        match re.find(line) {
            Some(_m) => found += 1,
            None => (),
        }
    }
    Ok(found)
}

pub fn do_match_regex_pcre(texts: &[&str], re: &pcre2::bytes::Regex) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        /*
        if re.is_match(line).unwrap() {
            found += 1;
        }
        */
        match re.find(line.as_bytes()).unwrap() {
            Some(_m) => found += 1,
            None => (),
        }
    }
    Ok(found)
}

pub fn do_match_glob_glob(texts: &[&str], pat: &glob::Pattern) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if pat.matches(line) {
            found += 1;
        }
    }
    Ok(found)
}

pub fn do_match_glob_globber(texts: &[&str], pat: &globber::Pattern) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if pat.matches(line) {
            found += 1;
        }
    }
    Ok(found)
}

pub fn do_match_glob_capturing(
    texts: &[&str],
    pat: &capturing_glob::Pattern,
) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if pat.matches(line) {
            found += 1;
        }
    }
    Ok(found)
}

pub fn do_match_glob_globset(texts: &[&str], pat: &globset::GlobMatcher) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for line in texts {
        if pat.is_match(line) {
            found += 1;
        }
    }
    Ok(found)
}
