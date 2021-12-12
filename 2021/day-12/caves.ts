import { Image } from "https://deno.land/x/terminal@0.1.0-dev.3/src/mod.ts";
import { PALETTE } from "../renderer/render.ts";

export const SMALL_CAVE: Image = {
  width: 7,
  height: 5,
  pixels: [
    [PALETTE.BLACK, ...Array(5).fill(PALETTE.GRAY_DARK), PALETTE.BLACK],
    [
      ...Array(2).fill(PALETTE.GRAY_DARK),
      ...Array(3).fill(PALETTE.WHITE),
      ...Array(2).fill(PALETTE.GRAY_DARK),
    ],
    [PALETTE.GRAY_DARK, ...Array(5).fill(PALETTE.WHITE), PALETTE.GRAY_DARK],
    [
      ...Array(2).fill(PALETTE.GRAY_DARK),
      ...Array(3).fill(PALETTE.WHITE),
      ...Array(2).fill(PALETTE.GRAY_DARK),
    ],
    [PALETTE.BLACK, ...Array(5).fill(PALETTE.GRAY_DARK), PALETTE.BLACK],
  ],
};

export const BIG_CAVE: Image = {
  width: 9,
  height: 7,
  pixels: [
    [PALETTE.BLACK, ...Array(7).fill(PALETTE.GRAY_DARK), PALETTE.BLACK],
    [
      ...Array(2).fill(PALETTE.GRAY_DARK),
      ...Array(5).fill(PALETTE.WHITE),
      ...Array(2).fill(PALETTE.GRAY_DARK),
    ],
    [PALETTE.GRAY_DARK, ...Array(7).fill(PALETTE.WHITE), PALETTE.GRAY_DARK],
    [PALETTE.GRAY_DARK, ...Array(7).fill(PALETTE.WHITE), PALETTE.GRAY_DARK],
    [PALETTE.GRAY_DARK, ...Array(7).fill(PALETTE.WHITE), PALETTE.GRAY_DARK],
    [
      ...Array(2).fill(PALETTE.GRAY_DARK),
      ...Array(5).fill(PALETTE.WHITE),
      ...Array(2).fill(PALETTE.GRAY_DARK),
    ],
    [PALETTE.BLACK, ...Array(7).fill(PALETTE.GRAY_DARK), PALETTE.BLACK],
  ],
};

export const START_CAVE: Image = {
  width: 7,
  height: 5,
  pixels: [
    [PALETTE.BLACK, ...Array(5).fill(PALETTE.GRAY_DARK), PALETTE.BLACK],
    [
      ...Array(2).fill(PALETTE.GRAY_DARK),
      ...Array(3).fill(PALETTE.YELLOW),
      ...Array(2).fill(PALETTE.GRAY_DARK),
    ],
    [PALETTE.GRAY_DARK, ...Array(5).fill(PALETTE.YELLOW), PALETTE.GRAY_DARK],
    [
      ...Array(2).fill(PALETTE.GRAY_DARK),
      ...Array(3).fill(PALETTE.YELLOW),
      ...Array(2).fill(PALETTE.GRAY_DARK),
    ],
    [PALETTE.BLACK, ...Array(5).fill(PALETTE.GRAY_DARK), PALETTE.BLACK],
  ],
};

export const END_CAVE = {
  width: 7,
  height: 5,
  pixels: [
    [PALETTE.BLACK, ...Array(5).fill(PALETTE.GRAY_DARK), PALETTE.BLACK],
    [
      ...Array(2).fill(PALETTE.GRAY_DARK),
      ...Array(3).fill(PALETTE.GREEN_DARK),
      ...Array(2).fill(PALETTE.GRAY_DARK),
    ],
    [
      PALETTE.GRAY_DARK,
      ...Array(5).fill(PALETTE.GREEN_DARK),
      PALETTE.GRAY_DARK,
    ],
    [
      ...Array(2).fill(PALETTE.GRAY_DARK),
      ...Array(3).fill(PALETTE.GREEN_DARK),
      ...Array(2).fill(PALETTE.GRAY_DARK),
    ],
    [PALETTE.BLACK, ...Array(5).fill(PALETTE.GRAY_DARK), PALETTE.BLACK],
  ],
};

export const SPACING = 10;
