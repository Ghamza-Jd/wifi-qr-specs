# Wi-Fi QR Specifications

A small lib to create Wi-Fi spepcifications and pass them into a QR code generator to connect to a wifi.

- [Example Usage](#example-usage)
  - [No Password](#no-password)
  - [WEP](#wep)
  - [WPA](#wpa)
  - [WPA2 EAP](#wpa2-eap)

## Example Usage

### No Password

```rust
use wifi_qr_specs::no_pass::NoPass;

let no_pass = NoPass::builder()
    .ssid("ghamza")
    .hidden(true)
    .build()
    .ok()
    .map(|x| { println!("{}", x.encode()) });
```

### WEP

```rust
use wifi_qr_specs::wep::WEP;

let no_pass = let wep = WEP::builder()
      .ssid("ghamza")
      .password("P@ssw0rd")
      .hidden(true)
      .build()
      .ok()
      .map(|x| { println!("{}", x.encode()) });
```

### WPA
```rust
use wifi_qr_specs::wpa::WPA;

let no_pass = let wep = WPA::builder()
      .ssid("ghamza")
      .password("P@ssw0rd")
      .hidden(true)
      .build()
      .ok()
      .map(|x| { println!("{}", x.encode()) });
```

### WPA2 EAP
```rust
use wifi_qr_specs::wpa2_eap::{EAP, PHASE2METHOD, WPA2_EAP};

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
      .map(|x| { println!("{}", x.encode()) });
```
