use std::{fs, path::Path};

use regex::Regex;

use kv3::{from_str, to_string};

#[test]
fn test_to_str() {
    let data = Path::new(file!()).parent().unwrap().join("data");

    let files = vec![
        Path::new("cs2/weapons.vdata"),
        Path::new("deadlock/abilities.vdata"),
        Path::new("deadlock/heroes.vdata"),
    ];

    for file in &files {
        let path = data.join(file);

        let old_str = fs::read_to_string(&path).unwrap();
        let value = from_str(&old_str).unwrap();
        let new_str = to_string(&value);

        // remove empty lines
        let regex = Regex::new(r"(?m)^$\n").unwrap();
        let old_str = regex.replace_all(&old_str, "").into_owned();
        let new_str = regex.replace_all(&new_str, "").into_owned();

        // remove trailing whitespaces
        let regex = Regex::new(r"(?m)\s+$").unwrap();
        let old_str = regex.replace_all(&old_str, "").into_owned();
        let new_str = regex.replace_all(&new_str, "").into_owned();

        // remove trailing zeros
        let regex = Regex::new(r"(?m)\.0+(?<comma>,)?$").unwrap();
        let old_str = regex.replace_all(&old_str, "$comma").into_owned();
        let new_str = regex.replace_all(&new_str, "$comma").into_owned();

        // remove trailing zeros
        let regex = Regex::new(r"(?m)(?<dot>\.)(?<digits>([0]*[1-9]+)*)0+(?<comma>,)?$").unwrap();
        let old_str = regex
            .replace_all(&old_str, "$dot$digits$comma")
            .into_owned();
        let new_str = regex
            .replace_all(&new_str, "$dot$digits$comma")
            .into_owned();

        assert_eq!(old_str, new_str);
    }
}
