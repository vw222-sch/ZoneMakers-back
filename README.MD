# ZoneMakers backend

## Overview
This repository contains the backend service for ZoneMakers. It provides the API surfaces used by the frontend for authentication, user profile management, badges, notifications, posts, zone moderation, reports, travel logs, and admin actions.

The backend is implemented as a single Axum-based web service with a local Turso/SQLite database.

## Related Documentation

- [Frontend Application Documentation](../ZoneMakers-front/README.md) - Details on the frontend architecture and user interface

## Key Responsibilities
- Provide secure authentication and session management for frontend users.
- Expose user profile and account management endpoints.
- Maintain badge, notification, and travel-log workflows.
- Support zone discovery, creation requests, moderation, and admin approvals.
- Handle user-generated content: posts, replies, and reporting flows.
- Enforce authorization and role-based admin actions.
- Map database rows into typed response models for the frontend.

## Inputs & Outputs
### Common request inputs
- Protected routes require a `Token` header containing the JWT.
- JSON request bodies are used for create/update operations.
- Route parameters are used for resource lookup and pagination.

### Parameter formats
- `id` values are integers for users, badges, zones, reports, and travel entries.
- `region` values are integers representing geographic zones.
- `page` values are integers used for paginated collections.
- `query` values are simple search strings used by zone search.
- `type` fields are string literals used for notification or travel record categories.

### Example request bodies
- Register: `{ "email": "user@example.com", "username": "user", "password": "pass", "region": 1 }`
- Login: `{ "handle": "user", "password": "pass" }`
- Create post: `{ "content": "Hello", "image": "", "region": 1, "reply_id": "" }`
- Update handle: `{ "new_handle": "newUser" }`
- Create zone request: `{ "name": "Zone Name", "coordinates": "[lat,lng]", "hazard_level": "medium", "description": "...", "hazards": "...", "images": "[]" }`

### Response shapes
- Successful authentication returns `{ "token": "...", "id": <int> }`.
- Read operations return JSON objects or arrays of objects representing domain models.
- Update/delete operations often return plain strings such as `"OK"`, `"Deleted"`, or `"Badge created"`.

### Data model notes
- `User` data omits raw password values in responses.
- `Badge`, `Post`, `Zone`, and `Travel` models are serialized from database rows via helper constructors.
- JSON arrays are stored in the database as strings for fields like `badges` and `pinned_badges`.
- `TokenClaims` encodes `id`, `username`, and `exp`.

## Where it fits in the system
- Frontend: `ZoneMakers-front` consumes this API.
- Backend: `ZoneMakers-back` owns persistence, authorization, and domain workflows.
- Runtime: listens on `0.0.0.0:3000` and serves JSON endpoints.

## Architecture
### Key components
- `src/main.rs` - application entrypoint, router setup, shared state, CORS.
- `src/endpoints/` - domain-specific handler modules.
- `turso` - local database driver for SQLite.
- `jsonwebtoken` - JWT creation and validation for user sessions.

### Technical areas
#### API endpoints
- Routes are registered in `src/main.rs`; each route maps directly to a domain handler.
- Endpoint methods and paths are documented in the API reference below.
- JSON responses are either typed objects or status strings, depending on the operation.

#### Authentication and authorization
- Protected handlers require a `Token` header containing a JWT.
- `token_to_claims()` decodes the token and provides user identity for authorization.
- Admin-only routes verify the requesting user’s `admin` flag before executing sensitive operations.

#### Database interactions
- The backend uses inline SQL with `turso::Connection` and a shared `State` object.
- Query results are mapped into Rust models using `from_row()` constructors.
- Some arrays are persisted as JSON strings in SQLite and deserialized on read.

#### Data validation and sanitization
- Request bodies are deserialized via Serde but field content is not deeply validated.
- SQL parameter binding prevents direct injection, but payload shape and lengths are not enforced.
- Handler-level validation is the current boundary; production should add stricter checks for email, handle, and array formats.

### Request flow
1. HTTP request arrives at the Axum router.
2. Route dispatch selects a handler from `src/endpoints/`.
3. Handler reads the `Token` header for protected routes.
4. The shared `State` holds the live `turso::Connection`.
5. Query results are mapped into structs via `from_row` helpers.

### Authentication model
- `POST /register` creates a new user and issues a JWT token.
- `POST /login` validates credentials and returns an existing token.
- Most protected routes require `Token` header with a JWT.
- Admin routes check the requesting user’s `admin` flag.

> Note: the current service performs plaintext password checks and uses a fixed JWT secret placeholder. This is sufficient for local development, but should be hardened before production.

## Core data models
The service defines several shared models in `src/main.rs`:

- `User` - internal user profile and authorization metadata.
- `Badge` - badge metadata.
- `Post` - post content plus joined author fields.
- `Zone` - zone record and request status.
- `Travel` - travel log entry.

Each model includes a `from_row` constructor that converts `turso::Row` into a serializable struct.

## Running the service
From `ZoneMakers-back`:

```bash
cargo run
```

The service prints `http://localhost:3000` and binds to port `3000`.
Database file: `./db.sqlite`.

## Usage Example
The backend is consumed over HTTP. Use `curl` or your client to hit the API:

```bash
# Register a new user
curl -X POST http://localhost:3000/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","username":"user","password":"pass","region":1}'

# Login and receive token
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"handle":"user","password":"pass"}'

# Fetch profile with Token header
curl http://localhost:3000/user/1 \
  -H "Token: <jwt-token>"
```

For protected routes, pass the `Token` header returned by login.

## Edge Cases & Error Handling
- Missing or invalid `Token` headers return `401 Unauthorized` in protected handlers.
- Duplicate registration data returns `409 Conflict` when an existing email or username is found.
- Handle conflicts return `409 Conflict` for duplicate user handles.
- Resource lookups return `404 Not Found` when requested users, badges, zones, reports, or travel entries do not exist.
- Invalid route payloads and malformed requests are surfaced as generic errors, often with `400` or `500` status codes.
- Database query failures fall back to `500 Internal Server Error` with JSON error messages.
- Admin-restricted actions return `403 Forbidden` when the current user is not authorized.
- Plaintext password validation and static JWT secrets are current development limitations; these should be replaced with secure hashing and environment-managed secrets.

## Implementation details
### Router setup
The router is assembled in `src/main.rs` with each route bound to a handler:

```rust
let app = Router::new()
    .route("/register", post(post_user_handler))
    .route("/login", post(post_login_handler))
    .route("/user/{id}", get(get_user_id_handler).delete(delete_user_handler))
    .route("/zones", get(get_zones_handler))
    .layer(Extension(state))
    .layer(cors);
```

### Token validation
Token handlers use `token_to_claims()` to decode the JWT:

```rust
fn token_to_claims(token: &str) -> Option<TokenClaims> {
    let valid = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret("super secret key placeholder".as_ref()),
        &Validation::default(),
    );
    match valid {
        Ok(data) => Some(data.claims),
        Err(_) => None,
    }
}
```

### Common helper
`collect_rows()` converts database rows into typed vectors:

```rust
pub async fn collect_rows<T>(mut rows: turso::Rows, convert: fn(turso::Row) -> T) -> Vec<T> {
    let mut out = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        out.push(convert(row));
    }
    out
}
```

## Internal Logic
### Request handling flow
1. Axum accepts an HTTP request and matches the route in `src/main.rs`.
2. The route handler receives shared state via `Extension(Arc<State>)`.
3. Protected endpoints extract the `Token` header and call `token_to_claims()`.
4. `token_to_claims()` decodes JWTs using the fixed secret and verifies expiration.
5. Handlers execute SQL queries via `state.connection`, then map rows to structs.
6. Responses are returned as JSON with a status code, either a domain object or a status string.

### Database interaction
- The service uses inline SQL strings for all operations.
- `turso::Connection` is stored in shared state and reused across requests.
- Result rows are materialized with model-specific `from_row()` constructors.
- Some JSON-like fields are persisted as strings in SQLite, then deserialized in Rust.

### Authorization and validation decisions
- Authentication is token-based, with `Token` header validation on protected routes.
- Admin-only routes rely on the user record’s `admin` boolean.
- Unique constraints are enforced in handlers, e.g. handle uniqueness and duplicate user detection.
- The service currently checks passwords in plaintext; this is a deliberate development simplification that must be replaced before production.

### Important implementation choices
- Single binary architecture keeps routing, models, and handlers in one service.
- Domain models are defined in `src/main.rs` rather than split into separate struct modules.
- The app favors direct SQL queries over ORM abstraction for simplicity.
- Handler responses are mostly primitive strings for success cases, which matches frontend expectations but limits response extensibility.

## Performance Considerations
- Inline SQL on every request may become a bottleneck under high load; consider query caching or prepared statements.
- `collect_rows()` eagerly loads all results into memory; use pagination or streaming for large collections.
- Shared `turso::Connection` is reused, but concurrent database access may still incur contention in SQLite/Turso.
- The current implementation does not batch related queries, so endpoints joining user and content data may execute multiple sequential statements.
- JSON string fields for arrays (`badges`, `pinned_badges`) require deserialization on every read; normalize these fields or keep them as separate relations for faster access.
- Token decoding and validation are lightweight, but the static JWT secret and lack of expiration enforcement can mask performance issues in auth logic.
- CORS is broad and permissive; tighten allowed origins in production to reduce exposure and request validation overhead.

## Security Considerations
- Input validation is minimal: handlers trust JSON payload structure and use SQL parameters, but they do not validate field content or length.
- The current auth model uses plaintext passwords in the database and direct password comparisons; replace with salted password hashing and secure storage.
- JWTs are decoded with a hardcoded secret; move secrets to environment variables and enforce token expiration.
- `Token` headers are the only auth mechanism; validate the token claim subject and user status before passing requests through.
- Admin routes rely on the `admin` boolean; protect these handlers from privilege escalation by verifying the token owner’s record.
- Sensitive fields like passwords are stripped from `User` responses, but the underlying model still contains password data in memory.
- Database query failures and malformed inputs should not expose raw SQL or stack traces in production.
- CORS is currently open; restrict allowed origins and headers in production deployments.

## Scalability Notes
- SQLite/Turso is suitable for local development, but may struggle with high concurrency and write-heavy workloads.
- The service’s single binary architecture means scaling horizontally requires multiple instances behind a load balancer.
- Inline SQL and direct row mapping are easy to maintain, but may become harder to optimize as the schema grows.
- Large collections are loaded in full by `collect_rows()`; paging is necessary to avoid memory pressure on endpoints like posts and reports.
- The current schema stores JSON arrays as strings, which limits efficient querying and filtering as data volume grows.
- Admin and analytics routes that query full request lists should be paginated and indexed to avoid slow scans.
- The service assumes a moderate dataset and user base; if usage grows significantly, move to a stronger database and introduce caching.

## External Dependencies
- `axum` for HTTP routing and request handling.
- `tokio` for asynchronous runtime support.
- `tower-http` for CORS handling.
- `turso` for SQLite database connection and query execution.
- `serde` / `serde_json` for request/response serialization.
- `jsonwebtoken` for JWT issuance and validation.
- `chrono` for date and time serialization support.
- `uuid` for generating unique identifiers where needed.

## API reference
### Authentication
- [x] POST /register {email, username, password, region} -> {token, id} | Error (409)
- [x] POST /login {username, password} -> {token, id} | Error (401)

### User Management
- [x] GET /user/{id} -> User data | Error (404)
- [x] GET /user/handle/{handle} -> User data | Error (404)
- [x] DELETE /user/{id} [Header: Token] -> "Deleted" | Error (401)
- [x] PATCH /user/name [Header: Token] {new_name} -> "OK" | Error
- [x] PATCH /user/handle [Header: Token] {new_handle} -> "OK" | Error (409)
- [x] PATCH /user/email [Header: Token] {new_email} -> "OK" | Error
- [x] PATCH /user/avatar [Header: Token] {new_avatar} -> "OK" | Error
- [x] PATCH /user/banner [Header: Token] {new_banner} -> "OK" | Error
- [x] PATCH /user/theme [Header: Token] {new_theme} -> "OK" | Error
- [x] PATCH /user/bio [Header: Token] {new_bio} -> "OK" | Error
- [x] PATCH /user/region [Header: Token] {new_region} -> "OK" | Error
- [x] PATCH /user/pinned_badges [Header: Token] {new_pinned_badges} -> "OK" | Error
- [x] PATCH /user/password [Header: Token] {new_password} -> "OK" | Error

### Badges
- [x] GET /badge/all -> [Badge data] | Error (500)
- [x] GET /badge/{id} -> Badge data | Error (404)
- [x] POST /badge [Header: Token (Admin)] {image, title} -> "Badge created" | Error (403/500)
- [x] DELETE /badge/{id} [Header: Token (Admin)] -> "Badge deleted" | Error (403/404)

### Notifications
- [x] GET /notifications [Header: Token] -> [Notifications] | Error (401)
- [x] POST /notifications [Header: Token (Admin)] {user_id, title, message, type} -> "Notification created" | Error (403/500)
- [x] PATCH /notifications/read/{id} [Header: Token] -> "Notification marked as read" | Error (404)
- [x] DELETE /notifications/{id} [Header: Token] -> "Notification deleted" | Error (404)

### Support
- [x] POST /support [Header: Token] {subject, topic, description} -> "OK" | Error (400)

### Posts
- [x] GET /posts/{region}/{page} -> [Posts (with author user data)] | Error (500)
- [x] GET /posts/{id}/replies -> [Replies (with author user data)] | Error (500)
- [x] POST /posts [Header: Token] {content, image?, region, reply_id?} -> {id} | Error (401/500)
- [x] PATCH /posts/{id} [Header: Token] {content?, image?} -> "OK" | Error (401/404)
- [x] DELETE /posts/{id} [Header: Token] -> "Deleted" | Error (401/404)

### Zones
- [x] GET /zones -> [Zone Summaries] | Error (500)
- [x] GET /zones/{id} -> Zone data | Error (404/500)
- [x] GET /zones/search/{query} -> [Zone Summaries] | Error (500)
- [x] POST /zones/requests [Header: Token] {name, coordinates, hazard_level, description, hazards, images} -> "Zone request created" | Error (401/500)
- [x] PUT /zones/{id} [Header: Token] {name?, coordinates?, hazard_level?, description?, hazards?, images?} -> "Zone updated and set to request status" | Error (401/403/404)
- [x] DELETE /zones/{id} [Header: Token] -> "Zone deleted" | Error (401/403/404)

### Reporting
- [x] POST /report/zone [Header: Token] {reason, report_id} -> "Report created"
- [x] GET /report/zones/{page} [Header: Token (Admin)] -> [Reports]
- [x] DELETE /report/zone/{id} [Header: Token (Admin)] -> "Report deleted"
- [x] POST /report/user [Header: Token] {reason, report_id} -> "Report created"
- [x] GET /report/users/{page} [Header: Token (Admin)] -> [Reports]
- [x] DELETE /report/user/{id} [Header: Token (Admin)] -> "Report deleted"
- [x] POST /report/post [Header: Token] {reason, report_id} -> "Report created"
- [x] GET /report/posts/{page} [Header: Token (Admin)] -> [Reports]
- [x] DELETE /report/post/{id} [Header: Token (Admin)] -> "Report deleted"

### Travel Log
- [x] GET /travel/page/{page} -> [Travel logs] | Error (500)
- [x] POST /travel [Header: Token] {title, message, type} -> "Travel log created" | Error (401/500)
- [x] DELETE /travel/{id} [Header: Token] -> "Travel log deleted" | Error (401/403/404)

### Admin (Restricted)
- [x] POST /admin/badge/grant/{id}/{badge_id} [Header: Token (Admin)] -> "OK" | Error (403/404)
- [x] POST /admin/badge/remove/{id}/{badge_id} [Header: Token (Admin)] -> "OK" | Error (403/404)
- [x] GET /admin/support/all [Header: Token (Admin)] -> [Support requests] | Error (403)
- [x] DELETE /admin/support/{id} [Header: Token (Admin)] -> "Deleted" | Error (403)
- [x] POST /admin/user/rep [Header: Token (Admin)] {user_id, rep} -> "OK" | Error (403/404)
- [x] GET /admin/zones/requests [Header: Token (Admin)] -> [Zone requests] | Error (403/500)
- [x] POST /admin/zones/{id}/accept [Header: Token (Admin)] -> "Zone request accepted" | Error (403/500)
- [x] POST /admin/zones/{id}/reject [Header: Token (Admin)] -> "Zone request rejected (deleted)" | Error (403/500)
