use misc::*;

struct DailyRecord {
    stationId: ~str,
    date: ~str,
    kind: ~str,
    value: ~str,
}

impl ToStr for DailyRecord {
    fn to_str(&self) -> ~str {
        self.stationId + "," + self.date + "," + self.kind + "," + self.value
    }
}

impl DailyRecord {
    pub fn parse(line: &str) -> DailyRecord {
        let parts = split_char(line, ',');
        DailyRecord {
            stationId: parts[0].to_owned(),
            date: parts[1].to_owned(),
            kind: parts[2].to_owned(),
            value: parts[3].to_owned(),
        }
    }
}

pub fn load_daily_from_file(file: &path::Path) -> ~[DailyRecord] {
    let mut r: ~[DailyRecord] = ~[];
    let reader = io::file_reader(file).unwrap();
    reader.each_line(|line| { r.push(DailyRecord::parse(line)); true });
    r
}
