#[cfg(test)]
mod tests {
    use wifi_qr_specs::wpa2_eap::{EAP, PHASE2METHOD, WPA2_EAP};

    #[test]
    fn it_encodes_valid_wifi_forms() {
        let wpa2eap = WPA2_EAP::builder()
            .ssid("ghamza")
            .password("P@ssw0rd")
            .hidden(true)
            .identity("Hamza")
            .anonymous_identity("someone")
            .eap_method(EAP::PEAP)
            .phase_2_method(PHASE2METHOD::MSCHAPV2)
            .build()
            .ok()
            .unwrap();
        assert_eq!(
            wpa2eap.encode(),
            "WIFI:T:WPA2-EAP;S:ghamza;H:true;I:Hamza;AI:someone;P:P@ssw0rd;E:PEAP;PH2:MSCHAPV2;;"
        );
    }
}
