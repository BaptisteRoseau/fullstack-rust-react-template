const path = require('path')
const fs = require('fs')

const featuresDir = path.join(process.cwd(), 'src/features')
const features = fs.readdirSync(featuresDir)

/**
 * Scaffolds a Zustand store (global *client* state only — server data belongs in
 * React Query). Co-locate it with the feature/component that owns it. The
 * starter exposes an `isOpen` flag with open/close/reset actions; replace with
 * the real state. See `.claude/skills/frontend-react-state`.
 *
 * @type {import('plop').PlopGenerator}
 */
module.exports = {
    description: 'Zustand store generator',
    prompts: [
        {
            type: 'input',
            name: 'name',
            message: 'store name (e.g. "cart")',
        },
        {
            type: 'list',
            name: 'feature',
            message: 'Where does this store live?',
            choices: ['components', ...features],
        },
        {
            type: 'input',
            name: 'folder',
            message: 'folder under src/components (e.g. "ui/cart")',
            when: ({ feature }) => feature === 'components',
        },
    ],
    actions: (answers) => {
        const base =
            answers.feature === 'components'
                ? 'src/components/{{folder}}'
                : 'src/features/{{feature}}/stores'

        return [
            {
                type: 'add',
                path: base + '/{{kebabCase name}}-store.ts',
                templateFile: 'generators/store/store.ts.hbs',
            },
        ]
    },
}
