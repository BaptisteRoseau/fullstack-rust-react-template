# Authenticator integration tests

Contains

- a Keycloak testcontainer to be reused for the tests, implementing
  `test_utils::TestSuite` (`common/containers.rs`)
- two `#[trait_test_suite]` modules for the Authenticator trait, reusable for any
  backend (`common/authenticator.rs` for credential validation, `common/oidc.rs` for
  the Backend-for-Frontend flow)
- the provider-side actor the suites drive (`common/provider.rs`)
- the realms imported into the container (`assets/`)

Both suites share their authenticator through the generated `trials_shared`: building
one re-fetches the realm's JWKS, and the login state they cache is keyed by a random
CSRF value, so parallel trials cannot collide.

## Realms

The container imports two realms, because the trait spans two provider roles that
need different client configurations:

| Realm | Client | Used for |
|-------|--------|----------|
| `test-realm` (`assets/realm-export.json`) | `backend`, public, direct access grants | `validate`: minting a token without a browser, plus the API-key path |
| `oidc-test-realm` (`assets/oidc-realm-export.json`) | `webapp`, confidential, standard flow + registration | `authorize_url`, `exchange_code`, `refresh_tokens`, `userinfo`, `logout` |

Both realms declare an audience mapper emitting `backend`, so a token obtained
through the login flow also passes `validate`.

## The login agent

`exchange_code` needs a real authorization code, which only a browser can obtain.
`common/provider.rs` implements `ProviderAgent` for the fixture: it fetches the
authorize URL, submits Keycloak's login form, and reads the `code` and `state` off
the redirect's `Location` header — nothing ever listens on the registered callback
URL.

Two details make this work and are easy to break:

- Keycloak marks its login cookies `Secure`, so an automatic cookie store silently
  drops them over plain HTTP and the login fails with "Restart login cookie not
  found". The agent echoes the `Set-Cookie` values back by hand instead.
- The form is located by its `kc-form-login` id. The image tag is pinned so a
  Keycloak upgrade surfaces here rather than silently changing what is tested.

## Running

```sh
cargo test -p authenticator
```

Requires a running Docker daemon; the Keycloak container takes a few seconds to
boot and is shared by every test in the binary.
