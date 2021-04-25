#[allow(unused_imports)]
use proconio::marker::Chars;
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

use rand::{thread_rng, Rng};
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const SIDE: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
impl Coord {
    fn new(p: (usize, usize)) -> Self {
        Coord { x: p.0, y: p.1 }
    }

    fn from_isize_pair(pos: (isize, isize)) -> Self {
        Coord {
            x: pos.0 as usize,
            y: pos.1 as usize,
        }
    }

    fn in_field(pos: (isize, isize)) -> bool {
        (0 <= pos.0 && pos.0 < SIDE as isize) && (0 <= pos.1 && pos.1 < SIDE as isize)
    }

    // ペアへの変換
    fn to_pair(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    fn to_isize_pair(&self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }

    // マンハッタン距離
    fn distance(&self, that: &Self) -> usize {
        let dist_x = max(self.x, that.x) - min(self.x, that.x);
        let dist_y = max(self.y, that.y) - min(self.y, that.y);
        dist_x + dist_y
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let (ix, iy) = self.to_isize_pair();
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&(dx, dy)| (ix + dx, iy + dy))
            .filter(|&p| Coord::in_field(p))
            .map(|p| Coord::from_isize_pair(p))
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
        &mat[self.y][self.x]
    }

    fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y][self.x] = e;
    }
}

#[fastout]
fn main() {
    let system_time = SystemTime::now();

    input! {
        si: usize,
        sj: usize,
        t: [[isize; SIDE]; SIDE],
        p: [[isize; SIDE]; SIDE],
    }

    let ans = "D";

    println!("{}", ans);

    println!("{}ms", system_time.elapsed().unwrap().as_millis());
}
