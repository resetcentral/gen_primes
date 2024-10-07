use std::collections::VecDeque;

fn init_wheel_factorization(max_size: usize) -> (VecDeque<usize>, usize, Vec<usize>)  {
    let mut wheel = VecDeque::with_capacity(max_size/10);
    let mut primes = vec![2, 3];

    let mut product = 6;
    wheel.push_back(1);
    wheel.push_back(5);
    
    loop {
        let next_prime = wheel[1];
        primes.push(next_prime);

        let next_product = product * next_prime;

        let next_wheel_size_est = (next_product - wheel[wheel.len()-1]) / product * wheel.len();
        if next_wheel_size_est > max_size {
            return (wheel, product, primes);
        }

        let mut skip = VecDeque::<(usize, usize)>::new();
        let mut idx = 0;
        loop {
            let n;
            if !skip.is_empty() && idx == skip[0].0 {
                n = skip.pop_front().unwrap().1 + product;
            } else {
                n = wheel[idx] + product;
                idx += 1;
            }

            if n > next_product {
                break;
            }

            if n % next_prime != 0 || n % next_product == 1 {
                wheel.push_back(n);
            } else {
                skip.push_back((wheel.len(), n));
            }
        }

        wheel.remove(1);
        product = next_product;
    }
}


fn is_prime(n: usize, primes: &[usize]) -> bool {
    for prime in primes {
        if n % prime == 0 {
            return false;
        }
    }
    return true;
}

fn sieve(wheel: &mut VecDeque<usize>, span: usize, primes: &mut Vec<usize>, search_max: usize) {
    let mut idx = 1;
    loop {
        let n = wheel[idx];
        wheel[idx] += span;
        if is_prime(n, &primes) {
            if n > search_max {
                return;
            }
            primes.push(n);
        }

        idx = (idx + 1) % wheel.len();
    }
}


fn main() {
    const MAX_SIZE: usize = 134217728; // 1GB of u64 values
    let (mut wheel, product, mut primes) = init_wheel_factorization(MAX_SIZE);
    wheel[0] += product;

    println!("{:?}", primes);
    println!();
    println!("Final Product: {}", product);
    println!();
    // println!("{:#?}", wheel);
    println!("Wheel size: {}", wheel.len());
    sieve(&mut wheel, product, &mut primes, 10_000);
    println!("{:?}", primes);

    // Initialize wheel factorization
    // load from file
    // 
}
