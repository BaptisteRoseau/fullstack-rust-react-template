'use strict'

const js = require('@eslint/js')
const tsPlugin = require('@typescript-eslint/eslint-plugin')
const tsParser = require('@typescript-eslint/parser')
const vitestPlugin = require('@vitest/eslint-plugin')
const checkFilePlugin = require('eslint-plugin-check-file')
const prettierConfig = require('eslint-config-prettier')
const importPlugin = require('eslint-plugin-import')
const jestDomPlugin = require('eslint-plugin-jest-dom')
const jsxA11yPlugin = require('eslint-plugin-jsx-a11y')
const prettierPlugin = require('eslint-plugin-prettier')
const reactPlugin = require('eslint-plugin-react')
const reactHooksPlugin = require('eslint-plugin-react-hooks')
const testingLibraryPlugin = require('eslint-plugin-testing-library')
const globals = require('globals')

const reactVersion = require('./node_modules/react/package.json').version

const TS_FILES = ['**/*.ts', '**/*.tsx']
const TEST_FILES = [
    'src/**/__tests__/**/*.{ts,tsx}',
    'src/**/*.test.{ts,tsx}',
    'src/testing/**/*.{ts,tsx}',
]

module.exports = [
    {
        ignores: [
            'node_modules/**',
            'public/mockServiceWorker.js',
            'generators/**',
        ],
    },
    js.configs.recommended,
    // React recommended (flat config – uses ESLint v10 context API)
    { ...reactPlugin.configs.flat.recommended, files: TS_FILES },
    // React hooks (flat config)
    { ...reactHooksPlugin.configs.flat.recommended, files: TS_FILES },
    // jsx-a11y
    { ...jsxA11yPlugin.flatConfigs.recommended, files: TS_FILES },
    // import plugin
    { ...importPlugin.flatConfigs.recommended, files: TS_FILES },
    { ...importPlugin.flatConfigs.typescript, files: TS_FILES },
    // Testing library (test files only)
    { ...testingLibraryPlugin.configs['flat/react'], files: TEST_FILES },
    // jest-dom (test files only)
    { ...jestDomPlugin.configs['flat/recommended'], files: TEST_FILES },
    // Vitest (test files only) + vitest globals
    { ...vitestPlugin.configs.recommended, files: TEST_FILES },
    {
        files: TEST_FILES,
        languageOptions: {
            globals: vitestPlugin.environments.env.globals,
        },
    },
    // TypeScript + custom rules
    {
        files: TS_FILES,
        plugins: {
            '@typescript-eslint': tsPlugin,
            'check-file': checkFilePlugin,
            prettier: prettierPlugin,
        },
        languageOptions: {
            parser: tsParser,
            parserOptions: { ecmaFeatures: { jsx: true } },
            globals: {
                ...globals.browser,
                ...globals.node,
                ...globals.es2021,
            },
        },
        settings: {
            react: { version: reactVersion },
            'import/resolver': { typescript: {} },
        },
        rules: {
            // TypeScript handles undefined references better than ESLint
            'no-undef': 'off',
            // @typescript-eslint recommended rules
            ...tsPlugin.configs.recommended.rules,
            // Disable formatting rules (prettier takes over)
            ...prettierConfig.rules,
            'prettier/prettier': ['error', {}, { usePrettierrc: true }],
            // Import rules
            'import/no-restricted-paths': [
                'error',
                {
                    zones: [
                        {
                            target: './src/features/auth',
                            from: './src/features',
                            except: ['./auth'],
                        },
                        {
                            target: './src/features/comments',
                            from: './src/features',
                            except: ['./comments'],
                        },
                        {
                            target: './src/features/discussions',
                            from: './src/features',
                            except: ['./discussions'],
                        },
                        {
                            target: './src/features/teams',
                            from: './src/features',
                            except: ['./teams'],
                        },
                        {
                            target: './src/features/users',
                            from: './src/features',
                            except: ['./users'],
                        },
                        {
                            target: './src/features',
                            from: './src/app',
                        },
                        {
                            target: [
                                './src/components',
                                './src/hooks',
                                './src/lib',
                                './src/types',
                                './src/utils',
                            ],
                            from: ['./src/features', './src/app'],
                        },
                    ],
                },
            ],
            'import/no-cycle': 'error',
            'linebreak-style': ['error', 'unix'],
            'import/order': [
                'error',
                {
                    groups: [
                        'builtin',
                        'external',
                        'internal',
                        'parent',
                        'sibling',
                        'index',
                        'object',
                    ],
                    'newlines-between': 'always',
                    alphabetize: { order: 'asc', caseInsensitive: true },
                },
            ],
            'import/default': 'off',
            'import/no-named-as-default-member': 'off',
            'import/no-named-as-default': 'off',
            // React
            'react/prop-types': 'off',
            'react/react-in-jsx-scope': 'off',
            // jsx-a11y
            'jsx-a11y/anchor-is-valid': 'off',
            // TypeScript
            '@typescript-eslint/no-unused-vars': ['error'],
            '@typescript-eslint/explicit-function-return-type': ['off'],
            '@typescript-eslint/explicit-module-boundary-types': ['off'],
            '@typescript-eslint/no-empty-function': ['off'],
            '@typescript-eslint/no-explicit-any': ['off'],
            // File naming
            'check-file/filename-naming-convention': [
                'error',
                { '**/*.{ts,tsx}': 'KEBAB_CASE' },
                { ignoreMiddleExtensions: true },
            ],
        },
    },
    // Folder naming (non-test source files only)
    {
        files: ['src/**/!(__tests__)/*'],
        plugins: { 'check-file': checkFilePlugin },
        rules: {
            'check-file/folder-naming-convention': [
                'error',
                { '**/*': 'KEBAB_CASE' },
            ],
        },
    },
]
