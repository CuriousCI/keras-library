use std::collections::HashMap;
use std::fmt::Write;
use std::fs::{create_dir_all, read, read_to_string, write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::scope;

enum Note {
    None,
    Normal,
    Accidental,
    Unknown,
}

use Note::*;

static POOL_SIZE: usize = 12;

pub fn translate(score: &Vec<char>) -> (String, i32) {
    let mut song_duration = 0;
    let mut note = ' ';
    let mut accidental = ' ';
    let mut duration = 0;
    let mut note_type = None;
    let mut s = String::new();

    for staff in score.split(|char| char == &'\n') {
        for &symbol in staff.iter().rev() {
            note_type = if symbol == note {
                song_duration += 1;

                match note_type {
                    Normal => {
                        duration += 1;
                        Normal
                    }
                    Accidental => Unknown,
                    Unknown => {
                        write!(s, "{note}{accidental}{duration}").unwrap();
                        duration = 2;
                        Normal
                    }
                    _ => unreachable!(),
                }
            } else {
                match symbol {
                    '#' | 'b' => {
                        match note_type {
                            Normal => {
                                if duration > 1 {
                                    write!(s, "{note}{}", duration - 1).unwrap();
                                }

                                duration = 1;
                                accidental = symbol;
                            }
                            Unknown => {
                                if symbol == accidental {
                                    duration += 1;
                                } else {
                                    write!(s, "{note}{accidental}{duration}").unwrap();
                                    duration = 1;
                                    accidental = symbol;
                                }
                            }
                            _ => unreachable!(),
                        };

                        Accidental
                    }
                    _ => {
                        song_duration += 1;

                        match note_type {
                            None => Ok(()),
                            Normal => write!(s, "{note}{duration}"),
                            Accidental => write!(s, "{note}{accidental}{duration}"),
                            Unknown => write!(s, "{note}{accidental}{duration}{note}1"),
                        }
                        .unwrap();

                        note = symbol;
                        duration = 1;
                        Normal
                    }
                }
            }
        }
    }

    match note_type {
        Normal => write!(s, "{note}{duration}"),
        Accidental => write!(s, "{note}{accidental}{duration}"),
        Unknown => write!(s, "{note}{accidental}{duration}{note}1"),
        _ => unreachable!(), // _ => Ok(()), // _ => unreachable!(),
    }
    .unwrap();

    (s, song_duration)
}

pub fn umkansanize(source_folder: &Path, target_folder: &Path) -> HashMap<String, i32> {
    let (tx, rx) = channel(); // TODO: Find a better name

    scope(|scope| {
        let mut channels = vec![]; // TODO: I'd rather not have this one

        for _ in 0..POOL_SIZE {
            let (song_tx, songs_rx) = channel();
            let tx = tx.to_owned();
            channels.push(song_tx.to_owned());

            scope.spawn(move || {
                for (title, file) in songs_rx.iter() {
                    let (translation, duration) = translate(
                        &read(source_folder.join(&file))
                            .unwrap()
                            .iter()
                            .map(|byte| match byte {
                                10 => '\n',
                                32 => 'P',
                                43 => '#',
                                45 => 'b',
                                byte => (byte + 17) as char,
                            })
                            .collect(),
                    );

                    let target_file = target_folder.join(&file);
                    let target_path = target_file.parent().unwrap();
                    if !target_path.exists() {
                        create_dir_all(target_path).unwrap();
                    }

                    write(
                        target_file.with_file_name(&title).with_extension(".txt"),
                        translation,
                    )
                    .unwrap();

                    tx.send((title, duration)).unwrap();
                }
            });
        }

        read_to_string(source_folder.join("index.txt"))
            .unwrap()
            .split('\n')
            .filter_map(|line| {
                line.strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"').and_then(|s| s.split_once("\" \"")))
            })
            .enumerate()
            .for_each(|(index, (title, path))| {
                channels[index % POOL_SIZE]
                    .send((title.to_owned(), path.to_owned()))
                    .unwrap()
            });
    });

    drop(tx);
    let mut songs: Vec<_> = rx.iter().collect();
    songs.sort_unstable_by_key(|(title, duration)| (-duration, title.to_owned()));

    let mut s = String::new();
    for (title, duration) in songs.iter() {
        writeln!(s, "\"{title}\" {duration}").unwrap();
    }
    write(target_folder.join("index.txt"), s).unwrap();

    songs.iter().map(ToOwned::to_owned).collect()
}

// https://doc.rust-lang.org/rust-by-example/std_misc/channels.html

// let source = source_folder.join(&file);
// let (translation, duration) = scope
//     .spawn(|| {
//         translate(
//             // &read(source_folder.join(&file))
//             &read(source)
//                 .unwrap()
//                 .iter()
//                 .map(|byte| match byte {
//                     10 => '\n',
//                     32 => 'P',
//                     43 => '#',
//                     45 => 'b',
//                     byte => (byte + 17) as char,
//                 })
//                 .collect(),
//         )
//     })
//     .join()
//     .unwrap();
