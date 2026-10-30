use android_activity::Activity;

#[no_mangle]
pub extern "C" fn android_main(activity: Activity) {
    // This would normally start the app, but for now we just keep the activity alive
    std::thread::spawn(move || {
        // In a real app, this would run the eframe event loop
        // For now, just keep the thread alive
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}