use crate::commands::authorization::{FeatureAccess, SessionState, UserInfo};

#[derive(Default)]
pub struct AuthService {
    cached_token: Option<String>,
}

impl AuthService {
    pub fn check_cached_session(&self) -> Result<SessionState, String> {
        // Reads cached session from AppData / Registry
        Ok(SessionState {
            is_authorized: false,
            user: None,
            access: FeatureAccess {
                ets2: true,
                ets2mp: true,
                spoofer: false,
                dlc: false,
            },
        })
    }

    pub fn initiate_discord_oauth(&mut self) -> Result<String, String> {
        // Starts local HTTP callback server and opens Discord OAuth URL in default browser
        Ok("https://discord.com/oauth2/authorize?client_id=123456789&response_type=code".to_string())
    }

    pub fn cancel_oauth(&mut self) {
        // Closes local callback listener
    }

    pub fn clear_session(&mut self) {
        self.cached_token = None;
    }
}
