#[cfg(test)]
mod tests {
    use wifi_qr_specs::wpa2_eap::{WPA2_EAPError, EAP, PHASE2METHOD, WPA2_EAP};

    #[test]
    fn it_encodes_valid_wifi_forms() {
        let wpa2eap = WPA2_EAP::builder()
            .ssid("ghamza")
            .password("P@ssw0rd")
            .hidden(true)
            .identity("Hamza")
            .anonymous_identity("someone")
            .eap_method(EAP::TLS)
            .phase_2_method(PHASE2METHOD::GTC)
            .build()
            .ok()
            .unwrap();
        assert_eq!(
            wpa2eap.encode(),
            "WIFI:T:WPA2_EAP;S:ghamza;I:Hamza;A:someone;P:P@ssw0rd;E:TLS;PH2:GTC;;"
        );
    }
}
