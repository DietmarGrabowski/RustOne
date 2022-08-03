
mod utils;


#[cfg(test)]
mod tests {
    #[test]
    fn test_args()
    {
        use std::env;
        let args: Vec<String> = env::args().collect();
        eprintln!("ARGS: {:?}", args);
    }

}





fn main() {

    utils::foo();

    

}
