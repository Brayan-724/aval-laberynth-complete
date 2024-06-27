use crate::map::Map;
use crate::Cell;

pub fn print_map(map: &Map) {
    let n = map.len();
    let mut s = String::with_capacity(n.pow(2) * 3);

    for (y, row) in map.iter_rows().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            s.push(match cell {
                Cell::Wall => {
                    if x == 0 || x == n - 1 {
                        '|'
                    } else if y == 0 || y == n - 1 {
                        '-'
                    } else {
                        'X'
                    }
                }
                Cell::Path => '-',
                Cell::Bot => ' ',
            });
            s.push(' ');
            // s.push('-');
        }

        s.push('\n');
    }

    print!("{s}\n\n")
}
