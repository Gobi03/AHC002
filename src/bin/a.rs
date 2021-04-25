#[allow(unused_imports)]
use proconio::marker::Chars;
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const SIDE: usize = 50;
const COMS: [char; 4] = ['U', 'D', 'L', 'R'];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
impl Coord {
    fn new(p: (isize, isize)) -> Self {
        Coord { x: p.0, y: p.1 }
    }

    fn in_field(&self) -> bool {
        (0 <= self.x && self.x < SIDE as isize) && (0 <= self.y && self.y < SIDE as isize)
    }

    // ペアへの変換
    fn to_pair(&self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }

    // マンハッタン距離
    fn distance(&self, that: &Self) -> isize {
        let dist_x = max(self.x, that.x) - min(self.x, that.x);
        let dist_y = max(self.y, that.y) - min(self.y, that.y);
        dist_x + dist_y
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let (ix, iy) = self.to_pair();
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&(dx, dy)| (ix + dx, iy + dy))
            .map(|p| Coord::new(p))
            .filter(|&p| p.in_field())
            .collect()
    }

    // 四則演算
    fn plus(&self, that: &Self) -> Self {
        Coord::new((self.x + that.x, self.y + that.y))
    }
    fn minus(&self, that: &Self) -> Self {
        Coord::new((self.x - that.x, self.y - that.y))
    }

    fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<T>>) -> &'a T {
        &mat[self.y as usize][self.x as usize]
    }

    fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y as usize][self.x as usize] = e;
    }

    // new
    fn com_to_delta(com: char) -> Self {
        match com {
            'U' => Coord::new((0, -1)),
            'D' => Coord::new((0, 1)),
            'L' => Coord::new((-1, 0)),
            'R' => Coord::new((1, 0)),
            _ => unreachable!(),
        }
    }

    // フィールド外に出ることもある
    fn move_by(&self, c: char) -> Self {
        let delta = Coord::com_to_delta(c);
        self.plus(&delta)
    }
}

#[allow(dead_code)]
struct Input {
    start: Coord,
    tiles: Vec<Vec<usize>>,  // タイルナンバー
    points: Vec<Vec<isize>>, // points
}

struct Output {
    input: Input,
    rng: ThreadRng,
    pos: Coord,
    score: isize,
    gone: Vec<bool>,
    ans: Vec<char>,
}

impl Output {
    fn new(input: Input) -> Output {
        let start = input.start;
        let mut gone = vec![false; SIDE * SIDE];
        gone[*start.access_matrix(&input.tiles)] = true;
        let score = *start.access_matrix(&input.points);
        let rng = rand::thread_rng();
        Output {
            input,
            rng,
            pos: start,
            score,
            gone,
            ans: vec![],
        }
    }

    fn is_gone_pos(&self, pos: &Coord) -> bool {
        let tile = pos.access_matrix(&self.input.tiles);
        self.gone[*tile]
    }

    // valid な命令である前提
    fn do_command(&mut self, com: char) {
        self.ans.push(com);
        self.pos = self.pos.move_by(com);
        self.score += self.pos.access_matrix(&self.input.points);
        // goneを埋める
        let tile = self.pos.access_matrix(&self.input.tiles);
        self.gone[*tile] = true;
    }

    fn solve(&mut self) {
        loop {
            let mut coms = COMS.to_vec();
            coms.shuffle(&mut self.rng);
            let mut flag = false;
            for c in coms {
                let next = self.pos.move_by(c);
                if next.in_field() {
                    if !self.is_gone_pos(&next) {
                        flag = true;
                        self.do_command(c);
                    }
                }
            }

            if !flag {
                break;
            }
        }
    }
}

#[fastout]
fn main() {
    let system_time = SystemTime::now();

    input! {
        sy: isize,
        sx: isize,
        tiles: [[usize; SIDE]; SIDE],
        points: [[isize; SIDE]; SIDE],
    }

    let start = Coord::new((sx, sy));
    let input = Input {
        start,
        tiles,
        points,
    };

    let mut output = Output::new(input);

    output.solve();

    println!("{}", output.ans.iter().collect::<String>());

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}
