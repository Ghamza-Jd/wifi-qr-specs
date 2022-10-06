#[cfg(test)]
mod tests {
    use wifi_qr_specs::wep::{WEP, WEPErrors};

    #[test]
    fn it_encodes_valid_wifi_forms() {
        let wep = WEP::builder()
            .ssid("ghamza")
            .password("P@ssw0rd")
            .hidden(true)
            .build()
            .ok()
            .unwrap();
        assert_eq!(wep.encode(), "WIFI:T:WEP;S:ghamza;P:P@ssw0rd;H:true;;");

        let wep = WEP::builder()
            .ssid("ghamza")
            .password("P@ssw0rd")
            .hidden(false)
            .build()
            .ok()
            .unwrap();
        assert_eq!(wep.encode(), "WIFI:T:WEP;S:ghamza;P:P@ssw0rd;H:false;;");
    }

    #[test]
    fn it_throws_error_when_required_field_is_missing() {
        let wep = WEP::builder()
            .build()
            .err()
            .unwrap();
        assert!(matches!(wep, WEPErrors::NoPasswordAndSSID));

        let wep = WEP::builder()
            .ssid("ghamza")
            .build()
            .err()
            .unwrap();
        assert!(matches!(wep, WEPErrors::NoPassword));

        let wep = WEP::builder()
            .password("P@ssw0rd")
            .build()
            .err()
            .unwrap();
        assert!(matches!(wep, WEPErrors::NoSSID));
    }
}
