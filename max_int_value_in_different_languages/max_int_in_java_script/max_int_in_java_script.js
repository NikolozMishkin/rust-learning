const maxSafe = Number.MAX_SAFE_INTEGER; // 2^53 - 1
console.log('MAX_SAFE_INTEGER:', maxSafe);

const a = maxSafe + 1;
const b = maxSafe + 1_000_000_000;
const c = maxSafe + 2_000_000_000;

console.log('maxSafe + 1 (Number):', a);
console.log('maxSafe + 1_000_000_000 (Number):', b);
console.log('maxSafe + 2_000_000_000 (Number):', c);
console.log('a === b?', a === b);
console.log('b === c?', b === c);

// Contrast with BigInt (precise integers)
const bigMaxSafe = BigInt(Number.MAX_SAFE_INTEGER);
console.log('BigInt maxSafe + 1:', bigMaxSafe + 1n);
console.log('BigInt maxSafe + 1_000_000_000:', bigMaxSafe + 1_000_000_000n);
console.log('BigInt maxSafe + 2_000_000_000:', bigMaxSafe + 2_000_000_000n);
console.log('Are they equal with BigInt (1 vs 1_000_000_000)?', (bigMaxSafe + 1n) === (bigMaxSafe + 1_000_000_000n));
