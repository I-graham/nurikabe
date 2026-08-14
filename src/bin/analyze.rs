use nurikabe::*;
use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let easy = std::fs::File::create("easy.b64l")?;
    let medium = std::fs::File::create("medium.b64l")?;
    let hard = std::fs::File::create("hard.b64l")?;
    let crazy = std::fs::File::create("crazy.b64l")?;
    let fs = std::fs::read_to_string("allUnsolved.b64l")?;

    let v: Vec<_> = fs
        .lines()
        .map(|s| s.trim())
        .enumerate()
        .map(|(i, line)| {
            dbg!(i);
            dbg!(line);
            Board::from_b64(line)
        })
        .collect();

    let mut v: Vec<_> = v
        .into_iter()
        .enumerate()
        .map(|(i, board)| {
            dbg!(i);
            let soln = solve(&board);
            let ratio = soln.hypothetical_steps() as f32 / soln.steps() as f32;
            (board, soln, ratio)
        })
        .collect();

    v.sort_by(|(_, _, r), (_, _, s)| r.partial_cmp(s).unwrap());

    v.retain(|(_, _, r)| *r < 0.35);
    let len = dbg!(v.len());

    let div_1 = (len as f32 * 0.3).round() as usize;
    let div_2 = (len as f32 * 0.7).round() as usize;
    let div_3 = (len as f32 * 0.9).round() as usize;

    let slice = |boards: &[(Board, Solution, f32)], mut file: File| {
        for (board, soln, _) in boards.iter() {
            writeln!(file, "{}", board.b64()).unwrap();
            writeln!(file, "{}", soln.forced_board().b64()).unwrap();
        }
    };

    slice(&v[..div_1], easy);
    slice(&v[div_1..div_2], medium);
    slice(&v[div_2..div_3], hard);
    slice(&v[div_3..], crazy);
    Ok(())
}
