import { getegid } from "process";
import { readFileWithSeparator } from "../utils";

const findSecretLoopSize = (resultKey: number, subjectNumber = 7) => {
	let value = 1;
	let loopSize = 0;
	while (value !== resultKey) {
		value *= subjectNumber;
		value = value % 20201227;
		loopSize++;
	}
	return loopSize;
};

const getEncryptionKey = (loopSize: number, subjectKey: number) => {
	let value = 1;
	for (let loop = 0; loop < loopSize; loop++) {
		value *= subjectKey;
		value = value % 20201227;
	}
	return value;
};

export default () => {
	const [pbKeyCard, pbKeyDoor] = readFileWithSeparator(
		"day-25/input.txt",
		"\n"
	).map((key) => parseInt(key));
	const doorLoopSize = findSecretLoopSize(pbKeyDoor);
	const keyLoopSize = findSecretLoopSize(pbKeyCard);

	const encryptionKey = getEncryptionKey(doorLoopSize, pbKeyCard);
	return {
		keyLoopSize,
		doorLoopSize,
		encryptionKey,
	};
};
