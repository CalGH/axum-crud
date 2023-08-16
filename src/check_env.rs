use std::env;

macro_rules! getsslfile {
    ($folder1:expr, $folder2:expr, $filename:expr) => {
        env::var($folder1).unwrap()
            + env::var($folder2).unwrap().as_str()
            + env::var($filename).unwrap().as_str()
    };
}

pub fn get_key_cert(require: &[&str]) -> (String, String) {
    require.iter().for_each(|variable| {
        env::var(variable).expect(format!("{} unset", variable).as_str());
    });

    let key: String = getsslfile!("CARGO_MANIFEST_DIR", "CERT_FOLDER", "KEY_NAME");

    let cert: String = getsslfile!("CARGO_MANIFEST_DIR", "CERT_FOLDER", "CERT_NAME");

    (cert, key)
}
