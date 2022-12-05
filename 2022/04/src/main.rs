use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut count = 0;

        for line in lines {
            if let Ok(line_str) = line {
                let mut tokens = line_str.split(",");

                let first = tokens.next().unwrap();
                let second = tokens.next().unwrap();

                let mut first_range = first.split("-");
                let first_lower: i32 = first_range.next().unwrap().parse().unwrap();
                let first_upper: i32 = first_range.next().unwrap().parse().unwrap();

                let mut second_range = second.split("-");
                let second_lower: i32 = second_range.next().unwrap().parse().unwrap();
                let second_upper: i32 = second_range.next().unwrap().parse().unwrap();

                let first_tuple = (first_lower, first_upper);
                let second_tuple = (second_lower, second_upper);

                if range_contains(first_tuple, second_tuple) || range_contains(second_tuple, first_tuple) {
                    count += 1
                }
            }
        }

        println!("{}", count);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn range_contains(range: (i32, i32), subrange: (i32, i32)) -> bool {
    return range.0 <= subrange.0 && range.1 >= subrange.1
}
