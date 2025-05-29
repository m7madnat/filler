use std::io::{ self, BufRead, Write };

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut anfield = vec![];
    let mut piece = vec![];
    let mut player = String::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if line.starts_with("$$$") {
            player = line.split_whitespace().nth(2).unwrap_or("p1").to_string();
        }

        if line.starts_with("Anfield") {
            let rows: usize = line
                .split_whitespace()
                .nth(2)
                .unwrap()
                .trim_end_matches(':')
                .parse()
                .unwrap();
            lines.next(); // skip 012345.. header
            anfield.clear();
            for _ in 0..rows {
                let row = lines.next().unwrap().unwrap();
                anfield.push(row[4..].chars().collect());
            }
        }

        if line.starts_with("Piece") {
            let piece_rows: usize = line
                .split_whitespace()
                .nth(2)
                .unwrap()
                .trim_end_matches(':')
                .parse()
                .unwrap();
            piece.clear();
            for _ in 0..piece_rows {
                piece.push(lines.next().unwrap().unwrap().chars().collect());
            }

            let (enemy1, enemy2, me1, me2) = if player == "p1" {
                ('s', '$', '@', 'a')
            } else {
                ('a', '@', 's', '$')
            };

            let valid = find_valid_positions(&anfield, &piece, enemy1, enemy2, me1, me2);

            if let Some((x, y)) = choose_best(valid, &anfield, enemy1, enemy2) {
                println!("{} {}", y, x);
            } else {
                println!("0 0");
            }
            io::stdout().flush().unwrap();
        }
    }
}

fn find_valid_positions(
    board: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    enemy1: char,
    enemy2: char,
    me1: char,
    me2: char
) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if is_valid(board, piece, x, y, enemy1, enemy2, me1, me2) {
                result.push((x, y));
            }
        }
    }
    result
}

fn is_valid(
    board: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    enemy1: char, //old p
    enemy2: char, // new p
    me1: char,
    me2: char
) -> bool {
    let mut touch = 0;
    for i in 0..piece.len() {
        for j in 0..piece[i].len() {
            if piece[i][j] == '.' {
                continue;
            }
            let bx = x + i;
            let by = y + j;
            if bx >= board.len() || by >= board[0].len() {
                return false;
            }
            let cell = board[bx][by];
            //for example; $                 s
            if cell == enemy1 || cell == enemy2 {
                return false;
            }
            if cell == me1 || cell == me2 {
                touch += 1;
            }
        }
    }
    touch == 1
}

fn choose_best(
    options: Vec<(usize, usize)>,
    board: &Vec<Vec<char>>,
    enemy1: char,
    enemy2: char
) -> Option<(usize, usize)> {
    let mut enemies = Vec::new();

    // collect enemy positions from the board
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let cell = board[i][j];
            if cell == enemy1 || cell == enemy2 {
                enemies.push((i, j));
            }
        }
    }
    let mut best_option: Option<(usize, usize)> = None;
    let mut best_distance = f64::INFINITY;

    //try all options + calc closet dis to the enemy
    for &(x, y) in &options {
        let mut min_dist = f64::INFINITY;

        for &(ex, ey) in &enemies {
            let dx = (x as f64) - (ex as f64);
            let dy = (y as f64) - (ey as f64);
            let dist = dx * dx + dy * dy;

            if dist < min_dist {
                min_dist = dist;
            }
        }

        if min_dist < best_distance {
            best_distance = min_dist;
            best_option = Some((x, y));
        }
    }

    best_option
}
