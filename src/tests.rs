#[cfg(test)]
mod tests {
    use crate::umkansanize;
    use std::fs::{remove_dir_all, File};
    use std::io::{BufReader, Read};
    use std::path::Path;

    fn read_music_score(path: &String) -> String {
        "".to_string()
    }

    fn check_filesystem(test_name: &str) {
        let source_folder = format!("workdir/{}", test_name);
        let target_folder = format!("workdir/{}.out", test_name);
        let expected_results_folder = format!("workdir/{}.expected", test_name);

        if Path::new(&target_folder).exists() {
            remove_dir_all(&target_folder).expect(&format!("Couldn't remove {}", target_folder))
        }

        umkansanize(&source_folder, &target_folder);

        let file = File::open(format!("{}/index.txt", source_folder))
            .expect("Source index.txt file not found!");

        let mut index = String::new();
        BufReader::new(file)
            .read_to_string(&mut index)
            .expect("Failed reading index!");
        index = index.replace('"', "\r");

        let mut expected_songs_files: Vec<String> = index
            .trim()
            .split("\n")
            .map(|line| line.trim().split_once("\r \r").unwrap())
            .map(|(song_name, song_file)| {
                format!(
                    "{}/{}.txt",
                    Path::new(song_file)
                        .parent()
                        .expect("Path doesn't have a file")
                        .to_str()
                        .expect("Path doesn't have valid unicode"),
                    song_name
                )
            })
            .collect();
        expected_songs_files.push("index.txt".to_string());

        for song_file in expected_songs_files {
            let target_song = format!("{}/{}", target_folder, song_file);
            let target_song_file = File::open(&target_song)
                .expect(&format!("The file \"{}\" non esiste!", target_song));

            let mut target_song_score = String::new();
            BufReader::new(target_song_file)
                .read_to_string(&mut target_song_score)
                .expect(&format!("Failed reading \"{}\"!", target_song));
            target_song_score = target_song_score
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            let expected_song = format!("{}/{}", expected_results_folder, song_file);
            let expected_song_file = File::open(&expected_song)
                .expect(&format!("The file \"{}\" non esiste!", expected_song));

            let mut expected_song_score = String::new();
            BufReader::new(expected_song_file)
                .read_to_string(&mut expected_song_score)
                .expect(&format!("Failed reading \"{}\"!", expected_song));
            expected_song_score = expected_song_score
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            if target_song_score != expected_song_score {
                panic!("Songs \"{}\" are different!", target_song)
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
