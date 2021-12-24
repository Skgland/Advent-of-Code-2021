use crate::day24::Arg2::Literal;
use crate::day24::Operation::{Add, Div, Eq, Inp, Mod, Mul};
use crate::day24::Register::*;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};
use RegisterState::{Input, Value};

#[derive(Clone)]
pub enum Register {
    W,
    X,
    Y,
    Z,
}

pub struct AluState {
    next_input: usize,
    shared: Vec<Weak<Expression>>,
    w: RegisterState,
    x: RegisterState,
    y: RegisterState,
    z: RegisterState,
}

impl Default for AluState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub enum Arg2 {
    Register(Register),
    Literal(isize),
}

#[derive(Clone)]
pub enum RegisterState {
    Value(isize),
    Input(usize),
    Expr(usize, Rc<Expression>),
}

impl Display for RegisterState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value(x) => Display::fmt(x, f),
            RegisterState::Input(idx) => f.write_fmt(format_args!("input[{}]", idx)),
            RegisterState::Expr(idx, _) => f.write_fmt(format_args!("s{}", idx)),
        }
    }
}

#[derive(Clone)]
pub struct Expression {
    op: Operation,
    arg1: RegisterState,
    arg2: RegisterState,
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let op = match &self.op {
            Inp => "inp",
            Add => "+",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Eq => return f.write_fmt(format_args!("({} == {}) as isize", self.arg1, self.arg2)),
        };

        f.write_fmt(format_args!("{} {} {}", self.arg1, op, self.arg2))
    }
}

#[derive(Clone)]
pub struct Instruction {
    op: Operation,
    arg1: Register,
    arg2: Arg2,
}

#[derive(Clone)]
pub enum Operation {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eq,
}

fn parse_register(reg: &str) -> Register {
    match reg {
        "w" => W,
        "x" => X,
        "y" => Y,
        "z" => Z,
        _ => panic!(),
    }
}

fn parse_arg2(lit_or_reg: &str) -> Arg2 {
    match lit_or_reg {
        "w" => Arg2::Register(W),
        "x" => Arg2::Register(X),
        "y" => Arg2::Register(Y),
        "z" => Arg2::Register(Z),
        lit => Arg2::Literal(lit.parse().unwrap()),
    }
}

pub fn parse_operator(op: &str) -> Operation {
    match op {
        "inp" => Inp,
        "add" => Add,
        "mul" => Mul,
        "div" => Div,
        "mod" => Mod,
        "eql" => Eq,
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().flat_map(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();
        match parts.as_slice() {
            ["inp", reg] => Some(Instruction {
                op: Inp,
                arg1: parse_register(reg),
                arg2: Literal(0),
            }),
            [op, reg, arg2] => Some(Instruction {
                op: parse_operator(op),
                arg1: parse_register(reg),
                arg2: parse_arg2(arg2),
            }),
            _ => None,
        }
    })
}

impl AluState {
    pub fn new() -> Self {
        AluState {
            next_input: 0,
            shared: Default::default(),
            w: Value(0),
            x: Value(0),
            y: Value(0),
            z: Value(0),
        }
    }

    pub fn get_register(&mut self, reg: &Register) -> &mut RegisterState {
        match reg {
            W => &mut self.w,
            X => &mut self.x,
            Y => &mut self.y,
            Z => &mut self.z,
        }
    }

    pub fn next_input(&mut self) -> usize {
        let idx = self.next_input;
        self.next_input += 1;
        idx
    }

    pub fn apply_instruction(&mut self, inst: &Instruction) {
        let left = self.get_register(&inst.arg1).clone();
        let right = match &inst.arg2 {
            Arg2::Register(reg) => self.get_register(reg).clone(),
            Literal(lit) => Value(*lit),
        };

        *self.get_register(&inst.arg1) = match inst.op {
            Inp => {
                let next_idx = self.next_input();
                RegisterState::Input(next_idx)
            }
            Add => match (left, right) {
                (Value(x), Value(y)) => RegisterState::Value(x + y),
                (Value(0), other) | (other, Value(0)) => other,
                (left, right) => {
                    let expr = Expression {
                        op: Add,
                        arg1: left,
                        arg2: right,
                    };
                    let rc = Rc::new(expr);
                    let weak = Rc::downgrade(&rc);
                    let shared_id = self.shared.len();
                    self.shared.insert(shared_id, weak);
                    RegisterState::Expr(shared_id, rc)
                }
            },
            Mul => match (left, right) {
                (Value(x), Value(y)) => RegisterState::Value(x * y),
                (Value(0), _) | (_, Value(0)) => RegisterState::Value(0),
                (Value(1), other) | (other, Value(1)) => other,
                (left, right) => {
                    let expr = Expression {
                        op: Mul,
                        arg1: left,
                        arg2: right,
                    };
                    let rc = Rc::new(expr);
                    let weak = Rc::downgrade(&rc);
                    let shared_id = self.shared.len();
                    self.shared.insert(shared_id, weak);
                    RegisterState::Expr(shared_id, rc)
                }
            },
            Div => match (left, right) {
                (Value(x), Value(y)) => RegisterState::Value(x / y),
                (Value(0), _) => Value(0),
                (other, Value(1)) => other,
                (left, right) => {
                    let expr = Expression {
                        op: Div,
                        arg1: left,
                        arg2: right,
                    };
                    let rc = Rc::new(expr);
                    let weak = Rc::downgrade(&rc);
                    let shared_id = self.shared.len();
                    self.shared.insert(shared_id, weak);
                    RegisterState::Expr(shared_id, rc)
                }
            },
            Mod => match (left, right) {
                (Value(x), Value(y)) => RegisterState::Value(x % y),
                (Value(0), _) => Value(0),
                (_, Value(1)) => Value(1),
                (left, right) => {
                    let expr = Expression {
                        op: Mod,
                        arg1: left,
                        arg2: right,
                    };
                    let rc = Rc::new(expr);
                    let weak = Rc::downgrade(&rc);
                    let shared_id = self.shared.len();
                    self.shared.insert(shared_id, weak);
                    RegisterState::Expr(shared_id, rc)
                }
            },
            Eq => match (left, right) {
                (Value(x), Value(y)) => Value((x == y) as isize),
                (Value(x), Input(_)) | (Input(_), Value(x)) if !(0..=9).contains(&x) => Value(0),
                (Input(x), Input(y)) if x == y => Value(1),
                (left, right) => {
                    let expr = Expression {
                        op: Eq,
                        arg1: left,
                        arg2: right,
                    };
                    let rc = Rc::new(expr);
                    let weak = Rc::downgrade(&rc);
                    let shared_id = self.shared.len();
                    self.shared.insert(shared_id, weak);
                    RegisterState::Expr(shared_id, rc)
                }
            },
        }
    }
}

pub fn part1_instructions_to_code(input: &str) -> String {
    let iter = parse_input(input);
    let mut alu = AluState::new();
    iter.for_each(|inst| alu.apply_instruction(&inst));

    let AluState { shared, z, .. } = alu;

    let mut str = String::new();

    use std::fmt::Write;

    for (idx, expr) in shared.iter().enumerate() {
        if let Some(expr) = expr.upgrade() {
            let _ = writeln!(str, "let s{} = {};", idx, expr);
        }
    }

    let _ = writeln!(str, "{}", z);
    str
}

pub fn to_digits(serial: isize) -> [isize; 14] {
    serial
        .to_string()
        .chars()
        .map(|c| c as u8 - b'0')
        .map(|d| d as isize)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn part1(_: &str) -> isize {
    let d0 = 1;
    let tmp = d0 + 12;
    for d1 in (1..3).rev() {
        let tmp = process_no_div::<1>(d1, tmp);

        for d2 in (8..10).rev() {
            let tmp = process_no_div::<2>(d2, tmp);
            for d3 in (4..10).rev() {
                let tmp = process_no_div::<3>(d3, tmp);

                assert_eq!(-3, LOOKUP1[3] + LOOKUP2[4]);
                let d4 = d3 - 3;
                let tmp = process_with_div::<4>(d4, tmp);

                for d5 in (2..10).rev() {
                    let tmp = process_no_div::<5>(d5, tmp);

                    for d6 in (3..10).rev() {
                        let tmp = process_no_div::<6>(d6, tmp);

                        assert_eq!(-2, LOOKUP1[6] + LOOKUP2[7]);
                        let d7 = d6 - 2;
                        let tmp = process_with_div::<7>(d7, tmp);

                        assert_eq!(-1, LOOKUP1[5] + LOOKUP2[8]);
                        let d8 = d5 - 1;
                        let tmp = process_with_div::<8>(d8, tmp);

                        assert_eq!(-7, LOOKUP1[2] + LOOKUP2[9]);
                        let d9 = d2 - 7;
                        let tmp = process_with_div::<9>(d9, tmp);

                        for d10 in (7..10).rev() {
                            let tmp = process_no_div::<10>(d10, tmp);
                            assert_eq!(-6, LOOKUP1[10] + LOOKUP2[11]);
                            let d11 = d10 - 6;
                            let tmp = process_with_div::<11>(d11, tmp);
                            assert_eq!(7, LOOKUP1[1] + LOOKUP2[12]);
                            let d12 = d1 + 7;
                            let tmp = process_with_div::<12>(d12, tmp);

                            assert_eq!(8, LOOKUP1[0] + LOOKUP2[13]);
                            let d13 = d0 + 8;
                            let tmp = process_with_div::<13>(d13, tmp);

                            let digits =
                                [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13];

                            let result = digits.into_iter().fold(0, |acc, next| acc * 10 + next);

                            // println!("{:?}", digits);
                            assert_eq!(tmp, part1_fn(digits));
                            assert_eq!(tmp, part1_fn2(digits));

                            if tmp == 0 {
                                return result;
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("Tried all serial numbers!")
}

#[allow(clippy::let_and_return)]
pub const fn part1_fn(input: [isize; 14]) -> isize {
    let s0 = input[0] + 12;

    let s1 = s0 % 26;
    let s2 = s1 + 15;
    let s3 = (s2 == input[1]) as isize;
    let s4 = (s3 == 0) as isize;
    let s5 = 25 * s4;
    let s6 = s5 + 1;
    let s7 = s0 * s6;
    let s8 = input[1] + 7;
    let s9 = s8 * s4;
    let s10 = s7 + s9;

    let s11 = s10 % 26;
    let s12 = s11 + 12;
    let s13 = (s12 == input[2]) as isize;
    let s14 = (s13 == 0) as isize;
    let s15 = 25 * s14;
    let s16 = s15 + 1;
    let s17 = s10 * s16;
    let s18 = input[2] + 1;
    let s19 = s18 * s14;
    let s20 = s17 + s19;

    let s21 = s20 % 26;
    let s22 = s21 + 11;
    let s23 = (s22 == input[3]) as isize;
    let s24 = (s23 == 0) as isize;
    let s25 = 25 * s24;
    let s26 = s25 + 1;
    let s27 = s20 * s26;
    let s28 = input[3] + 2;
    let s29 = s28 * s24;
    let s30 = s27 + s29;

    let s31 = s30 % 26;
    let s32 = s30 / 26;
    let s33 = s31 + -5;
    let s34 = (s33 == input[4]) as isize;
    let s35 = (s34 == 0) as isize;
    let s36 = 25 * s35;
    let s37 = s36 + 1;
    let s38 = s32 * s37;
    let s39 = input[4] + 4;
    let s40 = s39 * s35;
    let s41 = s38 + s40;

    let s42 = s41 % 26;
    let s43 = s42 + 14;
    let s44 = (s43 == input[5]) as isize;
    let s45 = (s44 == 0) as isize;
    let s46 = 25 * s45;
    let s47 = s46 + 1;
    let s48 = s41 * s47;
    let s49 = input[5] + 15;
    let s50 = s49 * s45;
    let s51 = s48 + s50;

    let s52 = s51 % 26;
    let s53 = s52 + 15;
    let s54 = (s53 == input[6]) as isize;
    let s55 = (s54 == 0) as isize;
    let s56 = 25 * s55;
    let s57 = s56 + 1;
    let s58 = s51 * s57;
    let s59 = input[6] + 11;
    let s60 = s59 * s55;
    let s61 = s58 + s60;

    let s62 = s61 % 26;
    let s63 = s61 / 26;
    let s64 = s62 + -13;
    let s65 = (s64 == input[7]) as isize;
    let s66 = (s65 == 0) as isize;
    let s67 = 25 * s66;
    let s68 = s67 + 1;
    let s69 = s63 * s68;
    let s70 = input[7] + 5;
    let s71 = s70 * s66;
    let s72 = s69 + s71;

    let s73 = s72 % 26;
    let s74 = s72 / 26;
    let s75 = s73 + -16;
    let s76 = (s75 == input[8]) as isize;
    let s77 = (s76 == 0) as isize;
    let s78 = 25 * s77;
    let s79 = s78 + 1;
    let s80 = s74 * s79;
    let s81 = input[8] + 3;
    let s82 = s81 * s77;
    let s83 = s80 + s82;

    let s84 = s83 % 26;
    let s85 = s83 / 26;
    let s86 = s84 + -8;
    let s87 = (s86 == input[9]) as isize;
    let s88 = (s87 == 0) as isize;
    let s89 = 25 * s88;
    let s90 = s89 + 1;
    let s91 = s85 * s90;
    let s92 = input[9] + 9;
    let s93 = s92 * s88;
    let s94 = s91 + s93;

    let s95 = s94 % 26;
    let s96 = s95 + 15;
    let s97 = (s96 == input[10]) as isize;
    let s98 = (s97 == 0) as isize;
    let s99 = 25 * s98;
    let s100 = s99 + 1;
    let s101 = s94 * s100;
    let s102 = input[10] + 2;
    let s103 = s102 * s98;
    let s104 = s101 + s103;

    let s105 = s104 % 26;
    let s106 = s104 / 26;
    let s107 = s105 + -8;
    let s108 = (s107 == input[11]) as isize;
    let s109 = (s108 == 0) as isize;
    let s110 = 25 * s109;
    let s111 = s110 + 1;
    let s112 = s106 * s111;
    let s113 = input[11] + 3;
    let s114 = s113 * s109;
    let s115 = s112 + s114;

    let s116 = s115 % 26;
    let s117 = s115 / 26;
    let s118 = (s116 == input[12]) as isize;
    let s119 = (s118 == 0) as isize;
    let s120 = 25 * s119;
    let s121 = s120 + 1;
    let s122 = s117 * s121;
    let s123 = input[12] + 3;
    let s124 = s123 * s119;
    let s125 = s122 + s124;

    let s126 = s125 % 26;
    let s127 = s125 / 26;
    let s128 = s126 + -4;
    let s129 = (s128 == input[13]) as isize;
    let s130 = (s129 == 0) as isize;
    let s131 = 25 * s130;
    let s132 = s131 + 1;
    let s133 = s127 * s132;
    let s134 = input[13] + 11;
    let s135 = s134 * s130;
    let s136 = s133 + s135;

    s136
}

const LOOKUP1: [isize; 14] = [12, 7, 1, 2, 4, 15, 11, 5, 3, 9, 2, 3, 3, 11];
const LOOKUP2: [isize; 14] = [0, 15, 12, 11, -5, 14, 15, -13, -16, -8, 15, -8, 0, -4];

#[allow(clippy::let_and_return)]
const fn process_no_div<const IDX: usize>(input: isize, in_0: isize) -> isize {
    in_0 * 26 + input + LOOKUP1[IDX]
}

#[allow(clippy::let_and_return)]
const fn process_with_div<const IDX: usize>(input: isize, in_0: isize) -> isize {
    if in_0 % 26 + LOOKUP2[IDX] != input {
        (in_0 / 26) * 26 + input + LOOKUP1[IDX]
    } else {
        in_0 / 26
    }
}

pub const fn part1_fn2(input: [isize; 14]) -> isize {
    let mut tmp = process_no_div::<0>(input[0], 0);
    tmp = process_no_div::<1>(input[1], tmp);
    tmp = process_no_div::<2>(input[2], tmp);
    tmp = process_no_div::<3>(input[3], tmp);
    tmp = process_with_div::<4>(input[4], tmp);
    tmp = process_no_div::<5>(input[5], tmp);
    tmp = process_no_div::<6>(input[6], tmp);
    tmp = process_with_div::<7>(input[7], tmp);
    tmp = process_with_div::<8>(input[8], tmp);
    tmp = process_with_div::<9>(input[9], tmp);
    tmp = process_no_div::<10>(input[10], tmp);
    tmp = process_with_div::<11>(input[11], tmp);
    tmp = process_with_div::<12>(input[12], tmp);
    tmp = process_with_div::<13>(input[13], tmp);
    tmp
}

pub fn part2(_: &str) -> isize {
    let d0 = 1;
    let tmp = d0 + 12;
    for d1 in 1..3 {
        let tmp = process_no_div::<1>(d1, tmp);

        for d2 in 8..10 {
            let tmp = process_no_div::<2>(d2, tmp);
            for d3 in 4..10 {
                let tmp = process_no_div::<3>(d3, tmp);

                assert_eq!(-3, LOOKUP1[3] + LOOKUP2[4]);
                let d4 = d3 - 3;
                let tmp = process_with_div::<4>(d4, tmp);

                for d5 in 2..10 {
                    let tmp = process_no_div::<5>(d5, tmp);

                    for d6 in 3..10 {
                        let tmp = process_no_div::<6>(d6, tmp);

                        assert_eq!(-2, LOOKUP1[6] + LOOKUP2[7]);
                        let d7 = d6 - 2;
                        let tmp = process_with_div::<7>(d7, tmp);

                        assert_eq!(-1, LOOKUP1[5] + LOOKUP2[8]);
                        let d8 = d5 - 1;
                        let tmp = process_with_div::<8>(d8, tmp);

                        assert_eq!(-7, LOOKUP1[2] + LOOKUP2[9]);
                        let d9 = d2 - 7;
                        let tmp = process_with_div::<9>(d9, tmp);

                        for d10 in 7..10 {
                            let tmp = process_no_div::<10>(d10, tmp);
                            assert_eq!(-6, LOOKUP1[10] + LOOKUP2[11]);
                            let d11 = d10 - 6;
                            let tmp = process_with_div::<11>(d11, tmp);
                            assert_eq!(7, LOOKUP1[1] + LOOKUP2[12]);
                            let d12 = d1 + 7;
                            let tmp = process_with_div::<12>(d12, tmp);

                            assert_eq!(8, LOOKUP1[0] + LOOKUP2[13]);
                            let d13 = d0 + 8;
                            let tmp = process_with_div::<13>(d13, tmp);

                            let digits =
                                [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13];

                            let result = digits.into_iter().fold(0, |acc, next| acc * 10 + next);

                            // println!("{:?}", digits);
                            assert_eq!(tmp, part1_fn(digits));
                            assert_eq!(tmp, part1_fn2(digits));

                            if tmp == 0 {
                                return result;
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("Tried all serial numbers!")
}

#[test]
fn equivalent() {
    let serial = 13579246899999;
    let digits = to_digits(serial);

    assert_eq!(part1_fn(digits), part1_fn2(digits))
}

#[test]
fn part1_full() {
    assert_eq!(part1(""), 12996997829399);
}

#[test]
fn part2_full() {
    assert_eq!(part2(""), 11841231117189);
}
