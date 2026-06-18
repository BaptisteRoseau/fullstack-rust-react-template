/**
 * Scaffolds a new vertical-slice feature under `src/features/<name>/` with a
 * starter read endpoint and list component. Only the most common folders are
 * created — add `hooks/`, `stores/`, `types/`, etc. as needed.
 *
 * Remember to register the boundary in `eslint.config.cjs`
 * (`import/no-restricted-paths`) and add the domain model to `@/types/api.ts`,
 * following `.claude/skills/frontend-react-feature`.
 *
 * @type {import('plop').PlopGenerator}
 */
module.exports = {
    description: 'Feature module generator (api query + list component)',
    prompts: [
        {
            type: 'input',
            name: 'name',
            message: 'feature name (e.g. "teams")',
        },
    ],
    actions: () => [
        {
            type: 'add',
            path: 'src/features/{{kebabCase name}}/api/get-{{kebabCase name}}.ts',
            templateFile: 'generators/feature/get-feature.ts.hbs',
        },
        {
            type: 'add',
            path: 'src/features/{{kebabCase name}}/components/{{kebabCase name}}-list.tsx',
            templateFile: 'generators/feature/feature-list.tsx.hbs',
        },
    ],
}
