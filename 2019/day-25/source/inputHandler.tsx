import React, { FC, useState } from 'react'
import { useInput, Text } from 'ink'

export const InputHandler: FC<{ setInput: (str: string) => void }> = ({ setInput }) => {
    const [inputString, setInputString] = useState('')

    useInput((e, key) => {
        if(key.return) {
            setInput(inputString)
            setInputString('')
        } else if(key.delete) {
            setInputString(prev => prev.slice(0, prev.length - 1))
        } else {
            setInputString(prev => prev + e)
        }
    })

    return <Text color="cyan">{inputString}</Text>
} 
