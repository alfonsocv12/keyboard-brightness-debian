use std::{fs, path::Path, process::Command};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Monitor {
    device_id: String,
    brightness_bin_location: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MonitorBrightness {
    monitors: Vec<Monitor>,
}

impl MonitorBrightness {
    const FILE_NAME: &'static str = "monitors.cbor";

    pub fn new() -> MonitorBrightness {
        let (monitor_brightness, file_exists) = Self::load_from_file();

        if file_exists {
            return monitor_brightness;
        }

        let ddccontrol_desc  = Command::new("ddccontrol")
                .arg("-p")
                .output()
                .expect("Failed to get monitor list");

        let ddccontrol_desc_str = String::from_utf8_lossy(&ddccontrol_desc.stdout);
       
        // Remove header
        let ddccontrol_desc_str = ddccontrol_desc_str.split("Detected monitors :").collect::<Vec<&str>>()[1];        
        let ddccontrol_desc_arr = ddccontrol_desc_str.split("Reading EDID").collect::<Vec<&str>>();

        // Get Monitor list
        monitor_brightness.get_monitors_from_str(ddccontrol_desc_arr[0]);
 
        return monitor_brightness; 
    }

    fn get_monitors_from_str(&self, monitor_list: &str) -> Vec<Monitor> {
        let devices = monitor_list.split("- Device").collect::<Vec<&str>>();

        println!("{:?}", devices);

        return Vec::new();
    }

    fn load_from_file() -> (MonitorBrightness, bool) {
        let default = MonitorBrightness {
            monitors: Vec::new(),
        }; 

        if !Path::new("monitors.cbor").exists() {
            return (default, false);
        }
        
        let class_cbor_bytes = fs::File::open(Self::FILE_NAME).expect("Failed to open file");
        let monitor_brightness: MonitorBrightness = serde_cbor::from_reader(class_cbor_bytes);

        return (monitor_brightness, true);
    }

    fn save_to_file(&self) {
        let class_file = File::create(Self::FILE_NAME).expect("Failed to create file");

        let class_postcard_bytes = serde_cbor::to_writer(class_file, &self);
    }
}
