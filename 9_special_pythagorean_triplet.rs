// Special Pythagorean triplet

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn sqrt (x: u32) -> Option<u32> {
    for i in 1..=x {
        if i * i == x {
            return Some(i);
        }
        if i * i > x {
            break;
        }
    }
    None
}

fn main () {
    for a in 1..1000 {
        for b in 1..=a {
            let maybe_c = sqrt(a * a + b * b);
            let c = match maybe_c {
                None => continue,
                Some(c) => c,
            };
            if a + b + c == 1000 && a * a + b * b == c * c {
                println!("{} * {} * {} = {}", a, b, c, a * b * c);
                return;
            }
        }
    }
}
