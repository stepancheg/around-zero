
pub fn words(line: &str) -> ~[~str] {
    let mut r: ~[~str] = ~[];
    str::each_word(line, |item| { r.push(item.to_owned()); true });
    r
}

pub fn split_char(line: &str, sep: char) -> ~[~str] {
    let mut r: ~[~str] = ~[];
    str::each_split_char(line, sep, |item| { r.push(item.to_owned()); true });
    r
}

pub fn sqr(f: f32) -> f32 {
    f * f
}

