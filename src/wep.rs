use crate::utils::escape;

pub struct WEP {
    ssid: String,
    password: String,
    is_hidden: bool,
}

impl WEP {
    pub fn builder() -> WEPBuilder {
        WEPBuilder::new()
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
        String::from("T:WEP;")
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

pub struct WEPBuilder {
    ssid: Option<String>,
    password: Option<String>,
    is_hidden: bool,
}

pub enum WEPErrors {
    NoSSID,
    NoPassword,
    NoPasswordAndSSID,
}

impl WEPBuilder {
    pub fn new() -> Self {
        WEPBuilder {
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

    pub fn build(&self) -> Result<WEP, WEPErrors> {
        match (self.ssid.as_ref(), self.password.as_ref()) {
            (None, None) => Err(WEPErrors::NoPasswordAndSSID),
            (None, Some(_)) => Err(WEPErrors::NoSSID),
            (Some(_), None) => Err(WEPErrors::NoPassword),
            (Some(ssid), Some(password)) => Ok(WEP {
                ssid: ssid.to_string(),
                password: password.to_string(),
                is_hidden: self.is_hidden,
            }),
        }
    }
}
