//pg_app/server/src/users/auth_functions.rs
// verify_password(input: &str, stored_hash: &str) -> Result<bool>
// login_user(email: &str, password: &str) -> Result<User>
// logout_user(session_id: Uuid)
// // Session management
// create_session(user_id: i32) -> Result<Session>
// get_user_from_session(session_token: &str) -> Option<User>
// invalidate_session(session_id: Uuid)
// refresh_token(old_token: &str) -> Result<NewToken>


// // Client Account Lifecycle Functions
// request_password_reset(email: &str)
// reset_password(token: &str, new_password: &str)
// change_password(user_id: i32, old_password: &str, new_password: &str)
// verify_email(token: &str)
// resend_verification_email(email: &str)


// 🛡️ Security Utilities

// 1. Rate limiting / brute-force protection
// 2. 2FA verification (TOTP, SMS, Email codes)
// 3. Password strength checks
// 4. Audit logs for logins / failed attempts