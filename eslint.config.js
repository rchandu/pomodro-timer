import js from "@eslint/js";
import ts from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import prettierConfig from "eslint-config-prettier";
import prettier from "eslint-plugin-prettier";
import solid from "eslint-plugin-solid";
import globals from "globals";

export default [
  js.configs.recommended,
  {
    files: ["**/*.{js,jsx,ts,tsx}"],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: "module",
        ecmaFeatures: {
          jsx: true,
        },
      },
      globals: { ...globals.browser },
    },
    plugins: {
      "@typescript-eslint": ts,
      solid,
      prettier,
    },
    rules: {
      ...ts.configs.recommended.rules,
      ...solid.configs.recommended.rules,
      ...prettierConfig.rules,
      "prettier/prettier": "error",
      "@typescript-eslint/no-unused-vars": [
        "error",
        { argsIgnorePattern: "^_" },
      ],
      "@typescript-eslint/explicit-function-return-type": "off",
      "@typescript-eslint/explicit-module-boundary-types": "off",
      "@typescript-eslint/no-explicit-any": "warn",
      "solid/reactivity": "error",
      "solid/no-destructure": "error",
      "solid/jsx-no-duplicate-props": "error",
    },
  },
  {
    files: ["**/*.config.{js,ts}", "**/vite.config.{js,ts}"],
    languageOptions: {
      globals: {
        process: "readonly",
        __dirname: "readonly",
      },
    },
  },
  {
    ignores: ["dist/", "node_modules/", "src-tauri/target/", "*.d.ts"],
  },
];
