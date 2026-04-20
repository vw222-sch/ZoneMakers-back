# ZoneMakers backend
Preliminary README & endpoints

## Endpoints

### Authentication
- [x] POST /register {email, username, password} -> {token, id} | Error (409)
- [x] POST /login {username, password} -> {token, id} | Error (401)

### User Management
- [x] GET /user/{id} -> User data | Error (404)
- [x] DELETE /user/{token} -> "Deleted" | Error (401)
- [x] PATCH /user/name {token, new_name} -> "OK" | Error
- [x] PATCH /user/email {token, new_email} -> "OK" | Error
- [x] PATCH /user/avatar {token, new_avatar} -> "OK" | Error
- [x] PATCH /user/banner {token, new_banner} -> "OK" | Error
- [x] PATCH /user/bio {token, new_bio} -> "OK" | Error
- [x] PATCH /user/pinned_badges {token, new_pinned_badges} -> "OK" | Error
- [x] PATCH /user/password {token, new_password} -> "OK" | Error

### Badges
- [x] GET /badge/{id} -> Badge data | Error (404)

### Support
- [x] POST /support {token, subject, topic, description} -> "OK" | Error (400)

### Admin (Restricted)
- [x] GET /admin/support/all?token={token} -> [Support requests] | Error (403)
- [x] DELETE /admin/support/:id?token={token} -> "Deleted" | Error (403)
