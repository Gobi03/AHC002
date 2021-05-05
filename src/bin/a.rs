#[allow(unused_imports)]
use proconio::marker::Chars;
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::time::SystemTime;

use std::fs;
use std::io::Write;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const SIDE: usize = 50;
const COMS: [char; 4] = ['U', 'L', 'D', 'R'];
const DIV: isize = 10;
const W: isize = SIDE as isize;
const H: isize = SIDE as isize;

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
        (0 <= self.x && self.x < W) && (0 <= self.y && self.y < H)
    }

    // ペアへの変換
    fn to_pair(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    // マンハッタン距離
    fn distance(&self, that: &Self) -> isize {
        let dist_x = max(self.x, that.x) - min(self.x, that.x);
        let dist_y = max(self.y, that.y) - min(self.y, that.y);
        dist_x + dist_y
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&p| self.plus(&Coord::new(p)))
            .filter(|&pos| pos.in_field())
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

    fn block_coord(&self) -> Coord {
        Coord::new((self.x / DIV, self.y / DIV))
    }

    fn next_block(&self) -> Coord {
        let end = SIDE as isize / DIV - 1;
        // Coord::new((x, y))
        let &Coord { x, y } = self;
        if x == end {
            if y == end {
                Coord::new((x - 1, y))
            } else if y % 2 == 0 {
                Coord::new((x, y + 1))
            } else {
                Coord::new((x - 1, y))
            }
        } else if x == end - 1 {
            if y == end {
                Coord::new((x - 1, y))
            } else if y % 2 == 0 {
                Coord::new((x + 1, y))
            } else {
                Coord::new((x, y + 1))
            }
        } else if y == 0 {
            if x == end {
                Coord::new((x, y + 1))
            } else {
                Coord::new((x + 1, y))
            }
        } else {
            if x == 0 && y == 1 {
                Coord::new((x, y - 1))
            } else {
                if (y == 1 && x % 2 == 0) || (y == end && x % 2 == 1) {
                    Coord::new((x - 1, y))
                } else {
                    if x % 2 == 0 {
                        Coord::new((x, y - 1))
                    } else {
                        Coord::new((x, y + 1))
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
struct Input {
    start: Coord,
    tiles: Vec<Vec<usize>>,  // タイルナンバー
    points: Vec<Vec<isize>>, // points
}

#[allow(dead_code)]
struct Output {
    input: Input,
    rng: ThreadRng,
}

impl Output {
    fn new(input: Input) -> Output {
        let rng = rand::thread_rng();
        Output { input, rng }
    }

    fn solve(&self, sys_time: &SystemTime) -> String {
        // dfs
        let dfs_init = State::new(&self.input);

        const DFS_TIMEOUT: u128 = 200;
        let mut dfs_ans = dfs_init.clone();
        let mut best_score = 0;

        let mut reprs = vec![dfs_init];
        while !reprs.is_empty() && sys_time.elapsed().unwrap().as_millis() < DFS_TIMEOUT {
            for _ in 0..500 {
                if reprs.is_empty() {
                    break;
                }

                let now = reprs.pop().unwrap();

                for &c in COMS.iter() {
                    let next = now.pos.move_by(c);
                    if next.in_field() {
                        if !now.is_gone_pos(&next, &self.input) {
                            let mut next_st = now.clone();
                            next_st.do_command(c, &self.input);

                            if best_score < next_st.score {
                                best_score = next_st.score;
                                dfs_ans = next_st.clone();
                            }

                            reprs.push(next_st);
                        }
                    }
                }
            }
        }

        let mut file_cnt = 1;

        let mut f = fs::File::create(format!("tools/output/{}.txt", file_cnt)).unwrap();
        f.write_all(dfs_ans.ans.iter().collect::<String>().as_bytes())
            .unwrap();

        // 山登り
        let mut rng = thread_rng();
        const CLIMB_TIMEOUT: u128 = 5900;

        let mut res = dfs_ans;

        while sys_time.elapsed().unwrap().as_millis() < CLIMB_TIMEOUT {
            let mut st = res.clone();

            let ansl = st.ans.len();
            let l = rng.gen_range(5, 100);
            let si = rng.gen_range(0, ansl - l);

            // 指定の範囲の操作を抜き出す(中抜きのケースと右端が無いケースがあるのに注意)
            let mut coms = st.ans.clone();
            let mut cut = coms.split_off(si);
            let lasts = cut.split_off(min(cut.len(), l));

            // eprintln!("{:?}", cut);

            let mut pos = self.input.start.clone();
            for &c in &coms {
                pos = pos.move_by(c);
            }
            // gone を消す & goal地点を求める
            let mut goal_pos = pos.clone();
            for &c in &cut {
                goal_pos = goal_pos.move_by(c);
                st.set_gone_pos(&goal_pos, false, &self.input);
                st.score -= goal_pos.access_matrix(&self.input.points);
            }

            // eprintln!("hoge");
            // 接続先に向けてルート探索
            st.ans = coms;
            st.pos = pos;
            //eprintln!("{}", st.score);
            let mut q = vec![st.clone()];
            for _ in 0..l * 5 {
                if q.is_empty() {
                    break;
                }

                let mut next_q = vec![];
                if lasts.is_empty() {
                    let nst = &q[0];
                    if nst.score > res.score {
                        let mut hoge = nst.clone();
                        hoge.ans.append(&mut lasts.clone());
                        res = hoge;

                        file_cnt += 1;
                        let mut f =
                            fs::File::create(format!("tools/output/{}.txt", file_cnt)).unwrap();
                        f.write_all(res.ans.iter().collect::<String>().as_bytes())
                            .unwrap();
                    }
                }

                for k in 0..min(q.len(), 500) {
                    let nst = &q[k];
                    if !lasts.is_empty() && nst.pos == goal_pos {
                        if nst.score > res.score {
                            // eprintln!("{} {} {}", si, res.score, nst.score);
                            let mut hoge = nst.clone();
                            hoge.ans.append(&mut lasts.clone());
                            res = hoge;

                            file_cnt += 1;
                            let mut f =
                                fs::File::create(format!("tools/output/{}.txt", file_cnt)).unwrap();
                            f.write_all(res.ans.iter().collect::<String>().as_bytes())
                                .unwrap();
                        }
                    } else {
                        for &c in &COMS {
                            let mut next_st = nst.clone();
                            let next_pos = next_st.pos.move_by(c);
                            if next_pos.in_field() && !next_st.is_gone_pos(&next_pos, &self.input) {
                                next_st.do_command(c, &self.input);
                                next_q.push(next_st);
                            }
                        }
                    }
                }

                next_q.sort_by(|st1, st2| st2.score.partial_cmp(&st1.score).unwrap());
                q = next_q;
            }
        }

        eprintln!("{}", file_cnt);
        res.ans.iter().collect::<String>()
    }
}

#[derive(Clone)]
struct State {
    pos: Coord,
    score: isize,
    gone: Vec<bool>,
    ans: Vec<char>,
}

impl State {
    fn new(input: &Input) -> Self {
        let start = input.start;
        let mut gone = vec![false; SIDE * SIDE];
        gone[*start.access_matrix(&input.tiles)] = true;
        let score = *start.access_matrix(&input.points);
        State {
            pos: start,
            score,
            gone,
            ans: vec![],
        }
    }

    fn is_gone_pos(&self, pos: &Coord, input: &Input) -> bool {
        let tile = pos.access_matrix(&input.tiles);
        self.gone[*tile]
    }

    fn set_gone_pos(&mut self, pos: &Coord, b: bool, input: &Input) {
        let tile = pos.access_matrix(&input.tiles);
        self.gone[*tile] = b;
    }

    // valid な命令である前提
    fn do_command(&mut self, com: char, input: &Input) {
        self.ans.push(com);
        self.pos = self.pos.move_by(com);
        self.score += self.pos.access_matrix(&input.points);
        // goneを埋める
        let tile = self.pos.access_matrix(&input.tiles);
        self.gone[*tile] = true;
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

    let output = Output::new(input);

    let ans = output.solve(&system_time);

    println!("{}", ans);

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}
