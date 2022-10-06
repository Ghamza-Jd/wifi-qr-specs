use crate::utils::escape;

/// Enterprise configuration details for Wi-Fi. Stores details about the EAP method and any associated credentials.
pub struct WPA2_EAP {
    ssid: String,
    password: String,
    is_hidden: bool,
    phase_2_method: PHASE2METHOD,
    identity: String,
    anonymous_identity: String,
    eap_method: EAP
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
    SIM
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
    WAPI_CERT
}