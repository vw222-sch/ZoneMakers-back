# ZoneMakers backend
Preliminary README & endpoints

## Authentication & Headers
Most endpoints (except `/register` and `/login`) require a `Token` header for authentication.

## Endpoints

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
- [x] GET /admin/zones/requests [Header: Token (Admin)] -> [Zone requests] | Error (403/500)
- [x] POST /admin/zones/{id}/accept [Header: Token (Admin)] -> "Zone request accepted" | Error (403/500)
- [x] POST /admin/zones/{id}/reject [Header: Token (Admin)] -> "Zone request rejected (deleted)" | Error (403/500)
