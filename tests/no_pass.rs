#[cfg(test)]
mod tests {
    use wifi_qr_specs::no_pass::{NoPass, NoPassErrors};

    #[test]
    fn it_encodes_valid_wifi_forms() {
        let no_pass = NoPass::builder()
            .ssid("ghamza")
            .hidden(true)
            .build()
            .ok()
            .unwrap();
        assert_eq!(no_pass.encode(), "WIFI:T:nopass;S:ghamza;H:true;;");

        let no_pass = NoPass::builder()
            .ssid("ghamza")
            .hidden(false)
            .build()
            .ok()
            .unwrap();
        assert_eq!(no_pass.encode(), "WIFI:T:nopass;S:ghamza;H:false;;");
    }

    #[test]
    fn it_throws_error_when_required_field_is_missing() {
        let no_pass = NoPass::builder()
            .build()
            .err()
            .unwrap();
        assert!(matches!(no_pass, NoPassErrors::NoSSID));
    }
}
