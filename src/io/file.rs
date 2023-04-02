use std::{fs, io};

pub fn read_file(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let f = fs::File::open(name);
    let ff = match f {
        Ok(file) => file,
        // 简单粗暴
        // Err(e) => {panic!("error")}
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => match fs::File::create(name) {
                Ok(fc) => fc,
                Err(e) => panic!("Panic:{:?}", e)
            },
            some_error => {
                panic!("Panic:{:?}", some_error)
            }
        }
    };
    Ok(())
}

pub fn read_dir() -> io::Result<()> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. 
    // If reproducible ordering is required the entries should be explicitly sorted.

    // entries.sort();
    println!("1......");
    for entry in entries.iter(){
        println!("{entry:?}")
    }
    println!("2......");
    for entry in entries.iter(){
        println!("{entry:?}")
    }
    println!("3......");
    for entry in entries{
        println!("{entry:?}")
    }

    // The entries have now been sorted by their path.

    Ok(())
}
#[test]
fn test_read_dir(){
    read_dir();
}

#[test]
fn test_read_file(){
    read_file("wooo");
}