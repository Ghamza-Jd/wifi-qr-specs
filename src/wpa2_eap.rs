use std::fmt::format;

use crate::utils::escape;

/// Enterprise configuration details for Wi-Fi.
/// Stores details about the EAP method and any associated credentials.
pub struct WPA2_EAP {
    ssid: String,
    is_hidden: bool,
    identity: String,
    anonymous_identity: String,
    password: String,
    eap_method: EAP,
    phase_2_method: PHASE2METHOD,
}

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
            "WIFI:{}{}{}{}{}{}{};",
            self.encode_type(),
            self.encode_ssid(),
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
        format!("E:{};", escape(&self.eap_method.to_string()))
    }

    fn encode_phase_2_method(&self) -> String {
        format!("PH2:{};", escape(&self.phase_2_method.to_string()))
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
}

impl std::fmt::Display for EAP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EAP::None => write!(f, ""),
            EAP::AKA => write!(f, "AKA"),
            EAP::AKA_PRIME => write!(f, "AKA_PRIME"),
            EAP::PEAP => write!(f, "PEAP"),
            EAP::PWD => write!(f, "PWD"),
            EAP::SIM => write!(f, "SIM"),
            EAP::TLS => write!(f, "TLS"),
            EAP::TTLS => write!(f, "TTLS"),
            EAP::UNAUTH_TLS => write!(f, "UNAUTH_TLS"),
            EAP::WAPI_CERT => write!(f, "WAPI_CERT"),
        }
    }
}

impl std::fmt::Display for PHASE2METHOD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PHASE2METHOD::None => write!(f, ""),
            PHASE2METHOD::AKA => write!(f, "AKA"),
            PHASE2METHOD::AKA_PRIME => write!(f, "AKA_PRIME"),
            PHASE2METHOD::GTC => write!(f, "GTC"),
            PHASE2METHOD::MSCHAP => write!(f, "MSCHAP"),
            PHASE2METHOD::MSCHAPV2 => write!(f, "MSCHAPV2"),
            PHASE2METHOD::PAP => write!(f, "PAP"),
            PHASE2METHOD::SIM => write!(f, "SIM"),
        }
    }
}
