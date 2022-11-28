import * as LANGUAGES_HANDLERS from "../languages";
import { logger } from "./logger";

export type LanguageOption = keyof typeof LANGUAGES_HANDLERS;
const LANGUAGES = Object.keys(LANGUAGES_HANDLERS);
const isLanguageOption = (value: string): value is LanguageOption =>
  LANGUAGES.includes(value as any);

export const getLanguage = (language: string) => {
  if (!isLanguageOption(language)) {
    logger.error(
      `Unknown language "${language}", please provide Santa a language he knows: ${LANGUAGES.join(
        ", "
      )}`
    );
    throw new Error('Unknown language')
  }

  return LANGUAGES_HANDLERS[language];
};
