use std::collections::HashMap;

pub fn is_prime(n: usize) -> bool {
    if n < 4 {
        n > 1
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let max_p = (n as f64).sqrt().ceil() as usize;
        !(5..=max_p)
            .step_by(6)
            .any(|p| n % p == 0 || n % (&p + 2) == 0)
    }
}

pub struct Prime {
    curr: usize,
    next: usize,
    trial1: usize,
    trial2: usize,
}

impl Prime {
    pub fn new() -> Prime {
        Prime {
            curr: 2,
            next: 3,
            trial1: 5,
            trial2: 7,
        }
    }
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = self.curr;
        self.curr = self.next;
        loop {
            self.next = self.trial1;
            self.trial1 = self.trial2;
            self.trial2 = self.next + 6;
            if is_prime(self.next) {
                break;
            }
        }
        Some(prime)
    }
}

pub fn list_primes(upto: usize) -> Vec<usize> {
    let sieve = prime_sieve(upto);
    (2..=upto).filter(|n| sieve[*n]).collect()
}

pub fn prime_sieve(upto: usize) -> Vec<bool> {
    let mut sieve = vec![true; upto + 1];
    for i in 0..=upto {
        match i {
            0 | 1 => sieve[i] = false,
            n => {
                for k in (n + n..=upto).step_by(n) {
                    sieve[k] = false;
                }
            }
        }
    }
    sieve
}

pub fn prime_factors(mut number: usize) -> HashMap<usize, u32> {
    let mut factors = HashMap::new();
    for n in Prime::new() {
        let mut count = 0;
        while number % n == 0 {
            number /= n;
            count += 1
        }
        if count > 0 {
            factors.insert(n, count);
        }
        if n > number {
            break;
        }
    }
    if number > 1 {
        factors.insert(number, 1);
    }
    factors
}

pub fn prime_factors_flat(mut number: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    for n in Prime::new() {
        while number % n == 0 {
            number /= n;
            factors.push(n);
        }
        if n > number {
            break;
        }
    }
    if number > 1 {
        factors.push(number);
    }
    factors
}
