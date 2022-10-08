use crate::utils::escape;

/// Enterprise configuration details for Wi-Fi.
/// Stores details about the EAP method and any associated credentials.
#[allow(non_camel_case_types)]
pub struct WPA2_EAP {
    ssid: String,
    is_hidden: bool,
    identity: String,
    anonymous_identity: String,
    password: String,
    eap_method: EAP,
    phase_2_method: PHASE2METHOD,
}

#[allow(non_camel_case_types)]
pub struct WPA2_EAPBuilder {
    ssid: Option<String>,
    is_hidden: bool,
    identity: Option<String>,
    anonymous_identity: Option<String>,
    password: Option<String>,
    eap_method: EAP,
    phase_2_method: PHASE2METHOD,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum EAP {
    /// No EAP method used.
    None,
    /// EAP-Authentication and Key Agreement [RFC-4187]
    AKA,
    /// EAP-Authentication and Key Agreement Prime [RFC-5448]
    AKA_PRIME,
    /// Protected EAP
    PEAP,
    /// EAP-Password
    PWD,
    /// EAP-Subscriber Identity Module [RFC-4186]
    SIM,
    /// EAP-Transport Layer Security
    TLS,
    /// EAP-Tunneled Transport Layer Security
    TTLS,
    /// Hotspot 2.0 r2 OSEN
    UNAUTH_TLS,
    /// WAPI Certificate
    WAPI_CERT,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum PHASE2METHOD {
    /// No phase 2 method used.
    None,
    /// EAP-Authentication and Key Agreement [RFC-4187]
    AKA,
    /// EAP-Authentication and Key Agreement Prime [RFC-5448]
    AKA_PRIME,
    /// Generic Token Card
    GTC,
    /// Microsoft Challenge Handshake Authentication Protocol
    MSCHAP,
    /// Microsoft Challenge Handshake Authentication Protocol v2
    MSCHAPV2,
    /// Password Authentication Protocol
    PAP,
    /// EAP-Subscriber Identity Module [RFC-4186]
    SIM,
}

#[allow(non_camel_case_types)]
pub enum WPA2_EAPError {
    NoSSID,
    NoPassword,
    NoIdentity,
    NoAnonymousIdentity,
    NoEAPMethod,
    NoPhaseTwoMethod,
}

impl WPA2_EAP {
    pub fn builder() -> WPA2_EAPBuilder {
        WPA2_EAPBuilder {
            ssid: None,
            is_hidden: false,
            identity: None,
            anonymous_identity: None,
            password: None,
            eap_method: EAP::None,
            phase_2_method: PHASE2METHOD::None,
        }
    }

    pub fn encode(&self) -> String {
        format!(
            "WIFI:{}{}{}{}{}{}{}{};",
            self.encode_type(),
            self.encode_ssid(),
            self.encode_hidden(),
            self.encode_identity(),
            self.encode_anonymous_identity(),
            self.encode_password(),
            self.encode_eap_method(),
            self.encode_phase_2_method()
        )
    }

    fn encode_type(&self) -> String {
        String::from("T:WPA2_EAP;")
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

    fn encode_identity(&self) -> String {
        format!("I:{};", escape(&self.identity))
    }

    fn encode_anonymous_identity(&self) -> String {
        format!("A:{};", escape(&self.anonymous_identity))
    }

    fn encode_eap_method(&self) -> String {
        let eap = match self.eap_method {
            EAP::None => String::from(""),
            EAP::AKA => String::from("AKA"),
            EAP::AKA_PRIME => String::from("AKA_PRIME"),
            EAP::PEAP => String::from("PEAP"),
            EAP::PWD => String::from("PWD"),
            EAP::SIM => String::from("SIM"),
            EAP::TLS => String::from("TLS"),
            EAP::TTLS => String::from("TTLS"),
            EAP::UNAUTH_TLS => String::from("UNAUTH_TLS"),
            EAP::WAPI_CERT => String::from("WAPI_CERT"),
        };
        format!("E:{};", eap)
    }

    fn encode_phase_2_method(&self) -> String {
        let p2m = match self.phase_2_method {
            PHASE2METHOD::None => String::from(""),
            PHASE2METHOD::AKA => String::from("AKA"),
            PHASE2METHOD::AKA_PRIME => String::from("AKA_PRIME"),
            PHASE2METHOD::GTC => String::from("GTC"),
            PHASE2METHOD::MSCHAP => String::from("MSCHAP"),
            PHASE2METHOD::MSCHAPV2 => String::from("MSCHAPV2"),
            PHASE2METHOD::PAP => String::from("PAP"),
            PHASE2METHOD::SIM => String::from("SIM"),
        };
        format!("PH2:{};", p2m)
    }
}

impl WPA2_EAPBuilder {
    pub fn new() -> Self {
        Self {
            ssid: None,
            is_hidden: false,
            identity: None,
            anonymous_identity: None,
            password: None,
            eap_method: EAP::None,
            phase_2_method: PHASE2METHOD::None,
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

    pub fn identity(&mut self, idnty: &str) -> &mut Self {
        self.identity = Some(idnty.to_owned());
        self
    }

    pub fn anonymous_identity(&mut self, idnty: &str) -> &mut Self {
        self.anonymous_identity = Some(idnty.to_owned());
        self
    }

    pub fn eap_method(&mut self, eap: EAP) -> &mut Self {
        self.eap_method = eap;
        self
    }

    pub fn phase_2_method(&mut self, p2m: PHASE2METHOD) -> &mut Self {
        self.phase_2_method = p2m;
        self
    }

    pub fn build(&self) -> Result<WPA2_EAP, WPA2_EAPError> {
        let ssid = self.ssid.as_ref().ok_or(WPA2_EAPError::NoSSID)?.to_string();
        let identity = self
            .identity
            .as_ref()
            .ok_or(WPA2_EAPError::NoIdentity)?
            .to_string();
        let anonymous_identity = self
            .anonymous_identity
            .as_ref()
            .ok_or(WPA2_EAPError::NoAnonymousIdentity)?
            .to_string();
        let password = self
            .password
            .as_ref()
            .ok_or(WPA2_EAPError::NoPassword)?
            .to_string();
        Ok(WPA2_EAP {
            ssid,
            is_hidden: self.is_hidden,
            identity,
            anonymous_identity,
            password,
            eap_method: self.eap_method,
            phase_2_method: self.phase_2_method,
        })
    }
}
