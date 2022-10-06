use crate::utils::escape;

pub struct WPA {
    ssid: String,
    password: String,
    is_hidden: bool,
}

impl WPA {
    pub fn builder() -> WPABuilder {
        WPABuilder::new()
    }

    pub fn encode(&self) -> String {
        format!(
            "WIFI:{}{}{}{};",
            self.encode_type(),
            self.encode_ssid(),
            self.encode_password(),
            self.encode_hidden()
        )
    }

    fn encode_type(&self) -> String {
        String::from("T:WPA;")
    }

    fn encode_ssid(&self) -> String {
        format!("S:{};", escape(&self.ssid))
    }

    fn encode_password(&self) -> String {
        format!("P:{};", escape(&self.password))
    }

    fn encode_hidden(&self) -> String {
        format!("H:{};", self.is_hidden)
    }
}

pub struct WPABuilder {
    ssid: Option<String>,
    password: Option<String>,
    is_hidden: bool,
}

pub enum WPAErrors {
    NoSSID,
    NoPassword,
    NoPasswordAndSSID,
}

impl WPABuilder {
    pub fn new() -> Self {
        WPABuilder {
            ssid: None,
            password: None,
            is_hidden: false,
        }
    }

    pub fn ssid(&mut self, s: &str) -> &mut Self {
        self.ssid = Some(s.to_owned());
        self
    }

    pub fn password(&mut self, pwd: &str) -> &mut Self {
        self.password = Some(pwd.to_owned());
        self
    }

    pub fn hidden(&mut self, h: bool) -> &mut Self {
        self.is_hidden = h;
        self
    }

    pub fn build(&self) -> Result<WPA, WPAErrors> {
        match (self.ssid.as_ref(), self.password.as_ref()) {
            (None, None) => Err(WPAErrors::NoPasswordAndSSID),
            (None, Some(_)) => Err(WPAErrors::NoSSID),
            (Some(_), None) => Err(WPAErrors::NoPassword),
            (Some(ssid), Some(password)) => Ok(WPA {
                ssid: ssid.to_string(),
                password: password.to_string(),
                is_hidden: self.is_hidden,
            }),
        }
    }
}

impl Default for WPABuilder {
    fn default() -> Self {
        Self::new()
    }
}
