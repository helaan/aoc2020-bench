use criterion::Criterion;
use aoc_henk::map_file;

type AocExecFn = fn(usize, &[u8]) -> Option<String>;

pub const EXECS: &[(&str, AocExecFn)] =
    &[("Henk", aoc_henk::execute), ("Stefan", aoc_stefan::execute), ("Michiel", aoc_michiel::execute)];

fn main() -> std::io::Result<()> {
    let output_dir = std::path::Path::new("./measurement");
    let mut c = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_millis(2000))
        .output_directory(output_dir);

    let args = std::env::args();
    if args.len() == 1 {
        for day in 1..=25 {
            let input = if let Ok(mmap) = map_file(format!("henk/input/{:02}.in", day)) {
                mmap
            } else {
                break;
            };
            let mut group = c.benchmark_group(format!("day_{}", day));

            for (name, exec) in EXECS {
                group.bench_function(*name, |b| b.iter(|| exec(day, &input) ));
            }

            group.finish();
        }
    } else {
        let day = args.skip(1).next().unwrap().parse::<usize>().unwrap();
        let input = map_file(format!("henk/input/{:02}.in", day)).unwrap();
        let mut group = c.benchmark_group(format!("day_{}", day));

        for (name, exec) in EXECS {
            group.bench_function(*name, |b| b.iter(|| exec(day, &input) ));
        }

        group.finish();
    }

    Ok(())
}
