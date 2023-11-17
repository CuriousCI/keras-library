#![feature(test)]
pub mod tests;

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;

// TODO: use later in threads
// pub fn translate_song() {}

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
        .collect();
    // Instead of collecting here, make threads and  send them rx and tx to hashmap later

    let mut songs_durations: HashMap<String, usize> = HashMap::new();

    let (songs_transmitter, songs_receiver) = channel();
    let threads_number = musical_scores_index.len();

    for (song_name, song_file) in musical_scores_index {
        // TODO: make a thread for each file, pay attention to shared hashmap
        let song_name = song_name.to_owned();
        let song_file = song_file.to_owned();
        let songs_transmitter = songs_transmitter.clone();
        let source_folder = source_folder.to_owned();
        let target_folder = target_folder.to_owned();

        thread::spawn(move || {
            let tarahumara_umkansanian_dictionary = vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 98, 0, 0, 65, 66, 67, 68,
                69, 70, 71,
            ];

            let mut musical_score = Vec::new();
            BufReader::new(File::open(source_folder.join(&song_file)).unwrap())
                .read_to_end(&mut musical_score)
                .unwrap();

            musical_score = musical_score
                .iter()
                .map(|&byte| tarahumara_umkansanian_dictionary[byte as usize])
                .collect();

            // TODO: prealloc musical score size, in reality you can't know!
            let mut result = vec![];
            let mut song_duration = 0;

            enum NoteType {
                None,
                Normal,
                Altered,
            }

            let mut note_type = NoteType::None;
            let mut note_value: u8 = 0;
            let mut note_duration: usize = 0;
            let mut alterator: u8 = 0;

            for staff in musical_score.split(|&byte| byte == 10 as u8) {
                for &note in staff.iter().rev() {
                    if note != 35 && note != 98 {
                        song_duration += 1;
                    }

                    match note_type {
                        NoteType::None => {
                            note_value = note;
                            note_duration = 1;
                            note_type = NoteType::Normal;
                        }
                        NoteType::Normal => {
                            if alterator > 0 {
                                if note == 35 || note == 98 {
                                    if alterator != note {
                                        result.push(note_value);
                                        result.push(alterator);
                                        result.extend_from_slice(
                                            note_duration.to_string().as_bytes(),
                                        );

                                        alterator = note;
                                        note_duration = 1;
                                    } else {
                                        note_duration += 1;
                                    }

                                    note_type = NoteType::Altered;
                                } else if note_value != note {
                                    result.push(note_value);
                                    result.push(alterator);
                                    result.extend_from_slice(note_duration.to_string().as_bytes());
                                    result.push(note_value);
                                    result.push(49); // 1 in ASCII

                                    note_value = note;
                                    note_duration = 1;
                                    alterator = 0;
                                } else {
                                    result.push(note_value);
                                    result.push(alterator);
                                    result.extend_from_slice(note_duration.to_string().as_bytes());

                                    note_duration = 2;
                                    alterator = 0;
                                }
                            } else {
                                if note == 35 || note == 98 {
                                    if note_duration > 1 {
                                        result.push(note_value);
                                        result.extend_from_slice(
                                            (note_duration - 1).to_string().as_bytes(),
                                        );

                                        note_duration = 1;
                                    }

                                    alterator = note;
                                    note_type = NoteType::Altered;
                                } else if note_value != note {
                                    result.push(note_value);
                                    result.extend_from_slice(note_duration.to_string().as_bytes());

                                    note_value = note;
                                    note_duration = 1;
                                } else {
                                    note_duration += 1;
                                }
                            }
                        }
                        NoteType::Altered => {
                            if note_value != note {
                                result.push(note_value);
                                result.push(alterator);
                                result.extend_from_slice(note_duration.to_string().as_bytes());

                                note_duration = 1;
                                alterator = 0;
                                note_value = note;
                            }

                            note_type = NoteType::Normal;
                        }
                    }
                }
            }

            result.push(note_value);
            if alterator > 0 {
                result.push(alterator);
            }
            result.extend_from_slice(note_duration.to_string().as_bytes());

            match note_type {
                NoteType::Normal => {
                    if alterator > 0 {
                        result.push(note_value);
                        result.push(49);
                    }
                }
                _ => (),
            }

            songs_transmitter.send((song_name.to_string(), song_duration));
            // songs_durations.insert(song_name.to_string(), song_duration);

            // Save index.txt :)

            let song_path = target_folder
                .join(song_file)
                .with_file_name(song_name)
                .with_extension(".txt");

            create_dir_all(song_path.parent().unwrap()).unwrap();
            File::create(song_path).unwrap().write_all(&result).unwrap();
        });
    }

    // for (song_name, song_duration) in
    for _ in 0..threads_number {
        let (song_name, song_duration) = songs_receiver.recv().unwrap();
        songs_durations.insert(song_name, song_duration);
    }

    // TODO: make this cleaner
    let mut index = File::create(target_folder.join("index.txt")).unwrap();
    let mut songs: Vec<(&String, &usize)> = songs_durations.iter().collect();
    songs.sort_by_key(|(song_name, &song_duration)| (song_duration as i64 * -1, *song_name));
    for (song_name, songs_duration) in songs {
        writeln!(index, "\"{}\" {}", song_name, songs_duration).unwrap();
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
// TODO: set size of songs_paths to musical_scores_index.len()
