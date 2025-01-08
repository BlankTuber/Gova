pub struct PasswordEntry {
    username: String,
    password: String,
    place: String,
}

impl PasswordEntry {
    pub fn new(username: String, password: String, place: String) -> Result<Self, &'static str> {
        let entry = Self {
            username: username.trim().to_string(),
            password: password.trim().to_string(),
            place: place.trim().to_string(),
        };
        entry.validate()?;
        Ok(entry)
    }

    pub fn update(&mut self, new_username: Option<String>, new_password: Option<String>, new_place: Option<String>) -> Result<(), &'static str> {
        if let Some(new_username) = new_username {
            self.username = new_username.trim().to_string();
        }
        if let Some(new_password) = new_password {
            self.password = new_password.trim().to_string();
        }
        if let Some(new_place) = new_place {
            self.place = new_place.trim().to_string();
        }

        self.validate()?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.username.is_empty() {
            return Err("Username cannot be empty");
        }
        if self.password.is_empty() {
            return Err("Password cannot be empty");
        }
        if self.place.is_empty() {
            return Err("Place cannot be empty");
        }
        Ok(())
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn place(&self) -> &str {
        &self.place
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}