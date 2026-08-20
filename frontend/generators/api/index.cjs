'use strict'

module.exports = {
    description: 'An API domain: declaration, service, manual mock and test',
    prompts: [
        { type: 'input', name: 'name', message: 'Domain name (camelCase, e.g. apiKeys):' },
        { type: 'input', name: 'path', message: 'Endpoint path (e.g. /api/api-key):' },
    ],
    actions: [
        {
            type: 'add',
            path: 'src/api/{{camelCase name}}.ts',
            templateFile: 'generators/api/domain.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/service/{{camelCase name}}.ts',
            templateFile: 'generators/api/service.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/service/__mocks__/{{camelCase name}}.ts',
            templateFile: 'generators/api/mock.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/service/{{camelCase name}}.test.ts',
            templateFile: 'generators/api/service.test.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/test-utils/mocks/handlers/{{camelCase name}}.ts',
            templateFile: 'generators/api/handlers.ts.hbs',
        },
    ],
}
