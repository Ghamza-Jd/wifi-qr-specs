use crate::utils::escape;

pub struct NoPass {
    ssid: String,
    is_hidden: bool,
}

impl NoPass {
    pub fn builder() -> NoPassBuilder {
        NoPassBuilder::new()
    }

    pub fn encode(&self) -> String {
        format!(
            "WIFI:{}{}{};",
            self.encode_type(),
            self.encode_ssid(),
            self.encode_hidden()
        )
    }

    fn encode_type(&self) -> String {
        String::from("T:nopass;")
    }

    fn encode_ssid(&self) -> String {
        format!("S:{};", escape(&self.ssid))
    }

    fn encode_hidden(&self) -> String {
        format!("H:{};", self.is_hidden)
    }
}

pub struct NoPassBuilder {
    ssid: Option<String>,
    is_hidden: bool,
}

pub enum NoPassErrors {
    NoSSID
}

impl NoPassBuilder {
    pub fn new() -> Self {
        NoPassBuilder {
            ssid: None,
            is_hidden: false,
        }
    }

    pub fn ssid(&mut self, s: &str) -> &mut Self {
        self.ssid = Some(s.to_owned());
        self
    }

    pub fn hidden(&mut self, h: bool) -> &mut Self {
        self.is_hidden = h;
        self
    }

    pub fn build(&self) -> Result<NoPass, NoPassErrors> {
        match self.ssid.as_ref() {
            None => Err(NoPassErrors::NoSSID),
            Some(ssid) => Ok(NoPass {
                ssid: ssid.to_string(),
                is_hidden: self.is_hidden,
            }),
        }
    }
}
