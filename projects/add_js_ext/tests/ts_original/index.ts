import { greet } from './a';
import { helloWorld } from './a/hello_world';
import { calculateSum } from './b';
import { generateRandomNumber } from './c';

helloWorld();
greet('Alice');
const sum = calculateSum(3, 5);
console.log(`Sum: ${sum}`);
console.log(`Random number: ${generateRandomNumber()}`);