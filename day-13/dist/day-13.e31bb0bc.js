// modules are defined as an array
// [ module function, map of requires ]
//
// map of requires is short require name -> numeric require
//
// anything defined in a previous bundle is accessed via the
// orig method which is the require for previous bundles
parcelRequire = (function (modules, cache, entry, globalName) {
  // Save the require from previous bundle to this closure if any
  var previousRequire = typeof parcelRequire === 'function' && parcelRequire;
  var nodeRequire = typeof require === 'function' && require;

  function newRequire(name, jumped) {
    if (!cache[name]) {
      if (!modules[name]) {
        // if we cannot find the module within our internal map or
        // cache jump to the current global require ie. the last bundle
        // that was added to the page.
        var currentRequire = typeof parcelRequire === 'function' && parcelRequire;
        if (!jumped && currentRequire) {
          return currentRequire(name, true);
        }

        // If there are other bundles on this page the require from the
        // previous one is saved to 'previousRequire'. Repeat this as
        // many times as there are bundles until the module is found or
        // we exhaust the require chain.
        if (previousRequire) {
          return previousRequire(name, true);
        }

        // Try the node require function if it exists.
        if (nodeRequire && typeof name === 'string') {
          return nodeRequire(name);
        }

        var err = new Error('Cannot find module \'' + name + '\'');
        err.code = 'MODULE_NOT_FOUND';
        throw err;
      }

      localRequire.resolve = resolve;
      localRequire.cache = {};

      var module = cache[name] = new newRequire.Module(name);

      modules[name][0].call(module.exports, localRequire, module, module.exports, this);
    }

    return cache[name].exports;

    function localRequire(x){
      return newRequire(localRequire.resolve(x));
    }

    function resolve(x){
      return modules[name][1][x] || x;
    }
  }

  function Module(moduleName) {
    this.id = moduleName;
    this.bundle = newRequire;
    this.exports = {};
  }

  newRequire.isParcelRequire = true;
  newRequire.Module = Module;
  newRequire.modules = modules;
  newRequire.cache = cache;
  newRequire.parent = previousRequire;
  newRequire.register = function (id, exports) {
    modules[id] = [function (require, module) {
      module.exports = exports;
    }, {}];
  };

  var error;
  for (var i = 0; i < entry.length; i++) {
    try {
      newRequire(entry[i]);
    } catch (e) {
      // Save first error but execute all entries
      if (!error) {
        error = e;
      }
    }
  }

  if (entry.length) {
    // Expose entry point to Node, AMD or browser globals
    // Based on https://github.com/ForbesLindesay/umd/blob/master/template.js
    var mainExports = newRequire(entry[entry.length - 1]);

    // CommonJS
    if (typeof exports === "object" && typeof module !== "undefined") {
      module.exports = mainExports;

    // RequireJS
    } else if (typeof define === "function" && define.amd) {
     define(function () {
       return mainExports;
     });

    // <script>
    } else if (globalName) {
      this[globalName] = mainExports;
    }
  }

  // Override the current require with this new one
  parcelRequire = newRequire;

  if (error) {
    // throw error from earlier, _after updating parcelRequire_
    throw error;
  }

  return newRequire;
})({"read-intcode.js":[function(require,module,exports) {
function _slicedToArray(arr, i) { return _arrayWithHoles(arr) || _iterableToArrayLimit(arr, i) || _nonIterableRest(); }

function _nonIterableRest() { throw new TypeError("Invalid attempt to destructure non-iterable instance"); }

function _iterableToArrayLimit(arr, i) { if (!(Symbol.iterator in Object(arr) || Object.prototype.toString.call(arr) === "[object Arguments]")) { return; } var _arr = []; var _n = true; var _d = false; var _e = undefined; try { for (var _i = arr[Symbol.iterator](), _s; !(_n = (_s = _i.next()).done); _n = true) { _arr.push(_s.value); if (i && _arr.length === i) break; } } catch (err) { _d = true; _e = err; } finally { try { if (!_n && _i["return"] != null) _i["return"](); } finally { if (_d) throw _e; } } return _arr; }

function _arrayWithHoles(arr) { if (Array.isArray(arr)) return arr; }

var STATE = {
  ON: 'ON',
  INPUT: 'INPUT',
  EXIT: 'EXIT'
};
var PARAMETER_MODE = {
  POSITION: '0',
  IMMEDIATE: '1',
  RELATIVE: '2'
};
var debugParam = false;

var makeProgram = function makeProgram(intcode) {
  intcode = intcode.reduce(function (map, code, address) {
    map[address] = BigInt(code);
    return map;
  }, {});
  var outputBuffer = [];
  var inputBuffer = [];
  var currentState = STATE.ON;
  var pointer = BigInt(0);
  var relativeBase = BigInt(0);

  var getParameterAddress = function getParameterAddress(intcode, pointer, mode) {
    switch (mode) {
      case PARAMETER_MODE.POSITION:
        return intcode[pointer];

      case PARAMETER_MODE.IMMEDIATE:
        return pointer;

      case PARAMETER_MODE.RELATIVE:
        return sanitizeParam(intcode[pointer]) + relativeBase;
    }
  };

  var getParameterAddresses = function getParameterAddresses(intcode, pointer, modes) {
    return modes.map(function (mode, index) {
      return getParameterAddress(intcode, pointer + BigInt(index + 1), mode);
    });
  };

  var makeOperation = function makeOperation(operation) {
    return function (intcode, pointer, modes) {
      var addresses = getParameterAddresses(intcode, pointer, modes);
      return operation(intcode, pointer, addresses);
    };
  };

  var sanitizeParam = function sanitizeParam(param) {
    return param ? typeof param === 'bigint' ? param : BigInt(param) : BigInt(0);
  };

  var addOperation = makeOperation(function (intcode, pointer, params) {
    var _params = _slicedToArray(params, 3),
        paramAddress1 = _params[0],
        paramAddress2 = _params[1],
        outputAddress = _params[2];

    var param1 = sanitizeParam(intcode[paramAddress1]);
    var param2 = sanitizeParam(intcode[paramAddress2]);
    var output = param1 + param2;
    intcode[outputAddress] = output;
    return {
      pointer: pointer + BigInt(4)
    };
  });
  var multiplyOperation = makeOperation(function (intcode, pointer, params) {
    var _params2 = _slicedToArray(params, 3),
        paramAddress1 = _params2[0],
        paramAddress2 = _params2[1],
        outputAddress = _params2[2];

    var param1 = sanitizeParam(intcode[paramAddress1]);
    var param2 = sanitizeParam(intcode[paramAddress2]);
    var output = param1 * param2;
    intcode[outputAddress] = output;
    return {
      pointer: pointer + BigInt(4)
    };
  });
  var inputOperation = makeOperation(function (intcode, pointer, params) {
    var _params3 = _slicedToArray(params, 1),
        inputAddress = _params3[0];

    var inputValue = sanitizeParam(inputBuffer.pop());
    currentState = STATE.INPUT;
    intcode[inputAddress] = inputValue;
    return {
      pointer: pointer + BigInt(2)
    };
  });
  var outputOperation = makeOperation(function (intcode, pointer, params) {
    var _params4 = _slicedToArray(params, 1),
        outputAddress = _params4[0];

    var output = intcode[outputAddress];
    outputBuffer.push(output.toString());
    return {
      pointer: pointer + BigInt(2),
      output: output
    };
  });
  var jumpIfTrueOperation = makeOperation(function (intcode, pointer, params) {
    var _params5 = _slicedToArray(params, 2),
        paramAddress1 = _params5[0],
        paramAddress2 = _params5[1];

    var checkValue = sanitizeParam(intcode[paramAddress1]);
    var gotoValue = pointer + BigInt(3);

    if (checkValue != 0) {
      gotoValue = sanitizeParam(intcode[paramAddress2]);
    }

    return {
      pointer: gotoValue
    };
  });
  var jumpIfFalseOperation = makeOperation(function (intcode, pointer, params) {
    var _params6 = _slicedToArray(params, 2),
        paramAddress1 = _params6[0],
        paramAddress2 = _params6[1];

    var checkValue = sanitizeParam(intcode[paramAddress1]);
    var gotoValue = pointer + BigInt(3);

    if (checkValue == 0) {
      gotoValue = sanitizeParam(intcode[paramAddress2]);
    }

    return {
      pointer: gotoValue
    };
  });
  var lessThanOperation = makeOperation(function (intcode, pointer, params) {
    var _params7 = _slicedToArray(params, 3),
        paramAddress1 = _params7[0],
        paramAddress2 = _params7[1],
        outputAddress = _params7[2];

    var oneValue = sanitizeParam(intcode[paramAddress1]);
    var anotherValue = sanitizeParam(intcode[paramAddress2]);
    var outputValue = oneValue < anotherValue ? 1 : 0;
    intcode[outputAddress] = BigInt(outputValue);
    return {
      pointer: pointer + BigInt(4)
    };
  });
  var equalsOperation = makeOperation(function (intcode, pointer, params) {
    var _params8 = _slicedToArray(params, 3),
        paramAddress1 = _params8[0],
        paramAddress2 = _params8[1],
        outputAddress = _params8[2];

    var oneValue = sanitizeParam(intcode[paramAddress1]);
    var anotherValue = sanitizeParam(intcode[paramAddress2]);
    var outputValue = oneValue == anotherValue ? 1 : 0;
    intcode[outputAddress] = BigInt(outputValue);
    return {
      pointer: pointer + BigInt(4)
    };
  });
  var adjustRelativeBaseOperation = makeOperation(function (intcode, pointer, params) {
    var _params9 = _slicedToArray(params, 1),
        paramAddress1 = _params9[0];

    var adjustValue = sanitizeParam(intcode[paramAddress1]);
    relativeBase = relativeBase + adjustValue;
    return {
      pointer: pointer + BigInt(2)
    };
  });
  var OP = {
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

  var getInstructions = function getInstructions(instructionString) {
    instructionString = instructionString.toString().padStart(4, '0');
    var opcode = instructionString.slice(instructionString.length - 2);
    var operation = OP[opcode] || false;
    var modes = instructionString.slice(0, instructionString.length - 2).padStart(3, '0').split('').reverse();
    return {
      operation: operation,
      modes: modes,
      opcode: opcode | 0
    };
  };

  var run = function run(input) {
    if (input !== undefined) {
      inputBuffer.push(input);
    }

    while (currentState !== STATE.EXIT) {
      var _getInstructions = getInstructions(intcode[pointer]),
          operation = _getInstructions.operation,
          modes = _getInstructions.modes,
          opcode = _getInstructions.opcode;

      if (debugParam) {
        console.log({
          pointer: pointer,
          opcode: opcode,
          modes: modes
        });
      }

      if (operation) {
        var result = operation(intcode, pointer, modes);
        pointer = result.pointer;
      } else {
        currentState = STATE.EXIT;
        return [];
      }

      if (outputBuffer.length === 6) {
        return outputBuffer.splice(0);
      }
    }
  };

  return {
    run: run,

    get state() {
      return currentState;
    }

  };
};

module.exports = makeProgram;
},{}],"index.js":[function(require,module,exports) {
function _slicedToArray(arr, i) { return _arrayWithHoles(arr) || _iterableToArrayLimit(arr, i) || _nonIterableRest(); }

function _nonIterableRest() { throw new TypeError("Invalid attempt to destructure non-iterable instance"); }

function _iterableToArrayLimit(arr, i) { if (!(Symbol.iterator in Object(arr) || Object.prototype.toString.call(arr) === "[object Arguments]")) { return; } var _arr = []; var _n = true; var _d = false; var _e = undefined; try { for (var _i = arr[Symbol.iterator](), _s; !(_n = (_s = _i.next()).done); _n = true) { _arr.push(_s.value); if (i && _arr.length === i) break; } } catch (err) { _d = true; _e = err; } finally { try { if (!_n && _i["return"] != null) _i["return"](); } finally { if (_d) throw _e; } } return _arr; }

function _arrayWithHoles(arr) { if (Array.isArray(arr)) return arr; }

var makeProgram = require('./read-intcode');

var intcode = "2,380,379,385,1008,2367,810138,381,1005,381,12,99,109,2368,1101,0,0,383,1102,0,1,382,21001,382,0,1,20101,0,383,2,21102,37,1,0,1106,0,578,4,382,4,383,204,1,1001,382,1,382,1007,382,36,381,1005,381,22,1001,383,1,383,1007,383,24,381,1005,381,18,1006,385,69,99,104,-1,104,0,4,386,3,384,1007,384,0,381,1005,381,94,107,0,384,381,1005,381,108,1105,1,161,107,1,392,381,1006,381,161,1102,1,-1,384,1106,0,119,1007,392,34,381,1006,381,161,1101,0,1,384,20101,0,392,1,21101,22,0,2,21101,0,0,3,21101,0,138,0,1105,1,549,1,392,384,392,21001,392,0,1,21102,22,1,2,21101,3,0,3,21101,161,0,0,1106,0,549,1101,0,0,384,20001,388,390,1,21001,389,0,2,21102,1,180,0,1106,0,578,1206,1,213,1208,1,2,381,1006,381,205,20001,388,390,1,20101,0,389,2,21101,0,205,0,1106,0,393,1002,390,-1,390,1101,0,1,384,21002,388,1,1,20001,389,391,2,21102,228,1,0,1106,0,578,1206,1,261,1208,1,2,381,1006,381,253,21002,388,1,1,20001,389,391,2,21101,0,253,0,1105,1,393,1002,391,-1,391,1102,1,1,384,1005,384,161,20001,388,390,1,20001,389,391,2,21102,279,1,0,1105,1,578,1206,1,316,1208,1,2,381,1006,381,304,20001,388,390,1,20001,389,391,2,21101,0,304,0,1106,0,393,1002,390,-1,390,1002,391,-1,391,1101,1,0,384,1005,384,161,21002,388,1,1,20102,1,389,2,21102,1,0,3,21102,1,338,0,1105,1,549,1,388,390,388,1,389,391,389,21001,388,0,1,21002,389,1,2,21101,4,0,3,21101,365,0,0,1105,1,549,1007,389,23,381,1005,381,75,104,-1,104,0,104,0,99,0,1,0,0,0,0,0,0,213,16,19,1,1,18,109,3,21202,-2,1,1,22102,1,-1,2,21101,0,0,3,21102,1,414,0,1105,1,549,21201,-2,0,1,21201,-1,0,2,21101,429,0,0,1106,0,601,2101,0,1,435,1,386,0,386,104,-1,104,0,4,386,1001,387,-1,387,1005,387,451,99,109,-3,2106,0,0,109,8,22202,-7,-6,-3,22201,-3,-5,-3,21202,-4,64,-2,2207,-3,-2,381,1005,381,492,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,481,21202,-4,8,-2,2207,-3,-2,381,1005,381,518,21202,-2,-1,-1,22201,-3,-1,-3,2207,-3,-2,381,1006,381,507,2207,-3,-4,381,1005,381,540,21202,-4,-1,-1,22201,-3,-1,-3,2207,-3,-4,381,1006,381,529,21201,-3,0,-7,109,-8,2106,0,0,109,4,1202,-2,36,566,201,-3,566,566,101,639,566,566,1202,-1,1,0,204,-3,204,-2,204,-1,109,-4,2105,1,0,109,3,1202,-1,36,594,201,-2,594,594,101,639,594,594,20102,1,0,-2,109,-3,2105,1,0,109,3,22102,24,-2,1,22201,1,-1,1,21102,1,439,2,21102,1,233,3,21102,1,864,4,21101,0,630,0,1106,0,456,21201,1,1503,-2,109,-3,2105,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,2,0,0,2,2,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,2,2,0,2,0,2,0,0,0,0,0,0,0,1,1,0,0,2,2,0,0,0,2,2,2,0,0,0,2,0,2,2,0,0,0,0,2,2,2,0,0,0,2,2,0,0,0,2,0,1,1,0,0,0,2,2,2,2,2,0,0,2,0,2,0,2,0,2,0,2,0,0,0,0,2,2,0,0,2,0,2,2,2,2,0,1,1,0,0,2,0,2,0,0,2,0,2,0,0,0,2,2,2,0,0,2,0,2,2,0,2,2,2,0,0,0,0,0,2,2,0,1,1,0,2,0,2,0,2,2,2,2,0,0,2,2,0,0,2,0,2,2,0,2,2,2,0,0,0,0,0,0,0,0,0,2,0,1,1,0,0,2,0,0,0,2,2,2,0,0,0,0,0,2,0,2,0,0,0,2,0,2,2,2,0,0,2,0,2,0,0,0,0,1,1,0,2,0,0,0,2,0,2,2,0,0,0,0,2,2,0,2,0,0,2,2,0,0,0,0,0,2,0,2,2,0,0,0,0,1,1,0,2,2,2,0,2,0,0,2,0,2,0,0,0,2,2,0,2,0,0,0,0,2,0,0,2,0,2,0,2,0,0,2,0,1,1,0,0,0,0,0,0,2,2,0,0,2,2,0,2,2,0,2,0,0,0,2,2,0,0,0,0,0,2,0,0,2,2,2,0,1,1,0,2,2,0,2,0,2,2,0,2,2,2,0,2,0,0,2,2,0,0,2,2,0,2,0,0,0,0,0,2,0,0,0,0,1,1,0,0,2,0,0,2,2,0,2,2,0,2,2,0,2,2,0,0,2,0,0,0,0,0,2,2,0,0,0,2,2,0,0,0,1,1,0,0,2,2,0,2,0,2,2,0,2,2,0,0,2,2,2,0,0,0,0,0,0,0,2,0,0,2,0,0,0,2,0,0,1,1,0,2,2,2,0,0,2,0,0,0,2,0,0,2,0,2,0,0,2,2,0,0,0,2,0,2,2,2,0,0,0,0,2,0,1,1,0,0,0,2,2,0,2,0,0,2,0,0,0,2,2,2,0,0,0,2,2,0,0,2,0,0,0,0,2,0,0,0,0,0,1,1,0,0,2,2,2,0,0,2,0,0,0,0,0,2,0,2,2,2,2,0,0,0,2,0,0,2,0,2,0,2,0,2,2,0,1,1,0,0,0,2,0,0,0,0,0,0,0,2,0,0,2,2,0,2,2,0,0,2,0,2,2,0,2,0,0,0,0,2,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,6,16,92,72,80,96,26,54,61,9,5,36,81,14,76,83,59,88,74,16,69,57,76,35,51,42,88,8,89,80,10,96,1,1,18,93,46,94,5,40,63,18,90,31,7,8,46,96,13,53,21,13,66,64,67,26,11,77,23,46,8,20,97,24,26,9,11,65,36,71,13,35,18,81,66,40,88,89,56,9,24,8,78,16,44,20,57,1,69,13,71,77,37,68,31,18,32,60,37,70,8,3,46,41,18,50,78,87,4,94,91,82,96,76,15,73,47,85,6,8,92,7,46,72,68,90,22,78,6,66,1,26,98,85,80,66,95,39,62,81,52,35,98,71,58,8,55,16,93,75,77,47,36,41,91,39,20,97,13,5,31,67,91,96,10,75,7,95,59,43,90,94,25,89,21,66,98,11,2,49,67,25,96,91,72,51,41,47,1,76,30,55,20,5,97,61,73,43,79,13,16,81,1,27,11,96,47,89,48,26,25,34,37,64,13,53,69,41,30,39,93,80,34,75,41,30,42,42,49,68,49,37,40,62,93,77,86,49,72,52,5,78,31,86,86,62,60,56,17,19,80,2,39,49,50,85,63,48,61,95,82,21,18,85,16,86,45,75,28,97,33,21,56,96,40,33,95,69,53,75,47,70,51,80,92,4,54,79,59,42,15,30,86,27,86,63,64,36,27,98,49,58,78,5,16,57,61,32,14,25,51,75,96,93,25,87,20,76,32,96,96,39,84,48,62,82,11,36,1,11,44,71,86,58,4,74,35,3,31,3,27,52,78,96,10,43,16,93,93,61,23,54,90,47,81,70,81,26,89,17,63,60,48,29,77,53,80,80,12,79,80,76,37,80,79,54,17,73,68,15,40,64,81,5,62,74,27,42,72,93,2,21,46,29,76,51,61,13,19,21,96,45,38,47,87,47,67,95,82,56,51,32,1,73,59,83,65,33,92,8,94,14,45,60,20,87,82,1,29,9,15,10,76,90,27,80,30,65,9,79,2,97,41,75,8,68,23,37,19,80,22,15,52,93,79,79,23,61,37,5,88,28,5,44,31,36,20,37,71,45,21,25,16,2,79,28,67,19,47,9,19,64,46,8,88,29,75,65,22,64,32,78,20,88,48,72,90,84,50,59,63,20,86,58,50,97,14,61,10,68,45,81,43,27,95,95,80,91,68,17,83,55,49,41,9,33,51,19,60,54,24,43,68,36,60,5,20,97,14,55,70,35,27,96,80,32,3,63,52,70,31,2,58,3,70,54,35,83,87,83,50,14,97,47,38,44,71,52,3,97,83,24,36,11,45,5,87,21,80,88,98,45,42,37,96,28,42,72,47,39,58,78,23,24,50,78,1,87,81,32,49,21,60,28,33,29,5,38,36,8,59,52,66,67,15,95,87,61,67,80,54,58,36,89,72,96,78,32,58,37,39,76,43,69,20,96,26,71,98,50,36,46,18,68,24,50,43,32,95,70,18,18,66,84,18,13,44,44,6,4,42,37,31,88,18,82,29,41,88,12,96,58,61,72,72,79,80,60,48,15,26,24,29,45,7,36,2,16,31,13,60,13,84,53,4,5,94,52,39,8,14,6,30,70,75,46,13,38,57,24,24,69,51,87,96,65,57,57,14,10,27,97,98,18,4,92,47,6,17,66,93,3,82,83,56,75,82,75,92,35,68,1,43,51,24,13,57,33,87,62,92,38,61,90,1,95,45,4,70,63,34,43,67,5,91,75,23,55,27,70,52,16,78,87,46,2,56,89,88,58,23,95,31,98,96,22,11,61,29,55,77,50,55,96,64,33,14,51,25,47,48,3,15,2,18,63,12,56,47,88,74,32,87,21,74,53,37,93,21,37,9,42,16,39,57,57,59,57,96,88,17,14,5,85,18,40,54,47,80,22,35,84,10,43,91,10,82,85,52,70,69,64,44,93,77,72,80,39,86,20,44,48,24,72,810138".split(','); //require('../utils').readArray('./day-13/input.txt');

var getMapKey = function getMapKey(x, y) {
  return "(".concat(x, ",").concat(y, ")");
};

var program = makeProgram(intcode);
var TILE_TYPES = ['EMPTY', 'WALL', 'BLOCK', 'PADDLE', 'BALL'];
var TILE_WIDTH = 10;
var TILE_HEIGHT = 10;
var COLORS = {
  'EMPTY': '#ffffff',
  'BLOCK': '#000000',
  'WALL': '#ececec',
  'PADDLE': '#00ff00',
  'BALL': '#0000ff'
};

var game = function game() {
  var tiles = {};
  var score = 0;
  var playerPos;
  var ballPos;
  var input_buffer = [];
  var canvas = document.getElementById('game');
  var context = canvas.getContext('2d');
  var scoreElement = document.getElementById('score');

  var updateTile = function updateTile(x, y, type) {
    if (x === -1 && y === 0) {
      // Update score
      score = type | 0;
    } else {
      var mapKey = getMapKey(x | 0, y | 0);
      var tileType = TILE_TYPES[type | 0];

      if (tileType === 'BALL') {
        ballPos = {
          x: x,
          y: y
        };
      } else if (tileType === 'PADDLE') {
        playerPos = {
          x: x,
          y: y
        };
      } else {
        if (mapKey in tiles) {
          tiles[mapKey].type = TILE_TYPES[type | 0];
        } else {
          tiles[mapKey] = {
            x: x,
            y: y,
            type: TILE_TYPES[type | 0]
          };
        }
      }
    }
  };

  var draw = function draw() {
    context.clearRect(0, 0, canvas.width, canvas.height);
    Object.values(tiles).forEach(function (tile) {
      context.fillStyle = COLORS[tile.type];
      context.fillRect(tile.x * TILE_WIDTH, tile.y * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
    });

    if (playerPos) {
      context.fillStyle = COLORS.PADDLE;
      context.fillRect(playerPos.x * TILE_WIDTH, playerPos.y * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
    }

    if (ballPos) {
      context.fillStyle = COLORS.BALL;
      context.fillRect(ballPos.x * TILE_WIDTH, ballPos.y * TILE_HEIGHT, TILE_WIDTH, TILE_HEIGHT);
    }

    scoreElement.innerHTML = "Score: ".concat(score);
  };

  var update = function update() {
    var input = input_buffer.pop();

    if (program.state === 'EXIT') {
      scoreElement.innerHTML = 'GAME OVER';
    } else {
      if (program.state !== 'INPUT' || input !== undefined) {
        console.log({
          input: input
        });
        var output = program.run(input);
        console.log({
          output: output
        });

        if (output) {
          var _output = _slicedToArray(output, 6),
              cx1 = _output[0],
              cy1 = _output[1],
              ct1 = _output[2],
              cx2 = _output[3],
              cy2 = _output[4],
              ct2 = _output[5];

          updateTile(cx1, cy1, ct1);
          updateTile(cx2, cy2, ct2);
          draw();
        }
      }

      window.requestAnimationFrame(update);
    }
  };

  document.addEventListener('keyup', function (e) {
    switch (e.key) {
      case 'ArrowRight':
        input_buffer.push(1);
        break;

      case 'ArrowLeft':
        input_buffer.push(-1);
        break;

      default:
        input_buffer.push(0);
        break;
    }
  });
  update();
};

game();
},{"./read-intcode":"read-intcode.js"}],"C:/Users/Johnh/AppData/Roaming/npm/node_modules/parcel/src/builtins/hmr-runtime.js":[function(require,module,exports) {
var global = arguments[3];
var OVERLAY_ID = '__parcel__error__overlay__';
var OldModule = module.bundle.Module;

function Module(moduleName) {
  OldModule.call(this, moduleName);
  this.hot = {
    data: module.bundle.hotData,
    _acceptCallbacks: [],
    _disposeCallbacks: [],
    accept: function (fn) {
      this._acceptCallbacks.push(fn || function () {});
    },
    dispose: function (fn) {
      this._disposeCallbacks.push(fn);
    }
  };
  module.bundle.hotData = null;
}

module.bundle.Module = Module;
var checkedAssets, assetsToAccept;
var parent = module.bundle.parent;

if ((!parent || !parent.isParcelRequire) && typeof WebSocket !== 'undefined') {
  var hostname = "" || location.hostname;
  var protocol = location.protocol === 'https:' ? 'wss' : 'ws';
  var ws = new WebSocket(protocol + '://' + hostname + ':' + "53896" + '/');

  ws.onmessage = function (event) {
    checkedAssets = {};
    assetsToAccept = [];
    var data = JSON.parse(event.data);

    if (data.type === 'update') {
      var handled = false;
      data.assets.forEach(function (asset) {
        if (!asset.isNew) {
          var didAccept = hmrAcceptCheck(global.parcelRequire, asset.id);

          if (didAccept) {
            handled = true;
          }
        }
      }); // Enable HMR for CSS by default.

      handled = handled || data.assets.every(function (asset) {
        return asset.type === 'css' && asset.generated.js;
      });

      if (handled) {
        console.clear();
        data.assets.forEach(function (asset) {
          hmrApply(global.parcelRequire, asset);
        });
        assetsToAccept.forEach(function (v) {
          hmrAcceptRun(v[0], v[1]);
        });
      } else if (location.reload) {
        // `location` global exists in a web worker context but lacks `.reload()` function.
        location.reload();
      }
    }

    if (data.type === 'reload') {
      ws.close();

      ws.onclose = function () {
        location.reload();
      };
    }

    if (data.type === 'error-resolved') {
      console.log('[parcel] ✨ Error resolved');
      removeErrorOverlay();
    }

    if (data.type === 'error') {
      console.error('[parcel] 🚨  ' + data.error.message + '\n' + data.error.stack);
      removeErrorOverlay();
      var overlay = createErrorOverlay(data);
      document.body.appendChild(overlay);
    }
  };
}

function removeErrorOverlay() {
  var overlay = document.getElementById(OVERLAY_ID);

  if (overlay) {
    overlay.remove();
  }
}

function createErrorOverlay(data) {
  var overlay = document.createElement('div');
  overlay.id = OVERLAY_ID; // html encode message and stack trace

  var message = document.createElement('div');
  var stackTrace = document.createElement('pre');
  message.innerText = data.error.message;
  stackTrace.innerText = data.error.stack;
  overlay.innerHTML = '<div style="background: black; font-size: 16px; color: white; position: fixed; height: 100%; width: 100%; top: 0px; left: 0px; padding: 30px; opacity: 0.85; font-family: Menlo, Consolas, monospace; z-index: 9999;">' + '<span style="background: red; padding: 2px 4px; border-radius: 2px;">ERROR</span>' + '<span style="top: 2px; margin-left: 5px; position: relative;">🚨</span>' + '<div style="font-size: 18px; font-weight: bold; margin-top: 20px;">' + message.innerHTML + '</div>' + '<pre>' + stackTrace.innerHTML + '</pre>' + '</div>';
  return overlay;
}

function getParents(bundle, id) {
  var modules = bundle.modules;

  if (!modules) {
    return [];
  }

  var parents = [];
  var k, d, dep;

  for (k in modules) {
    for (d in modules[k][1]) {
      dep = modules[k][1][d];

      if (dep === id || Array.isArray(dep) && dep[dep.length - 1] === id) {
        parents.push(k);
      }
    }
  }

  if (bundle.parent) {
    parents = parents.concat(getParents(bundle.parent, id));
  }

  return parents;
}

function hmrApply(bundle, asset) {
  var modules = bundle.modules;

  if (!modules) {
    return;
  }

  if (modules[asset.id] || !bundle.parent) {
    var fn = new Function('require', 'module', 'exports', asset.generated.js);
    asset.isNew = !modules[asset.id];
    modules[asset.id] = [fn, asset.deps];
  } else if (bundle.parent) {
    hmrApply(bundle.parent, asset);
  }
}

function hmrAcceptCheck(bundle, id) {
  var modules = bundle.modules;

  if (!modules) {
    return;
  }

  if (!modules[id] && bundle.parent) {
    return hmrAcceptCheck(bundle.parent, id);
  }

  if (checkedAssets[id]) {
    return;
  }

  checkedAssets[id] = true;
  var cached = bundle.cache[id];
  assetsToAccept.push([bundle, id]);

  if (cached && cached.hot && cached.hot._acceptCallbacks.length) {
    return true;
  }

  return getParents(global.parcelRequire, id).some(function (id) {
    return hmrAcceptCheck(global.parcelRequire, id);
  });
}

function hmrAcceptRun(bundle, id) {
  var cached = bundle.cache[id];
  bundle.hotData = {};

  if (cached) {
    cached.hot.data = bundle.hotData;
  }

  if (cached && cached.hot && cached.hot._disposeCallbacks.length) {
    cached.hot._disposeCallbacks.forEach(function (cb) {
      cb(bundle.hotData);
    });
  }

  delete bundle.cache[id];
  bundle(id);
  cached = bundle.cache[id];

  if (cached && cached.hot && cached.hot._acceptCallbacks.length) {
    cached.hot._acceptCallbacks.forEach(function (cb) {
      cb();
    });

    return true;
  }
}
},{}]},{},["C:/Users/Johnh/AppData/Roaming/npm/node_modules/parcel/src/builtins/hmr-runtime.js","index.js"], null)
//# sourceMappingURL=/day-13.e31bb0bc.js.map