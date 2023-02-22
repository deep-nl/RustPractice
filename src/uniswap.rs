#![allow(unused)]
#![allow(non_snake_case)]
struct Token {
    name: String,
    symbol: String,
    decimals: u8,
}

struct UniswapPool {
    token1: Token,
    token2: Token,
    token1_amount: u128,
    token2_amount: u128,
    k: u128,
}

impl UniswapPool {
    fn new(token1: Token, token2: Token, token1_amount: u128, token2_amount: u128) -> Self {
        let token1_amount =  token1_amount * 1000000;
        let token2_amount = token2_amount * 1000000;
        UniswapPool {
            token1,
            token2,
            token1_amount,
            token2_amount,
            k: u128::from(token1_amount) * u128::from(token2_amount)
        }
    }

    fn exactInputToken1ToOutput(&mut self, Input: u128) -> u128 {
        self.token1_amount += Input;

        let original = self.token2_amount;
        self.token2_amount = self.k / self.token1_amount;
        return original - self.token2_amount;
    }

    fn exactInputToken2ToOutput(&mut self, Input: u128) -> u128 {
        self.token2_amount += Input;

        let original = self.token1_amount;
        self.token1_amount = self.k / self.token2_amount;
        return original - self.token1_amount;
    }
}

fn getLine() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    return line;
}

pub fn test_uniswap() {
    // Statements here are executed when the compiled binary is called
    let token1 = Token{name: String::from("FLUX"), symbol: String::from("FLX"), decimals: 6};
    let token1_amount = 100000;
    let token2 = Token{name: String::from("WRAPPED ETH"), symbol: String::from("WETH"), decimals: 6};
    let token2_amount = 100000;
    const USER_DECIMALS:u128 = 1000000;
     // Print text to the console
     println!("TOKEN 1
     \tname:{}
     \tsymbol:{}
     \tdecimals:{}", token1.name, token1.symbol, token1.decimals);

     println!("TOKEN 2
     \tname:{}
     \tsymbol:{}
     \tdecimals:{}\n", token2.name, token2.symbol, token2.decimals);

    let mut pool = UniswapPool::new(token1, token2, token1_amount, token2_amount);


    loop {
        println!("\tPOOL STATE
\t{}:{}
\t{}:{}\n", pool.token1.symbol, pool.token1_amount, pool.token2.symbol, pool.token2_amount);
        loop {
            println!("swap {} for {}? ({{amount}}/n)", pool.token1.symbol, pool.token2.symbol);
            let line = getLine();
            match line.parse::<u128>() {
                Ok(n) => {
                    let tokens = pool.exactInputToken1ToOutput(n * USER_DECIMALS);
                    println!("Received {} {}", tokens / USER_DECIMALS, pool.token2.symbol);
                    break;
                },
                Err(e) => {
                    if line == "n" { break; }
                }
            }
        }
        loop {
            println!("swap {} for {}? ({{amount}}/n)", pool.token2.symbol, pool.token1.symbol);
            let line = getLine();
            match line.parse::<u128>() {
                Ok(n) => {
                    let tokens = pool.exactInputToken2ToOutput(n * USER_DECIMALS);
                    println!("Received {} {}", tokens / USER_DECIMALS, pool.token1.symbol);
                    break;
                },
                Err(e) => {
                    if line == "n" { break; }
                }
            }
        }
    }
}
