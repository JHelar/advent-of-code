// INPUT URL: https://adventofcode.com/2021/day/4/input
type Tile = {
  value: number;
  taken: boolean;
};
type Board = Array<Tile>;

const BOARD_SIZE = 5;

const getBingoBoard = async () => {
  const inputString = await Deno.readTextFile("./day-4/input.txt");
  const [numberLine, , ...rest] = inputString.split("\n");

  const boards: Board[] = [];
  let currentBoard: Board = [];
  for (const line of rest) {
    if (line === "") {
      boards.push(currentBoard);
      currentBoard = [];
      continue;
    }
    currentBoard.push(
      ...line.split(" ").filter(Boolean).map((digit) => ({
        value: Number(digit.trim()),
        taken: false,
      })),
    );
  }
  boards.push(currentBoard);
  const numberSequence = numberLine.split(",").map(Number);

  return {
    numberSequence,
    boards,
  };
};

type BingoState = Tile[] | null;
const checkBingo = (board: Board): BingoState => {
  for (let i = 0; i < board.length; i += BOARD_SIZE) {
    const row = board.slice(i, i + BOARD_SIZE);
    if (row.every(({ taken }) => taken)) return row;
  }

  for (let i = 0; i < BOARD_SIZE; i++) {
    const column = Array(BOARD_SIZE).fill(0).map((_, j) =>
      board[i + j * BOARD_SIZE]
    );
    if (column.every(({ taken }) => taken)) return column;
  }

  return null;
};

const setNum = (num: number) =>
  (board: Board): Board =>
    board.map((tile) => ({
      taken: tile.taken || tile.value === num,
      value: tile.value,
    }));

const countInactive = (board: Board): number =>
  board.reduce((sum, { taken, value }) => !taken ? sum + value : sum, 0);

const runRound = (
  [num, ...sequence]: number[],
  boards: Board[],
): [Board[], number, number[]] => {
  const newBoards = boards.map(setNum(num));
  for (let i = 0; i < newBoards.length; i++) {
    const board = newBoards[i];
    const bingo = checkBingo(board);
    if (bingo !== null) {
      const inactive = countInactive(board);
      return [
        newBoards.filter((board) => checkBingo(board) === null),
        inactive * num,
        sequence,
      ];
    }
  }
  return [newBoards, -1, sequence];
};

export const part1 = async () => {
  let { numberSequence, boards } = await getBingoBoard();
  let winner = -1;
  while (numberSequence.length) {
    [boards, winner, numberSequence] = runRound(numberSequence, boards);
    if (winner > -1) return winner;
  }
};

export const part2 = async () => {
  let { numberSequence, boards } = await getBingoBoard();
  let winner = -1;
  while (boards.length) {
    [boards, winner, numberSequence] = runRound(numberSequence, boards);
  }
  return winner;
};
