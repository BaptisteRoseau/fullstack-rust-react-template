'use strict'

module.exports = {
    description: 'A shared hook under src/hooks',
    prompts: [
        { type: 'input', name: 'name', message: 'Hook name without the "use" prefix (camelCase):' },
    ],
    actions: [
        {
            type: 'add',
            path: 'src/hooks/use{{pascalCase name}}/use{{pascalCase name}}.ts',
            templateFile: 'generators/hook/hook.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/hooks/use{{pascalCase name}}/use{{pascalCase name}}.test.ts',
            templateFile: 'generators/hook/hook.test.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/hooks/use{{pascalCase name}}/index.ts',
            templateFile: 'generators/hook/index.ts.hbs',
        },
    ],
}
