//! Game board logic.

/// Size of game board.
const SIZE: usize = 9;

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[u8; SIZE]; SIZE],
}

impl Gameboard {
    /// Creates a new game board.
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
        }
    }

    /// Get the charactor at cell location
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(
            match self.cells[ind[1]][ind[0]] {
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => return None,
            }
        )
    }

    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]] = val;
    }

    /// Generate start matrix
    pub fn question_generate(&mut self) {
        generate_sudoku_matrix(self);
        dig_holes(self);
    }
}

/// Dig holes randomly in the matrix
fn dig_holes(gameboard: &mut Gameboard) {
    for i in 0..9 {
        let y_p = match i / 3 {
            0 => 0..3,
            1 => 3..6,
            2 => 6..9,
            _ => 0..3,
        };
        let x_p = match i % 3 {
            0 => 0..3,
            1 => 3..6,
            2 => 6..9,
            _ => 0..3,
        };
        for y in y_p {
            x_p.clone().for_each(|x| {
                if random_num(1, 10) > 6 {
                    gameboard.set([y, x], 0);
                }
            });
        }
    }
}

/// Generate sudoku matrix
fn generate_sudoku_matrix(gameboard: &mut Gameboard) {
    let mut matrix = vec![vec![0; 9]; 9];

    let mut y = 0;
    let mut x = 0;
    let mut c_g = 0; // 回退次数计数
    loop {
        if y >= 9 {
            break;
        }
        if c_g >= 10 { // 回退次数过多时释放整个 row
            {
                let m = &mut matrix[y];
                m.into_iter()
                    .for_each(|x| {
                        *x = 0;
                    });
                y -= 1;
                c_g = 0;
            }
            {
                let m = &mut matrix[y];
                m.into_iter()
                    .for_each(|x| {
                        *x = 0;
                    });
                x = 0;
            }
        }

        matrix[y][x] = random_num(1, 9);

        let mut c = 0; // 计数器

        while !matrix_check(&matrix) {
            c += 1;
            matrix[y][x] = random_num(1, 9);
            if c >= 20 {
                c_g += 1;
                matrix[y][x] = 0;
                if x == 0 && y > 0 { // matrix 换行
                    y -= 1;
                    x = 8;
                } else if x > 0 {
                    x -= 1;
                }
                c = 0;
            }
        }
        x += 1;
        if x >= 9 { // matrix 换行
            y += 1;
            x = 0;
        }
        println!("{:?}", &matrix);
    }

    for y in 0..9 {
        for x in 0..9 {
            gameboard.set([x, y], matrix[y][x] as u8);
        }
    }
}

/// Return a random usize number with given bound
/// e.g. s = 1, e = 5, return 1~5 usize
fn random_num(s: usize, e: usize) -> usize {
    use rand::random;
    random::<usize>() % e + s
}

/// Check is the matrix legal
/// return true if matrix legal
fn matrix_check(m: &Vec<Vec<usize>>) -> bool {
    for y in 0..9 { // check row
        let mut checked_value: Vec<usize> = vec![];
        // 直接使用 序列操作符[] 获得的是实际的对象而不是一个reference | pointer | copy
        let row = &m[y];
        row.into_iter()
            .for_each(|x| {
                checked_value.push(*x);
            });
        checked_value.sort();
        if !arr_repeat_check(&mut checked_value) {
            return false;
        }
    }

    for x in 0..9 {
        { // check if there is any duplication of numbers in a column
            let mut checked_value: Vec<usize> = vec![];
            for y in 0..9 { // check column
                checked_value.push(m[y][x]);
            }
            checked_value.sort();
            if !arr_repeat_check(&mut checked_value) {
                return false;
            }
        }
        { // check 3 x 3 matrix
            // x use to point which matrix
            let mut mm_pos: Vec<(usize, usize)> = vec![];
            let y_range = match x / 3 {
                0 => 0..3,
                1 => 3..6,
                2 => 6..9,
                _ => panic!("index err"),
            };
            let x_range = match x % 3 {
                0 => 0..3,
                1 => 3..6,
                2 => 6..9,
                _ => panic!("index err"),
            };
            for y in y_range {
                for x_inm in x_range.clone() {
                    mm_pos.push((y, x_inm));
                }
            }
            let mut checked_value: Vec<usize> = vec![];
            mm_pos.into_iter().for_each(|(y, x)| {
                checked_value.push(m[y][x]);
            });
            checked_value.sort();
            if !arr_repeat_check(&mut checked_value) {
                return false;
            }
        }
    }
    return true;
}

/// Check if there is is any duplication in given array
fn arr_repeat_check(arr: &mut Vec<usize>) -> bool {
    let len = arr.len();
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if arr[i] != 0 && arr[i] == arr[j] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_check_works() {
        let test_example = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![4,5,6,7,8,9,1,2,3],
            vec![7,8,9,1,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,4,5,6,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![9,1,2,3,4,5,6,7,8],
        ];

        assert!(matrix_check(&test_example));
    }
    #[test]
    fn matrix_check_works_2() {
        let test_example = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![0;9],
            vec![7,8,9,1,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,4,5,6,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![0;9],
        ];

        assert!(matrix_check(&test_example));
    }

    #[test]
    fn matrix_check_works_3() {
        let test_example = vec![
            vec![1,2,3,4,0,0,7,8,9],
            vec![0;9],
            vec![7,8,9,1,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,4,5,6,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![0;9],
        ];

        assert!(matrix_check(&test_example));
    }

    #[test]
    fn matrix_check_works_err() {
        let test_example = vec![
            vec![2,2,3,4,5,6,7,8,9],
            vec![4,5,6,7,8,9,1,2,3],
            vec![7,8,9,1,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,4,5,6,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![9,1,2,3,4,5,6,7,8],
        ];

        assert_eq!(matrix_check(&test_example), false);
    }

    #[test]
    fn matrix_check_works_err_2() {
        let test_example = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![4,5,6,7,8,9,1,5,3],
            vec![7,8,9,1,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,4,5,6,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![9,1,2,3,4,5,6,7,8],
        ];

        assert_eq!(matrix_check(&test_example), false);
    }
}
