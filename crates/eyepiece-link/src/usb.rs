//! USB transport: the goggles' "tinkerglass video" interface (vendor class, one bulk OUT and one
//! bulk IN endpoint). Its byte stream is the same as the bridge's TCP stream.

use nusb::descriptors::{ConfigurationDescriptor, TransferType};
use nusb::transfer::Direction;
use nusb::{Device, DeviceInfo, Interface, MaybeFuture};

const DJI_VENDOR_ID: u16 = 0x2ca3;
const GOGGLES_PRODUCT_ID: u16 = 0x0020;
/// The bridge's interface is found by class / subclass / protocol, never by its number or name
/// (the other tinkerglass interfaces are the same class with protocol 1 and 2 and stay untouched).
const VIDEO_CLASS: u8 = 0xff;
const VIDEO_SUBCLASS: u8 = 0x54;
const VIDEO_PROTOCOL: u8 = 0x03;

fn is_video_interface(class: u8, subclass: u8, protocol: u8) -> bool {
    (class, subclass, protocol) == (VIDEO_CLASS, VIDEO_SUBCLASS, VIDEO_PROTOCOL)
}

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

    for device in devices.filter(|device| {
        device.vendor_id() == DJI_VENDOR_ID && device.product_id() == GOGGLES_PRODUCT_ID
    }) {
        if !serial_matches(&device, serial)? {
            continue;
        }
        goggles_seen += 1;

        for interface in device
            .interfaces()
            .filter(|i| is_video_interface(i.class(), i.subclass(), i.protocol()))
        {
            candidates.push(Candidate {
                device: device.clone(),
                interface_number: interface.interface_number(),
            });
        }
    }

    match candidates.len() {
        0 if goggles_seen == 0 => {
            let selection = serial.map(|s| format!(" with USB serial {s}")).unwrap_or_default();
            Err(format!("no DJI Goggles 3 USB device{selection} found"))
        }
        0 => Err("Goggles found, but without the tinkerglass video interface (not in tinkerglass USB mode)".into()),
        1 => Ok(candidates),
        count => Err(format!("found {count} tinkerglass video interfaces; specify --serial to select one")),
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
    // Only this interface is opened; the control interfaces belong to other programs.
    let interface = device
        .claim_interface(candidate.interface_number)
        .wait()
        .map_err(|e| {
            let busy = if claim_denied(&e) {
                "; the video interface is held by another program (a second eyepiece-pc, or another \
                 USB client): close it and try again"
            } else {
                ""
            };
            format!(
                "cannot claim the video interface {} ({e}){busy}",
                candidate.interface_number
            )
        })?;

    Ok(UsbDevice {
        device,
        interface,
        endpoints,
    })
}

/// Claiming an interface another process holds: access denied (Windows), busy (Linux, macOS).
fn claim_denied(error: &nusb::Error) -> bool {
    use nusb::ErrorKind;
    matches!(error.kind(), ErrorKind::Busy | ErrorKind::PermissionDenied)
}

fn bulk_endpoints(
    config: &ConfigurationDescriptor<'_>,
    interface_number: u8,
) -> Result<BulkEndpoints, String> {
    let mut descriptors = config.interface_alt_settings().filter(|descriptor| {
        descriptor.interface_number() == interface_number && descriptor.alternate_setting() == 0
    });
    let descriptor = descriptors.next().ok_or_else(|| {
        format!("video interface {interface_number} is missing alternate setting 0")
    })?;
    if descriptors.next().is_some() {
        return Err(format!(
            "video interface {interface_number} has duplicate alternate setting 0"
        ));
    }
    if !is_video_interface(descriptor.class(), descriptor.subclass(), descriptor.protocol())
        || descriptor.num_endpoints() != 2
    {
        return Err(format!(
            "video interface {interface_number} has unexpected class or endpoint count"
        ));
    }

    // Endpoints are found by direction and type, never by address.
    let mut input = None;
    let mut output = None;
    for endpoint in descriptor.endpoints() {
        if endpoint.transfer_type() != TransferType::Bulk || endpoint.max_packet_size() == 0 {
            return Err(format!(
                "video interface {interface_number} has an invalid endpoint descriptor"
            ));
        }
        let value = (endpoint.address(), endpoint.max_packet_size());
        let slot = match endpoint.direction() {
            Direction::In => &mut input,
            Direction::Out => &mut output,
        };
        if slot.replace(value).is_some() {
            return Err(format!(
                "video interface {interface_number} has duplicate endpoint directions"
            ));
        }
    }
    let (in_address, in_max_packet_size) = input
        .ok_or_else(|| format!("video interface {interface_number} has no bulk IN endpoint"))?;
    let (out_address, out_max_packet_size) = output
        .ok_or_else(|| format!("video interface {interface_number} has no bulk OUT endpoint"))?;

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

    /// Interface 9: ff/54/03, OUT 0x01 then IN 0x82, 512-byte packets (high speed).
    fn config() -> ConfigurationDescriptor<'static> {
        const BYTES: &[u8] = &[
            9, 2, 32, 0, 1, 1, 0, 0x80, 50, 9, 4, 9, 0, 2, 0xff, 0x54, 0x03, 1, 7, 5, 0x01, 2, 0,
            2, 0, 7, 5, 0x82, 2, 0, 2, 0,
        ];
        ConfigurationDescriptor::new(BYTES).unwrap()
    }

    #[test]
    fn matches_by_class_subclass_protocol() {
        assert!(is_video_interface(0xff, 0x54, 0x03));
        // the control interfaces of the same mode, and the natartos-era layout
        assert!(!is_video_interface(0xff, 0x54, 0x01));
        assert!(!is_video_interface(0xff, 0x54, 0x02));
        assert!(!is_video_interface(0xff, 0x00, 0x00));
        assert!(!is_video_interface(0x08, 0x54, 0x03));
    }

    #[test]
    fn discovers_bulk_pair_by_direction() {
        let endpoints = bulk_endpoints(&config(), 9).unwrap();
        assert_eq!(
            endpoints,
            BulkEndpoints {
                interface_number: 9,
                in_address: 0x82,
                in_max_packet_size: 512,
                out_address: 0x01,
                out_max_packet_size: 512,
            }
        );
    }

    #[test]
    fn endpoint_order_and_addresses_do_not_matter() {
        let mut bytes = config().as_bytes().to_vec();
        // IN first at 0x85, OUT second at 0x04
        bytes[20] = 0x85;
        bytes[27] = 0x04;
        let config = ConfigurationDescriptor::new(&bytes).unwrap();
        let endpoints = bulk_endpoints(&config, 9).unwrap();
        assert_eq!((endpoints.in_address, endpoints.out_address), (0x85, 0x04));
    }

    #[test]
    fn rejects_other_tinkerglass_interfaces() {
        for protocol in [1u8, 2] {
            let mut bytes = config().as_bytes().to_vec();
            bytes[16] = protocol;
            let config = ConfigurationDescriptor::new(&bytes).unwrap();
            assert!(bulk_endpoints(&config, 9).is_err());
        }
    }

    #[test]
    fn rejects_non_vendor_interface() {
        let mut bytes = config().as_bytes().to_vec();
        bytes[14] = 2;
        let config = ConfigurationDescriptor::new(&bytes).unwrap();
        assert!(bulk_endpoints(&config, 9).is_err());
    }

    #[test]
    fn rejects_non_bulk_endpoint() {
        let mut bytes = config().as_bytes().to_vec();
        bytes[21] = 3;
        let config = ConfigurationDescriptor::new(&bytes).unwrap();
        assert!(bulk_endpoints(&config, 9).is_err());
    }
}
