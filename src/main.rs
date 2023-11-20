use keras::umkansanize;
use std::{fs::remove_dir_all, path::Path};

pub fn main() {
    let tests = vec![
        "test01", "test02", "test03", "test04", "test05", "test06", "test07", "test08", "test09",
        "test10",
    ];

    let base_path = Path::new("/home/cicio/projects/keras-library/workdir/");

    for test in tests {
        remove_dir_all(base_path.join(format!("{test}.out"))).unwrap();

        umkansanize(
            &base_path.join(test),
            &base_path.join(&format!("{test}.out")),
        );
    }
}
