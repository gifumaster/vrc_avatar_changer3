use keyring::Entry;

use crate::models::StoredSession;

const SERVICE_NAME: &str = "com.gifum.avatarchanger";
const ENTRY_NAME: &str = "vrchat-session";

pub struct CredentialStore {
    entry: Entry,
}

impl CredentialStore {
    pub fn new() -> Result<Self, String> {
        let entry = Entry::new(SERVICE_NAME, ENTRY_NAME).map_err(|error| error.to_string())?;

        Ok(Self { entry })
    }

    pub fn load_session(&self) -> Result<Option<StoredSession>, String> {
        match self.entry.get_password() {
            Ok(secret) => {
                let session =
                    serde_json::from_str::<StoredSession>(&secret).map_err(|error| error.to_string())?;
                Ok(Some(session))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn save_session(&self, session: &StoredSession) -> Result<(), String> {
        let payload = serde_json::to_string(session).map_err(|error| error.to_string())?;

        self.entry
            .set_password(&payload)
            .map_err(|error| error.to_string())
    }

    pub fn clear_session(&self) -> Result<(), String> {
        match self.entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}
