# Internationalization (i18n) Guide

This guide explains how internationalization is implemented in Brewget and how to add new translations.

## Overview

Brewget supports internationalization in both the frontend (Vue.js) and backend (Rust). Currently supported languages:
- 🇬🇧 English (en) - Default
- 🇪🇸 Spanish (es)
- 🇫🇷 French (fr)
- 🇩🇪 German (de)

## Frontend (Vue.js)

### Architecture

- **Library**: vue-i18n v10
- **Translation Files**: `/frontend/src/locales/*.json`
- **Configuration**: `/frontend/src/i18n.ts`
- **Language Detection**: `/frontend/src/composables/useLanguage.ts`

### How It Works

1. **For Non-Logged Users**: Uses browser language or defaults to English
2. **For Logged-In Users**: Uses language from user settings
3. **Language Switching**: Automatically switches when user changes language in settings

### Adding New Translations

1. Add the key-value pair to all language files in `/frontend/src/locales/`:
```json
{
  "section": {
    "key": "Translation text"
  }
}
```

2. Use in Vue components:
```vue
<script setup>
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
</script>

<template>
  <div>{{ t('section.key') }}</div>
</template>
```

3. Use in stores/composables:
```typescript
import { useI18n } from 'vue-i18n';
const { t } = useI18n();
const message = t('section.key');
```

### Adding a New Language

1. Create a new locale file: `/frontend/src/locales/{lang}.json`
2. Copy structure from `en.json` and translate all strings
3. Update `/frontend/src/i18n.ts`:
```typescript
import newLang from "./locales/{lang}.json";

// Add to supported languages
const supportedLanguages = ["en", "es", "fr", "de", "{lang}"];

// Add to messages
messages: {
  en,
  es,
  fr,
  de,
  {lang}: newLang,
}
```

4. Update `/frontend/src/composables/useLanguage.ts`:
```typescript
const supportedLanguages = ["en", "es", "fr", "de", "{lang}"];
```

5. Add language option in `/frontend/src/views/SettingsView.vue`:
```typescript
const languageOptions = [
  // ... existing options
  { label: t("languages.{lang}"), value: "{lang}" },
];
```

## Backend (Rust)

### Architecture

- **Module**: `shared-types::i18n`
- **Translation Function**: `i18n::translate(key, lang)`
- **Language Detection**: `i18n::extract_language(accept_language_header)`

### How It Works

1. Extracts language from `Accept-Language` HTTP header
2. Looks up translation key in language-specific function
3. Falls back to English if key not found

### Adding New Translations

1. Add translation key to `/backend/shared-types/src/i18n.rs`:

```rust
fn translate_en(key: &str) -> String {
    match key {
        // ... existing keys
        "new.key" => "English translation",
        _ => key,
    }
    .to_string()
}

// Repeat for translate_es, translate_fr, translate_de
```

2. Use in route handlers:

```rust
use shared_types::i18n;
use axum::http::{HeaderMap, header::ACCEPT_LANGUAGE};

async fn handler(
    headers: HeaderMap,
    // ... other params
) -> Result<impl IntoResponse, Error> {
    // Extract language
    let lang = i18n::extract_language(
        headers.get(ACCEPT_LANGUAGE).and_then(|v| v.to_str().ok())
    );
    
    // Use translation
    let msg = i18n::translate("new.key", &lang);
    // Use msg in response
}
```

### Adding a New Language

1. Update `/backend/shared-types/src/i18n.rs`:

```rust
// Add translation function
fn translate_{lang}(key: &str) -> String {
    match key {
        "auth.username_or_password_invalid" => "Translation...",
        // ... all other keys
        _ => return translate_en(key),
    }
    .to_string()
}

// Update main translate function
pub fn translate(key: &str, lang: &str) -> String {
    match lang {
        "es" => translate_es(key),
        "fr" => translate_fr(key),
        "de" => translate_de(key),
        "{lang}" => translate_{lang}(key),
        _ => translate_en(key),
    }
}

// Update extract_language to support new language
.filter(|lang| matches!(lang.as_str(), "en" | "es" | "fr" | "de" | "{lang}"))
```

2. Add tests:

```rust
#[test]
fn test_translate_{lang}() {
    assert_eq!(
        translate("auth.username_or_password_invalid", "{lang}"),
        "Expected translation"
    );
}
```

## Best Practices

1. **Keep keys organized**: Use hierarchical structure (e.g., `auth.errors.loginFailed`)
2. **Consistent fallback**: Always fall back to English for unsupported languages
3. **Test translations**: Ensure all language files have the same keys
4. **Context in keys**: Make key names descriptive (e.g., `settings.saveButton` not just `save`)
5. **Backend minimalism**: Only translate user-facing error messages, not internal logs
6. **Frontend completeness**: Translate all user-visible text including labels, buttons, and messages

## Testing

### Frontend
```bash
cd frontend
npm run type-check  # Verify TypeScript
npm run lint        # Check code style
npm run build       # Verify build works
```

### Backend
```bash
cd backend
cargo test -p shared-types i18n  # Run i18n tests
cargo build -p auth-service      # Verify integration
```

## Future Improvements

Potential enhancements for the i18n system:
- [ ] Load translations dynamically to reduce bundle size
- [ ] Add more languages (Portuguese, Italian, Chinese, Japanese, etc.)
- [ ] Translate more backend error messages
- [ ] Add date/time formatting based on locale
- [ ] Add number/currency formatting based on locale
- [ ] Create translation management tool or integrate with service like Crowdin
- [ ] Add RTL (right-to-left) language support
- [ ] Implement pluralization rules for different languages
