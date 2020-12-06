use std::io::Result as IoResult;
use std::time::Instant;

use aoc_henk::map_file;

type AocExecFn = fn(usize, &[u8]) -> Option<String>;

pub const EXECS: &[(&str, AocExecFn)] =
    &[("Henk", aoc_henk::execute), ("Stefan", aoc_stefan::execute)];

fn main() -> IoResult<()> {
    let args = std::env::args();
    if args.len() == 1 {
        println!("Running all programs");
        for i in 1..25 {
            for (name, execute) in EXECS {
                run_prog(execute, name, i)?;
            }
        }
    } else {
        for arg in args.skip(1) {
            let i = arg.parse::<usize>().unwrap(); //FIXME
            for (name, execute) in EXECS {
                run_prog(execute, name, i)?;
            }
        }
    }
    Ok(())
}

fn run_prog(exec: &AocExecFn, name: &str, id: usize) -> IoResult<()> {
    let input = map_file(format!("henk/input/{:02}.in", id))?;
    let expected_output = map_file(format!("henk/input/{:02}.out", id))?;

    let start_ts = Instant::now();
    let output = exec(id, &input);
    let end_ts = Instant::now();

    //println!("{}", output);
    if output.is_none() {
        println!("{}/{} could not be executed", id, name);
        return Ok(());
    }
    //FIXME: C++ code does not return data yet
    //assert_eq!(*output.unwrap().as_bytes(), *expected_output);

    let duration = end_ts.duration_since(start_ts);
    println!("{}/{} took {:?}", id, name, duration);
    Ok(())
}
