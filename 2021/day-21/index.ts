// INPUT URL: https://adventofcode.com/2021/day/21/input
type PlayerState = {
  boardPosition: number;
  points: number;
};

type Players = [PlayerState, PlayerState];

type GameState = {
  prev?: GameState;
  dieValue: number;
  players: Players;
  currentPlayer: 0 | 1;
};

const getStartPositions = async () => {
  const inputString = await Deno.readTextFile("./day-21/input.txt");
  const [p1Start, p2Start] = inputString.split("\n").map((line) =>
    Number(line.trim().split(": ")[1])
  );

  return [p1Start, p2Start];
};

const runDeterministic = (game: GameState) => {
  const state: GameState = {
    prev: game,
    currentPlayer: (game.currentPlayer + 1) % 2 as 0 | 1,
    players: [...game.players.map((p) => ({ ...p }))] as Players,
    dieValue: game.dieValue,
  };
  let dieValue =
    (state.dieValue + 1 + state.dieValue + 2 + state.dieValue + 3) % 100;
  if (dieValue === 0) dieValue = 100;

  let boardPosition = (state.players[state.currentPlayer].boardPosition +
    dieValue) % 10;
  if (boardPosition === 0) boardPosition = 10;

  state.players[state.currentPlayer].points += boardPosition;
  state.players[state.currentPlayer].boardPosition = boardPosition;
  state.dieValue += 3;

  return state;
};

const runQuantum = (game: GameState) => {
  const state: GameState = {
    currentPlayer: (game.currentPlayer + 1) % 2 as 0 | 1,
    players: [...game.players.map((p) => ({ ...p }))] as Players,
    dieValue: game.dieValue,
  };

  let boardPosition = (state.players[state.currentPlayer].boardPosition +
    state.dieValue) % 10;
  if (boardPosition === 0) boardPosition = 10;

  state.players[state.currentPlayer].points += boardPosition;
  state.players[state.currentPlayer].boardPosition = boardPosition;

  return state;
};

const runDeterministicGame = (rootState: GameState) => {
  let nextState = rootState;
  let runs = 0;
  while (nextState.players[nextState.currentPlayer].points < 1000) {
    nextState = runDeterministic(nextState);
    runs++;
  }
  return {
    runs,
    state: nextState,
  };
};

const runQuantumGame = (
  state: GameState,
  gameCache: Record<string, [number, number]> = {},
) => {
  const cacheKey = JSON.stringify(state);
  if (cacheKey in gameCache) return gameCache[cacheKey];

  const result: [number, number] = [0, 0];
  for (let toss1 = 1; toss1 <= 3; toss1++) {
    for (let toss2 = 1; toss2 <= 3; toss2++) {
      for (let toss3 = 1; toss3 <= 3; toss3++) {
        const resultState = runQuantum({
          ...state,
          dieValue: toss1 + toss2 + toss3,
        });
        if (resultState.players[resultState.currentPlayer].points >= 21) {
          result[resultState.currentPlayer]++;
        } else {
          const gameResult = runQuantumGame(resultState, gameCache);
          result[0] += gameResult[0];
          result[1] += gameResult[1];
        }
      }
    }
  }
  gameCache[cacheKey] = result;
  return result;
};

export const part1 = async () => {
  const [p1Start, p2Start] = await getStartPositions();
  const rootState: GameState = {
    currentPlayer: 1,
    dieValue: 0,
    players: [
      {
        boardPosition: p1Start,
        points: 0,
      },
      {
        boardPosition: p2Start,
        points: 0,
      },
    ],
  };
  const { runs, state } = runDeterministicGame(rootState);
  return state.players[(state.currentPlayer + 1) % 2].points *
    (runs * 3);
};

export const part2 = async () => {
  const [p1Start, p2Start] = await getStartPositions();
  const rootState: GameState = {
    currentPlayer: 1,
    dieValue: 0,
    players: [
      {
        boardPosition: p1Start,
        points: 0,
      },
      {
        boardPosition: p2Start,
        points: 0,
      },
    ],
  };
  const games = runQuantumGame(rootState);
  return Math.max(...games);
};
