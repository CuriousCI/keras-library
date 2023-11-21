use std::collections::HashMap;
use std::fmt::Write;
use std::fs::{create_dir_all, read, read_to_string, write};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::spawn;

enum Note {
    None,
    Normal,
    Accidental,
    Unknown,
}

use Note::*;

static POOL_SIZE: usize = 12;

pub fn umkansanize<'a>(source_folder: &Path, target_folder: &Path) -> HashMap<&'a str, i32> {
    let (trx, rrx) = channel(); // TODO: Find a better name
    let mut channels = vec![]; // TODO: I'd rather not have this one

    for _ in 0..POOL_SIZE {
        let (tx, rx) = channel();
        let trx = trx.to_owned();
        let source_folder = source_folder.to_owned();
        let target_folder = target_folder.to_owned();

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

                let mut song_duration = 0;
                let mut note = '0';
                let mut accidental = '0';
                let mut duration = 0;
                let mut note_type = None;
                let mut s = String::new();

                for staff in score.split(|char| char == &'\n') {
                    for &symbol in staff.iter().rev() {
                        note_type = if symbol == note {
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
                                    song_duration += 1;

                                    match note_type {
                                        Normal => {
                                            if duration > 1 {
                                                write!(s, "{note}{}", duration - 1).unwrap();
                                            }

                                            duration = 1;
                                            accidental = symbol;
                                        }
                                        Unknown => {
                                            if symbol != accidental {
                                                write!(s, "{note}{accidental}{duration}").unwrap();
                                                duration = 1;
                                                accidental = symbol;
                                            } else {
                                                duration += 1;
                                            }
                                        }
                                        _ => unreachable!(),
                                    };

                                    Accidental
                                }
                                _ => {
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
                    Accidental => {
                        write!(s, "{note}{accidental}{duration}")
                    }
                    Unknown => write!(s, "{note}{accidental}{duration}{note}1"),
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
                trx.send((title, song_duration)).unwrap();
            }
        });

        channels.push(tx.to_owned());
    }

    for (id, (title, file)) in read_to_string(source_folder.join("index.txt"))
        .unwrap()
        .split('\n')
        .filter_map(|line| {
            line.strip_prefix('"')
                .and_then(|s| s.strip_suffix('"').and_then(|s| s.split_once("\" \"")))
        })
        .enumerate()
    {
        let transmitter = channels[id % POOL_SIZE].to_owned();

        transmitter
            .send((title.to_owned(), file.to_owned()))
            .unwrap();
    }

    // TODO: cleaner drop, without explicitly dropping maybe?
    drop(trx);
    drop(channels);

    let mut songs: Vec<_> = rrx.iter().collect();
    songs.sort_unstable_by_key(|(title, duration)| (-duration, title.to_owned()));

    let mut s = String::new();
    for (title, duration) in songs.iter() {
        writeln!(s, "\"{title}\" {duration}").unwrap();
    }
    write(target_folder.join("index.txt"), s).unwrap();

    HashMap::new()
}

// https://doc.rust-lang.org/rust-by-example/std_misc/channels.html

// songs
//     .iter()
//     .map(ToOwned::to_owned)
//     // .map(|(title, duration)| (title, duration))
// .collect()
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7f49e2da3b52d8887aa0e18425121e1f
// Copy dirs https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
// .collect::<HashMap<(String, i32)>>();
// songs.into_iter().collect()
// songs.into_iter().collect()
// songs_durations
// let index: Vec<(&str, &str)> = index
//     .split('\n')
//     .filter_map(|line| line.split_once('\r'))
//     .collect();
// for (id, (title, file)) in index
//     .split('\n')
//     .filter_map(|line| line.split_once('\r'))
//     .enumerate()
// {
// for (id, (title, file)) in index.iter().enumerate() {
//

//         if symbol != '#' && symbol != 'b' {
//             duration += 1;
//         }
//
//         note = match note {
//             None => Normal(symbol, 1),
//             Normal(note, duration) => match symbol {
//                 '#' | 'b' => {
//                     if duration > 1 {
//                         write!(s, "{note}{}", duration - 1).unwrap();
//                     }
//
//                     Accidental(note, 1, symbol)
//                 }
//                 _ => {
//                     if symbol == note {
//                         Normal(note, duration + 1)
//                     } else {
//                         write!(s, "{note}{duration}").unwrap();
//                         Normal(symbol, 1)
//                     }
//                 }
//             },
//             Accidental(note, duration, accidental) => {
//                 if symbol == note {
//                     Unknown(note, duration, accidental)
//                 } else {
//                     write!(s, "{note}{accidental}{duration}").unwrap();
//                     Normal(symbol, 1)
//                 }
//             }
//             Unknown(note, duration, accidental) => match symbol {
//                 '#' | 'b' => {
//                     if symbol != accidental {
//                         write!(s, "{note}{accidental}{duration}").unwrap();
//                         Accidental(note, 1, symbol)
//                     } else {
//                         Accidental(note, duration + 1, accidental)
//                     }
//                 }
//                 _ => {
//                     if symbol == note {
//                         write!(s, "{note}{accidental}{duration}").unwrap();
//                         Normal(note, 2)
//                     } else {
//                         write!(s, "{note}{accidental}{duration}{note}1").unwrap();
//                         Normal(symbol, 1)
//                     }
//                 }
//             },
//         }
//     }
// }
//
// match note {
//     Normal(note, duration) => write!(s, "{note}{duration}"),
//     Accidental(note, duration, accidental) => {
//         write!(s, "{note}{accidental}{duration}")
//     }
//     Unknown(note, d, accidental) => write!(s, "{note}{accidental}{d}{note}1"),
//     _ => unreachable!(), // _ => Ok(()), // _ => unreachable!(),
// }
// .unwrap();
// note_type = match symbol {
//     '#' | 'b' => {
//         song_duration += 1;
//
//         match note_type {
//             Normal => {
//                 if duration > 1 {
//                     write!(s, "{note}{}", duration - 1).unwrap();
//                 }
//
//                 duration = 1;
//                 accidental = symbol;
//                 Accidental
//             }
//             Unknown => {
//                 if symbol != accidental {
//                     write!(s, "{note}{accidental}{duration}").unwrap();
//                     duration = 1;
//                     accidental = symbol;
//                 } else {
//                     duration += 1;
//                 }
//
//                 Accidental
//             }
//             _ => unreachable!(),
//         }
//     }
//     _ => match note_type {
//         None => {
//             note = symbol;
//             duration = 1;
//             Normal
//         }
//         Normal => {
//             if symbol == note {
//                 duration += 1;
//             } else {
//                 write!(s, "{note}{duration}").unwrap();
//                 note = symbol;
//                 duration = 1;
//             }
//
//             Normal
//         }
//         Accidental => {
//             if symbol == note {
//                 Unknown
//             } else {
//                 write!(s, "{note}{accidental}{duration}").unwrap();
//                 note = symbol;
//                 duration = 1;
//                 Normal
//             }
//         }
//         Unknown => {
//             if symbol == note {
//                 write!(s, "{note}{accidental}{duration}").unwrap();
//                 duration = 2;
//             } else {
//                 write!(s, "{note}{accidental}{duration}{note}1").unwrap();
//                 note = symbol;
//                 duration = 1;
//             }
//
//             Normal
//         }
//     },
// }
// enum Note {
//     None,
//     Normal(char, usize),
//     Accidental(char, usize, char),
//     Unknown(char, usize, char),
// }
// let index = read_to_string(source_folder.join("index.txt")).unwrap();
