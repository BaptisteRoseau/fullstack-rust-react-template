'use strict'

module.exports = {
    description: 'A design-system primitive or a shared component',
    prompts: [
        {
            type: 'list',
            name: 'layer',
            message: 'Which layer?',
            choices: [
                { name: 'design-system (domain-agnostic primitive)', value: 'design-system' },
                { name: 'components (domain-aware, shared)', value: 'components' },
            ],
        },
        {
            type: 'input',
            name: 'group',
            message: 'Grouping folder (kebab-case, empty for none):',
            default: '',
        },
        {
            type: 'input',
            name: 'name',
            message: 'Component name (PascalCase):',
        },
    ],
    actions(answers) {
        const base = answers.group
            ? `src/${answers.layer}/${answers.group}/{{pascalCase name}}`
            : `src/${answers.layer}/{{pascalCase name}}`

        const actions = [
            {
                type: 'add',
                path: `${base}/{{pascalCase name}}.tsx`,
                templateFile: 'generators/component/component.tsx.hbs',
            },
            {
                type: 'add',
                path: `${base}/{{kebabCase name}}.module.scss`,
                templateFile: 'generators/component/component.module.scss.hbs',
            },
            {
                type: 'add',
                path: `${base}/{{pascalCase name}}.test.tsx`,
                templateFile: 'generators/component/component.test.tsx.hbs',
            },
            {
                type: 'add',
                path: `${base}/index.ts`,
                templateFile: 'generators/component/index.ts.hbs',
            },
        ]

        if (answers.layer === 'design-system') {
            actions.push({
                type: 'add',
                path: `${base}/{{pascalCase name}}.stories.tsx`,
                templateFile: 'generators/component/component.stories.tsx.hbs',
            })
        }

        return actions
    },
}
