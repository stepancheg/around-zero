
pub fn words(line: &str) -> ~[~str] {
    let mut r: ~[~str] = ~[];
    str::each_word(line, |item| { r.push(item.to_owned()); true });
    r
}

pub fn sqr(f: f32) -> f32 {
    f * f
}

