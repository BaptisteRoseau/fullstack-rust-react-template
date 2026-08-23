'use strict'

module.exports = {
    description:
        'An API domain under src/api/domains: fetchers, domain types, converters, cache keys and its MSW handler',
    prompts: [
        {
            type: 'input',
            name: 'name',
            message: 'Domain name (camelCase, e.g. apiKeys):',
        },
        {
            type: 'input',
            name: 'path',
            message: 'Backend path, with its /api prefix (e.g. /api/api-key):',
        },
    ],
    actions: [
        {
            type: 'add',
            path: 'src/api/domains/{{camelCase name}}/{{camelCase name}}.ts',
            templateFile: 'generators/api/domain.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/domains/{{camelCase name}}/{{camelCase name}}.test.ts',
            templateFile: 'generators/api/domain.test.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/domains/{{camelCase name}}/types.ts',
            templateFile: 'generators/api/types.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/domains/{{camelCase name}}/converters.ts',
            templateFile: 'generators/api/converters.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/domains/{{camelCase name}}/converters.test.ts',
            templateFile: 'generators/api/converters.test.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/domains/{{camelCase name}}/keys.ts',
            templateFile: 'generators/api/keys.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/domains/{{camelCase name}}/index.ts',
            templateFile: 'generators/api/index.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/test-utils/mocks/handlers/{{camelCase name}}.ts',
            templateFile: 'generators/api/handlers.ts.hbs',
        },
    ],
}
