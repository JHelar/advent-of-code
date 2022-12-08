import fs from "fs/promises";
import path from "path";

type Directory = {
  parent?: Directory;
  directories: Record<string, Directory>;
  files: Record<string, number>;
  fileSizes: number;
  size: number;
};

const readInput = async () => {
  const content = await fs.readFile(path.resolve(__dirname, "input.txt"));
  return content.toString("utf8");
};

const parseDirectoryTree = async () => {
  const content = await readInput();
  const rootDirectory: Directory = {
    files: {},
    directories: {},
    fileSizes: 0,
    size: 0,
  };
  let currentDirectory = rootDirectory;
  const lines = content
    .split("\n")
    .filter(Boolean)
    .map((line) => line.trim());
  for (let index = 0; index < lines.length; index++) {
    const [first, ...rest] = lines[index].split(" ");
    switch (first) {
      case "$":
        {
          const [command, ...args] = rest;
          switch (command) {
            case "cd": {
              const [destination] = args;
              switch (destination) {
                case "..":
                  currentDirectory = currentDirectory.parent ?? rootDirectory;
                  break;
                case "/":
                  currentDirectory = rootDirectory;
                  break;
                default:
                  if (!(destination in currentDirectory.directories)) {
                    let newDir: Directory = {
                      directories: {},
                      files: {},
                      fileSizes: 0,
                      size: 0,
                      parent: currentDirectory,
                    };

                    currentDirectory.directories[destination] = newDir;
                  }
                  currentDirectory = currentDirectory.directories[destination];
                  break;
              }
              break;
            }
            case "ls":
              {
                while (true) {
                  index++;
                  if (index >= lines.length) {
                    break;
                  }

                  const [ls_command, ...rest] = lines[index].split(" ");
                  if (ls_command === "$") {
                    index--;
                    break;
                  }

                  if (ls_command === "dir") {
                    const [directoryName] = rest;
                    if (!(directoryName in currentDirectory.directories)) {
                      currentDirectory.directories[directoryName] = {
                        directories: {},
                        files: {},
                        fileSizes: 0,
                        size: 0,
                        parent: currentDirectory,
                      };
                    }
                  } else {
                    const [fileName] = rest;
                    if (!(fileName in currentDirectory.directories)) {
                      const size = Number(ls_command);
                      currentDirectory.files[fileName] = size;
                    }
                  }
                }
              }
              break;
            default:
              break;
          }
        }
        break;

      default:
        break;
    }
  }

  return rootDirectory;
};

const stopid = (dir: Directory, limit: number): number[] => {
  const sizes = Object.values(dir.files).filter((value) => value <= limit);

  return [
    ...sizes,
    ...Object.values(dir.directories).flatMap((d) => stopid(d, limit)),
  ];
};

const setDirSize = (dir: Directory, directories: Directory[]) => {
  directories.push(dir)
  for (const childDir of Object.values(dir.directories)) {
    setDirSize(childDir, directories);
    dir.size += childDir.size;
  }

  dir.fileSizes = Object.values(dir.files)
    .reduce((sum, size) => sum + size, 0);

  dir.size += dir.fileSizes;
};

const getLimitedSum = (dir: Directory, limit: number): number => {
  let sum = 0;
  if(dir.size <= limit) {
    sum += dir.size
  }

  return Object.values(dir.directories).reduce((sum, childDir) => sum + getLimitedSum(childDir, limit), sum)
}

const removeParent = (dir: Directory) => {
  dir.parent = undefined
  Object.values(dir.directories).forEach(removeParent)
}

const part1 = async () => {
  const limit = 100000;
  const dir = await parseDirectoryTree();

  setDirSize(dir, []);
  return `Result: ${getLimitedSum(dir, limit)}`
};

const part2 = async () => {
  const rootDir = await parseDirectoryTree();
  const directories: Directory[] = []
  setDirSize(rootDir, directories);


  const availableSpace = 70000000 - rootDir.size
  const spaceNeeded = 30000000 - availableSpace

  const result = directories.reduce((best, dir) => {
    if(dir.size >= spaceNeeded && dir.size < best.size) return dir
    return best 
  }, rootDir)

  return `Result: ${result.size}`;
};

// Generated code to run on cli
(async () => {
  const [, , part] = process.argv;
  if (part === "1") {
    const result = await part1();
    console.log(result);
  }
  if (part === "2") {
    const result = await part2();
    console.log(result);
  }
})();
