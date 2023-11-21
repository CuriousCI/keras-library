#[cfg(test)]
mod tests {
    extern crate test;

    use std::fmt::Write;
    use test::Bencher;

    fn run_on_folder(test_name: &str, bencher: &mut Bencher, f: &dyn Fn(&Path, &Path) -> ()) {
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
    fn bench_test_01(bencher: &mut Bencher) {
        run_on_folder("test01", bencher, &umkansanize);
    }

    #[bench]
    fn bench_test_02(bencher: &mut Bencher) {
        run_on_folder("test02", bencher, &umkansanize);
    }

    // #[bench]
    // fn bench_test_03(bencher: &mut Bencher) {
    //     run_on_folder("test03", bencher, &umkansanize);
    // }

    #[bench]
    fn bench_test_04(bencher: &mut Bencher) {
        run_on_folder("test04", bencher, &umkansanize);
    }

    #[bench]
    fn bench_test_05(bencher: &mut Bencher) {
        run_on_folder("test05", bencher, &umkansanize);
    }

    #[bench]
    fn bench_test_06(bencher: &mut Bencher) {
        run_on_folder("test06", bencher, &umkansanize);
    }

    #[bench]
    fn bench_test_07(bencher: &mut Bencher) {
        run_on_folder("test07", bencher, &umkansanize);
    }

    #[bench]
    fn bench_test_08(bencher: &mut Bencher) {
        run_on_folder("test08", bencher, &umkansanize);
    }

    #[bench]
    fn bench_test_09(bencher: &mut Bencher) {
        run_on_folder("test09", bencher, &umkansanize);
    }

    // #[bench]
    // fn bench_test_10(bencher: &mut Bencher) {
    //     run_on_folder("test10", bencher, &umkansanize);
    // }

    #[bench]
    fn bench_test_threads_01(bencher: &mut Bencher) {
        run_on_folder("test01", bencher, &thread::umkansanize);
    }

    #[bench]
    fn bench_test_threads_02(bencher: &mut Bencher) {
        run_on_folder("test02", bencher, &thread::umkansanize);
    }

    // #[bench]
    // fn bench_test_threads_03(bencher: &mut Bencher) {
    //     run_on_folder("test03", bencher, &thread::umkansanize);
    // }

    #[bench]
    fn bench_test_threads_04(bencher: &mut Bencher) {
        run_on_folder("test04", bencher, &thread::umkansanize);
    }

    #[bench]
    fn bench_test_threads_05(bencher: &mut Bencher) {
        run_on_folder("test05", bencher, &thread::umkansanize);
    }

    #[bench]
    fn bench_test_threads_06(bencher: &mut Bencher) {
        run_on_folder("test06", bencher, &thread::umkansanize);
    }

    #[bench]
    fn bench_test_threads_07(bencher: &mut Bencher) {
        run_on_folder("test07", bencher, &thread::umkansanize);
    }

    #[bench]
    fn bench_test_threads_08(bencher: &mut Bencher) {
        run_on_folder("test08", bencher, &thread::umkansanize);
    }

    #[bench]
    fn bench_test_threads_09(bencher: &mut Bencher) {
        run_on_folder("test09", bencher, &thread::umkansanize);
    }

    // #[bench]
    // fn bench_test_threads_10(bencher: &mut Bencher) {
    //     run_on_folder("test10", bencher, &thread::umkansanize);
    // }

    #[bench]
    fn bench_test_pool_01(bencher: &mut Bencher) {
        run_on_folder("test01", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn bench_test_pool_02(bencher: &mut Bencher) {
        run_on_folder("test02", bencher, &thread::pool::umkansanize);
    }

    // #[bench]
    // fn bench_test_pool_03(bencher: &mut Bencher) {
    //     run_on_folder("test03", bencher, &thread::pool::umkansanize);
    // }

    #[bench]
    fn bench_test_pool_04(bencher: &mut Bencher) {
        run_on_folder("test04", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn bench_test_pool_05(bencher: &mut Bencher) {
        run_on_folder("test05", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn bench_test_pool_06(bencher: &mut Bencher) {
        run_on_folder("test06", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn bench_test_pool_07(bencher: &mut Bencher) {
        run_on_folder("test07", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn bench_test_pool_08(bencher: &mut Bencher) {
        run_on_folder("test08", bencher, &thread::pool::umkansanize);
    }

    #[bench]
    fn bench_test_pool_09(bencher: &mut Bencher) {
        run_on_folder("test09", bencher, &thread::pool::umkansanize);
    }

    // #[bench]
    // fn bench_test_pool_10(bencher: &mut Bencher) {
    //     run_on_folder("test10", bencher, &thread::pool::umkansanize);
    // }

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
