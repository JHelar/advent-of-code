const STATE = {
	ON: 'ON',
	EXIT: 'EXIT'
};

const PARAMETER_MODE = {
	POSITION: '0',
	IMMEDIATE: '1',
	RELATIVE: '2'
};
let debugParam = false;

const makeProgram = intcode => {
	intcode = intcode.reduce((map, code, address) => {
		map[address] = BigInt(code);
		return map;
	}, {});

	const outputBuffer = [];
	const inputBuffer = [];

	let currentState = STATE.ON;
	let pointer = BigInt(0);
	let relativeBase = BigInt(0);

	const getParameterAddress = (intcode, pointer, mode) => {
		switch (mode) {
			case PARAMETER_MODE.POSITION:
				return intcode[pointer];
			case PARAMETER_MODE.IMMEDIATE:
				return pointer;
			case PARAMETER_MODE.RELATIVE:
				return sanitizeParam(intcode[pointer]) + relativeBase;
		}
	};
	const getParameterAddresses = (intcode, pointer, modes) =>
		modes.map((mode, index) => {
			return getParameterAddress(
				intcode,
				pointer + BigInt(index + 1),
				mode
			);
		});
	const makeOperation = operation => (intcode, pointer, modes) => {
		const addresses = getParameterAddresses(intcode, pointer, modes);
		return operation(intcode, pointer, addresses);
	};
	const sanitizeParam = param =>
		param ? (typeof param === 'bigint' ? param : BigInt(param)) : BigInt(0);

	const addOperation = makeOperation((intcode, pointer, params) => {
		const [paramAddress1, paramAddress2, outputAddress] = params;

		const param1 = sanitizeParam(intcode[paramAddress1]);
		const param2 = sanitizeParam(intcode[paramAddress2]);

		let output = param1 + param2;
		intcode[outputAddress] = output;
		return {
			pointer: pointer + BigInt(4)
		}
	});
	const multiplyOperation = makeOperation((intcode, pointer, params) => {
		const [paramAddress1, paramAddress2, outputAddress] = params;

		const param1 = sanitizeParam(intcode[paramAddress1]);
		const param2 = sanitizeParam(intcode[paramAddress2]);

		let output = param1 * param2;
		intcode[outputAddress] = output;

		return {
			pointer: pointer + BigInt(4)
		}
	});

	const inputOperation = makeOperation((intcode, pointer, params) => {
		const [inputAddress] = params;
		const inputValue = sanitizeParam(inputBuffer.pop());
		intcode[inputAddress] = inputValue

		return {
			pointer: pointer + BigInt(2)
		};
	});
	const outputOperation = makeOperation((intcode, pointer, params) => {
		const [outputAddress] = params;
		const output = intcode[outputAddress];
		outputBuffer.push(output.toString());

		return {
			pointer: pointer + BigInt(2),
			output
		}
	});
	const jumpIfTrueOperation = makeOperation((intcode, pointer, params) => {
		const [paramAddress1, paramAddress2] = params;
		const checkValue = sanitizeParam(intcode[paramAddress1]);
		let gotoValue = pointer + BigInt(3);

		if (checkValue != 0) {
			gotoValue = sanitizeParam(intcode[paramAddress2]);
		}

		return {
			pointer: gotoValue
		}
	});

	const jumpIfFalseOperation = makeOperation((intcode, pointer, params) => {
		const [paramAddress1, paramAddress2] = params;
		const checkValue = sanitizeParam(intcode[paramAddress1]);
		let gotoValue = pointer + BigInt(3);

		if (checkValue == 0) {
			gotoValue = sanitizeParam(intcode[paramAddress2]);
		}

		return {
			pointer: gotoValue
		}
	});

	const lessThanOperation = makeOperation((intcode, pointer, params) => {
		const [paramAddress1, paramAddress2, outputAddress] = params;
		const oneValue = sanitizeParam(intcode[paramAddress1]);
		const anotherValue = sanitizeParam(intcode[paramAddress2]);
		const outputValue = oneValue < anotherValue ? 1 : 0;

		intcode[outputAddress] = BigInt(outputValue);

		return {
			pointer: pointer + BigInt(4)
		}
	});

	const equalsOperation = makeOperation((intcode, pointer, params) => {
		const [paramAddress1, paramAddress2, outputAddress] = params;
		const oneValue = sanitizeParam(intcode[paramAddress1]);
		const anotherValue = sanitizeParam(intcode[paramAddress2]);
		const outputValue = oneValue == anotherValue ? 1 : 0;

		intcode[outputAddress] = BigInt(outputValue);

		return {
			pointer: pointer + BigInt(4)
		}
	});

	const adjustRelativeBaseOperation = makeOperation(
		(intcode, pointer, params) => {
			const [paramAddress1] = params;
			const adjustValue = sanitizeParam(intcode[paramAddress1]);
			relativeBase = relativeBase + adjustValue;

			return {
				pointer: pointer + BigInt(2)
			};
		}
	);

	const OP = {
		'01': addOperation,
		'02': multiplyOperation,
		'03': inputOperation,
		'04': outputOperation,
		'05': jumpIfTrueOperation,
		'06': jumpIfFalseOperation,
		'07': lessThanOperation,
		'08': equalsOperation,
		'09': adjustRelativeBaseOperation
	};

	const getInstructions = instructionString => {
		instructionString = instructionString.toString().padStart(4, '0');
		const opcode = instructionString.slice(instructionString.length - 2);

		const operation = OP[opcode] || false;

		const modes = instructionString
			.slice(0, instructionString.length - 2)
			.padStart(3, '0')
			.split('')
			.reverse();

		return {
			operation,
			modes,
			opcode: opcode | 0
		};
	};

	const run = input => {
		if(input !== undefined) {
			inputBuffer.push(input);
		}
		while(currentState === STATE.ON) {
			const { operation, modes, opcode } = getInstructions(intcode[pointer]);
			if (debugParam) {
				console.log({
					pointer,
					opcode,
					modes
				});
			}
			if (operation) {
				const result = operation(intcode, pointer, modes);
				pointer = result.pointer;
			} else {
				currentState = STATE.EXIT;
				return [];
			}

			if(outputBuffer.length === 1) {
				return outputBuffer.splice(0);
			}
		}
	};

	return {
		run,
		get state() {
			return currentState;
		}
	};
};
module.exports = makeProgram;
