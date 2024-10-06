
fn init_wheel_factorization(max_size: usize) -> (Vec<usize>, usize, Vec<usize>)  {
    let mut wheel = Vec::with_capacity(max_size);
    let mut primes = vec![2];

    let mut product = 2;
    wheel.push(1);
    wheel.push(3);
    
    loop {
        let next_prime = wheel[1];
        primes.push(next_prime);

        let next_product = product * next_prime;

        let next_wheel_size_est = (next_product - wheel[wheel.len()-1]) / product * wheel.len();
        if next_wheel_size_est > max_size {
            return (wheel, product, primes);
        }

        let mut idx = 0;
        loop {
            let n = wheel[idx] + product;
            if n > next_product {
                break;
            }

            if n % next_prime != 0 {
                wheel.push(n);
            }
            idx += 1;
        }
        wheel.remove(1);
        println!("Wheel size: {}", wheel.len());
        product = next_product;
    }
}



fn main() {
    const MAX_SIZE: usize = 134217728; // 1GB of u64 values
    let (wheel, product, primes) = init_wheel_factorization(40000);
    println!("{:#?}", primes);
    println!();
    println!("Final Product: {}", product);
    println!();
    // println!("{:#?}", wheel);
    // println!("Wheel size: {}", wheel.len());
    // Initialize wheel factorization
    // load from file
    // 
}
