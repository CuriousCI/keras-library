pub mod tests;

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

// TODO: use later in threads
pub fn translate_song() {}

// TODO: Change HashMap into HashMap<&str, usize>
pub fn umkansanize(source_folder: &str, target_folder: &str) -> HashMap<String, usize> {
    let source_folder = Path::new(source_folder);
    let target_folder = Path::new(target_folder);

    let mut source_index = String::new();
    BufReader::new(File::open(source_folder.join("index.txt")).unwrap())
        .read_to_string(&mut source_index)
        .unwrap();

    source_index = source_index.replace('"', "\r");
    let musical_scores_index: Vec<(&str, &str)> = source_index
        .split('\n')
        .filter_map(|line| line.trim().split_once("\r \r"))
        // Instead of collecting here, make threads and  send them rx and tx to hashmap later
        .collect();

    let mut songs_durations: HashMap<String, usize> = HashMap::new();
    let mut songs_paths: Vec<(PathBuf, Vec<u8>)> = vec![];
    // TODO: set size of songs_paths to musical_scores_index.len()

    for (song_name, song_file) in musical_scores_index {
        // TODO: make a thread for each file, pay attention to shared hashmap
        let mut musical_score = Vec::new();
        BufReader::new(File::open(source_folder.join(song_file)).unwrap())
            .read_to_end(&mut musical_score)
            .unwrap();

        let tarahumara_umkansanian_dictionary = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 98, 0, 0, 65, 66, 67, 68, 69, 70, 71,
        ];

        musical_score = musical_score
            .iter()
            .map(|&byte| tarahumara_umkansanian_dictionary[byte as usize])
            .collect();

        // TODO: prealloc musical score size, in reality you can't know!
        let mut result = vec![];
        let mut prev_token1 = 0;
        let mut token1 = 0; // Works directly with result
        let mut prev_token2 = 0;
        let mut prev_token2_sign = 0;
        let mut token2 = 0; // Works with token1
        let mut song_duration = 0;
        // let mut previous_note: u8 = 0;
        // let mut current_note: u8 = 0;
        // let mut current_base: u8 = 0;
        // let mut previous_note_count: u8 = 0;

        // if note == 98 || note == 35 {
        //     current_note = current_base + note;
        // } else {
        //     current_base = note;
        // }
        //
        // if current_note != previous_note {
        //     if previous_note > 99 {
        //         let offset = if previous_note > 162 { 98 } else { 35 };
        //         result.push(previous_note - offset);
        //         result.push(offset);
        //     } else if previous_note > 0 {
        //         result.push(previous_note);
        //     }
        //     result.push(48 + previous_note_count); // '0' + count
        //
        //     // previous_note = current_note;
        //     previous_note = note;
        //     duration += previous_note_count as usize;
        //     previous_note_count = 1;
        // }
        for staff in musical_score.split(|&byte| byte == 10 as u8) {
            for &note in staff.iter().rev() {
                let mut checktoken1 = false;
                if note == 98 || note == 35 {
                    prev_token2_sign = note;
                    if prev_token2 != token2 {
                        checktoken1 = true;
                    }
                } else {
                    token2 = note;
                    // prev bemolle
                }

                if checktoken1 {
                    // Do token1 thingy
                }
            }
        }

        songs_durations.insert(song_name.to_string(), song_duration);
        // TODO: vector with tuples with song dir/file and actual song Vec<u8>

        // TODO: do this cleaner
        let new_path = Path::new(song_file)
            .parent()
            .unwrap()
            .join(Path::new(format!("{}.txt", song_name).as_str()));

        songs_paths.push((new_path, result));
    }

    for (song_path, song) in songs_paths {
        create_dir_all(Path::new(target_folder).join(&song_path).parent().unwrap()).unwrap();

        File::create(Path::new(target_folder).join(&song_path))
            .unwrap()
            .write_all(&song)
            .unwrap();
    }

    songs_durations
}

// .write_all(song.leak())
// .write_all(song.as_slice())
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
//
//' '- 20 -> 80 - P
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

// musical_score.iter().rev()

// Go from bottom to top, and put new char to the left of the previous one
//
// let lines: Vec<&[u8]> = musical_score
//     .split(|&byte| byte == 10 as u8)
//     .rev()
//     .collect();
// Save in HashMap song_name and duration
// songs_durations[song_name, duration];
// result.push(previous_note);
// result.push(previous_note_count);

// if previous_note > di certo valore che indica diesis o bemolle
// scomponi previous_note e aggiungi
// result.push(previous_note)
// result.push(bemolle / diesis in base
//
// + - 43 -> 35 - #
// - - 45 -> 98 - b

// Translate song into new language using vector map
// Replace # and b
// Tokenize directly with double token?
// Save bytes into vec, and save data to file
// if previous_note > 162 {
//     // Use if inside if to assign 'offset'
//     result.push(previous_note - 98);
//     result.push(98);
// } else if previous_note > 99 {
//     result.push(previous_note - 35);
//     result.push(35);
// } else if previous_note > 0 {
//     result.push(previous_note);
// }
// result.push(previous_note_count);
// let index_file = File::open(source_folder.join("index.txt"))
// let index_file = File::open(format!("{}/index.txt", source_folder))
// .expect("Source index.txt file not found!");
// .expect("Failed reading index!");
// BufReader::new(File::open(format!("{}/{}", source_folder, song_file)).unwrap())
// TODO: NOPE
// if note == 98 || note == 35 {
//     current_note = current_base + note;
// } else {
//     current_base = note;
// }
//
// if current_note != previous_note {
//     if previous_note > 99 {
//         let offset = if previous_note > 162 { 98 } else { 35 };
//         result.push(previous_note - offset);
//         result.push(offset);
//     } else if previous_note > 0 {
//         result.push(previous_note);
//     }
//     result.push(48 + previous_note_count); // '0' + count
//
//     // previous_note = current_note;
//     previous_note = note;
//     duration += previous_note_count as usize;
//     previous_note_count = 1;
// }
// TODO: enum for diesis and bemolle?
// if previous_note > 99 {
//     let offset = if previous_note > 162 { 98 } else { 35 };
//     result.push(previous_note - offset);
//     result.push(offset);
// } else if previous_note > 0 {
//     result.push(previous_note);
// }
// result.push(previous_note_count);
// duration += previous_note_count as usize;
//
//
//
//
//
//
//
//
//
//
// AAbAbAbAAAbAbAAAAA
// A ~
// A ~
// b ! => A salvata, Ab nuova
// A ~
// b ! => Ab + 1
// A ~
// b ! => Ab + 1
// A ~
// A ! => Ab salvata, A nuova
// A ~
// b ! => A + 1 salvata, Ab nuova
// A ~
// b ! => Ab + 1
// A ~
// A ! => Ab salvata, A nuova
// A ! => A + 1
// A ! => A + 1
// ~ ! => A + 1salvata
