# Debug

Contains the services used to inspect the application while developing it:

- Homepage
- Mailhog

One directory per component, each with its own `kustomization.yaml`. See
[../README.md](../README.md) for the conventions these manifests follow.

These components are development conveniences: none of them authenticates its
callers, and Mailhog swallows every mail the stack sends instead of delivering it.
Only the `dev` overlay includes this group — never add it to `production`.
