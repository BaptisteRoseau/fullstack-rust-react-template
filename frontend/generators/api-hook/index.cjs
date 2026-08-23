'use strict'

module.exports = {
    description: 'An SWR binding under src/api/hooks, over an existing domain fetcher',
    prompts: [
        {
            type: 'input',
            name: 'name',
            message:
                'Operation, without the "useApi" prefix (PascalCase, e.g. ApiKeys):',
        },
        {
            type: 'input',
            name: 'domain',
            message: 'Domain folder it reads from (camelCase, e.g. apiKeys):',
        },
    ],
    actions: [
        {
            type: 'add',
            path: 'src/api/hooks/useApi{{pascalCase name}}/useApi{{pascalCase name}}.ts',
            templateFile: 'generators/api-hook/hook.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/hooks/useApi{{pascalCase name}}/useApi{{pascalCase name}}.test.ts',
            templateFile: 'generators/api-hook/hook.test.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/api/hooks/useApi{{pascalCase name}}/index.ts',
            templateFile: 'generators/api-hook/index.ts.hbs',
        },
    ],
}
