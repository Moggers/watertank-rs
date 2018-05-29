use rtl8710_bindings;
use cty::c_void;

pub struct Wifi<'a> {
    pub ssid: &'a str,
    pub security: SecurityTypes,
    pub pass: &'a str,
}

/// Struct which abstracts around rtl8710's internal wifi controls
impl<'a> Default for Wifi<'a> {
    fn default() -> Self {
        Wifi {
            ssid: "",
            security: SecurityTypes::OPEN,
            pass: "",
        }
    }
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SecurityTypes {
    OPEN = 0i32,
    WEP_PSK = 1i32,
    WEP_SHARED = 2i32,
    WPA_TKIP_PSK = 3i32,
    WPA2_AES_PSK = 4i32,
    WPA2_TKIP_PSK = 5i32,
    WPA2_MIXED_PSK = 6i32,
}

impl<'a> Wifi<'a> {
    pub fn connect(self) {
        unsafe {
            rtl8710_bindings::wifi_connect(
                self.ssid.as_ptr(),
                self.security as i32,
                self.pass.as_ptr(),
                self.ssid.len() as i32,
                self.pass.len() as i32,
                0,
                &mut c_void::__variant1,
            );
        }
    }
}
