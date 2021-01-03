import React, { FC, useEffect, useState, useRef } from 'react';
import { Text, Newline, Box } from 'ink';
import { IntProgram } from '../../intcode'
import { InputHandler } from './inputHandler'
import { MapView } from './map'
import fs from 'fs'

const input = fs.readFileSync('game.txt').toString()
const program = new IntProgram(input)

const getString = (buffer: bigint[]) => buffer.map(v => String.fromCharCode(Number(v))).join('').split('\n')

const App: FC<{name?: string}> = () => {
	const [output, setOutput] = useState<bigint[]>([])
	const input = useRef<bigint[]>([])
	const currentCommand = useRef('')
	
	const [initialized, setInitialized] = useState(false)

	const advanceGame = () => {
		let programOutput: bigint[] = []
		while(true) {
			program.exec(input.current, programOutput)
			if(!program.running) break
		}
		setOutput(programOutput)
	}

	const setInput = (command: string) => {
		input.current = command.split('').map(v => BigInt(v.charCodeAt(0)))
		input.current.push(10n)
		currentCommand.current = command
		advanceGame()
	}

	useEffect(() => {
		if(!initialized) {
			advanceGame()
			setInitialized(true)
		}
	}, [initialized, setInitialized])

	return <Box width="100%">
			<Box flexDirection="column" borderStyle="classic" borderColor="redBright" width="80%">
				<Box width="100%">
					<Text>
						{getString(output).map((str, strIndex) => <Text key={strIndex}>{str}<Newline /></Text>)}
					</Text>
				</Box>
				<Box width="100%">
					<InputHandler setInput={setInput} />
				</Box>
			</Box>
		<Box width="20%">
			<MapView output={getString(output)} command={currentCommand.current}/>
		</Box>
	</Box>
}

module.exports = App;
export default App;
