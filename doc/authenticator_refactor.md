# Authenticator refactor — merge OIDC into the `Authenticator` trait

## Context

`crates/authenticator` exposes two unrelated things:

- an `Authenticator` trait (`src/authenticator.rs`) implemented by one backend,
  `backends::Keycloak` (`src/backends/keycloak.rs`) — JWT/JWKS + API-key validation;
- a **concrete** `OidcClient` struct (`src/oidc.rs`, 279 lines) driving the OAuth
  Backend-for-Frontend flow, with no trait at all.

`crates/api/src/app_state.rs` therefore holds `pub oauth: Arc<OidcClient>` next to the four
trait objects (`dyn Database`, `dyn Storage`, `dyn Cache`, `dyn Authenticator`). That breaks
the rule stated in `crates/README.md`: *"The crate exposes a public trait that is
`Send + Sync`, this is the one that will be used in `app_core` and `api` crates. In
`src/backends` are stored structs that implement this trait."* A concrete struct in
`AppState` cannot be substituted, cannot be faked in tests, and `src/oidc.rs` sits at the
crate root although it is Keycloak-specific (it hardcodes `/protocol/openid-connect/*` and
rewrites the authorize path to `/registrations`, a Keycloak-only extension).

Three further problems this refactor closes:

1. **Dead code.** `Authenticator::refresh(&mut self)` is never called through the trait —
   its only call site in the whole repo is `Keycloak::try_new` calling itself
   (`src/backends/keycloak.rs:47`). It is also the *only* reason the trait needs `&mut self`.
   `AuthenticatorError::Expired` and `AuthenticatorError::InvalidSignature` are never
   constructed by the crate (expiry and bad signatures arrive as `JwtError(..)`); they only
   appear in `api` match arms and in `api` unit tests. The `logging` dependency in
   `crates/authenticator/Cargo.toml` is unused (the crate emits no logs).
2. **Duplicated, contradictory configuration.** The crate reads two unrelated sections:
   `config.authenticator.provider_url` (JWKS) and `config.oidc.issuer_url` (everything else).
   `doc/authentication/configuration.md` has to warn readers to keep them on the same realm.
   The JWKS URL is derivable from the issuer, so the duplication is pure footgun.
3. **No test coverage for the OIDC half.** All 279 lines of `src/oidc.rs` are untested. The
   Keycloak testcontainer fixture already exists (`tests/common/containers.rs`) but its
   `test_config()` leaves `OidcConfig` empty, and its `NoopCache` returns `None` from `get()`,
   which would make `exchange_code` always fail with `InvalidState`.

**Outcome:** one `Authenticator` trait covering both the resource-server and the BFF side, one
`Keycloak` backend implementing it, `AppState` holding only trait objects plus the shared
`Arc<Config>`, dead code removed, and a container-based integration suite that exercises the
whole trait — including a real Authorization Code + PKCE login — following the same shape as
the `storage`, `cache` and `database` suites.

## Decisions taken (agreed with the user)

| Question | Decision |
|---|---|
| Trait shape | **One merged `Authenticator` trait.** `AppState.oauth` disappears. |
| Config | **One `AuthenticatorConfig`.** `OidcConfig` and `AUTHENTICATOR_PROVIDER_URL` are removed; the JWKS URL is derived from the issuer. |
| `frontend_url` / `cookie_secure` | Stay in the **config crate** (moved to `ApiConfig`); handlers read them from an `Arc<Config>` held by `AppState`. No getters on the trait, no ad-hoc "web state". |
| Integration tests | Same shape as `storage`/`database`: spawn Keycloak, write **trait-level** `assert_*` functions, then run them against the `Keycloak` backend from `tests/keycloak.rs`. A small test-side `ProviderLogin` trait supplies the browser step so the suite stays backend-agnostic. |
| `AppState` field type | Keep `Arc<RwLock<dyn Authenticator>>` for uniformity with `Database`/`Storage`/`Cache`. (After this refactor no trait method takes `&mut self`, so `Arc<dyn Authenticator>` would also compile — do **not** change it, uniformity wins.) |

---

## 1. Target layout

```
crates/authenticator/
├── Cargo.toml
├── README.md                        # NEW (crates/README.md mandates one per crate)
├── src
│   ├── lib.rs                       # mod / pub use only
│   ├── authenticator.rs             # the trait, nothing else
│   ├── models.rs                    # NEW: UserToken, LoginScreen, AuthTokens, AuthSession, UserInfo
│   ├── error.rs
│   └── backends
│       ├── mod.rs                   # mod keycloak; pub use keycloak::Keycloak;
│       └── keycloak
│           ├── mod.rs               # mod …; pub use backend::Keycloak;   (no code)
│           ├── backend.rs           # struct Keycloak + try_new + impl Authenticator
│           ├── endpoints.rs         # Endpoints::from_issuer — URL derivation
│           ├── jwt.rs               # JWKS fetch + JWT validation + realm_from_iss
│           ├── api_key.rs           # sha256 + cache + database lookup
│           └── oidc.rs              # oauth2 flow: authorize/exchange/refresh/userinfo/logout
└── tests
    ├── README.md                    # NEW (mirrors crates/storage/tests/README.md)
    ├── assets/realm-export.json     # extended: webapp confidential client + registration
    ├── common
    │   ├── mod.rs
    │   ├── containers.rs            # KeycloakFixture: container + config + backend builder
    │   ├── login.rs                 # ProviderLogin trait + Keycloak HTML-form implementation
    │   ├── authenticator.rs         # trait suite: token validation asserts + the macro
    │   └── oidc.rs                  # trait suite: BFF flow asserts
    └── keycloak.rs                  # harness = false, fn main()
```

`src/backends/keycloak.rs` becomes a directory because the backend now carries three
responsibilities; every file stays well under 200 lines. `mod.rs` files contain only `mod` /
`pub use`, per `crates/README.md`.

---

## 2. `crates/config` — merge the two sections

### `src/config.rs`

Replace `AuthenticatorConfig` and delete `OidcConfig`:

```rust
#[derive(Debug, Clone)]
pub struct AuthenticatorConfig {
    /// Realm base URL, e.g. `http://localhost:8090/realms/app`. Every provider
    /// endpoint (JWKS, authorize, token, logout, userinfo) is derived from it.
    pub issuer_url: String,
    /// Audiences the access token must carry.
    pub audiences: Vec<String>,
    /// Confidential client used by the Backend-for-Frontend.
    pub client_id: String,
    pub client_secret: String,
    /// Backend callback URL registered as a redirect URI on the client.
    pub redirect_url: String,
}
```

Move the two presentation settings into `ApiConfig`:

```rust
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub timeout_sec: u16,
    pub rate_limiter_refresh_per_second: u64,
    pub rate_limiter_burst_size: u32,
    /// Frontend origin: post-login redirect target and the allowed CORS origin.
    pub frontend_url: String,
    /// Whether auth cookies carry the `Secure` attribute (enable behind HTTPS).
    pub cookie_secure: bool,
}
```

Remove `pub oidc: OidcConfig` from `Config` and update the `TryFrom<CliConfig> for Config`
impl (`config.rs:100-175`) accordingly.

### `src/cli.rs` (lines 140-178) and `src/defaults.rs` (lines 32-45)

The file documents the convention *"CLI arguments grouped together into a single struct
should be prefixed with the same name"*, so all five authenticator fields take the
`authenticator_` prefix:

| Old CLI field / env | New CLI field / env |
|---|---|
| `authenticator_provider_url` / `AUTHENTICATOR_PROVIDER_URL` | **removed** |
| `authenticator_audiences` / `AUTHENTICATOR_AUDIENCES` | unchanged |
| `oidc_issuer_url` / `OIDC_ISSUER_URL` | `authenticator_issuer_url` / `AUTHENTICATOR_ISSUER_URL` |
| `oidc_client_id` / `OIDC_CLIENT_ID` | `authenticator_client_id` / `AUTHENTICATOR_CLIENT_ID` |
| `oidc_client_secret` / `OIDC_CLIENT_SECRET` | `authenticator_client_secret` / `AUTHENTICATOR_CLIENT_SECRET` |
| `oidc_redirect_url` / `OIDC_REDIRECT_URL` | `authenticator_redirect_url` / `AUTHENTICATOR_REDIRECT_URL` |
| `frontend_url` / `FRONTEND_URL` | unchanged (now feeds `ApiConfig`) |
| `cookie_secure` / `COOKIE_SECURE` | unchanged (now feeds `ApiConfig`) |

Rename the constants in `defaults.rs` to match (`DEFAULT_AUTHENTICATOR_ISSUER_URL`,
`…_CLIENT_ID`, `…_CLIENT_SECRET`, `…_REDIRECT_URL`); delete
`DEFAULT_AUTHENTICATOR_PROVIDER_URL`. Values are unchanged. Also update the `Default for
CliConfig` impl at `config.rs:225-245`.

### NEW: `src/testing.rs` behind a `test-utils` feature

Two places build a `Config` literal field-by-field and must be edited whenever `Config`
changes — `crates/api/src/routes/observability.rs:203-215` and
`crates/authenticator/tests/common/containers.rs:128-169`. Mirror
`database::testing::MockDatabase`:

```toml
# crates/config/Cargo.toml
[features]
# Ready-made `Config` for downstream crates' tests. Enable via [dev-dependencies].
test-utils = []
```

```rust
// crates/config/src/testing.rs
/// A fully populated [`Config`] with inert values, for tests that need a `Config`
/// but only care about a few fields. Mutate the returned value as needed.
pub fn test_config() -> Config { /* every field, using the `defaults` values */ }
```

`lib.rs` gains `#[cfg(feature = "test-utils")] pub mod testing;`.

---

## 3. `crates/cache` — shared in-memory test double

There are currently two hand-written `Cache` stubs — `MockCache` inside
`crates/authenticator/src/backends/keycloak.rs:157-217` (`#[cfg(test)]`, unreachable from
`tests/`) and `NoopCache` in `crates/authenticator/tests/common/containers.rs:171-206`
(returns `None` from `get`, so it cannot back the OIDC state round-trip). Replace both with
one shared double, mirroring `database::testing`:

```toml
# crates/cache/Cargo.toml
[features]
# In-memory `Cache` test double (`cache::testing::MockCache`).
test-utils = []
```

```rust
// crates/cache/src/testing.rs
/// In-memory [`Cache`] backed by a `HashMap`, ignoring expiry timeouts.
#[derive(Default)]
pub struct MockCache { store: Mutex<HashMap<String, Value>> }

#[async_trait]
impl Cache for MockCache { /* set, get, delete, set_many, get_many, delete_many */ }
```

`lib.rs` gains `#[cfg(feature = "test-utils")] pub mod testing;`. `crates/authenticator`
adds `cache = { path = "../cache", features = ["test-utils"] }` to `[dev-dependencies]`.

---

## 4. `crates/authenticator` — the merged trait

### 4.1 `src/models.rs` (new)

```rust
/// The caller resolved from a credential.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserToken { pub id: Uuid, pub realm: String }

/// Which provider page the browser should land on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginScreen { Login, Register }

/// Tokens issued by the provider after a code exchange or a refresh.
#[derive(Debug, Clone)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub access_expires_in: Option<Duration>,
}

/// The result of a successful code exchange: the tokens plus the post-login
/// redirect that was stored when the flow started.
#[derive(Debug, Clone)]
pub struct AuthSession { pub tokens: AuthTokens, pub redirect: Option<String> }

/// Identity claims returned by the provider's userinfo endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub sub: Uuid,
    #[serde(default)] pub preferred_username: String,
    #[serde(default)] pub given_name: String,
    #[serde(default)] pub family_name: String,
    #[serde(default)] pub email: String,
    #[serde(default)] pub email_verified: bool,
}
```

`UserInfo` replaces the untyped `serde_json::Value` that `api` currently re-parses in two
places (`GetMeResponse::from_userinfo` and `register_user`). A `sub` that is not a UUID must
surface as `AuthenticatorError::Oidc(..)` — consistent with `validate`, which already
requires a UUID `sub`.

### 4.2 `src/authenticator.rs` — the trait only

```rust
/// Everything the application needs from an identity provider: validating the
/// credentials callers present, and driving the browser login flow on their behalf.
#[async_trait]
pub trait Authenticator: Send + Sync {
    /// Resolves a caller-supplied credential — a provider JWT or an API key —
    /// into the user it identifies.
    async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>>;

    /// Builds the provider URL the browser must be sent to, persisting the PKCE
    /// verifier and the post-login `redirect` under a freshly generated CSRF state.
    async fn authorize_url(
        &self,
        screen: LoginScreen,
        redirect: Option<&str>,
    ) -> Result<String, Box<AuthenticatorError>>;

    /// Exchanges an authorization code for a session, validating the CSRF state
    /// and consuming it so it cannot be replayed.
    async fn exchange_code(
        &self,
        code: &str,
        state: &str,
    ) -> Result<AuthSession, Box<AuthenticatorError>>;

    /// Exchanges a refresh token for a fresh token pair.
    async fn refresh_tokens(
        &self,
        refresh_token: &str,
    ) -> Result<AuthTokens, Box<AuthenticatorError>>;

    /// Fetches the identity claims backing the current-user endpoint.
    async fn userinfo(
        &self,
        access_token: &str,
    ) -> Result<UserInfo, Box<AuthenticatorError>>;

    /// Revokes the provider-side session.
    async fn logout(&self, refresh_token: &str) -> Result<(), Box<AuthenticatorError>>;
}
```

Notes for the implementer:

- `refresh(&mut self)` is **gone**. The JWKS fetch becomes a private function called by
  `Keycloak::try_new`. This is what makes every method `&self`.
- The OIDC token refresh is named `refresh_tokens` to avoid the old name collision.
- Arguments are `&str` (the old `OidcClient` took `String` by value for no reason).
- No `frontend_url()` / `cookie_secure()` getters — see §5.

### 4.3 `src/lib.rs`

```rust
mod authenticator;
mod models;

pub mod backends;
pub mod error;

pub use authenticator::Authenticator;
pub use models::{AuthSession, AuthTokens, LoginScreen, UserInfo, UserToken};
```

`pub mod oidc` disappears; `src/oidc.rs` is deleted (its content moves to
`src/backends/keycloak/oidc.rs`).

### 4.4 `src/error.rs`

- **Delete** `Expired` and `InvalidSignature` (never constructed — see §7 for the `api`
  call sites that must be updated).
- Keep `NoJwk`, `RequestError`, `AuthenticationFailure`, `JwtError`, `InvalidRealm`,
  `Oidc(String)`, `OidcRejected`, `InvalidState`, `Message(String)` and the three
  `From<..> for Box<AuthenticatorError>` impls.
- Move the `oidc_err` helper out of `src/oidc.rs` into this file as
  `pub(crate) fn oidc_error(message: impl Into<String>) -> Box<AuthenticatorError>`.

### 4.5 `src/backends/keycloak/endpoints.rs`

Single place where Keycloak's URL shape is encoded — the JWKS/issuer duplication dies here.

```rust
const AUTH_PATH: &str = "/protocol/openid-connect/auth";
const REGISTRATIONS_PATH: &str = "/protocol/openid-connect/registrations";
// … token, logout, userinfo, certs

/// Keycloak's well-known endpoints, all derived from the realm base URL.
pub(super) struct Endpoints {
    pub(super) authorize: String,
    pub(super) registrations: String,
    pub(super) token: String,
    pub(super) logout: String,
    pub(super) userinfo: String,
    pub(super) jwks: String,
}

impl Endpoints {
    pub(super) fn from_issuer(issuer_url: &str) -> Self { /* trim trailing '/', format! */ }
}
```

Unit-test `from_issuer` (with and without a trailing slash).

### 4.6 `src/backends/keycloak/jwt.rs`

Moves `Claims`, `realm_from_iss` and `validate_jwt` out of the old backend file.

```rust
/// RS256 validation of provider-issued access tokens against the realm's JWKS.
pub(super) struct JwtValidator { keys: JwkSet, audiences: Vec<String> }

impl JwtValidator {
    /// Fetches the realm's signing keys once, at construction.
    pub(super) async fn fetch(jwks_url: &str, audiences: Vec<String>)
        -> Result<Self, Box<AuthenticatorError>>;

    pub(super) fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>>;
}
```

`keys` becomes a plain `JwkSet` instead of `Option<JwkSet>`: it is always populated after
`fetch`, which removes the `NoJwk` "never fetched" state from the happy path (keep the
`NoJwk` variant — `fetch` returns it when the JWKS document contains no key).
Keep the three `realm_from_iss_*` unit tests here.

### 4.7 `src/backends/keycloak/api_key.rs`

```rust
/// API-key credentials: a sha256 hex digest looked up in the database, memoised
/// in the shared cache for 300s.
pub(super) struct ApiKeyValidator {
    cache: Arc<RwLock<dyn Cache>>,
    database: Arc<RwLock<dyn Database>>,
}

impl ApiKeyValidator {
    pub(super) fn new(cache: …, database: …) -> Self;
    pub(super) async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>>;
    async fn try_get_cache(&self, hashed: &str) -> Option<UserToken>;
    async fn set_cache(&self, user_token: &UserToken, hashed: &str);
}

/// Digest used as the API-key lookup key; must match how keys are stored.
pub(super) fn hex_sha256(input: &str) -> String;
```

Move the existing unit tests (`hex_sha256_matches_known_vectors`,
`set_cache_then_try_get_cache_round_trips`, `try_get_cache_returns_none_on_miss`,
`validate_api_key_*`) here, swapping the local `MockCache` for `cache::testing::MockCache`.

### 4.8 `src/backends/keycloak/oidc.rs`

The body of the old `src/oidc.rs`, minus `frontend_url` / `cookie_secure`, minus the
endpoint-string derivation (now injected), returning the new model types.

```rust
type ConfiguredClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

const STATE_CACHE_PREFIX: &str = "oidc_state:";
const STATE_TTL_SECONDS: u32 = 600;
const SCOPES: [&str; 3] = ["openid", "email", "profile"];

/// Authorization Code + PKCE flow against Keycloak.
pub(super) struct OidcFlow {
    client: ConfiguredClient,
    http: oauth2::reqwest::Client,
    cache: Arc<RwLock<dyn Cache>>,
    registrations_url: String,
    logout_url: String,
    userinfo_url: String,
    client_id: String,
    client_secret: String,
}

impl OidcFlow {
    pub(super) fn try_new(
        config: &AuthenticatorConfig,
        endpoints: &Endpoints,
        cache: Arc<RwLock<dyn Cache>>,
    ) -> Result<Self, Box<AuthenticatorError>>;

    pub(super) async fn authorize_url(&self, screen: LoginScreen, redirect: Option<&str>)
        -> Result<String, Box<AuthenticatorError>>;
    pub(super) async fn exchange_code(&self, code: &str, state: &str)
        -> Result<AuthSession, Box<AuthenticatorError>>;
    pub(super) async fn refresh_tokens(&self, refresh_token: &str)
        -> Result<AuthTokens, Box<AuthenticatorError>>;
    pub(super) async fn userinfo(&self, access_token: &str)
        -> Result<UserInfo, Box<AuthenticatorError>>;
    pub(super) async fn logout(&self, refresh_token: &str)
        -> Result<(), Box<AuthenticatorError>>;
}

/// State persisted between the authorize redirect and the callback.
#[derive(Serialize, Deserialize)]
struct PendingLogin { verifier: String, redirect: Option<String> }

fn state_key(state: &str) -> String;
fn tokens_from_response(token: &BasicTokenResponse) -> AuthTokens;
```

Behaviour to preserve verbatim: PKCE S256 challenge, CSRF state persisted in the shared
`Cache` under `oidc_state:{state}` with a 600s TTL, state deleted on exchange (replay
protection), `LoginScreen::Register` swapping the authorize path for `/registrations`,
`redirect::Policy::none()` on the HTTP client (required by `oauth2`), and the 401/403 →
`OidcRejected` mapping in `userinfo`.

One behaviour change to make while moving the code: `logout` currently discards the
response (`send().await?` then `Ok(())`), silently swallowing a failed revoke, which
`CLAUDE.md` forbids. Check the status and return `AuthenticatorError::Oidc(..)` on a
non-success. The `/auth/logout` handler already treats logout as best-effort, so the
endpoint behaviour is unchanged.

### 4.9 `src/backends/keycloak/backend.rs`

```rust
/// Keycloak-backed [`Authenticator`]: JWKS/JWT and API-key validation for the
/// resource-server side, Authorization Code + PKCE for the login side.
pub struct Keycloak {
    jwt: JwtValidator,
    api_keys: ApiKeyValidator,
    oidc: OidcFlow,
}

impl Keycloak {
    pub async fn try_new(
        config: &Config,
        cache: Arc<RwLock<dyn Cache>>,
        database: Arc<RwLock<dyn Database>>,
    ) -> Result<Self, Box<AuthenticatorError>> {
        let endpoints = Endpoints::from_issuer(&config.authenticator.issuer_url);
        let jwt = JwtValidator::fetch(&endpoints.jwks, config.authenticator.audiences.clone()).await?;
        let oidc = OidcFlow::try_new(&config.authenticator, &endpoints, Arc::clone(&cache))?;
        Ok(Self { jwt, api_keys: ApiKeyValidator::new(cache, database), oidc })
    }
}

#[async_trait]
impl Authenticator for Keycloak {
    async fn validate(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>> {
        // Only a JWT contains dots; anything else is an API key.
        if token.contains('.') { self.jwt.validate(token) } else { self.api_keys.validate(token).await }
    }
    // the five remaining methods delegate straight to `self.oidc`
}
```

### 4.10 `Cargo.toml`

- Remove the unused `logging` dependency.
- `[dev-dependencies]`: add `cache = { path = "../cache", features = ["test-utils"] }`,
  `config = { path = "../config", features = ["test-utils"] }`, `async-trait = "0"` is
  already a normal dependency (usable from tests), and add
  `reqwest = { version = "0.12", features = ["json", "cookies"] }` — the login actor needs
  a cookie store to walk Keycloak's login form.
- `[[test]] name = "keycloak", harness = false` stays.

---

## 5. `crates/api` — consume the merged trait

### 5.1 `src/app_state.rs`

```rust
#[derive(Clone)]
pub struct AppState {
    pub database: Arc<RwLock<dyn Database>>,
    pub storage: Arc<RwLock<dyn Storage>>,
    pub cache: Arc<RwLock<dyn Cache>>,
    pub authenticator: Arc<RwLock<dyn Authenticator>>,
    pub config: Arc<Config>,
}
```

- Drop the `oauth` field, its constructor parameter and its `FromRef` impl.
- Add `impl FromRef<AppState> for Arc<Config>` so handlers can extract `State(config)`.

### 5.2 `src/endpoints/auth/` — split the 245-line `endpoints.rs`

| File | Contents |
|---|---|
| `endpoints.rs` | the six utoipa handlers only |
| `cookies.rs` (new) | `ACCESS_COOKIE`, `REFRESH_COOKIE`, `token_cookie`, `set_token_cookies`, `clear_token_cookies` |
| `redirects.rs` (new) | `frontend_target` (the open-redirect guard) |
| `models.rs` | request/response types; `GetMeResponse::from_userinfo(&UserInfo)` |

`crates/api/src/extractors/user.rs:14` declares its own `const ACCESS_COOKIE` — make it use
the one from `cookies.rs` so the cookie name is defined once.

Handler signature change, applied to all six handlers:

```rust
pub(crate) async fn callback(
    State(authenticator): State<Arc<RwLock<dyn Authenticator>>>,
    State(database): State<Arc<RwLock<dyn database::Database>>>,
    State(config): State<Arc<Config>>,
    jar: CookieJar,
    Query(params): Query<GetCallbackParams>,
) -> Result<(CookieJar, Redirect), ApiError> {
    let frontend_url = &config.api.frontend_url;
    let (Some(code), Some(state)) = (params.code, params.state) else {
        return Ok((jar, Redirect::to(frontend_url)));
    };

    let session = authenticator.read().await.exchange_code(&code, &state).await?;
    let info = authenticator.read().await.userinfo(&session.tokens.access_token).await?;
    register_user(&info, &database).await?;

    let jar = set_token_cookies(jar, &session.tokens, config.api.cookie_secure);
    Ok((jar, Redirect::to(&frontend_target(frontend_url, session.redirect.as_deref()))))
}
```

`register_user` loses its `claim` closure and its `Uuid::parse_str` — it now takes
`&UserInfo` and forwards the typed fields to `app_core::user::register`.

### 5.3 Remaining `api` edits

- `src/routes/middlewares.rs:91-95` — `config.oidc.frontend_url` → `config.api.frontend_url`.
- `src/error/response.rs:186-203` — drop the `AuthenticatorError::InvalidSignature` and
  `AuthenticatorError::Expired` arms. `AuthenticationFailure` keeps its `forbidden()`
  mapping; expiry is already covered by the `JwtError(e) => e.into()` arm, which maps
  `ExpiredSignature` to `token_expired()`. **The silent-refresh 401 must keep working** —
  verify with the existing `api` tests.
- `src/extractors/user.rs:37-47` — remove `AuthenticatorError::Expired` from
  `keeps_own_status`; the `JwtError(ExpiredSignature)` arm below it already covers it.
- `src/extractors/user.rs:137-193` (tests) — drop `InvalidSignature` from the `unusable`
  list, and rewrite `expired_token_keeps_its_own_unauthorized` to build a real
  `jsonwebtoken::errors::Error::from(ErrorKind::ExpiredSignature)` wrapped in
  `AuthenticatorError::JwtError`.
- `src/routes/observability.rs:203-215` — replace the inline `Config` literal with
  `config::testing::test_config()`; add
  `config = { path = "../config", features = ["test-utils"] }` to the api dev-dependencies.

### 5.4 `crates/binaries/backend`

`src/program.rs:21-58`:

```rust
pub(crate) async fn run(config: Config) -> Result<(), anyhow::Error> {
    logging::init_logger(config.debug, config.log_json);
    let config = Arc::new(config);
    // … database / storage / cache unchanged …

    info!("Initializing Authenticator...");
    let authenticator =
        Keycloak::try_new(&config, Arc::clone(&cache), Arc::clone(&database)).await?;
    info!("Initialized Authenticator");

    let state = AppState::new(
        database, storage, cache,
        Arc::new(RwLock::new(authenticator)),
        Arc::clone(&config),
    );
    let mut public_routes = public_routes(&config, state);
```

Delete the `use authenticator::OidcClient;` import and the "Initializing OAuth client" block.
`src/main.rs` passes the `Config` by value instead of by reference.

---

## 6. Integration tests

Same architecture as `crates/storage/tests` and `crates/database/tests`: one container
started in `fn main()`, a backend-agnostic suite of `assert_*` functions, a
`macro_rules!` producing `Vec<Trial>`, `libtest-mimic` with `harness = false`.

### 6.1 `tests/assets/realm-export.json`

Extend the existing export (keep `realm: "test-realm"`, `sslRequired: "none"`, the `backend`
public client with its audience mapper, and the fixed `testuser` / `11111111-…-111111111111`):

- add `"registrationAllowed": true` so the `/registrations` screen answers 200;
- add a confidential client mirroring the production `webapp` client from
  `infrastructure/keycloak/import/realm-export.json`:

```json
{
  "clientId": "webapp",
  "enabled": true,
  "publicClient": false,
  "secret": "webapp-secret",
  "standardFlowEnabled": true,
  "directAccessGrantsEnabled": false,
  "protocol": "openid-connect",
  "redirectUris": ["http://localhost:9999/callback"],
  "attributes": { "pkce.code.challenge.method": "S256" },
  "protocolMappers": [{
    "name": "backend-audience",
    "protocol": "openid-connect",
    "protocolMapper": "oidc-audience-mapper",
    "consentRequired": false,
    "config": {
      "included.client.audience": "backend",
      "id.token.claim": "false",
      "access.token.claim": "true"
    }
  }]
}
```

The audience mapper is what lets a token obtained through the code flow also pass
`validate()` (which requires `aud: backend`), so one login exercises both halves of the
trait. The redirect URI never has to be served: the test client refuses redirects and reads
the `Location` header.

### 6.2 `tests/common/containers.rs`

Keep `KeycloakFixture::start()` exactly as it is (image `quay.io/keycloak/keycloak:26.6.4`,
`start-dev --import-realm`, `WaitFor::message_on_stdout("started in")`, realm copied in via
`with_copy_to`). Change what it exposes:

```rust
pub const REALM: &str = "test-realm";
pub const BACKEND_CLIENT_ID: &str = "backend";      // public, direct access grants
pub const WEBAPP_CLIENT_ID: &str = "webapp";        // confidential, BFF
pub const WEBAPP_CLIENT_SECRET: &str = "webapp-secret";
pub const REDIRECT_URL: &str = "http://localhost:9999/callback";
pub const USERNAME: &str = "testuser";
pub const PASSWORD: &str = "testpass";
pub const USER_ID: &str = "11111111-1111-1111-1111-111111111111";
pub const EMAIL: &str = "testuser@example.com";
/// Raw API key seeded in the fixture's database.
pub const API_KEY: &str = "integration-test-api-key";

impl KeycloakFixture {
    pub async fn start() -> Self;                 // unchanged
    pub fn issuer_url(&self) -> String;           // {base_url}/realms/test-realm
    pub fn config(&self) -> Config;               // config::testing::test_config() + issuer/client/audiences
    /// A `Keycloak` backend pointed at this container, backed by an in-memory
    /// cache (the OIDC state round-trip needs a real one) and a database
    /// pre-seeded with `API_KEY`.
    pub async fn authenticator(&self) -> Keycloak;
    /// Access token for `testuser` via the direct access grant on `backend`.
    pub async fn fetch_token(&self) -> String;    // unchanged
}
```

`provider_url()` and `NoopCache` are deleted. `test_config()` is replaced by
`config::testing::test_config()` plus three field assignments. The seeded API key is stored
in `MockDatabase.api_keys_by_hash` under its sha256 hex digest — computed in the test with
the `sha2` crate (already a dependency of `authenticator`), with a comment stating it must
match `hex_sha256` in `src/backends/keycloak/api_key.rs`.

### 6.3 `tests/common/login.rs` (new) — the browser step

```rust
/// Query parameters the provider hands back to the redirect URI.
pub struct CallbackParams { pub code: String, pub state: String }

/// Acts as the end user in front of the provider's login page. Implemented per
/// provider so the trait test suite stays backend-agnostic.
#[async_trait]
pub trait ProviderLogin {
    async fn login(&self, authorize_url: &str) -> CallbackParams;
}
```

`impl ProviderLogin for KeycloakFixture`, in four small private helpers:

1. build a `reqwest::Client` with `.cookie_store(true)` and
   `.redirect(reqwest::redirect::Policy::none())` — Keycloak's login flow needs the session
   cookie, and we must read the final `Location` instead of following it;
2. `GET authorize_url` → 200 HTML; `fn login_form_action(html: &str) -> String` extracts the
   `action="…"` of the login form and unescapes `&amp;` → `&`;
3. `POST` that action with `username` / `password` as
   `application/x-www-form-urlencoded` → expect **302**;
4. `fn query_param(url: &str, key: &str) -> Option<String>` pulls `code` and `state` out of
   the `Location` header. (Written by hand rather than adding a `url` dependency; both values
   are URL-safe.)

Every step `panic!`s with the status and a snippet of the body on failure, so a Keycloak
upgrade produces a readable error. The Keycloak tag stays pinned for this reason.

### 6.4 `tests/common/authenticator.rs` and `tests/common/oidc.rs`

Keep the existing house contract comment, but with the corrected signature — the current
suite is typed against `&KeycloakFixture`, which defeats its purpose:

```rust
// When adding a new test here:
// - helpers are regular private functions
// - tests signature is `pub async fn assert_<my test>(authenticator: &impl Authenticator)`
//   (plus `&impl ProviderLogin` for the tests that need a browser login)
// - new tests should be added in the `authenticator_trait_tests` macro
```

`common/authenticator.rs` — credential validation + the macro:

| Trial | Assertion |
|---|---|
| `validates_valid_jwt` | direct-grant token → `UserToken { id: USER_ID, realm: REALM }` |
| `rejects_garbage_jwt` | `"aaaa.bbbb.cccc"` errors |
| `rejects_tampered_jwt` | flipped signature char errors (keep `tamper_signature`) |
| `validates_seeded_api_key` | `API_KEY` → `UserToken { realm: "api_key" }` |
| `rejects_unknown_api_key` | `"plain-api-key-without-dots"` errors |

`common/oidc.rs` — the BFF flow:

| Trial | Assertion |
|---|---|
| `authorize_url_targets_the_login_screen` | URL contains `/protocol/openid-connect/auth`, `client_id=webapp`, `code_challenge_method=S256`, a `state`, and the redirect URI |
| `authorize_url_targets_the_registration_screen` | `LoginScreen::Register` → URL contains `/protocol/openid-connect/registrations` |
| `exchange_code_returns_tokens_and_redirect` | full login → access token non-empty, refresh token present, `redirect == Some("/app/dashboard")` |
| `exchange_code_accepts_a_token_that_validates` | the access token from the exchange passes `validate()` with `id == USER_ID` |
| `exchange_code_rejects_an_unknown_state` | `InvalidState` |
| `exchange_code_rejects_a_replayed_state` | second exchange with the same code/state → `InvalidState` (proves the cache entry is consumed) |
| `userinfo_returns_the_identity_claims` | `sub == USER_ID`, `email == EMAIL`, `given_name == "Test"` |
| `refresh_tokens_issues_a_new_pair` | new access token, refresh token present |
| `logout_revokes_the_session` | after `logout`, `refresh_tokens` on the revoked token errors **and** `userinfo` returns `OidcRejected` |

Every `assert!` / `assert_eq!` carries an interpolated message naming the values, per
`CLAUDE.md`.

The macro lives in `common/authenticator.rs` and pulls both modules in:

```rust
macro_rules! authenticator_trait_tests {
    ($authenticator:expr, $login:expr, $rt:expr) => {{
        use common::{authenticator::*, oidc::*};
        use libtest_mimic::Trial;
        use std::sync::Arc;

        let rt: Arc<tokio::runtime::Runtime> = $rt;
        let authenticator = $authenticator;   // Arc<A: Authenticator>
        let login = $login;                   // Arc<L: ProviderLogin>
        vec![ /* one block per trial, cloning the three Arcs */ ]
    }};
}
```

**Why one shared authenticator instead of `storage`'s per-test builder:** `authorize_url`
and `exchange_code` must share the same cache instance, the backend is stateless once built,
its cache keys are the random CSRF state (so parallel trials cannot collide), and building
per test would re-fetch the JWKS every time. State this in a comment above the macro.

### 6.5 `tests/keycloak.rs`

Unchanged in shape — start the fixture, build the backend once, expand the macro, run
`libtest_mimic::run`, then drop the fixture inside `rt.enter()` so `ContainerAsync::Drop`
can run its async cleanup:

```rust
fn main() {
    let args = Arguments::from_args();
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let fixture = Arc::new(rt.block_on(KeycloakFixture::start()));
    let authenticator = Arc::new(rt.block_on(fixture.authenticator()));

    let tests = authenticator_trait_tests!(authenticator, fixture.clone(), rt.clone());
    let conclusion = libtest_mimic::run(&args, tests);

    let _guard = rt.enter();
    drop(fixture);
    drop(_guard);
    drop(rt);
    conclusion.exit();
}
```

---

## 7. Documentation to update

- **`crates/authenticator/README.md` (new)** — the trait, the `Keycloak` backend and its four
  modules, the config it reads, how to run the tests. `crates/README.md` mandates a README per
  crate and `CLAUDE.md` tells agents to read it; it does not exist today.
- **`crates/authenticator/tests/README.md` (new)** — mirror `crates/storage/tests/README.md`:
  what the fixture provides, what the suite covers, `cargo test -p authenticator`.
- **`doc/authentication/backend.md`** — rewrite the "OAuth logic" section: `OidcClient` no
  longer exists; the table of methods becomes the trait's methods; note that JWT validation
  and the OAuth flow are now one backend.
- **`doc/authentication/configuration.md`** — replace the two env-var tables with the single
  `AUTHENTICATOR_*` table from §2; delete the "keep both on the same realm" warning (it is now
  structurally impossible to get wrong).
- **`doc/authentication/overview.md`** — only the component wording mentions the OAuth client;
  adjust if it names `OidcClient`.
- **`.env.dev` / `infrastructure/docker-compose/*.yml`** — grep for the removed env var names;
  today they are not set anywhere, so no change is expected. Verify.
- Note: `doc/authentication/README.md` links to `doc/authentication/keycloak.md`, which does
  not exist. Out of scope — mention it to the user, do not create it.

---

## 8. Execution order

Each step should compile (or at least fail only in the not-yet-migrated crate) before moving
to the next.

1. `crates/config`: merge the sections, rename the CLI/env fields, add `testing` behind
   `test-utils`. Fix `crates/api/src/routes/observability.rs` and
   `crates/api/src/routes/middlewares.rs` so `config` + `api` compile.
2. `crates/cache`: add `testing::MockCache` behind `test-utils`.
3. `crates/authenticator`: create `models.rs`, rewrite `authenticator.rs`, split
   `backends/keycloak.rs` into the five files, delete `src/oidc.rs`, prune `error.rs`, drop
   the `logging` dependency, move the unit tests into their new homes.
4. `crates/api`: `AppState`, the auth endpoint split, the error-mapping cleanups, the
   extractor cookie-constant de-duplication.
5. `crates/binaries/backend`: wiring.
6. Tests: realm export, fixture, `login.rs`, the two suite files, `keycloak.rs`.
7. Documentation.

---

## 9. Verification

```bash
cargo fmt
cargo clippy --workspace --all-targets      # scripts/test_lint.sh runs `cargo clippy`
cargo test -p config -p cache -p api -p authenticator
cargo test -p authenticator --test keycloak -- --nocapture   # the container suite
```

Then the end-to-end flow, which is the real proof the merge did not break the BFF:

```bash
docker compose up -d
cargo run -p backend
cd frontend && VITE_APP_ENABLE_API_MOCKING=false bun run dev
```

Walk through `doc/authentication/manual-testing.md`: hit a protected route, register/log in
on Keycloak, confirm the `access_token` / `refresh_token` cookies are set, that
`GET /api/auth/me` returns the profile, that the silent refresh still fires on
`401 TokenExpired`, and that logout clears the session.

**Known pre-existing breakage:** `crates/storage/tests/common/containers.rs:16` does
`include_str!("../../../../infrastructure/garage/garage.toml")`, but that file was deleted in
commit `d43afc3` ("Remove usage of Garage FS"). A bare `cargo test` / `scripts/test_units.sh`
therefore fails on `main` today, independently of this refactor. Scope the verification to
the crates listed above and report the storage breakage rather than fixing it here.

---

## 10. Explicitly out of scope (report, do not fix)

- `AuthenticatorError::Oidc(String)` has no match arm in `crates/api/src/error/response.rs`,
  so a failed code exchange (e.g. an expired authorization code) answers **500** instead of
  401. Worth a follow-up variant (`InvalidGrant` → 401), but it is a behaviour change beyond
  this refactor.
- `AuthenticatorError::Message(String)` is a stringly-typed catch-all collapsing three
  distinct JWT failures (missing `kid`, unknown `kid`, non-UUID `sub`).
- The JWKS is fetched once at startup and never re-fetched, so a Keycloak key rotation breaks
  validation until the backend restarts. Removing `refresh(&mut self)` does not make this
  worse — it was already never called — but a lazy re-fetch on an unknown `kid` would fix it.
- The stale root `README.md` (still describes an Authelia/OpenLDAP design) and the missing
  `doc/authentication/keycloak.md`.
- The `crates/storage` build breakage described in §9.
