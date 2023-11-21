use std::collections::HashMap;
use std::fmt::Write;
use std::fs::{create_dir_all, read, read_to_string, write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::spawn;

enum Note {
    None,
    Normal(char, usize),
    Accidental(char, usize, char),
    Unknown(char, usize, char),
}

use Note::*;

// static POOL_SIZE: u8 = 8;

// pub fn umkansanize(source_folder: &Path, target_folder: &Path) -> HashMap<String, i32> {
pub fn umkansanize(source_folder: &Path, target_folder: &Path) {
    let index = read_to_string(source_folder.join("index.txt"))
        .unwrap()
        .replace("\" \"", "\r")
        .replace('"', "");

    let index: Vec<(&str, &str)> = index
        .split('\n')
        .filter_map(|line| line.split_once('\r'))
        .collect();

    let (trx, rrx) = channel();

    // let channels = [channel(), channel(), channel(), channel(), channel()];

    let pool_size = 8;
    let mut channels = vec![];
    for _ in 0..pool_size {
        let (tx, rx) = channel();
        let trx = trx.clone();
        let source_folder = source_folder.to_owned();
        let target_folder = target_folder.to_owned();

        channels.push(tx.to_owned());

        spawn(move || {
            for (title, file) in rx.iter() {
                let score: Vec<_> = read(source_folder.join(&file))
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

                let path = target_folder.join(&file);
                let path = path.parent().unwrap();
                if !path.exists() {
                    create_dir_all(path).unwrap();
                }

                write(
                    target_folder
                        .join(file)
                        .with_file_name(&title)
                        .with_extension(".txt"),
                    s,
                )
                .unwrap();
                trx.send((title, duration)).unwrap();
            }
        });
    }

    for (id, (title, file)) in index.iter().enumerate() {
        let transmitter = channels[id % pool_size].to_owned();

        transmitter
            .send((title.to_string(), file.to_string()))
            .unwrap();
    }

    // drop(tx);
    drop(trx);
    drop(channels);

    // let mut songs: Vec<_> = rx.iter().collect();
    let mut songs: Vec<_> = rrx.iter().collect();
    songs.sort_by_key(|(title, duration)| (-duration, title.to_owned()));

    let mut s = String::new();
    for (title, duration) in songs {
        write!(s, "\"{title}\" {duration}\n").unwrap();
    }
    write(target_folder.join("index.txt"), s).unwrap();

    // songs.into_iter().collect()
    // songs.into_iter().collect()
    // songs_durations
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7f49e2da3b52d8887aa0e18425121e1f
// Copy dirs https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust

// TODO: make this cleaner
// let mut songs: Vec<(&String, &usize)> = songs_durations.iter().collect();
// writeln!(index, "\"{}\" {}", song_name, songs_duration).unwrap();
// let mut index = File::create(target_folder.join("index.txt")).unwrap();
// for (song_name, songs_duration) in songs {
//     writeln!(index, "\"{}\" {}", song_name, songs_duration).unwrap();
// }
// for path in index
//     .iter()
//     .map(|(_, file)| target_folder.join(file).parent().unwrap().to_owned())
//     .collect::<HashSet<PathBuf>>()
// {
//     create_dir_all(path).unwrap();
// }
// let cnt = index.len() as i32;
// println!("{}\n{cnt} files number", source_folder.to_str().unwrap());
// let mut d = 0;
// d += duration;
// println!("Average duration {}\n", d / cnt);
//
// create_dir_all(target_folder.join(&file).parent().unwrap()).unwrap();
// target_folder.join(&file).parent().unwrap();
// create_dir_all(target_folder.join(&file).parent().unwrap()).unwrap();
// songs.push((title.to_string(), song_duration));
// create_dir(target_folder).unwrap();
// let mut directories: Vec<_> = index
//     .iter()
//     .map(|(_, file)| target_folder.join(file).parent().unwrap().to_owned())
//     .collect();
// directories.sort();
// for directory in directories {
//     match create_dir(directory) {
//         Ok(_) => (),
//         Err(_) => (),
//     };
// }
// let mut songs = vec![];
// let mut songs: Vec<(String, i32)> = rx.iter().collect();
// songs.push((title, duration));
