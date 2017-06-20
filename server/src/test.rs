use serde_json;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Distribution {
    pub codename: String,
    pub description: String,
    pub distributor_id: String,
    pub release: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Result {
    pub id: String,
    pub name: String,
    pub certification_status: String,
    pub category: String,
    pub status: String,
    pub comments: Option<String>,
    pub io_log: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub project: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub bus: Option<String>,
    pub category: Option<String>,
    pub driver: Option<String>,
    pub interface: Option<String>,
    pub mac: Option<String>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub product: Option<String>,
    pub product_id: Option<u16>,
    pub product_slug: Option<String>,
    pub subproduct_id: Option<u16>,
    pub subvendor_id: Option<u16>,
    pub vendor: Option<String>,
    pub vendor_id: Option<u16>,
    pub vendor_slug: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModprobeInfo {
    module: String,
    options: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Processor {
    pub bogomips: String,
    pub cache: String,
    pub count: String,
    pub model: String,
    pub model_number: String,
    pub model_revision: String,
    pub model_version: String,
    pub other: String,
    pub platform: String,
    pub speed: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Test {
    pub packages: Option<Vec<Package>>,
    pub distribution: Option<Distribution>,
    pub results: Option<Vec<Result>>,
    //pub dkms_info: DkmsInfo
    pub devices: Option<Vec<Device>>,
    //pub raw_devices_dmi: Vec<RawDevicesDmi>
    #[serde(rename = "modprobe-info")]
    pub modprobe_info: Option<Vec<ModprobeInfo>>,
    pub pci_subsystem_id: Option<String>,
    pub kernel: Option<String>,
    pub architecture: Option<String>,
    pub processor: Option<Processor>,
}

impl Test {
    fn merge(tests: Vec<Test>) -> io::Result<Test> {
        let mut merged = Test::default();

        for test in tests {
            macro_rules! merge {
                ($key:ident) => {{
                    if let Some($key) = test.$key {
                        if merged.$key.is_some() {
                            return Err(io::Error::new(io::ErrorKind::Other, format!("{} already exists", stringify!($key))));
                        }
                        merged.$key = Some($key);
                    }
                }}
            }

            merge!(packages);
            merge!(distribution);
            merge!(results);
            merge!(devices);
            merge!(modprobe_info);
            merge!(pci_subsystem_id);
            merge!(kernel);
            merge!(architecture);
            merge!(processor);
        }

        Ok(merged)
    }

    pub fn from_str(string: &str) -> io::Result<Test> {
        match serde_json::from_str::<Test>(&string) {
            Ok(test) => Ok(test),
            Err(_) => match serde_json::from_str::<Vec<Test>>(&string) {
                Ok(values) => Test::merge(values),
                Err(error) => Err(io::Error::new(io::ErrorKind::Other, format!("{}", error)))
            }
        }
    }
}
