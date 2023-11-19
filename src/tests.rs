#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    fn run_on_folder(test_name: &str, bencher: &mut Bencher) {
        let source_folder = Path::new("workdir").join(test_name);
        let target_folder = Path::new("workdir").join(format!("{test_name}.out"));

        if target_folder.exists() {
            remove_dir_all(&target_folder).expect("Couldn't remove targetr_folder");
        }

        bencher.iter(|| {
            umkansanize(&source_folder, &target_folder);
        })
    }
    #[bench]
    fn bench_test_folders(bencher: &mut Bencher) {
        run_on_folder("test01", bencher);
        run_on_folder("test02", bencher);
        run_on_folder("test03", bencher);
        run_on_folder("test04", bencher);
        run_on_folder("test05", bencher);
        run_on_folder("test06", bencher);
        run_on_folder("test07", bencher);
        run_on_folder("test08", bencher);
        run_on_folder("test09", bencher);
        run_on_folder("test10", bencher);
    }

    use crate::umkansanize;
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

// .expect("String doesn't start with \"")
// .expect("String doesn't end with \"")
// .expect("Index file has bad format!")
