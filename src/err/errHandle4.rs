use std::fs::File;
// Different io errors
use std::io::ErrorKind;


// ret1 和 ret2 效果是一样的
fn ret1() {
    let f = File:: open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // recover from the error and create the file when it originally was not found.
            // creating the file could also fail.
                ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic! ("Problem creating the file: {:?}", e),
                }
            other_error => {panic! ("Problem opening the file: {:?}", other_error)}
        }
    };
}

fn ret2() {
    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic! ("Problem creating the file: {:?}", error); 
            })
        } else {
            panic! ("Problem opening the file: {:?}", error); 
        }
    }); 
}
