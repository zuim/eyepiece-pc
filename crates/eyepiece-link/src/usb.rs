use nusb::descriptors::{ConfigurationDescriptor, TransferType};
use nusb::transfer::Direction;
use nusb::{Device, DeviceInfo, Interface, MaybeFuture};

const DJI_VENDOR_ID: u16 = 0x2ca3;
const GOGGLES_PRODUCT_ID: u16 = 0x0020;
const BULK_INTERFACE_CLASS: u8 = 0xff;
const BULK_INTERFACE_NAME: &str = "Eyepiece bulk";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BulkEndpoints {
    pub interface_number: u8,
    pub in_address: u8,
    pub in_max_packet_size: usize,
    pub out_address: u8,
    pub out_max_packet_size: usize,
}

pub struct UsbDevice {
    pub device: Device,
    pub interface: Interface,
    pub endpoints: BulkEndpoints,
}

struct Candidate {
    device: DeviceInfo,
    interface_number: u8,
}

fn serial_matches(device: &DeviceInfo, wanted: Option<&str>) -> Result<bool, String> {
    let Some(wanted) = wanted else {
        return Ok(true);
    };
    let serial = match device.serial_number() {
        Some(serial) => Some(serial.to_owned()),
        None => device
            .get_serial_number_string()
            .wait()
            .map_err(|e| format!("cannot read Goggles USB serial number ({e})"))?,
    };
    Ok(serial.as_deref() == Some(wanted))
}

fn find_candidates(serial: Option<&str>) -> Result<Vec<Candidate>, String> {
    let devices = nusb::list_devices()
        .wait()
        .map_err(|e| format!("cannot enumerate USB devices ({e})"))?;
    let mut goggles_seen = 0;
    let mut candidates = Vec::new();
    let mut descriptor_errors = Vec::new();

    for device in devices.filter(|device| {
        device.vendor_id() == DJI_VENDOR_ID && device.product_id() == GOGGLES_PRODUCT_ID
    }) {
        if !serial_matches(&device, serial)? {
            continue;
        }
        goggles_seen += 1;

        for interface in device
            .interfaces()
            .filter(|interface| interface.class() == BULK_INTERFACE_CLASS)
        {
            let name = match interface.interface_string() {
                Some(name) => Some(name.to_owned()),
                None => match interface.get_interface_string().wait() {
                    Ok(name) => name,
                    Err(e) => {
                        descriptor_errors
                            .push(format!("interface {}: {e}", interface.interface_number()));
                        continue;
                    }
                },
            };
            if name.as_deref() == Some(BULK_INTERFACE_NAME) {
                candidates.push(Candidate {
                    device: device.clone(),
                    interface_number: interface.interface_number(),
                });
            }
        }
    }

    match candidates.len() {
        0 if goggles_seen == 0 => {
            let selection = serial.map(|s| format!(" with USB serial {s}")).unwrap_or_default();
            Err(format!("no DJI Goggles 3 USB device{selection} found"))
        }
        0 if !descriptor_errors.is_empty() => {
            Err(format!(
                "Goggles found, but the Eyepiece bulk interface descriptor is inaccessible: {}",
                descriptor_errors.join(", ")
            ))
        }
        0 => Err("Goggles found, but no verified Eyepiece bulk interface; update the bridge/firmware and reconnect USB".into()),
        1 => Ok(candidates),
        count => Err(format!("found {count} Eyepiece bulk interfaces; specify --serial to select one")),
    }
}

pub fn probe(serial: Option<&str>) -> Result<(), String> {
    find_candidates(serial).map(|_| ())
}

pub fn discover(serial: Option<&str>) -> Result<UsbDevice, String> {
    let candidate = find_candidates(serial)?.pop().unwrap();

    let device = candidate.device.open().wait().map_err(|e| {
        format!("cannot open Goggles USB device (check driver and permissions: {e})")
    })?;
    let config = device.active_configuration().map_err(|e| {
        format!("Goggles USB device has no active configuration ({e}); reconnect the USB cable")
    })?;
    let endpoints = bulk_endpoints(&config, candidate.interface_number)?;
    let interface = device
        .claim_interface(candidate.interface_number)
        .wait()
        .map_err(|e| {
            format!(
                "cannot claim Eyepiece bulk interface {} ({e})",
                candidate.interface_number
            )
        })?;

    Ok(UsbDevice {
        device,
        interface,
        endpoints,
    })
}

fn bulk_endpoints(
    config: &ConfigurationDescriptor<'_>,
    interface_number: u8,
) -> Result<BulkEndpoints, String> {
    let mut descriptors = config.interface_alt_settings().filter(|descriptor| {
        descriptor.interface_number() == interface_number && descriptor.alternate_setting() == 0
    });
    let descriptor = descriptors.next().ok_or_else(|| {
        format!("Eyepiece bulk interface {interface_number} is missing alternate setting 0")
    })?;
    if descriptors.next().is_some() {
        return Err(format!(
            "Eyepiece bulk interface {interface_number} has duplicate alternate setting 0"
        ));
    }
    if descriptor.class() != BULK_INTERFACE_CLASS || descriptor.num_endpoints() != 2 {
        return Err(format!(
            "Eyepiece bulk interface {interface_number} has unexpected class or endpoint count"
        ));
    }

    let mut input = None;
    let mut output = None;
    for endpoint in descriptor.endpoints() {
        if endpoint.transfer_type() != TransferType::Bulk || endpoint.max_packet_size() == 0 {
            return Err(format!(
                "Eyepiece bulk interface {interface_number} has an invalid endpoint descriptor"
            ));
        }
        let value = (endpoint.address(), endpoint.max_packet_size());
        let slot = match endpoint.direction() {
            Direction::In => &mut input,
            Direction::Out => &mut output,
        };
        if slot.replace(value).is_some() {
            return Err(format!(
                "Eyepiece bulk interface {interface_number} has duplicate endpoint directions"
            ));
        }
    }
    let (in_address, in_max_packet_size) = input.ok_or_else(|| {
        format!("Eyepiece bulk interface {interface_number} has no bulk IN endpoint")
    })?;
    let (out_address, out_max_packet_size) = output.ok_or_else(|| {
        format!("Eyepiece bulk interface {interface_number} has no bulk OUT endpoint")
    })?;

    Ok(BulkEndpoints {
        interface_number,
        in_address,
        in_max_packet_size,
        out_address,
        out_max_packet_size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> ConfigurationDescriptor<'static> {
        const BYTES: &[u8] = &[
            9, 2, 32, 0, 1, 1, 0, 0x80, 50, 9, 4, 3, 0, 2, 0xff, 0, 0, 1, 7, 5, 0x81, 2, 0, 2, 0,
            7, 5, 0x01, 2, 0, 2, 0,
        ];
        ConfigurationDescriptor::new(BYTES).unwrap()
    }

    #[test]
    fn discovers_bulk_pair_from_descriptors() {
        let endpoints = bulk_endpoints(&config(), 3).unwrap();
        assert_eq!(
            endpoints,
            BulkEndpoints {
                interface_number: 3,
                in_address: 0x81,
                in_max_packet_size: 512,
                out_address: 0x01,
                out_max_packet_size: 512,
            }
        );
    }

    #[test]
    fn rejects_non_vendor_interface() {
        let mut bytes = config().as_bytes().to_vec();
        bytes[14] = 2;
        let config = ConfigurationDescriptor::new(&bytes).unwrap();
        assert!(bulk_endpoints(&config, 3).is_err());
    }

    #[test]
    fn rejects_non_bulk_endpoint() {
        let mut bytes = config().as_bytes().to_vec();
        bytes[21] = 3;
        let config = ConfigurationDescriptor::new(&bytes).unwrap();
        assert!(bulk_endpoints(&config, 3).is_err());
    }
}
