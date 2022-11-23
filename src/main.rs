
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



fn fac( x: u64) -> u64
{
    if x > 2{
        return x*fac (x-1);
    }
    x
}



fn main() {

    utils::foo();
    println!("Hello {}",fac(3));
    

}
