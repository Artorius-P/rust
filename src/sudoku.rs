fn main() {
    let mut board = vec![vec!['5','3','.','.','7','.','.','.','.'],
                         vec!['6','.','.','1','9','5','.','.','.'],
                         vec!['.','9','8','.','.','.','.','6','.'],
                         vec!['8','.','.','.','6','.','.','.','3'],
                         vec!['4','.','.','8','.','3','.','.','1'],
                         vec!['7','.','.','.','2','.','.','.','6'],
                         vec!['.','6','.','.','.','.','2','8','.'],
                         vec!['.','.','.','4','1','9','.','.','5'],
                         vec!['.','.','.','.','8','.','.','7','9']];
    backtrack(&mut board, 0, 0);
    for i in board {
        for j in i {
            print!("{} ", j);
        }
        print!("\n");
    }
}

fn backtrack(board:&mut Vec<Vec<char>>, i: i32, j:i32) -> bool {
    let m = 9;
    let n = 9;
    if j == n {
        return backtrack(board, i+1, 0);
    }
    
    if i == m {
        return true;
    }

    if board[i as usize][j as usize] != '.' {
        return backtrack(board, i, j+1);
    }

    for ch in '1'..':' {
        if !is_valid(board, i, j, ch) {
            continue;
        }
        board[i as usize][j as usize] = ch;

        if backtrack(board, i, j + 1) {
            return true;
        }
        board[i as usize][j as usize] = '.';

    }
    return false;
}

fn is_valid(board: &Vec<Vec<char>>, r: i32, c:i32, n:char) -> bool {
    for i in 0..9 {
        if board[r as usize][i as usize] == n {
            return false;
        }

        if board[i as usize][c as usize] == n {
            return false;
        }

        if board[((r/3)*3 + i/3) as usize ][((c/3)*3 + i%3) as usize] == n {
            return false;
        }
    }
    return true;
}
