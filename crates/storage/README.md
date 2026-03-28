# Storage

This is the storage layer. It provides an interface to save blobs and files.

It should provide a `Storage` trait and structures for client code, and as a backend use an S3-compatible tool.

## Features

It transparently optimizes the size of blobs:

- Images and videos are optimized using the `caesium` crate
- Every blob is compressed before being stored, and decompressed when accessed

The control of the compression behavior is done through the `StorageParameters` struct.
