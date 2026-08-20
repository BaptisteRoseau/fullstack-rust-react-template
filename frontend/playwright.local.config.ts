import base from './playwright.config'

export default {
    ...base,
    use: {
        ...base.use,
        launchOptions: { executablePath: '/opt/google/chrome/chrome' },
    },
}
