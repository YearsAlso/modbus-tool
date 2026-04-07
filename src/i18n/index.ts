/**
 * i18n - Internationalization setup
 * Structured for easy extension to additional languages
 */

import { en, type Translations } from "./locales/en";
import { zh } from "./locales/zh";

// Supported locales
export const locales = ["en", "zh"] as const;
export type Locale = (typeof locales)[number];

// Translation map
const translations: Record<Locale, Translations> = {
  en,
  zh,
};

// Current locale - can be extended to use settings store or browser preference
function getCurrentLocale(): Locale {
  // TODO: integrate with settings store when available
  // const { settings } = useSettingsStore.getState();
  // if (settings.locale && locales.includes(settings.locale as Locale)) {
  //   return settings.locale as Locale;
  // }
  const browserLang = navigator.language.split("-")[0];
  if (locales.includes(browserLang as Locale)) {
    return browserLang as Locale;
  }
  return "en";
}

let currentLocale: Locale = getCurrentLocale();

export function setLocale(locale: Locale) {
  if (locales.includes(locale)) {
    currentLocale = locale;
  }
}

export function getLocale(): Locale {
  return currentLocale;
}

export function t(): Translations {
  return translations[currentLocale];
}

/**
 * Interpolate a translation string with placeholders
 * Usage: t("script.deleteConfirmDesc", { name: "My Script" })
 */
export function tKey(key: string, params?: Record<string, string | number>): string {
  const keys = key.split(".");
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let value: any = translations[currentLocale];
  for (const k of keys) {
    value = value?.[k];
  }
  if (typeof value !== "string") return key;

  if (params) {
    return Object.entries(params).reduce(
      (str, [k, v]) => str.replace(new RegExp(`\\{${k}\\}`, "g"), String(v)),
      value
    );
  }
  return value;
}
