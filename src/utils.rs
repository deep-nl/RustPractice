use serde::{de, Serialize};
use std::io;
use std::fs::{self, File, DirEntry};
use std::path::Path;

#[derive(Debug)]
struct MyError;

pub fn read_json<T>(file_name: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: de::DeserializeOwned,
{
    let file_path = format!("{}", file_name);
    match fs::read_to_string(file_path.clone()) {
        Ok(json_str) => match serde_json::from_str(&json_str) {
            Ok(json) => Ok(json),
            Err(err) => Err(Box::new(err)),
        },
        Err(err) => Err(Box::new(err)),
    }
}

pub fn save_json<T>(serializable: &T, name: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: ?Sized + Serialize,
{
    let file_path = format!("{}", name);
    match File::create(file_path.clone()) {
        Ok(file) => match serde_json::to_writer(file, serializable) {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        },
        Err(err) => Err(Box::new(err)),
    }
}

pub fn cartesian_product(lists: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut res = vec![];
    let mut list_iter = lists.iter();
    if let Some(first_list) = list_iter.next() {
        for &i in first_list {
            res.push(vec![i]);
        }
    }
    for l in list_iter {
        let mut tmp = vec![];
        for r in res {
            for &el in l {
                let mut tmp_el = r.clone();
                tmp_el.push(el);
                tmp.push(tmp_el);
            }
        }
        res = tmp;
    }
    res
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

#[test]
fn test_visit_dirs() -> io::Result<()> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
    entries.sort();

    // The entries have now been sorted by their path.
    Ok(())
}
