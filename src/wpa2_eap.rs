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
