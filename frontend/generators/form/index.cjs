const path = require('path')
const fs = require('fs')

const featuresDir = path.join(process.cwd(), 'src/features')
const features = fs.readdirSync(featuresDir)

/**
 * Scaffolds a `Create<Noun>` FormDrawer component in a feature's `components/`.
 * It reuses the `create<Noun>InputSchema` + `useCreate<Noun>` from the matching
 * api file, so generate the `create` mutation first (api generator) — the
 * default form binds a single `name` field, matching the mutation schema.
 * See `.claude/skills/frontend-react-form`.
 *
 * @type {import('plop').PlopGenerator}
 */
module.exports = {
    description: 'Feature form generator (FormDrawer create flow)',
    prompts: [
        {
            type: 'list',
            name: 'feature',
            message: 'Which feature does this form belong to?',
            choices: features,
        },
        {
            type: 'input',
            name: 'noun',
            message: 'resource noun (e.g. "teams")',
        },
    ],
    actions: () => [
        {
            type: 'add',
            path: 'src/features/{{feature}}/components/create-{{kebabCase noun}}.tsx',
            templateFile: 'generators/form/form.tsx.hbs',
        },
    ],
}
