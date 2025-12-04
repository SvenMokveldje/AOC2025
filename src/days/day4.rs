use std::fs::read_to_string;

struct Point {
    y: usize,
    x: usize,
}

struct Wall {
    point: Point,
    char: char
}

const W: usize = 136;
const H: usize = 136;

pub fn day() {
    let input = read_to_string("src/inputs/day4.txt").unwrap();
    let mut wall: [[char; W]; H] = [[' '; W]; H];

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            wall[y][x] = ch as char;
        }
    }


    let total_result = second(wall);

    print!("{:?}\n", total_result);
}

fn first(input: [[char; W]; H]) -> i32{
    let mut complete_total = 0;
    for y in 0.. H {
        let mut x = 0;
        while x < W {
            if input[y][x] == '@' {
                let ch = |yy: usize, xx: usize| input[yy][xx] as char;
                const DIRS: [(isize, isize); 8] =[
                    (-1, -1), (-1, 0), (-1, 1),
                    ( 0, -1),          ( 0, 1),
                    ( 1, -1), ( 1, 0), ( 1, 1),
                ];

                let mut total = 0;
                for (dy, dx) in DIRS {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;

                    if ny < 0 || ny >= H as isize || nx < 0 || nx >= W as isize{
                        continue;
                    }

                    let adj = ch(ny as usize, nx as usize);
                    if adj == '@' {
                        total += 1;
                    }
                }
                if total < 4 {
                    complete_total += 1;
                    print!("{:?} {:?} \n", y , x)
                }
                
            }

            x += 1;
        }
    }
    return complete_total
}

fn second(mut input: [[char; W]; H]) -> i32{
    let mut completed = false;
    let mut complete_total: i32 = 0;

    let mut removals: [[bool; W]; H] = [[false; W]; H];

    while !completed {
        completed = true;
        for y in 0.. H {
        let mut x = 0;
        while x < W {
            if input[y][x] == '@' {
                let ch = |yy: usize, xx: usize| input[yy][xx] as char;
                const DIRS: [(isize, isize); 8] =[
                    (-1, -1), (-1, 0), (-1, 1),
                    ( 0, -1),          ( 0, 1),
                    ( 1, -1), ( 1, 0), ( 1, 1),
                ];

                let mut total = 0;
                for (dy, dx) in DIRS {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;

                    if ny < 0 || ny >= H as isize || nx < 0 || nx >= W as isize{
                        continue;
                    }

                    let adj = ch(ny as usize, nx as usize);
                    if adj == '@' {
                        total += 1;
                    }
                }
                if total < 4 {
                    complete_total += 1;
                    removals[y][x] = true;
                    completed = false;
                }
                
            }

            x += 1;
        }
    }

        for y in 0.. H {
            let mut x = 0;
            while x < W {
                if removals[y][x] {
                    input[y][x] = 'x';
                }

                x += 1;
            }
        }
    }
    
    return complete_total

}