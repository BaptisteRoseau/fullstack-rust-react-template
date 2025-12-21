# TODO

- [ ] infrastructure
    - [ ] docker
        - [ ] migrations
        - [ ] mailhog
    - [ ] compose
- [ ] docker compose
- [X] crates:
    - [X] api
    - [X] binaries
    - [X] core
    - [X] database:
        - [X] migrations (sqlx)
- [ ] scripts
- [ ] doc
- [ ] README.md
- [ ] .env.dev
- [ ] Nix flake ?

- [ ] Database scripts:
    - [ ] Create users:
        - [ ] read-only
        - [ ] read-write
        - [ ] authentication
    - [ ] Create databases:
        - [ ] authentication
        - [ ] backend

- [ ] Make a clear difference between "client_query" and "ssr_query" in the frontend.

## RBAC

Entities that can have permissions:

- Users
- Groups
- Roles

Permissions

Entities can define their permissions ?

## SQL

- [X] Add a trigger on "create table" like this one:
- [ ] Test the trigger
- [X] Add an audit function to make sure
- [ ] Test the trigger
