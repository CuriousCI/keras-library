#[cfg(test)]
mod tests {
    extern crate test;

    use std::collections::HashMap;
    use std::fmt::Write;
    use std::fs::read;
    use test::Bencher;

    enum Note {
        None,
        Normal(char, usize),
        Accidental(char, usize, char),
        Unknown(char, usize, char),
    }

    use Note::*;

    #[bench]
    fn translation_01(bencher: &mut Bencher) {
        let path =
            Path::new("D:/university/python/hw4-rust/workdir/test10/0/03/032/0320/03201/0.txt");

        let score: Vec<_> = read(path)
            .unwrap()
            .iter()
            .map(|byte| match byte {
                10 => '\n',
                32 => 'P',
                43 => '#',
                45 => 'b',
                byte => (byte + 17) as char,
            })
            .collect();

        bencher.iter(|| {
            let mut duration = 0;
            let mut note = None;
            let mut s = String::new();

            for staff in score.split(|char| char == &'\n') {
                for &symbol in staff.iter().rev() {
                    if symbol != '#' && symbol != 'b' {
                        duration += 1;
                    }

                    note = match note {
                        None => Normal(symbol, 1),
                        Normal(note, duration) => match symbol {
                            '#' | 'b' => {
                                if duration > 1 {
                                    write!(s, "{note}{}", duration - 1).unwrap();
                                }

                                Accidental(note, 1, symbol)
                            }
                            _ => {
                                if symbol == note {
                                    Normal(note, duration + 1)
                                } else {
                                    write!(s, "{note}{duration}").unwrap();
                                    Normal(symbol, 1)
                                }
                            }
                        },
                        Accidental(note, duration, accidental) => {
                            if symbol == note {
                                Unknown(note, duration, accidental)
                            } else {
                                write!(s, "{note}{accidental}{duration}").unwrap();
                                Normal(symbol, 1)
                            }
                        }
                        Unknown(note, duration, accidental) => match symbol {
                            '#' | 'b' => {
                                if symbol != accidental {
                                    write!(s, "{note}{accidental}{duration}").unwrap();
                                    Accidental(note, 1, symbol)
                                } else {
                                    Accidental(note, duration + 1, accidental)
                                }
                            }
                            _ => {
                                if symbol == note {
                                    write!(s, "{note}{accidental}{duration}").unwrap();
                                    Normal(note, 2)
                                } else {
                                    write!(s, "{note}{accidental}{duration}{note}1").unwrap();
                                    Normal(symbol, 1)
                                }
                            }
                        },
                    }
                }
            }

            match note {
                Normal(note, duration) => write!(s, "{note}{duration}"),
                Accidental(note, duration, accidental) => {
                    write!(s, "{note}{accidental}{duration}")
                }
                Unknown(note, d, accidental) => write!(s, "{note}{accidental}{d}{note}1"),
                _ => unreachable!(), // _ => Ok(()), // _ => unreachable!(),
            }
            .unwrap();
        });
    }

    // TODO: find biggest file

    #[bench]
    fn translation_02(bencher: &mut Bencher) {
        enum Note {
            None,
            Normal,
            Accidental,
            Unknown,
        }

        let path =
            Path::new("D:/university/python/hw4-rust/workdir/test10/0/03/032/0320/03201/0.txt");

        let score: Vec<_> = read(path)
            .unwrap()
            .iter()
            .map(|byte| match byte {
                10 => '\n',
                32 => 'P',
                43 => '#',
                45 => 'b',
                byte => (byte + 17) as char,
            })
            .collect();

        bencher.iter(|| {
            let mut duration = 0;
            let mut v = '0';
            let mut d = 0;
            let mut a = '0';
            let mut note = Note::None;
            let mut s = String::new();

            // if symbol != '#' && symbol != 'b' {
            //     duration += 1;
            // }

            for staff in score.split(|char| char == &'\n') {
                for &symbol in staff.iter().rev() {
                    match symbol {
                        '#' | 'b' => duration += 1,
                        _ => (),
                    }

                    note = match note {
                        Note::None => {
                            v = symbol;
                            d = 1;
                            Note::Normal
                        }
                        Note::Normal => match symbol {
                            '#' | 'b' => {
                                if d > 1 {
                                    write!(s, "{v}{}", d - 1).unwrap();
                                }
                                d = 1;
                                a = symbol;
                                Note::Accidental
                            }
                            _ => {
                                if symbol == v {
                                    d += 1;
                                    Note::Normal
                                } else {
                                    write!(s, "{v}{d}").unwrap();
                                    v = symbol;
                                    d = 1;
                                    Note::Normal
                                }
                            }
                        },
                        Note::Accidental => {
                            if symbol == v {
                                Note::Unknown
                            } else {
                                write!(s, "{v}{a}{d}").unwrap();
                                v = symbol;
                                d = 1;
                                Note::Normal
                            }
                        }
                        Note::Unknown => match symbol {
                            '#' | 'b' => {
                                if symbol != a {
                                    write!(s, "{v}{a}{d}").unwrap();
                                    d = 1;
                                    a = symbol;
                                    Note::Accidental
                                } else {
                                    d += 1;
                                    Note::Accidental
                                }
                            }
                            _ => {
                                if symbol == v {
                                    write!(s, "{v}{a}{d}").unwrap();
                                    d = 2;
                                    Note::Normal
                                } else {
                                    write!(s, "{v}{a}{d}{v}1").unwrap();
                                    v = symbol;
                                    d = 1;
                                    Note::Normal
                                }
                            }
                        },
                    }
                }
            }

            match note {
                Note::Normal => write!(s, "{v}{d}"),
                Note::Accidental => {
                    write!(s, "{v}{a}{d}")
                }
                Note::Unknown => write!(s, "{v}{a}{d}{v}1"),
                _ => unreachable!(), // _ => Ok(()), // _ => unreachable!(),
            }
            .unwrap();
        });
    }

    #[bench]
    fn translation_03(bencher: &mut Bencher) {
        enum Note {
            None,
            Normal,
            Accidental,
            Unknown,
        }

        let path =
            Path::new("D:/university/python/hw4-rust/workdir/test10/0/03/032/0320/03201/0.txt");

        let score: Vec<_> = read(path)
            .unwrap()
            .iter()
            .map(|byte| match byte {
                10 => '\n',
                32 => 'P',
                43 => '#',
                45 => 'b',
                byte => (byte + 17) as char,
            })
            .collect();

        bencher.iter(|| {
            let mut duration = 0;
            let mut v = '0';
            let mut d = 0;
            let mut a = '0';
            let mut note = Note::None;
            let mut s = String::new();

            for staff in score.split(|char| char == &'\n') {
                for &symbol in staff.iter().rev() {
                    note = match symbol {
                        '#' | 'b' => {
                            duration += 1;
                            match note {
                                Note::Normal => {
                                    if d > 1 {
                                        write!(s, "{v}{}", d - 1).unwrap();
                                    }
                                    d = 1;
                                    a = symbol;
                                    Note::Accidental
                                }
                                Note::Unknown => {
                                    if symbol != a {
                                        write!(s, "{v}{a}{d}").unwrap();
                                        d = 1;
                                        a = symbol;
                                        Note::Accidental
                                    } else {
                                        d += 1;
                                        Note::Accidental
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => match note {
                            Note::None => {
                                v = symbol;
                                d = 1;
                                Note::Normal
                            }
                            Note::Normal => {
                                if symbol == v {
                                    d += 1;
                                    Note::Normal
                                } else {
                                    write!(s, "{v}{d}").unwrap();
                                    v = symbol;
                                    d = 1;
                                    Note::Normal
                                }
                            }
                            Note::Accidental => {
                                if symbol == v {
                                    Note::Unknown
                                } else {
                                    write!(s, "{v}{a}{d}").unwrap();
                                    v = symbol;
                                    d = 1;
                                    Note::Normal
                                }
                            }
                            Note::Unknown => {
                                if symbol == v {
                                    write!(s, "{v}{a}{d}").unwrap();
                                    d = 2;
                                    Note::Normal
                                } else {
                                    write!(s, "{v}{a}{d}{v}1").unwrap();
                                    v = symbol;
                                    d = 1;
                                    Note::Normal
                                }
                            }
                        },
                    };
                }
            }

            match note {
                Note::Normal => write!(s, "{v}{d}"),
                Note::Accidental => {
                    write!(s, "{v}{a}{d}")
                }
                Note::Unknown => write!(s, "{v}{a}{d}{v}1"),
                _ => unreachable!(), // _ => Ok(()), // _ => unreachable!(),
            }
            .unwrap();
        });
    }

    #[bench]
    fn translation_04(bencher: &mut Bencher) {
        enum Note {
            None,
            Normal,
            Accidental,
            Unknown,
        }

        let path =
            Path::new("D:/university/python/hw4-rust/workdir/test10/0/03/032/0320/03201/0.txt");

        let score: Vec<_> = read(path)
            .unwrap()
            .iter()
            .map(|byte| match byte {
                10 => '\n',
                32 => 'P',
                43 => '#',
                45 => 'b',
                byte => (byte + 17) as char,
            })
            .collect();

        bencher.iter(|| {
            let mut duration = 0;
            let mut v = '0';
            let mut d = 0;
            let mut a = '0';
            let mut note = Note::None;
            let mut s = String::new();

            for staff in score.split(|char| char == &'\n') {
                for &symbol in staff.iter().rev() {
                    note = match symbol {
                        '#' | 'b' => {
                            duration += 1;

                            match note {
                                Note::Normal => {
                                    if d > 1 {
                                        write!(s, "{v}{}", d - 1).unwrap();
                                    }
                                    d = 1;
                                    a = symbol;
                                    Note::Accidental
                                }
                                Note::Unknown => {
                                    if symbol != a {
                                        write!(s, "{v}{a}{d}").unwrap();
                                        d = 1;
                                        a = symbol;
                                        Note::Accidental
                                    } else {
                                        d += 1;
                                        Note::Accidental
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => match note {
                            Note::None => {
                                v = symbol;
                                d = 1;
                                Note::Normal
                            }
                            Note::Normal => {
                                if symbol == v {
                                    d += 1;
                                    Note::Normal
                                } else {
                                    write!(s, "{v}{d}").unwrap();
                                    v = symbol;
                                    d = 1;
                                    Note::Normal
                                }
                            }
                            Note::Accidental => {
                                if symbol == v {
                                    Note::Unknown
                                } else {
                                    write!(s, "{v}{a}{d}").unwrap();
                                    v = symbol;
                                    d = 1;
                                    Note::Normal
                                }
                            }
                            Note::Unknown => {
                                if symbol == v {
                                    write!(s, "{v}{a}{d}").unwrap();
                                    d = 2;
                                    Note::Normal
                                } else {
                                    write!(s, "{v}{a}{d}{v}1").unwrap();
                                    v = symbol;
                                    d = 1;
                                    Note::Normal
                                }
                            }
                        },
                    };
                }
            }

            match note {
                Note::Normal => write!(s, "{v}{d}"),
                Note::Accidental => {
                    write!(s, "{v}{a}{d}")
                }
                Note::Unknown => write!(s, "{v}{a}{d}{v}1"),
                _ => unreachable!(), // _ => Ok(()), // _ => unreachable!(),
            }
            .unwrap();
        });
    }

    #[bench]
    fn translation_05(bencher: &mut Bencher) {
        enum Note {
            None,
            Normal,
            Accidental,
            Unknown,
        }

        let path =
            Path::new("D:/university/python/hw4-rust/workdir/test10/0/03/032/0320/03201/0.txt");

        let score: Vec<_> = read(path)
            .unwrap()
            .iter()
            .map(|byte| match byte {
                10 => '\n',
                32 => 'P',
                43 => '#',
                45 => 'b',
                byte => (byte + 17) as char,
            })
            .collect();

        bencher.iter(|| {
            let mut duration = 0;
            let mut v = '0';
            let mut d = 0;
            let mut a = '0';
            let mut note = Note::None;
            let mut s = String::new();

            for staff in score.split(|char| char == &'\n') {
                for &symbol in staff.iter().rev() {
                    note = if symbol == v {
                        match note {
                            Note::Normal => {
                                d += 1;
                                Note::Normal
                            }
                            Note::Accidental => Note::Unknown,
                            Note::Unknown => {
                                write!(s, "{v}{a}{d}").unwrap();
                                d = 2;
                                Note::Normal
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        match symbol {
                            '#' | 'b' => {
                                duration += 1;

                                match note {
                                    Note::Normal => {
                                        if d > 1 {
                                            write!(s, "{v}{}", d - 1).unwrap();
                                        }
                                        d = 1;
                                        a = symbol;
                                    }
                                    Note::Unknown => {
                                        if symbol != a {
                                            write!(s, "{v}{a}{d}").unwrap();
                                            d = 1;
                                            a = symbol;
                                        } else {
                                            d += 1;
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                                Note::Accidental
                            }
                            _ => {
                                match note {
                                    Note::Normal => write!(s, "{v}{d}"),
                                    Note::Accidental => write!(s, "{v}{a}{d}"),
                                    Note::Unknown => write!(s, "{v}{a}{d}{v}1"),
                                    Note::None => Ok(()),
                                }
                                .unwrap();

                                v = symbol;
                                d = 1;
                                Note::Normal
                            }
                        }
                    }
                }
            }

            match note {
                Note::Normal => write!(s, "{v}{d}"),
                Note::Accidental => {
                    write!(s, "{v}{a}{d}")
                }
                Note::Unknown => write!(s, "{v}{a}{d}{v}1"),
                _ => unreachable!(), // _ => Ok(()), // _ => unreachable!(),
            }
            .unwrap();
        });
    }

    #[bench]
    fn string_push(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut s = String::new();
            let note = 'a';
            let mut duration = 1;

            for _ in 0..10000 {
                s.push(note);
                s.push_str(&duration.to_string());
                duration = (duration + 1) % 20;
            }
        })
    }

    #[bench]
    fn sort_stable(bencher: &mut Bencher) {
        let source_folder = Path::new("D:/university/python/hw4-rust/workdir/test03/");
        let index = read_to_string(source_folder.join("index.txt")).unwrap();

        let mut index: Vec<_> = index
            .lines()
            .map(|line| (line, line.len() as i32))
            .collect();

        bencher.iter(|| {
            index.sort_by_key(|(line, len)| (-len, line.to_owned()));
        })
    }

    #[bench]
    fn sort_unstable(bencher: &mut Bencher) {
        let source_folder = Path::new("D:/university/python/hw4-rust/workdir/test03/");
        let index = read_to_string(source_folder.join("index.txt")).unwrap();

        let mut index: Vec<_> = index
            .lines()
            .map(|line| (line, line.len() as i32))
            .collect();

        bencher.iter(|| {
            index.sort_unstable_by_key(|(line, len)| (-len, line.to_owned()));
        })
    }

    #[bench]
    fn string_format(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut s = String::new();
            let note = 'a';
            let mut duration = 1;

            for _ in 0..10000 {
                s.push_str(&format!("{note}{duration}"));
                duration = (duration + 1) % 20;
            }
        })
    }

    #[bench]
    fn string_write(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut s = String::new();
            let note = 'a';
            let mut duration = 1;

            for _ in 0..10000 {
                write!(s, "{note}{duration}").unwrap();
                duration = (duration + 1) % 20;
            }
        })
    }

    #[bench]
    fn string_vec_join(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut s = vec![];
            let note = 'a';
            let mut duration = 1;

            for _ in 0..10000 {
                s.push(format!("{note}{duration}"));
                duration = (duration + 1) % 20;
            }

            let a = s.join("");
            a
        })
    }

    #[bench]
    fn composition_01(bencher: &mut Bencher) {
        let source_folder = Path::new("/home/cicio/projects/keras-library/workdir/test03/");

        let index = read_to_string(source_folder.join("index.txt"))
            .unwrap()
            .replace("\" \"", "\r")
            .replace('"', "");

        bencher.iter(|| {
            let index: Vec<_> = index
                .split('\n')
                .filter_map(|line| line.split_once('\r'))
                .collect();

            let mut songs = Vec::with_capacity(index.len());

            for (title, file) in index {
                songs.push((title, file))
            }
        })
    }

    #[bench]
    fn composition_02(bencher: &mut Bencher) {
        let source_folder = Path::new("/home/cicio/projects/keras-library/workdir/test03/");

        let index = read_to_string(source_folder.join("index.txt"))
            .unwrap()
            .replace("\" \"", "\r")
            .replace('"', "");

        bencher.iter(|| {
            let mut songs = vec![];

            for (title, file) in index.split('\n').filter_map(|line| line.split_once('\r')) {
                songs.push((title, file))
            }
        })
    }

    #[bench]
    fn composition_03(bencher: &mut Bencher) {
        let source_folder = Path::new("/home/cicio/projects/keras-library/workdir/test03/");

        let index = read_to_string(source_folder.join("index.txt"))
            .unwrap()
            .replace("\" \"", "\r")
            .replace('"', "");

        bencher.iter(|| {
            let mut songs = Vec::new();

            for (title, file) in index.split('\n').filter_map(|line| line.split_once('\r')) {
                songs.push((title, file))
            }
        })
    }

    #[bench]
    fn separation_00(bencher: &mut Bencher) {
        // let source_folder = Path::new("/home/cicio/projects/keras-library/workdir/test03/");
        let source_folder = Path::new("D:/university/python/hw4-rust/workdir/test03/");

        let index = read_to_string(source_folder.join("index.txt")).unwrap();

        bencher.iter(|| {
            let index = index.replace("\" \"", "\r").replace('"', "");

            for _x in index.lines().filter_map(|line| line.split_once('\r')) {}
        })
    }

    #[bench]
    fn separation_01(bencher: &mut Bencher) {
        // let source_folder = Path::new("/home/cicio/projects/keras-library/workdir/test03/");
        let source_folder = Path::new("D:/university/python/hw4-rust/workdir/test03/");

        let index = read_to_string(source_folder.join("index.txt")).unwrap();

        bencher.iter(|| {
            let index = index.replace("\" \"", "\r").replace('"', "");

            for _x in index.split('\n').filter_map(|line| line.split_once('\r')) {}
        })
    }

    #[bench]
    fn separation_02(bencher: &mut Bencher) {
        // let source_folder = path::new("/home/cicio/projects/keras-library/workdir/test03/");
        let source_folder = Path::new("D:/university/python/hw4-rust/workdir/test03/");

        let index = read_to_string(source_folder.join("index.txt")).unwrap();

        bencher.iter(|| {
            let index = index.replace('"', "\r");

            for _x in index
                .split('\n')
                .filter_map(|line| line.trim().split_once("\r \r"))
            {}
        })
    }

    #[bench]
    fn separation_03(bencher: &mut Bencher) {
        // let source_folder = Path::new("/home/cicio/projects/keras-library/workdir/test03/");
        let source_folder = Path::new("D:/university/python/hw4-rust/workdir/test03/");

        let index = read_to_string(source_folder.join("index.txt")).unwrap();

        bencher.iter(|| {
            for _x in index.split('\n').filter_map(|line| {
                line.strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"').and_then(|s| s.split_once("\" \"")))
            }) {}
        })
    }

    #[bench]
    fn separation_04(bencher: &mut Bencher) {
        // let source_folder = Path::new("/home/cicio/projects/keras-library/workdir/test03/");
        let source_folder = Path::new("D:/university/python/hw4-rust/workdir/test03/");

        let mut index = read_to_string(source_folder.join("index.txt")).unwrap();
        index.pop();

        bencher.iter(|| {
            for _x in index
                .split('\n')
                .filter_map(|line| line[1..line.len() - 1].split_once("\" \""))
            // line.trim_matches('"').split_once("\" \"")
            {}
        })
    }

    fn run_on_folder<'a>(
        test_name: &str,
        bencher: &mut Bencher,
        f: &dyn Fn(&Path, &Path) -> HashMap<&'a str, i32>,
    ) {
        let workdir = Path::new("workdir");
        let source_folder = workdir.join(test_name);
        let target_folder = workdir.join(format!("{test_name}.out"));

        bencher.iter(|| {
            if target_folder.exists() {
                remove_dir_all(&target_folder).expect("Couldn't remove target_folder");
            }

            f(&source_folder, &target_folder);
            // umkansanize(&source_folder, &target_folder);
        })
    }

    #[bench]
    fn fast_bench_test_01(bencher: &mut Bencher) {
        run_on_folder("test01", bencher, &umkansanize);
    }

    #[bench]
    fn fast_bench_test_02(bencher: &mut Bencher) {
        run_on_folder("test02", bencher, &umkansanize);
    }

    #[bench]
    fn slow_bench_test_03(bencher: &mut Bencher) {
        run_on_folder("test03", bencher, &umkansanize);
    }

    #[bench]
    fn fast_bench_test_04(bencher: &mut Bencher) {
        run_on_folder("test04", bencher, &umkansanize);
    }

    #[bench]
    fn fast_bench_test_05(bencher: &mut Bencher) {
        run_on_folder("test05", bencher, &umkansanize);
    }

    #[bench]
    fn fast_bench_test_06(bencher: &mut Bencher) {
        run_on_folder("test06", bencher, &umkansanize);
    }

    #[bench]
    fn fast_bench_test_07(bencher: &mut Bencher) {
        run_on_folder("test07", bencher, &umkansanize);
    }

    #[bench]
    fn fast_bench_test_08(bencher: &mut Bencher) {
        run_on_folder("test08", bencher, &umkansanize);
    }

    #[bench]
    fn fast_bench_test_09(bencher: &mut Bencher) {
        run_on_folder("test09", bencher, &umkansanize);
    }

    #[bench]
    fn slow_bench_test_10(bencher: &mut Bencher) {
        run_on_folder("test10", bencher, &umkansanize);
    }

    #[bench]
    fn fast_bench_test_threads_01(bencher: &mut Bencher) {
        run_on_folder("test01", bencher, &thread::umkansanize);
    }

    #[bench]
    fn fast_bench_test_threads_02(bencher: &mut Bencher) {
        run_on_folder("test02", bencher, &thread::umkansanize);
    }

    #[bench]
    fn slow_bench_test_threads_03(bencher: &mut Bencher) {
        run_on_folder("test03", bencher, &thread::umkansanize);
    }

    #[bench]
    fn fast_bench_test_threads_04(bencher: &mut Bencher) {
        run_on_folder("test04", bencher, &thread::umkansanize);
    }

    #[bench]
    fn fast_bench_test_threads_05(bencher: &mut Bencher) {
        run_on_folder("test05", bencher, &thread::umkansanize);
    }

    #[bench]
    fn fast_bench_test_threads_06(bencher: &mut Bencher) {
        run_on_folder("test06", bencher, &thread::umkansanize);
    }

    #[bench]
    fn fast_bench_test_threads_07(bencher: &mut Bencher) {
        run_on_folder("test07", bencher, &thread::umkansanize);
    }

    #[bench]
    fn fast_bench_test_threads_08(bencher: &mut Bencher) {
        run_on_folder("test08", bencher, &thread::umkansanize);
    }

    #[bench]
    fn fast_bench_test_threads_09(bencher: &mut Bencher) {
        run_on_folder("test09", bencher, &thread::umkansanize);
    }

    #[bench]
    fn slow_bench_test_threads_10(bencher: &mut Bencher) {
        run_on_folder("test10", bencher, &thread::umkansanize);
    }

    #[bench]
    fn fast_bench_test_pool_01(bencher: &mut Bencher) {
        run_on_folder("test01", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn fast_bench_test_pool_02(bencher: &mut Bencher) {
        run_on_folder("test02", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn slow_bench_test_pool_03(bencher: &mut Bencher) {
        run_on_folder("test03", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn fast_bench_test_pool_04(bencher: &mut Bencher) {
        run_on_folder("test04", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn fast_bench_test_pool_05(bencher: &mut Bencher) {
        run_on_folder("test05", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn fast_bench_test_pool_06(bencher: &mut Bencher) {
        run_on_folder("test06", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn fast_bench_test_pool_07(bencher: &mut Bencher) {
        run_on_folder("test07", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn fast_bench_test_pool_08(bencher: &mut Bencher) {
        run_on_folder("test08", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn fast_bench_test_pool_09(bencher: &mut Bencher) {
        run_on_folder("test09", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn slow_bench_test_pool_10(bencher: &mut Bencher) {
        run_on_folder("test10", bencher, &thread::pool::umkansanize);
    }

    use crate::{thread, umkansanize};
    use std::fs::{read_to_string, remove_dir_all};
    use std::path::{Path, PathBuf};

    fn check_filesystem(test_name: &str) {
        let workdir = Path::new("workdir");
        let source_folder = workdir.join(test_name);
        let target_folder = workdir.join(&format!("{test_name}.out"));
        let expected_folder = workdir.join(&format!("{test_name}.expected"));

        if Path::new(&target_folder).exists() {
            remove_dir_all(&target_folder).expect("Couldn't remove target_folder")
        }

        umkansanize(&source_folder, &target_folder);

        let index = read_to_string(source_folder.join("index.txt"))
            .unwrap()
            .replace('"', "\r");

        let mut expected_songs_files: Vec<PathBuf> = index
            .trim()
            .split("\n")
            .map(|line| line.trim().split_once("\r \r").unwrap())
            .map(|(song_name, song_file)| {
                Path::new(song_file)
                    .parent()
                    .unwrap()
                    .join(format!("{}.txt", song_name))
            })
            .collect();
        expected_songs_files.push(Path::new("index.txt").to_owned());

        for song_file in expected_songs_files {
            let target_song: String = read_to_string(target_folder.join(&song_file))
                .expect(&format!("File not found {}", song_file.to_str().unwrap()))
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            let expected_song: String = read_to_string(expected_folder.join(&song_file))
                .expect(&format!("File not found {}", song_file.to_str().unwrap()))
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            if target_song != expected_song {
                panic!("Songs \"{}\" are different!", song_file.to_str().unwrap());
            }
        }
    }

    #[test]
    fn test_01() {
        check_filesystem("test01")
    }

    #[test]
    fn test_02() {
        check_filesystem("test02")
    }

    #[test]
    fn test_03() {
        check_filesystem("test03")
    }

    #[test]
    fn test_04() {
        check_filesystem("test04")
    }

    #[test]
    fn test_05() {
        check_filesystem("test05")
    }

    #[test]
    fn test_06() {
        check_filesystem("test06")
    }

    #[test]
    fn test_07() {
        check_filesystem("test07")
    }

    #[test]
    fn test_08() {
        check_filesystem("test08")
    }

    #[test]
    fn test_09() {
        check_filesystem("test09")
    }

    #[test]
    fn test_10() {
        check_filesystem("test10")
    }
}
