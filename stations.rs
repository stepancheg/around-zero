use geo::*;
use misc::*;

struct Station {
    id: ~str,
    coord: Coord,
    name: ~str,
}

impl ToStr for Station {
    fn to_str(&self) -> ~str {
        let mut s = ~"";
        s += self.id;
        s += " ";
        s += self.coord.to_str();
        s += " ";
        s += self.name;
        s
    }
}

impl Station {
    fn parse(line: &str) -> Station {
        assert!(line.len() == 85);
        assert!(line[11] == ' ' as u8);
        let id = line.slice(0, 11);

        let parts = words(line);
        assert!(parts.len() >= 5);

        let lat = f32::from_str(parts[1]).get();
        let long = f32::from_str(parts[2]).get();
        let coord = Coord { lat: lat, long: long };

        assert!(line[40] == ' ' as u8);
        let tail = line.slice(41, line.len());
        assert!(tail[0] != ' ' as u8);

        let doublespace = str::find_str(tail, "  ").get();
        let name = tail.slice(0, doublespace);

        Station { id: id.to_owned(), coord: coord, name: name.to_owned(), }
    }
}

pub fn load_stations() -> ~[Station] {
    let reader = io::file_reader(~path::Path("ghcnd-stations.txt")).unwrap();
    vec::map(reader.read_lines(), |&line| Station::parse(line))
}

