use crate::model::CrashList;
use crate::model::OperatingSystemCount;
use std::collections::HashMap;

pub struct Report {
    pub version: String,
    pub crash_list: CrashList,
}

impl Report {
    pub fn new(version: String, crash_list: CrashList) -> Report {
        Report {
            version,
            crash_list,
        }
    }

    pub fn assign_operating_system_details(
        &mut self,
        operating_systems: Option<HashMap<String, Vec<OperatingSystemCount>>>,
    ) {
        if let Some(operating_systems_hash) = operating_systems {
            for crash in &mut self.crash_list.crashes {
                if let Some(group_id) = &crash.error_group_id {
                    if let Some(oses) = operating_systems_hash.get(group_id) {
                        let os_distribution: Vec<OperatingSystemCount> = oses.to_vec();
                        crash.assign_operating_system_crash_distribution(os_distribution);
                    }
                }
            }
        }
    }
}
