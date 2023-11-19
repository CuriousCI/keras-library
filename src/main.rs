use keras::umkansanize;
use std::path::Path;

pub fn main() {
    let tests = vec![
        "test01", "test02", "test03", "test04", "test05", "test06", "test07", "test08", "test09",
        "test10",
    ];

    let base_path = Path::new("D:/university/python/hw4-rust/workdir");

    for test in tests {
        umkansanize(
            &base_path.join(test),
            &base_path.join(&format!("{}.out", test)),
        );
    }
}
