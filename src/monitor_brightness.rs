use std::process::Command;

struct Monitor {
    device_id: String,
    brightness_bin_location: String,
}

pub(crate) struct MonitorBrightness {
    monitors: Vec<Monitor>,
}

impl MonitorBrightness {
    
    pub fn new() -> MonitorBrightness {
        let monitor_brightness = MonitorBrightness {
            monitors: Vec::new(),
        };

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
}
