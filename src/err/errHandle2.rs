use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorA {
    err: ErrorB
}

impl fmt::Display for ErrorA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A")
    }
}

impl Error for ErrorA {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.err)
    }
}

fn new_a() -> Result<(), ErrorA> {
    let err_b = ErrorB{};
    Err(ErrorA{err: err_b})
}

#[derive(Debug)]
struct ErrorB {}

impl fmt::Display for ErrorB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "B")
    }
}
impl Error for ErrorB {}

fn new_b() -> Result<(), ErrorB> {
    Err(ErrorB{})
}

// 实现 a到b的转换
impl From<ErrorA> for ErrorB {
    fn from(_: ErrorA) -> Self {
        ErrorB{}
    }
    
}

#[test]
fn test_NewErr() -> Result<(), ErrorA> {
    match new_a() {
        Err(e) => {
            println!("err: {}", e);
            println!("err caused by : {:?}", e.source().ok_or(ErrorB{}))
        }
        _ => println!("running ok")

    }
    Ok(())
}
#[test]
fn test_NewErrB() -> Result<(), ErrorB> {
    // 需要实现errA 到 errB的转换
    // new_a()?;
    new_a()?; // use `?` to propagate error and then convert to `ErrorB`
    Ok(())
}
