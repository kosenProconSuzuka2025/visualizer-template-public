use wasm_bindgen::prelude::*;

#[allow(non_snake_case, unused_macros)]

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

const MOD: i64 = 998244353;

#[derive(Clone, Debug)]
pub struct Input {
    size: usize,
    board: Vec<Vec<usize>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.size)?;
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{} ", self.board[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let mut f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        size: usize,
        board: [[usize; size]; size],
    }
    Input { size, board }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

pub struct Output {
    out: Vec<(usize, usize, usize)>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut ss = f.split_whitespace();
    let L = read(ss.next(), 0..MOD)?;
    let mut out = vec![];
    for _ in 0..L {
        let x = read(ss.next(), 0..input.size)?;
        let y = read(ss.next(), 0..=input.size)?;
        let n = read(ss.next(), 0..=input.size)?;
        out.push((x, y, n));
    }
    if ss.next().is_some() {
        return Err(format!("Too many outputs"));
    }
    Ok(Output { out })
}

pub fn gen(seed: i32) -> Input { 
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let size = rng.gen_range(4..24);
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
    Input { size, board }
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, mut err, _) = compute_score_details(input, &out.out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub fn compute_score_details(input: &Input, out: &[(usize, usize, usize)]) -> (i64, String, Vec<Vec<usize>>) {
    let mut b = input.board.clone();
    for &(x, y, n) in out {
        for i in 0..n{
            for j in 0..n {
                if x + i >= input.size.try_into().unwrap() || y + j >= input.size.try_into().unwrap() {
                    return (0, "Out of range".to_string(), b);
                }
                let mut c = b.clone();
                // 右に90度回転
                let xi: usize = x + i;
                let yi: usize = y + n - 1 - i;
                let xj: usize = x + j;
                let yj: usize = y + j;
                c[xj][yi] = b[xi][yj];
            }
        }
    }
    let mut score = 0;

    // スコア計算あとで書いてね
    (score, String::new(), b)
}

pub fn color(mut val: i32) -> String {
    let (r, g, b) = if val == 0 {
        // 赤
        (255, 0, 0)
    }else {
        // 青
        (0, 0, 255)
    };
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn group(title: String) -> Group {
    Group::new().add(Title::new(title))
}

pub fn vis_default(input: &Input, out: &Output) -> (i64, String, String) {
    let (mut score, err, svg) = vis(input, &out.out, true);
    if err.len() > 0 {
        score = 0;
    }
    (score, err, svg)
}

pub fn vis(input: &Input, out: &[(usize, usize, usize)], show_number: bool) -> (i64, String, String) {
    let D = 10;
    let W = D * input.size;
    let H = D * input.size;
    let (score, err, b) = compute_score_details(input, &out);
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central;}}"
    )));
    for i in 0..input.size {
        for j in 0..input.size {
            let mut c = 0;
            // 上下左右に同じ数があれば青
            let dx = [-1, 1, 0, 0];
            let dy = [0, 0, -1, 1];
            for k in 0..4 {
                let ni = i as i32 + dx[k];
                let nj = j as i32 + dy[k];
                if ni >= 0 && ni < input.size as i32 && nj >= 0 && nj < input.size as i32 {
                    if b[i][j] == b[ni as usize][nj as usize] {
                        c += 1;
                        break;
                    }
                }
            }
            let mut g = group(format!("b[{},{}] = {}", i, j, b[i][j])).add(rect(j * D, i * D, D, D, &color(c)));
            if show_number {
                g = g.add(
                    Text::new(format!("{}", b[i][j]))
                        .set("x", j * D + D / 2)
                        .set("y", i * D + D / 2)
                        .set("font-size", D / 6)
                );
            }
            doc = doc.add(g);
        }
    }
    if out.len() > 0 {
        let (i,j,n) = out[out.len() - 1];
        doc = doc.add(
            rect(j * D, i * D, n * D, n * D, "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
        );
    }
    (score, err, doc.to_string())
}