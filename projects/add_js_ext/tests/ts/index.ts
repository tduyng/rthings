import { greet } from './a/index.js';
import { helloWorld } from './a/hello_world.js';
import { calculateSum } from './b/index.js';
import { generateRandomNumber } from './c/index.js';

helloWorld();
greet('Alice');
const sum = calculateSum(3, 5);
console.log(`Sum: ${sum}`);
console.log(`Random number: ${generateRandomNumber()}`);