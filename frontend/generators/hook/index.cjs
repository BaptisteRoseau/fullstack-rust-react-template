const path = require('path')
const fs = require('fs')

const featuresDir = path.join(process.cwd(), 'src/features')
const features = fs.readdirSync(featuresDir)

/**
 * Scaffolds a reusable hook. Shared hooks live in `src/hooks/` (with a colocated
 * `__tests__/` test); feature hooks live in `src/features/<feature>/hooks/`.
 * Provide the name WITHOUT the `use` prefix. See
 * `.claude/skills/frontend-react-hook`.
 *
 * @type {import('plop').PlopGenerator}
 */
module.exports = {
    description: 'Custom hook generator',
    prompts: [
        {
            type: 'input',
            name: 'name',
            message: 'hook name without "use" prefix (e.g. "toggle")',
        },
        {
            type: 'list',
            name: 'feature',
            message: 'Where does this hook live?',
            choices: ['hooks (shared)', ...features],
        },
    ],
    actions: (answers) => {
        const shared = answers.feature === 'hooks (shared)'
        const base = shared ? 'src/hooks' : 'src/features/{{feature}}/hooks'

        const actions = [
            {
                type: 'add',
                path: base + '/use-{{kebabCase name}}.ts',
                templateFile: 'generators/hook/hook.ts.hbs',
            },
        ]

        if (shared) {
            actions.push({
                type: 'add',
                path: base + '/__tests__/use-{{kebabCase name}}.test.ts',
                templateFile: 'generators/hook/hook.test.ts.hbs',
            })
        }

        return actions
    },
}
