import { getLanguage } from './getLanguage'

type GnomeArgs = {
    lang?: ReturnType<typeof getLanguage>
    day?: number
    part?: 1 | 2
}

export const parseArgs = () => {
    const [,,...args] = process.argv
    const defaultArgs: GnomeArgs = {
        lang: undefined,
        day: undefined,
        part: undefined
    }
    return args.join('=').split('=-').reduce((args, arg) => {
        const [key, value] = arg.replace(/-/g, '').split('=')
        if(!(key in defaultArgs)) return args
        return {
            ...args,
            [key]: key === 'lang' ? getLanguage(value) : Number(value)
        }
    }, {} as GnomeArgs)
}