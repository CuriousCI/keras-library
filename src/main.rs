use keras::umkansanize;

pub fn main() {
    let tests = vec![
        "test01", "test02", "test03", "test04", "test05", "test06", "test07", "test08", "test09",
        "test10",
    ];

    let base_path = "/home/cicio/projects/keras-library/workdir/";

    for test in tests {
        umkansanize(
            &format!("{}{}", base_path, test),
            &format!("{}{}.out", base_path, test),
        );
    }
}
