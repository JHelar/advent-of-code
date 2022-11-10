import {
  Color,
  Colors,
  createColor,
} from "https://deno.land/x/terminal@0.1.0-dev.3/src/mod.ts";

export const PALETTE = {
  ...Colors,
  BLUE_DARK: createColor(15, 15, 35),
  BLUE_LIGHT: createColor(9, 126, 192),
  BROWN: createColor(150, 122, 60),
  YELLOW: createColor(247, 251, 98),
  GRAY_LIGHT: createColor(93, 93, 93),
  GRAY_DARK: createColor(33, 33, 33),
  GREEN_LIGHT: createColor(5, 160, 11),
  GREEN_DARK: createColor(14, 94, 18),
  TEXT: createColor(186, 186, 183),
  RED: createColor(232, 11, 33),
};

export const mult = ({ r, g, b }: Color, val: number) =>
  createColor(r * val, g * val, b * val);
