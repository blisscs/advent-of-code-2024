import fs from 'node:fs';
// Read from file into variable
const fsPromises = fs.promises;
const inputBuffer = await fsPromises.readFile("../input.txt");
const input = inputBuffer.toString('utf8');  

let left = [];
let right = [];

const regex = /^(?<left>\d+)\s+(?<right>\d+)$/;

// TODO refactor into something nicer
input.split("\n").reduce((acc, line) =>
	{
		if (line != "") {
			const result = line.match(regex);
			const left_in_number = parseInt(result.groups["left"]);
			const right_in_number = parseInt(result.groups["right"]);
			left.push(left_in_number);
			right.push(right_in_number);
		}
		acc
		
	}
	, 0);

left.sort();
right.sort();

let totalDistance = 0;

left.map((element, index) => [element, right[index]]).reduce((acc, [left, right]) => {
	const diff = Math.abs(left - right);
	totalDistance = totalDistance + diff;
	acc
}, 0);

console.log(totalDistance);
