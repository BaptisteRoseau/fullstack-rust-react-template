// This file selects how to handle a file's compression, encryption
// and optimization based of its filemime.
//
// For saving, it will:
//     1. Optimize the file based on its type (ex. PNG)
//     2. Compress the file based on the given parameters
//     3. Encrypt the file based on the given parameters
//
// For loading, it will:
//     1. Decrypt the file
//     2. Decompress the file
//
// If the "auto" options are selected, it will select special options
// like lossy image compression or resizing based of the size of
// the file being stored.
