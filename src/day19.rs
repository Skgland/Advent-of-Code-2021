use std::collections::HashSet;

type Matrix<const N: usize, const M: usize> = [[isize; M]; N];
type Vector<const N: usize> = [isize; N];

pub struct Scanner {
    _number: usize,
    transform: Option<Matrix<4, 4>>,
    elements: HashSet<Vector<3>>,
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut result = vec![];
    let mut elements = HashSet::new();
    let mut idx = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            result.push(Scanner {
                _number: idx,
                transform: None,
                elements,
            });
            idx += 1;
            elements = HashSet::new();
        } else if line.starts_with("---") {
            // skip scanner header
        } else if let [x, y, z] = line.splitn(3, ',').collect::<Vec<_>>().as_slice() {
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let z = z.parse().unwrap();
            elements.insert([x, y, z]);
        }
    }
    // push last scanner
    result.push(Scanner {
        _number: idx,
        transform: None,
        elements,
    });
    result
}

fn vector_as_translation(trans: &Vector<3>) -> Matrix<4, 4> {
    [
        [1, 0, 0, trans[0]],
        [0, 1, 0, trans[1]],
        [0, 0, 1, trans[2]],
        [0, 0, 0, 1],
    ]
}

fn multiply_matrix<const N: usize, const M: usize, const O: usize>(
    a: &Matrix<N, M>,
    b: &Matrix<M, O>,
) -> Matrix<N, O> {
    let mut n_idx = 0;
    [(); N].map(|_| {
        let mut o_idx = 0;
        let n_result = [(); O].map(|_| {
            let o_result = (0..M).map(|m_idx| a[n_idx][m_idx] * b[m_idx][o_idx]).sum();
            o_idx += 1;
            o_result
        });
        n_idx += 1;
        n_result
    })
}

const ID4XD: Matrix<4, 4> = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
const X_90: Matrix<4, 4> = [[1, 0, 0, 0], [0, 0, -1, 0], [0, 1, 0, 0], [0, 0, 0, 1]];
const Y_90: Matrix<4, 4> = [[0, 0, 1, 0], [0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 1]];
const Z_90: Matrix<4, 4> = [[0, -1, 0, 0], [1, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];

// cos(90) = 0
// sin(90) = 1
fn rotation_matrices() -> impl Iterator<Item = Matrix<4, 4>> {
    let x_180 = multiply_matrix(&X_90, &X_90);
    let x_270 = multiply_matrix(&X_90, &x_180);
    let y_180 = multiply_matrix(&Y_90, &Y_90);
    let y_270 = multiply_matrix(&Y_90, &y_180);
    let z_180 = multiply_matrix(&Z_90, &Z_90);
    let z_270 = multiply_matrix(&Z_90, &z_180);
    let x_rotations = [ID4XD, X_90, x_180, x_270];
    let main_axis = [ID4XD, Y_90, y_180, y_270, Z_90, z_270];

    x_rotations.into_iter().flat_map(move |x_rotation| {
        main_axis
            .into_iter()
            .map(move |main_axis| multiply_matrix(&main_axis, &x_rotation))
    })
}

/// ```
/// assert_eq!(aoc2021::day19::transpose(&[[0,1],[-1,0]]), [[0,-1],[1,0]])
/// ```
pub fn transpose<const N: usize>(a: &Matrix<N, N>) -> Matrix<N, N> {
    let mut row_idx = 0;
    [(); N].map(|_| {
        let mut column_idx = 0;
        let result = [(); N].map(|_| {
            let result = a[column_idx][row_idx];
            column_idx += 1;
            result
        });
        row_idx += 1;
        result
    })
}

pub fn find_overlap(a: &Scanner, b: &Scanner) -> Option<Matrix<4, 4>> {
    for rotation in rotation_matrices() {
        for a_base_elem in &a.elements {
            let a_base_rotated = apply_transform(a_base_elem, &rotation);
            'next_base: for b_base_element in &b.elements {
                let x_offset = b_base_element[0] - a_base_rotated[0];
                let y_offset = b_base_element[1] - a_base_rotated[1];
                let z_offset = b_base_element[2] - a_base_rotated[2];

                let translation = vector_as_translation(&[x_offset, y_offset, z_offset]);

                let transformation = multiply_matrix(&translation, &rotation);

                assert_eq!(
                    &apply_transform(&a_base_rotated, &translation),
                    b_base_element,
                );

                assert_eq!(
                    &apply_transform(a_base_elem, &transformation),
                    b_base_element,
                );

                let mut count = 0;

                for elem in &a.elements {
                    let mapped = apply_transform(elem, &transformation);
                    if !mapped.iter().all(|e| (-1000..=1000).contains(e)) {
                        continue;
                    } else if b.elements.contains(&mapped) {
                        count += 1;
                    } else {
                        continue 'next_base;
                    }
                }

                if count < 12 {
                    continue 'next_base;
                }

                let inv_transformation = multiply_matrix(
                    &transpose(&rotation),
                    &vector_as_translation(&[-x_offset, -y_offset, -z_offset]),
                );

                for elem in &b.elements {
                    let mapped = apply_transform(elem, &inv_transformation);
                    if !mapped.iter().all(|e| (-1000..=1000).contains(e)) {
                        continue;
                    } else if !(a.elements.contains(&mapped)) {
                        continue 'next_base;
                    }
                }

                return Some(transformation);
            }
        }
    }
    None
}

fn apply_transform(pos: &Vector<3>, transform: &Matrix<4, 4>) -> Vector<3> {
    let [[b_x], [b_y], [b_z], _] = multiply_matrix(transform, &[[pos[0]], [pos[1]], [pos[2]], [1]]);
    [b_x, b_y, b_z]
}

pub fn calc_transforms(scanners: Vec<Scanner>) -> Vec<Scanner> {
    // scanners that have not yet a known position relative to scanner 0
    let mut todo_scanners = scanners;
    todo_scanners.reverse();

    let mut scanner_0 = todo_scanners.pop().unwrap();
    scanner_0.transform = Some(ID4XD);

    // scanners with a known transformation to scanner 0 and not yet used to orient others
    let mut set_scanners = vec![scanner_0];

    // scanners already used for orientation and with known transformation to scanner 0
    let mut done_scanners = vec![];

    while let Some(base) = set_scanners.pop() {
        let offset = base.transform.unwrap();
        let todo = std::mem::take(&mut todo_scanners);
        for mut scanner in todo {
            match find_overlap(&scanner, &base) {
                Some(transform) => {
                    let transform = multiply_matrix(&offset, &transform);
                    scanner.transform = Some(transform);
                    set_scanners.push(scanner);
                }
                None => todo_scanners.push(scanner),
            };
        }
        done_scanners.push(base)
    }

    assert!(todo_scanners.is_empty());

    done_scanners
}

pub fn part1(input: &str) -> usize {
    let scanners = parse_input(input);

    let scanners = calc_transforms(scanners);

    let global: HashSet<_> = scanners
        .iter()
        .flat_map(|scanner| {
            let transform = scanner.transform.unwrap();
            scanner
                .elements
                .iter()
                .map(move |pos| apply_transform(pos, &transform))
        })
        .collect();

    global.len()
}

pub fn part2(input: &str) -> usize {
    let scanners = parse_input(input);

    let scanners = calc_transforms(scanners);

    let positions = scanners
        .into_iter()
        .map(|scanner| apply_transform(&[0, 0, 0], &scanner.transform.unwrap()))
        .collect::<Vec<_>>();

    positions
        .iter()
        .flat_map(|a| positions.iter().map(|b| manhattan_distance(a, b)))
        .max()
        .unwrap()
}

pub fn manhattan_distance(a: &Vector<3>, b: &Vector<3>) -> usize {
    ((a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()) as usize
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day19.example.txt"));
    assert_eq!(part1(input), 79);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day19.txt"));
    assert_eq!(part1(input), 362);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day19.example.txt");
    assert_eq!(part2(input), 3621);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day19.txt"));
    assert_eq!(part2(input), 12204);
}
