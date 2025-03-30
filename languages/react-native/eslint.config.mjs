import js from '@eslint/js';
import typescriptEslint from 'typescript-eslint';
import prettier from 'eslint-config-prettier/flat';

export default typescriptEslint.config(
  js.configs.recommended,
  ...typescriptEslint.configs.recommendedTypeChecked,
  prettier,
  {
    plugins: ['@react-native'],
  }
);
