use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;

fn add_neighbours(pos: (i32, i32), n: usize, set: &mut HashSet<(i32, i32)>) {
    let dx: Vec<i32> = vec![1, 1, 1, 0, 0, -1, -1, -1];
    let dy: Vec<i32> = vec![1, 0, -1, 1, -1, 1, 0, -1];
    let (px, py) = pos;
    for i in 0..8 {
        let px1 = px + dx[i];
        let py1 = py + dy[i];
        if 0 <= px1 && px1 < n as i32 && 0 <= py1 && py1 < n as i32 {
            set.insert((px1, py1));
        }
    }
}

fn live_neighbours(pos: (i32, i32), n: usize, mat: &mut Vec<Vec<u32>>) -> u32 {
    let dx: Vec<i32> = vec![1, 1, 1, 0, 0, -1, -1, -1];
    let dy: Vec<i32> = vec![1, 0, -1, 1, -1, 1, 0, -1];
    let (px, py) = pos;
    let mut ret = 0;
    for i in 0..8 {
        let px1 = px + dx[i];
        let py1 = py + dy[i];
        if 0 <= px1 && px1 < n as i32 && 0 <= py1 && py1 < n as i32 && mat[px1 as usize][py1 as usize] == 1 {
            ret += 1;
        }
    }
    ret
}

fn next_val(pos: (i32, i32), n: usize, mat: &mut Vec<Vec<u32>>) -> u32 {
    let (px, py) = pos;
    let curr = mat[px as usize][py as usize];
    let live = live_neighbours(pos, n, mat);
    if curr == 1 && 2 <= live && live <= 3 {
        1
    } else if curr == 0 && live == 3 {
        1
    } else {
        0
    }
}

fn add_corners(n: usize, mat: &mut Vec<Vec<u32>>, set: &mut HashSet<(i32, i32)>) {
    mat[0][0] = 1;
    mat[0][n - 1] = 1;
    mat[n - 1][0] = 1;
    mat[n - 1][n - 1] = 1;

    let n = n as i32;

    set.insert((0, 1));
    set.insert((1, 0));
    set.insert((1, 1));
    set.insert((0, n - 2));
    set.insert((1, n - 1));
    set.insert((1, n - 2));
    set.insert((n - 2, 0));
    set.insert((n - 1, 1));
    set.insert((n - 2, 1));
    set.insert((n - 2, n - 1));
    set.insert((n - 1, n - 2));
    set.insert((n - 2, n - 2));
}

fn main() {
    let mut f = File::open(Path::new("/Users/PetarV/rust-proj/advent-of-rust/target/input.txt"))
    	.ok()
    	.expect("Failed to open the input file!");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.ok()
		.expect("Failed to read from the input file!");

    let n = 100;

    let mut mat = vec![vec![0; n]; n];

    let mut changed = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                mat[i][j] = 1;
                changed.insert((i as i32, j as i32));
                add_neighbours((i as i32, j as i32), n, &mut changed);
            } else {
                mat[i][j] = 0;
            }
        }
    }

    for _ in 0..100 {
        let mut next_changed = HashSet::new();
        let mut next_mat = mat.clone();
        for pos in changed.iter() {
            let (x, y) = *pos;
            let next = next_val((x, y), n, &mut mat);
            if next != mat[x as usize][y as usize] {
                next_changed.insert((x, y));
                add_neighbours((x, y), n, &mut next_changed);
            }
            next_mat[x as usize][y as usize] = next;
        }
        changed = next_changed;
        mat = next_mat;
    }

    let ret = mat.iter().fold(0, |acc, x| { 
        acc + x.iter().fold(0, |acc, y| {
            acc + y
        })
    });

    println!("There are {} lights in the \"on\" state.", ret);

    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                mat[i][j] = 1;
                changed.insert((i as i32, j as i32));
                add_neighbours((i as i32, j as i32), n, &mut changed);
            } else {
                mat[i][j] = 0;
            }
        }
    }

    add_corners(n, &mut mat, &mut changed);

    for _ in 0..100 {
        let mut next_changed = HashSet::new();
        let mut next_mat = mat.clone();
        for pos in changed.iter() {
            let (x, y) = *pos;
            let next = next_val((x, y), n, &mut mat);
            if next != mat[x as usize][y as usize] {
                next_changed.insert((x, y));
                add_neighbours((x, y), n, &mut next_changed);
            }
            next_mat[x as usize][y as usize] = next;
        }
        add_corners(n, &mut next_mat, &mut next_changed);
        changed = next_changed;
        mat = next_mat;
    }

    let ret = mat.iter().fold(0, |acc, x| { 
        acc + x.iter().fold(0, |acc, y| {
            acc + y
        })
    });

    println!("There are {} lights in the \"on\" state under the new rules.", ret);
}