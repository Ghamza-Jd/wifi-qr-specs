#[cfg(test)]
mod tests {
    use wifi_qr_specs::wpa::{WPA, WPAErrors};

    #[test]
    fn it_encodes_valid_wifi_forms() {
        let wpa = WPA::builder()
            .ssid("ghamza")
            .password("P@ssw0rd")
            .hidden(true)
            .build()
            .ok()
            .unwrap();
        assert_eq!(wpa.encode(), "WIFI:T:WPA;S:ghamza;P:P@ssw0rd;H:true;;");

        let wpa = WPA::builder()
            .ssid("ghamza")
            .password("P@ssw0rd")
            .hidden(false)
            .build()
            .ok()
            .unwrap();
        assert_eq!(wpa.encode(), "WIFI:T:WPA;S:ghamza;P:P@ssw0rd;H:false;;");
    }

    #[test]
    fn it_throws_error_when_required_field_is_missing() {
        let wpa = WPA::builder()
            .build()
            .err()
            .unwrap();
        assert!(matches!(wpa, WPAErrors::NoPasswordAndSSID));

        let wpa = WPA::builder()
            .ssid("ghamza")
            .build()
            .err()
            .unwrap();
        assert!(matches!(wpa, WPAErrors::NoPassword));

        let wpa = WPA::builder()
            .password("P@ssw0rd")
            .build()
            .err()
            .unwrap();
        assert!(matches!(wpa, WPAErrors::NoSSID));
    }
}
