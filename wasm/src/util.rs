#![allow(non_snake_case, unused_macros)]
use proconio::input;
use rand::prelude::*;
use svg::node::element::{Group, Style, Rectangle, Text, Title};

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Clone, Debug)]
pub struct Input {
    pub size: usize,
    pub board: Vec<Vec<usize>>,
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
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        size: usize,
        board: [[usize; size]; size],
    }
    Input { size, board }
}

pub struct Output {
    pub L: usize,
    pub out: Vec<(usize, usize, usize)>,
}

pub fn parse_output(f: &str) -> Output {
    let mut f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        L: usize,
        out: [(usize, usize, usize); L],
    }
    Output { L, out }
}

pub fn gen(seed: u64) -> Input { 
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let size = rng.gen_range(2..12) * 2;
    
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

pub fn color(val: i32) -> String {
    let (r, g, b) = if val == 0 {
        // 赤
        (228, 164, 222)
    }else {
        // 青
        (175, 223, 228)
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

fn calculate_score(input: &Input, output: &Output) -> i64 {
    0
}

pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    let score = calculate_score(input, output);
    let mut b = input.board.clone();
    let out = &output.out;

    let D = 50;
    let W = D * input.size;
    let H = D * input.size;
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");

    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
        6
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
            g = g.add(
                Text::new(format!("{}", b[i][j]))
                    .set("x", j * D + D / 2)
                    .set("y", i * D + D / 2)
                    .set("font-size", D / 3)
            );
            doc = doc.add(g);
        }
    }
    if out.len() > 0 {
        let (i,j,n) = out[turn];
        doc = doc.add(
            rect(j * D, i * D, n * D, n * D, "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
        );
    }
    (score as i64, "".to_string(), doc.to_string())
}