'use strict'

module.exports = {
    description: 'A Zustand store under src/stores (app-wide UI state only)',
    prompts: [{ type: 'input', name: 'name', message: 'Store name (camelCase):' }],
    actions: [
        {
            type: 'add',
            path: 'src/stores/{{camelCase name}}.ts',
            templateFile: 'generators/store/store.ts.hbs',
        },
    ],
}
