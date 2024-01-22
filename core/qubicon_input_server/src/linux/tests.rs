use crate::device_manager::DeviceManager;

#[test]
fn test_device_manager() {
    let mut manager = DeviceManager::new();

    loop {
        manager.update_device_list();
    }
}