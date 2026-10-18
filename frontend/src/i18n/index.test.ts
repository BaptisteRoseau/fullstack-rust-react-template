import { loadLocale } from '.'

it('sets the document language to the loaded locale', async () => {
    await loadLocale('fr')

    expect(
        document.documentElement.lang,
        `expected lang "fr", got "${document.documentElement.lang}"`,
    ).toBe('fr')

    await loadLocale('en')
})
