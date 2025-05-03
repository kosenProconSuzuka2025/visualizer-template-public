use wasm_bindgen::prelude::*;

use itertools::Itertools;
use proconio::input;
use rand::prelude::*;
use std::ops::RangeBounds;
use svg::node::element::{Group, Rectangle, Style, Text, Title};

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
($($e:expr),*) => { Vec::from(vec![$($e),*]) };
($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[wasm_bindgen]
pub fn gen(seed: i32) -> String { 
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let mut size = rng.gen_range(2..12);
    size = size * 2;
    let mut board = mat![0; size; size];
    let maxBoard = size * size / 2;
    let mut vec = vec![0; size * size];
    for i in 0..maxBoard {
        vec[i * 2] = i;
        vec[i * 2 + 1] = i;
    }
    for i in 0..size {
        for j in 0..size {
            // vecをシャッフル
            let idx = rng.gen_range(0..vec.len());
            board[i][j] = vec[idx];
            vec.remove(idx);
        }
    }
    let mut s = format!("{}\n", size);
    for i in 0..size {
        for j in 0..size {
            s += &format!("{} ", board[i][j]);
        }
        s += "\n";
    }
    s
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    Ret {
        score: 0,
        err: "".to_string(),
        svg: "".to_string(),
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    0
}