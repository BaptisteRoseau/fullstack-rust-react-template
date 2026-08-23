'use strict'

const js = require('@eslint/js')
const tsPlugin = require('@typescript-eslint/eslint-plugin')
const tsParser = require('@typescript-eslint/parser')
const vitestPlugin = require('@vitest/eslint-plugin')
const checkFilePlugin = require('eslint-plugin-check-file')
const cssModulesPlugin = require('eslint-plugin-css-modules')
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

// The converter layer only exists if nothing can bypass it: outside src/api the
// generated SDK is unreachable, and a domain is reachable only through its
// barrel. Both public names live one level down -- @/api/domains/<domain> and
// @/api/hooks/useApiXxx -- so the patterns block what sits deeper than either.
// Repeated into every no-restricted-imports block below, because a later flat
// config block replaces the rule rather than adding to it.
const API_BOUNDARY_PATTERNS = [
    {
        group: ['@/api/generated', '@/api/generated/*'],
        message:
            'Wire types stay inside src/api. Import the domain type from @/api/domains/<domain> instead.',
    },
    {
        group: [
            '@/api/domains/*/*',
            '@/api/domains/*/*/**',
            '@/api/hooks/*/*',
            '@/api/hooks/*/*/**',
        ],
        message:
            'Import a domain through its barrel: @/api/domains/<domain>, or @/api/hooks/useApiXxx.',
    },
]

// MSW handlers and the fixtures feeding them must speak the wire shape, and the
// compiler only enforces that if they can name the generated types.
const MOCK_BOUNDARY_PATTERNS = API_BOUNDARY_PATTERNS.slice(1)
const TEST_FILES = [
    'src/**/*.test.{ts,tsx}',
    'src/test-utils/**/*.{ts,tsx}',
    'src/**/__mocks__/**/*.{ts,tsx}',
]

module.exports = [
    {
        ignores: [
            'node_modules/**',
            'public/mockServiceWorker.js',
            'generators/**',
            'src/api/generated/**',
            'src/i18n/locales/**',
        ],
    },
    js.configs.recommended,
    { ...reactPlugin.configs.flat.recommended, files: TS_FILES },
    { ...reactHooksPlugin.configs.flat.recommended, files: TS_FILES },
    { ...jsxA11yPlugin.flatConfigs.recommended, files: TS_FILES },
    { ...importPlugin.flatConfigs.recommended, files: TS_FILES },
    { ...importPlugin.flatConfigs.typescript, files: TS_FILES },
    { ...testingLibraryPlugin.configs['flat/react'], files: TEST_FILES },
    { ...jestDomPlugin.configs['flat/recommended'], files: TEST_FILES },
    { ...vitestPlugin.configs.recommended, files: TEST_FILES },
    {
        files: TEST_FILES,
        languageOptions: {
            globals: vitestPlugin.environments.env.globals,
        },
    },
    {
        files: TS_FILES,
        plugins: {
            '@typescript-eslint': tsPlugin,
            'css-modules': cssModulesPlugin,
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
            'no-undef': 'off',
            ...tsPlugin.configs.recommended.rules,
            ...prettierConfig.rules,
            'prettier/prettier': ['error', {}, { usePrettierrc: true }],
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
            'react/prop-types': 'off',
            'react/react-in-jsx-scope': 'off',
            'jsx-a11y/anchor-is-valid': 'off',
            '@typescript-eslint/no-unused-vars': ['error'],
            '@typescript-eslint/explicit-function-return-type': ['off'],
            '@typescript-eslint/explicit-module-boundary-types': ['off'],
            '@typescript-eslint/no-empty-function': ['off'],
            '@typescript-eslint/no-explicit-any': ['error'],
            'css-modules/no-undef-class': 'error',
        },
    },
    {
        files: ['e2e/**/*.ts', 'playwright.config.ts'],
        rules: {
            'react-hooks/rules-of-hooks': 'off',
        },
    },
    {
        files: TS_FILES,
        ignores: ['src/api/**'],
        rules: {
            'no-restricted-imports': ['error', { patterns: API_BOUNDARY_PATTERNS }],
        },
    },
    {
        files: ['src/design-system/**'],
        rules: {
            'no-restricted-imports': [
                'error',
                {
                    patterns: [
                        {
                            group: [
                                '@/api/*',
                                '@/api/domains/*',
                                '@/contexts/*',
                                '@/components/*',
                                '@/pages/*',
                                '@/layouts/*',
                            ],
                            message:
                                'The design system must stay domain-agnostic.',
                        },
                        ...API_BOUNDARY_PATTERNS,
                    ],
                },
            ],
        },
    },
    {
        files: ['src/pages/*/**'],
        rules: {
            'no-restricted-imports': [
                'error',
                {
                    patterns: [
                        {
                            group: ['@/pages/*/*'],
                            message:
                                'Pages must not import each other. Move shared code to src/components.',
                        },
                        ...API_BOUNDARY_PATTERNS,
                    ],
                },
            ],
        },
    },
    {
        files: ['src/test-utils/**/*.{ts,tsx}'],
        rules: {
            'no-restricted-imports': [
                'error',
                { patterns: MOCK_BOUNDARY_PATTERNS },
            ],
        },
    },
    {
        files: ['src/**/*.{ts,tsx}'],
        plugins: { 'check-file': checkFilePlugin },
        rules: {
            'check-file/folder-naming-convention': [
                'error',
                {
                    'src/pages/*/': 'PASCAL_CASE',
                    'src/layouts/*/': 'PASCAL_CASE',
                    'src/api/*/': 'CAMEL_CASE',
                    'src/api/domains/*/': 'CAMEL_CASE',
                    'src/api/hooks/*/': 'CAMEL_CASE',
                },
            ],
            'check-file/filename-naming-convention': [
                'error',
                {
                    'src/{hooks,utils,types,router,stores,constants}/**/*.{ts,tsx}':
                        'CAMEL_CASE',
                    'src/api/**/*.{ts,tsx}': 'CAMEL_CASE',
                },
                { ignoreMiddleExtensions: true },
            ],
        },
    },
]
