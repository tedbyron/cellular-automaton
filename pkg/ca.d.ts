/* tslint:disable */
/* eslint-disable */
/**
* A two-dimensional cellular automaton with a finite number of cells.
*/
export class Automaton {
  free(): void;
/**
* Constructs a new automaton with all cell states set to 0. Defaults to rules
* from Conway's Game of Life.
*
* # Examples
*
* ```
* todo!();
* ```
* @param {number} rows
* @param {number} cols
*/
  constructor(rows: number, cols: number);
/**
* Returns a raw pointer to the automaton cells' buffer.
*
* # Examples
*
* ```
* todo!();
* ```
* @returns {number}
*/
  getCellsPtr(): number;
/**
* Toggles the state of a cell. If the cell state is 0, it is set to 1. If the
* cell is any other state, it is set to 0.
*
* # Examples
* ```
* todo!();
* ```
* @param {number} row
* @param {number} col
*/
  toggleCell(row: number, col: number): void;
/**
* Sets the state of cells in `locations` to 1.
*
* `locations` is a list of alternating row and column coordinates. This
* function is implemented with an array as the parameter because
* `wasm_bindgen` does not support nested arrays.
*
* # Examples
* ```
* todo!();
* ```
* @param {Uint32Array} locations
*/
  setCellsOn(locations: Uint32Array): void;
/**
* Sets the cell state of all the automaton's cells to `n`.
*
* Only changes the automaton if `n` is less than or equal to the generation
* rule.
*
* # Examples
* ```
* todo!();
* ```
* @param {number} n
*/
  setAllCells(n: number): void;
/**
* Randomizes the cell state of all the automaton's cells.
*
* Loops through the automaton's cells and if `rand::random()` is less than the
* percentage `n`, the cell state is set to 1.
*
* # Examples
* ```
* todo!();
* ```
* @param {number} n
*/
  randomizeCells(n: number): void;
/**
* Calculates and the state of all cells in the automaton after `n` generations
*
* # Examples
* ```
* todo!();
* ```
* @param {number} n
*/
  step(n: number): void;
/**
* Resizes the automaton so that `cols` is equal to `width`.
*
* If `width` is greater than `cols`, the automaton's rows are extended by the
* difference, with each additional column filled with 0. If `width` is less
* than `cols`, the automaton's rows are simply truncated.
*
* # Examples
*
* ```
* todo!();
* ```
* @param {number} width
*/
  cols: number;
/**
* Resizes the automaton so that `rows` is equal to `height`.
*
* If `height` is greater than `rows`, the automaton's columns are extended by
* the difference, with each additional row filled with 0. If `height` is less
* than `rows`, the automaton's columns are simply truncated.
*
* # Examples
*
* ```
* todo!();
* ```
* @param {number} height
*/
  rows: number;
}
/**
*r" A ruleset containing birth and survival (B/S) rules.
*/
export class RulesetBS {
  free(): void;
/**
* Constructs a new ruleset containing birth, survival, and generation rules.
*
* # Examples
*
* ```
* todo!()
* ```
* @param {Int8Array} b
* @param {Int8Array} s
*/
  constructor(b: Int8Array, s: Int8Array);
/**
* @returns {Int8Array}
*/
  birth: Int8Array;
/**
* Sets the birth rule.
*
* # Examples
*
* ```
* todo!()
* ```
* @param {Int8Array} b
*/
  setBirthRule: Int8Array;
/**
* Sets the survival rule.
*
* # Examples
*
* ```
* todo!()
* ```
* @param {Int8Array} s
*/
  setSurvivalRule: Int8Array;
/**
* @returns {Int8Array}
*/
  survival: Int8Array;
}
/**
*r" A ruleset containing birth, survival, and generation (B/S/C) rules.
*/
export class RulesetBSC {
  free(): void;
/**
* Constructs a new ruleset containing birth, survival, and generation rules.
*
* # Examples
*
* ```
* todo!()
* ```
* @param {Int8Array} b
* @param {Int8Array} s
* @param {number} c
*/
  constructor(b: Int8Array, s: Int8Array, c: number);
/**
* @returns {Int8Array}
*/
  birth: Int8Array;
/**
* @returns {number}
*/
  generation: number;
/**
* Sets the birth rule.
*
* # Examples
*
* ```
* todo!()
* ```
* @param {Int8Array} b
*/
  setBirthRule: Int8Array;
/**
* Sets the generation rule.
*
* # Examples
*
* ```
* todo!()
* ```
* @param {number} c
*/
  setGenerationRule: number;
/**
* Sets the survival rule.
*
* # Examples
*
* ```
* todo!()
* ```
* @param {Int8Array} s
*/
  setSurvivalRule: Int8Array;
/**
* @returns {Int8Array}
*/
  survival: Int8Array;
}
