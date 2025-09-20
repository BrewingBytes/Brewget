import skipFormatting from "@vue/eslint-config-prettier/skip-formatting";
import vueTsEslintConfig from "@vue/eslint-config-typescript";
import importPlugin from "eslint-plugin-import";
import pluginVue from "eslint-plugin-vue";

export default [
  {
    name: "app/files-to-lint",
    files: ["**/*.{ts,mts,tsx,vue}"],
  },

  {
    name: "app/files-to-ignore",
    ignores: ["**/dist/**", "**/dist-ssr/**", "**/coverage/**"],
  },

  ...pluginVue.configs["flat/essential"],
  ...vueTsEslintConfig(),
  importPlugin.flatConfigs.recommended,
  skipFormatting,
  {
    rules: {
      "quotes": ["error", "double"],
      "semi": "error",
      "comma-dangle": ["error", "always-multiline"],
      "no-restricted-imports": ["error", {
        "patterns": ["../*"],
      }],
      "no-console": [
        "warn",{
          allow: ["error"],
        },
      ],
      "spaced-comment": [
        "error",
        "always",
        {
          markers: ["/"],
        },
      ],
      curly: "error",
      "prefer-template": "error",
      "import/no-unresolved": "off",
      "import/order": [
        "error",
        {
          named: true,
          "newlines-between": "always",
          alphabetize: {
            order: "asc",
          },
          groups: [
            "builtin",
            ["external", "internal"],
            ["parent", "sibling", "index", "object"],
            "type",
          ],
          pathGroups: [
            {
              group: "builtin",
              pattern: "react",
              position: "before",
            },
            {
              group: "external",
              pattern: "@mui/icons-material",
              position: "after",
            },
          ],
        },
      ],
    },
  },
];
