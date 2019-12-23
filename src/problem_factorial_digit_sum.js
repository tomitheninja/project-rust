#!/bin/env/node
// Factorial digit sum

// n! means n × (n − 1) × ... × 3 × 2 × 1

// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

// Find the sum of the digits in the number 100!

function factorial (x) {
    let product = BigInt(1);
    for (let i = 1n; i <= BigInt(x); i++) {
        product *= i;
    }
    return product;
}

function main () {
    let f100 = factorial(100);
    let str = f100.toString()
    let chars = str.split('')
    let digits = chars.map(c => c == 'n' ? 0 : Number.parseInt(c))
    let sumOfDigits = digits.reduce((sum, x) => sum + x)
    console.log(sumOfDigits)
}

main()
