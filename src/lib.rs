#![feature(test)]
pub mod tests;

use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, read, read_to_string, write, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread::spawn;

enum Note {
    None,
    Normal(char, usize),
    Accidental(char, usize, char),
    Unknown(char, usize, char),
}

use Note::*;

// pub fn umkansanize(source_folder: &Path, target_folder: &Path) -> HashMap<String, i32> {
pub fn umkansanize(source_folder: &Path, target_folder: &Path) {
    let source_index = read_to_string(source_folder.join("index.txt"))
        .unwrap()
        .replace("\" \"", "\r")
        .replace('"', "");

    let songs_index: Vec<(&str, &str)> = source_index
        .split('\n')
        .filter_map(|line| line.split_once('\r'))
        .collect();

    let tarahumara_umkansanian_dictionary: &Vec<u8> = &vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 98, 0, 0, 65, 66, 67, 68, 69, 70, 71,
    ];

    let paths: HashSet<PathBuf> = songs_index
        .iter()
        .map(|(_, file)| target_folder.join(file).parent().unwrap().to_owned())
        .collect();

    for path in paths {
        create_dir_all(path).unwrap();
    }

    let threads_number = songs_index.len();
    let (tx, rx) = channel();

    for (song_name, song_file) in songs_index {
        let musical_score: Vec<char> = read(source_folder.join(&song_file))
            .unwrap()
            .iter()
            .map(|&byte| tarahumara_umkansanian_dictionary[byte as usize] as char)
            .collect();

        let song_name = song_name.to_owned();
        let song_file = song_file.to_owned();
        let target_folder = target_folder.to_owned();
        let tx = tx.clone();

        spawn(move || {
            let mut result = String::new();
            let mut song_duration = 0;
            let mut note = None;

            for staff in musical_score.split(|&byte| byte == '\n') {
                for &symbol in staff.iter().rev() {
                    if symbol != '#' && symbol != 'b' {
                        song_duration += 1;
                    }

                    note = match note {
                        None => Normal(symbol, 1),
                        Normal(note, duration) => match symbol {
                            '#' | 'b' => {
                                if duration > 1 {
                                    result.push_str(&format!("{note}{}", duration - 1));
                                }

                                Accidental(note, 1, symbol)
                            }
                            _ => {
                                if symbol == note {
                                    Normal(note, duration + 1)
                                } else {
                                    result.push_str(&format!("{note}{duration}"));
                                    Normal(symbol, 1)
                                }
                            }
                        },
                        Accidental(note, duration, accidental) => {
                            if symbol == note {
                                Unknown(note, duration, accidental)
                            } else {
                                result.push_str(&format!("{note}{accidental}{duration}"));
                                Normal(symbol, 1)
                            }
                        }
                        Unknown(note, duration, accidental) => match symbol {
                            '#' | 'b' => {
                                if symbol != accidental {
                                    result.push_str(&format!("{note}{accidental}{duration}"));
                                    Accidental(note, 1, symbol)
                                } else {
                                    Accidental(note, duration + 1, accidental)
                                }
                            }
                            _ => {
                                if symbol == note {
                                    result.push_str(&format!("{note}{accidental}{duration}"));
                                    Normal(note, 2)
                                } else {
                                    result
                                        .push_str(&format!("{note}{accidental}{duration}{note}1"));
                                    Normal(symbol, 1)
                                }
                            }
                        },
                    }
                }
            }

            result.push_str(&match note {
                Normal(note, duration) => format!("{note}{duration}"),
                Accidental(note, duration, accidental) => format!("{note}{accidental}{duration}"),
                Unknown(note, duration, accidental) => {
                    format!("{note}{accidental}{duration}{note}1")
                }
                _ => unreachable!(),
            });

            let song_path = target_folder
                .join(song_file)
                .with_file_name(&song_name)
                .with_extension(".txt");

            write(song_path, result).unwrap();
            tx.send((song_name.to_string(), song_duration)).unwrap();
        });
    }

    // let mut songs_durations: HashMap<String, usize> = HashMap::new();

    let mut songs: Vec<(String, i32)> = rx.iter().collect();
    // for _ in 0..threads_number {
    //     let (name, duration) = rx.recv().unwrap();
    //     songs.push((name, duration));
    // }

    songs.sort_by_key(|(song_name, song_duration)| (-song_duration, song_name.to_owned()));

    // TODO: make this cleaner
    // let mut songs: Vec<(&String, &usize)> = songs_durations.iter().collect();
    let mut index = File::create(target_folder.join("index.txt")).unwrap();
    for (song_name, songs_duration) in songs {
        writeln!(index, "\"{}\" {}", song_name, songs_duration).unwrap();
    }

    // songs.into_iter().collect()
    // songs_durations
}

// let mut t = vec![];
// t.push((name, duration));
// t.sort_by_key(|(song_name, song_duration)| (*song_duration as i64 * -1, *song_name));
// for (name, file, duration, result) in results {
// let song_path = target_folder
//     .join(file)
//     .with_file_name(&name)
//     .with_extension(".txt");

// write(song_path, result).unwrap();
// songs_durations.insert(name.to_string(), duration);
// }
// results.push((song_name, song_file, song_duration, result));
// paths.insert(target_folder.join(song_file).parent().unwrap().to_owned());
// let mut results = vec![];
//
//
//
//
//
//
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7f49e2da3b52d8887aa0e18425121e1f
