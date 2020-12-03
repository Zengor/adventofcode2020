use crate::util::Matrix;

pub fn parse_grid(input: &str) -> Matrix<bool> {
    let width = input.lines().next().unwrap().trim().chars().count();
    let data: Vec<bool> = input
        .lines()
        .flat_map(|l| l.trim().chars())
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!(),
        })
        .collect();
    Matrix::wrap(data, width)
}

pub fn go_down_slopes(field: &Matrix<bool>, slope_dx: usize, slope_dy: usize) -> u32 {
    let (mut x, mut y) = (0, 0);
    let (width, height) = (field.width(), field.height());
    let mut count = 0;
    while y < height - 1 {
        x = (x + slope_dx) % width;
        y += slope_dy;
        if field[(x, y)] {
            count += 1;
        }
    }
    count
}
pub fn part1(input: &str) -> u32 {
    let field = parse_grid(input);
    go_down_slopes(&field, 3, 1)
}

pub fn part2(input: &str) -> u32 {
    let field = parse_grid(input);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|s| go_down_slopes(&field, s.0, s.1))
        .product()
}
