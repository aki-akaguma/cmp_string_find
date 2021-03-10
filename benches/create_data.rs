pub fn create_data() -> (Vec<String>, &'static str, &'static str, &'static str) {
    create_data_en()
    //create_data_ja()
}

pub fn create_data_en() -> (Vec<String>, &'static str, &'static str, &'static str) {
    let s1 =
        "abcdefghijk1234567890".repeat(10) + "ErrWarnAlert" + "abcdefghijklmno".repeat(10).as_str();
    let s2 = "abcdefghijk1234567890".repeat(10) + "abcdefghijklmno".repeat(10).as_str();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 300 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    (v, "ErrWarnAlert", "ErrWarnAlert", "*ErrWarnAlert*")
}

pub fn create_data_ja() -> (Vec<String>, &'static str, &'static str, &'static str) {
    let s1 = "吾輩は猫である".repeat(10) + "夏目漱石" + "坊っちゃん".repeat(10).as_str();
    let s2 = "名前はまだない".repeat(10) + "坊っちゃん".repeat(10).as_str();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 300 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    (v, "夏目漱石", "夏目漱石", "*夏目漱石*")
}
