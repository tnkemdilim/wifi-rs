use crate::{
    connectivity::{handlers::NetworkXmlProfileHandler, Connectivity, WifiConnectionError},
    platforms::{Connection, WiFi, WifiError, WifiInterface},
};
use std::process::Command;

impl WiFi {
    /// Add the wireless network profile of network to connect to,
    /// (this is specific to windows operating system).
    fn add_profile(ssid: &str, password: &str) -> Result<(), WifiConnectionError> {
        let mut handler = NetworkXmlProfileHandler::new();
        handler.content = handler
            .content
            .replace("{SSID}", ssid)
            .replace("{password}", password);

        let temp_file = handler.write_to_temp_file()?;

        Command::new("netsh")
            .args(&[
                "wlan",
                "add",
                "profile",
                &format!("filename={}", temp_file.path().to_str().unwrap()),
            ])
            .output()
            .map_err(|_| WifiConnectionError::AddNetworkProfileFailed)?;

        Ok(())
    }

    /// Check if the status output contains the ssid.
    fn check_connection(ssid: &str) -> bool {
        let output = Command::new("netsh")
            .args(&["wlan", "show", "interfaces"])
            .output()
            .unwrap();

        String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains(ssid)
    }
}

/// Wireless network connectivity functionality.
impl Connectivity for WiFi {
    /// Attempts to connect to a wireless network with a given SSID and password.
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError> {
        if !WiFi::is_wifi_enabled().map_err(|err| WifiConnectionError::Other { kind: err })? {
            return Err(WifiConnectionError::Other {
                kind: WifiError::WifiDisabled,
            });
        }

        // Check if the ssid is already connected.
        if Self::check_connection(ssid) {
            return Ok(true);
        }

        Self::add_profile(ssid, password)?;

        Command::new("netsh")
            .args(&["wlan", "connect", &format!("name={}", ssid)])
            .status()
            .map_err(|err| WifiConnectionError::FailedToConnect(format!("{}", err)))?;

        // Check if the ssid is connected.
        if !Self::check_connection(ssid)
        {
            return Ok(false);
        }

        self.connection = Some(Connection {
            ssid: String::from(ssid),
        });

        Ok(true)
    }

    /// Attempts to disconnect from a wireless network currently connected to.
    fn disconnect(&self) -> Result<bool, WifiConnectionError> {
        let output = Command::new("netsh")
            .args(&["wlan", "disconnect"])
            .output()
            .map_err(|err| WifiConnectionError::FailedToDisconnect(format!("{}", err)))?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .contains("disconnect"))
    }
}
