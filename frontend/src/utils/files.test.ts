import { formatFileSize, isPdf, mimeTypeGroup, savedPercentage } from './files'

it('keeps a byte count in its largest readable unit', () => {
    const cases: [number, string][] = [
        [0, '0 B'],
        [512, '512 B'],
        [1024, '1.0 KB'],
        [1536, '1.5 KB'],
        [1024 * 1024 * 3, '3.0 MB'],
        [1024 * 1024 * 1024 * 2, '2.0 GB'],
    ]

    for (const [bytes, expected] of cases) {
        expect(
            formatFileSize(bytes),
            `expected ${expected} for ${bytes} bytes, got ${formatFileSize(bytes)}`,
        ).toBe(expected)
    }
})

it('drops the decimal once the number is big enough to not need it', () => {
    expect(
        formatFileSize(1024 * 512),
        `expected a whole number of KB, got ${formatFileSize(1024 * 512)}`,
    ).toBe('512 KB')
})

it('reads a nonsensical size as nothing rather than NaN', () => {
    expect(
        formatFileSize(Number.NaN),
        `expected 0 B, got ${formatFileSize(Number.NaN)}`,
    ).toBe('0 B')
})

it('reports what the compression saved', () => {
    expect(
        savedPercentage(1000, 250),
        `expected 75, got ${savedPercentage(1000, 250)}`,
    ).toBe(75)
})

it('reports nothing when the content did not shrink', () => {
    expect(
        savedPercentage(1000, 1000),
        `expected null, got ${savedPercentage(1000, 1000)}`,
    ).toBeNull()
    expect(
        savedPercentage(0, 0),
        `an empty file saves nothing, got ${savedPercentage(0, 0)}`,
    ).toBeNull()
})

it('splits a mime type into its group', () => {
    expect(
        mimeTypeGroup('image/png'),
        `expected "image", got "${mimeTypeGroup('image/png')}"`,
    ).toBe('image')
    expect(isPdf('application/pdf'), 'a pdf must be recognised').toBe(true)
    expect(isPdf('image/png'), 'an image is not a pdf').toBe(false)
})
