
fn test_if_prime(number:u32) -> bool {

    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }

    return true;
}

pub(crate) fn find_primes(starting_number:u32, count :u32) -> Vec<u32> {
    let mut primes = Vec::new();

    let mut found_count = 0;


    let mut current_mumber = match starting_number.is_multiple_of(2) {
        true => {starting_number + 1},
        false => {starting_number}
    };

    loop {

        if test_if_prime(current_mumber) {
            primes.push(current_mumber);
            found_count += 1;
            if found_count == count {
                break;
            }
        }

        current_mumber += 2;
    }


    primes
}