// Power digit sum

// 215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

// What is the sum of the digits of the number 21000?

const result = (2n ** 1000n)
	.toString()
	.split('')
	.map(x => Number(x))
    .reduce((sum, x) => sum + x)

console.log(result)
