use esp_idf_hal::modem::Modem;
use esp_idf_svc::eventloop::EspSystemEventLoop;

use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_sys::EspError;

pub(crate) fn configure(
    ssid: &str,
    pass: &str,
    modem: Modem,
) -> Result<BlockingWifi<EspWifi<'static>>, EspError> {
    // Configure Wifi
    let sysloop = EspSystemEventLoop::take()?;
    // The nvs stores the RF calibration data, which allows
    // for faster connection
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(EspWifi::new(modem, sysloop.clone(), Some(nvs))?, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        password: pass.into(),
        ..Default::default()
    }))?;

    wifi.start()?;
    wifi.connect()?;

    // Wait until the network interface is up
    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    println!("IP info: {:?}", ip_info);
    Ok(wifi)
}
