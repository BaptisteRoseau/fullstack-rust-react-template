/**
 * Scaffolds an authenticated `/app` page and wires it up:
 *  - adds the page component under `src/app/pages/app/`
 *  - injects a path entry in `src/config/paths.ts`
 *  - injects a lazy route in `src/app/router.tsx`
 *
 * Sidebar nav (`dashboard-layout.tsx`) and any data `clientLoader` are left to
 * flesh out following `.claude/skills/frontend-react-page`.
 *
 * @type {import('plop').PlopGenerator}
 */
module.exports = {
    description: 'Page / route generator (authenticated /app route)',
    prompts: [
        {
            type: 'input',
            name: 'name',
            message: 'page name (e.g. "teams")',
        },
    ],
    actions: () => [
        {
            type: 'add',
            path: 'src/app/pages/app/{{kebabCase name}}.tsx',
            templateFile: 'generators/page/page.tsx.hbs',
        },
        {
            type: 'append',
            path: 'src/config/paths.ts',
            pattern: /app: \{/,
            template: [
                '        {{camelCase name}}: {',
                "            path: '{{kebabCase name}}',",
                "            getHref: () => '/app/{{kebabCase name}}',",
                '        },',
            ].join('\n'),
        },
        {
            type: 'append',
            path: 'src/app/router.tsx',
            pattern: /children: \[/,
            template: [
                '                {',
                '                    path: paths.app.{{camelCase name}}.path,',
                '                    lazy: () =>',
                "                        import('./pages/app/{{kebabCase name}}').then(",
                '                            convert(queryClient),',
                '                        ),',
                '                },',
            ].join('\n'),
        },
    ],
}
