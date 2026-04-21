# ZoneMakers backend
Preliminary README & endpoints

## Authentication & Headers
Most endpoints (except `/register` and `/login`) require a `Token` header for authentication.

## Endpoints

### Authentication
- [x] POST /register {email, username, password} -> {token, id} | Error (409)
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
- [x] PATCH /user/bio [Header: Token] {new_bio} -> "OK" | Error
- [x] PATCH /user/pinned_badges [Header: Token] {new_pinned_badges} -> "OK" | Error
- [x] PATCH /user/password [Header: Token] {new_password} -> "OK" | Error

### Badges
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

### Admin (Restricted)
- [x] GET /admin/support/all [Header: Token (Admin)] -> [Support requests] | Error (403)
- [x] DELETE /admin/support/{id} [Header: Token (Admin)] -> "Deleted" | Error (403)
