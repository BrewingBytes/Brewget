import type { SupportedLocale } from "@/i18n";

/**
 * Translates a backend translation key to the appropriate localized message
 * @param translationKey The translation key from the backend (e.g., "ACCOUNT_CREATED")
 * @param locale The current locale
 * @param messages The i18n messages object
 * @returns The translated message
 */
export function translateKey(
  translationKey: string,
  locale: SupportedLocale,
  messages: Record<string, Record<string, unknown>>,
): string {
  const key = `translation_keys.${translationKey}`;
  const message = messages[locale];

  if (!message) {
    return translationKey;
  }

  // Navigate through the nested object
  const parts = key.split(".");
  let current: unknown = message;

  for (const part of parts) {
    if (current && typeof current === "object" && part in current) {
      current = (current as Record<string, unknown>)[part];
    } else {
      return translationKey;
    }
  }

  return typeof current === "string" ? current : translationKey;
}
