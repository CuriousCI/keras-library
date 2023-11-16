pub mod tests;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

// TODO: use later in threads
pub fn translate_song() {}

// Change HashMap into HashMap<&str, usize>
pub fn umkansanize(source_folder: &str, target_folder: &str) -> HashMap<String, usize> {
    let index_file = File::open(format!("{}/index.txt", source_folder))
        .expect("Source index.txt file not found!");

    let mut index = String::new();
    BufReader::new(index_file)
        .read_to_string(&mut index)
        .expect("Failed reading index!");

    index = index.replace('"', "\r");
    let musical_scores_index: Vec<(&str, &str)> = index
        .trim()
        .split("\n")
        .map(|line| line.trim().split_once("\r \r").unwrap())
        .collect();

    let mut songs_durations: HashMap<String, usize> = HashMap::new();

    for (song_name, song_file) in musical_scores_index {
        // TODO: make a thread for each file, pay attention to shared hashmap
        let mut musical_score = Vec::new();
        BufReader::new(File::open(format!("{}/{}", source_folder, song_file)).unwrap())
            .read_to_end(&mut musical_score)
            .unwrap();

        let tarahumara_umkansanian_dictionary = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 98, 0, 0, 65, 66, 67, 68, 69, 70, 71,
        ];

        // + - 43 -> 35 - #
        // - - 45 -> 98 - b
        // 0 - 48 -> 65 - A
        // 1 - 49 -> 66 - B
        // 2 - 50 -> 67 - C
        // 3 - 51 -> 68 - D
        // 4 - 52 -> 69 - E
        // 5 - 53 -> 70 - F
        // 6 - 54 -> 71 - G
        // mask: 111111
        // 6 bits are enough
        // 11 indices, so 11 * 6 bits are needed
        //>>> numbers = [43, 45, 48, 49, 50, 51, 52, 53, 54]
        // >>> print(list(map(lambda x: x - 43, numbers)))
        // [0, 2, 5, 6, 7, 8, 9, 10, 11]
        // >>> translated = [35, 98, 65, 66, 67, 68, 69, 70, 71]
        // >>> print(list(map(lambda x: x - 35, translated)))
        // [0, 63, 30, 31, 32, 33, 34, 35, 36]

        // println!("{:?}", musical_score);
        musical_score = musical_score
            .iter()
            .map(|&byte| tarahumara_umkansanian_dictionary[byte as usize])
            .collect();

        // Go from bottom to top, and put new char to the left of the previous one
        //
        // let lines: Vec<u8> = musical_score
        // .split(|&byte| byte == 10 as u8)
        // .map(|line| line.iter().rev().collect())
        // .chain()
        // .collect();

        for byte in musical_score {}

        // Translate song into new language using vector map
        // Replace # and b
        // Tokenize directly with double token?
        // Save bytes into vec, and save data to file
        songs_durations.insert(song_name.to_string(), 10);
    }

    songs_durations
    // println!("{:?}", source_songs);
}

// Possible file reading optimization
// let start = 10;
// let count = 10;
//
// let mut f = File::open("/etc/passwd")?;
// f.seek(SeekFrom::Start(start))?;
// let mut buf = vec![0; count];
// f.read_exact(&mut buf)?;
//
//
// Data Structure for paths to build the least amount of paths required
