// Special Pythagorean triplet

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn sqrt (x: u32) -> u32 {
    for i in 1..=x {
        if i * i == x {
            return i;
        }
        if i * i > x {
            break;
        }
    }
    return 0;
}

fn main () {
    for a in 1..1000 {
        for b in 1..=a {
            let c = sqrt(a * a + b * b);
            if c == 0 { continue }
            if a + b + c == 1000 && a * a + b * b == c * c{
                println!("{} * {} * {} = {}", a, b, c, a * b * c);
                return;
            }
        }
    }
}
