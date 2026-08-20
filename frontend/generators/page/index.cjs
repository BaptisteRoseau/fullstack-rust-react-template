'use strict'

module.exports = {
    description: 'A page folder under src/pages',
    prompts: [
        { type: 'input', name: 'name', message: 'Page name (PascalCase):' },
        { type: 'input', name: 'title', message: 'Page title shown to the user:' },
    ],
    actions: [
        {
            type: 'add',
            path: 'src/pages/{{pascalCase name}}/{{pascalCase name}}.tsx',
            templateFile: 'generators/page/page.tsx.hbs',
        },
        {
            type: 'add',
            path: 'src/pages/{{pascalCase name}}/{{kebabCase name}}.module.scss',
            templateFile: 'generators/page/page.module.scss.hbs',
        },
        {
            type: 'add',
            path: 'src/pages/{{pascalCase name}}/{{pascalCase name}}.test.tsx',
            templateFile: 'generators/page/page.test.tsx.hbs',
        },
        {
            type: 'add',
            path: 'src/pages/{{pascalCase name}}/index.ts',
            templateFile: 'generators/page/index.ts.hbs',
        },
    ],
}
