mod minesweeper {
    static DIRS: [[i32; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    pub fn annotate(minefield: &[&str]) -> Vec<String> {
        let board = minefield
            .iter()
            .map(|line| line.as_bytes())
            .collect::<Vec<_>>();

        board
            .iter()
            .enumerate()
            .map(|(y, line)| {
                let stat = line
                    .iter()
                    .enumerate()
                    .map(|(x, &code)| {
                        if code == b'*' {
                            return code;
                        }
                        let count = DIRS
                            .iter()
                            .filter(|[dx, dy]| {
                                x as i32 + dx >= 0
                                    && x as i32 + dx < line.len() as i32
                                    && y as i32 + dy >= 0
                                    && y as i32 + dy < board.len() as i32
                            })
                            .map(|[dx, dy]| {
                                let nx = x as i32 + dx;
                                let ny = y as i32 + dy;
                                board[ny as usize][nx as usize]
                            })
                            .filter(|&code| code == b'*')
                            .count() as u8;
                        if count > 0 {
                            b'0' + count
                        } else {
                            b' '
                        }
                    })
                    .collect::<Vec<_>>();

                String::from_utf8(stat).unwrap()
            })
            .collect::<Vec<_>>()
    }
}

use minesweeper::annotate;
fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}
fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}
#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}
#[test]
#[ignore]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}
#[test]
#[ignore]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}
#[test]
#[ignore]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}
#[test]
#[ignore]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}
#[test]
#[ignore]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}
#[test]
#[ignore]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}
#[test]
#[ignore]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}
#[test]
#[ignore]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}
#[test]
#[ignore]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}
#[test]
#[ignore]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}
#[test]
#[ignore]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
