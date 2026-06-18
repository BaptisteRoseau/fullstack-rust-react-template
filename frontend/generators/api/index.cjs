const path = require('path')
const fs = require('fs')

const featuresDir = path.join(process.cwd(), 'src/features')
const features = fs.readdirSync(featuresDir)

/**
 * Scaffolds a single endpoint file under `src/features/<feature>/api/`:
 *  - query  -> `get-<noun>.ts`        (schema-less read + queryOptions + useQuery)
 *  - mutation -> `<verb>-<noun>.ts`   (Zod schema + fetcher + useMutation)
 *
 * Swap the `unknown` payload types for the real domain model from
 * `@/types/api.ts`, and add a matching MSW handler
 * (`.claude/skills/frontend-react-mocks`) or the call fails in dev/tests. See
 * `.claude/skills/frontend-react-api`.
 *
 * @type {import('plop').PlopGenerator}
 */
module.exports = {
    description: 'Feature API endpoint generator (query or mutation)',
    prompts: [
        {
            type: 'list',
            name: 'feature',
            message: 'Which feature does this endpoint belong to?',
            choices: features,
        },
        {
            type: 'list',
            name: 'kind',
            message: 'Read or write?',
            choices: ['query', 'mutation'],
        },
        {
            type: 'input',
            name: 'verb',
            message: 'mutation verb (e.g. "create", "update", "delete")',
            when: ({ kind }) => kind === 'mutation',
        },
        {
            type: 'input',
            name: 'noun',
            message: 'resource noun (e.g. "teams")',
        },
    ],
    actions: (answers) => {
        const isQuery = answers.kind === 'query'
        const fileName = isQuery
            ? 'get-{{kebabCase noun}}.ts'
            : '{{kebabCase verb}}-{{kebabCase noun}}.ts'

        return [
            {
                type: 'add',
                path: 'src/features/{{feature}}/api/' + fileName,
                templateFile: isQuery
                    ? 'generators/api/query.ts.hbs'
                    : 'generators/api/mutation.ts.hbs',
            },
        ]
    },
}
