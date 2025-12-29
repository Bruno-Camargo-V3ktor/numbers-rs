use rand::{rng, Rng};

pub fn rand_rang(min: u16, max: u16, quantity: usize, rep: bool) -> Vec<u16> {
    let mut rng = rng();
    let mut numbers = Vec::with_capacity(quantity);

    while numbers.len() < quantity {
        let num = rng.random_range(min..=max);
        if !rep && numbers.contains(&num) {
            continue;
        }

        numbers.push(num);
    }

    numbers
}
