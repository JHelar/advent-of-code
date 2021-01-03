"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const react_1 = __importStar(require("react"));
const ink_1 = require("ink");
const intcode_1 = require("../../intcode");
const inputHandler_1 = require("./inputHandler");
const map_1 = require("./map");
const fs_1 = __importDefault(require("fs"));
const input = fs_1.default.readFileSync('game.txt').toString();
const program = new intcode_1.IntProgram(input);
const getString = (buffer) => buffer.map(v => String.fromCharCode(Number(v))).join('').split('\n');
const App = () => {
    const [output, setOutput] = react_1.useState([]);
    const input = react_1.useRef([]);
    const currentCommand = react_1.useRef('');
    const [initialized, setInitialized] = react_1.useState(false);
    const advanceGame = () => {
        let programOutput = [];
        while (true) {
            program.exec(input.current, programOutput);
            if (!program.running)
                break;
        }
        setOutput(programOutput);
    };
    const setInput = (command) => {
        input.current = command.split('').map(v => BigInt(v.charCodeAt(0)));
        input.current.push(10n);
        currentCommand.current = command;
        advanceGame();
    };
    react_1.useEffect(() => {
        if (!initialized) {
            advanceGame();
            setInitialized(true);
        }
    }, [initialized, setInitialized]);
    return react_1.default.createElement(ink_1.Box, { width: "100%" },
        react_1.default.createElement(ink_1.Box, { flexDirection: "column", borderStyle: "classic", borderColor: "redBright", width: "80%" },
            react_1.default.createElement(ink_1.Box, { width: "100%" },
                react_1.default.createElement(ink_1.Text, null, getString(output).map((str, strIndex) => react_1.default.createElement(ink_1.Text, { key: strIndex },
                    str,
                    react_1.default.createElement(ink_1.Newline, null))))),
            react_1.default.createElement(ink_1.Box, { width: "100%" },
                react_1.default.createElement(inputHandler_1.InputHandler, { setInput: setInput }))),
        react_1.default.createElement(ink_1.Box, { width: "20%" },
            react_1.default.createElement(map_1.MapView, { output: getString(output), command: currentCommand.current })));
};
module.exports = App;
exports.default = App;
