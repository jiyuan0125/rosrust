fn main() {
    env_logger::init();

    // Initialize node
    rosrust::init_with_master_uri_and_hostname_and_slave_port(
        "listener",
        "http://localhost:11311",
        "localhost",
        5557,
    );

    // Create subscriber
    // The subscriber is stopped when the returned object is destroyed
    let subscriber_info = rosrust::subscribe("chatter", 2, |v: rosrust_msg::std_msgs::String| {
        // Callback for handling received messages
        rosrust::ros_info!("Received: {}", v.data);
    })
    .unwrap();

    let log_names = rosrust::param("~log_names").unwrap().get().unwrap_or(false);

    if log_names {
        let rate = rosrust::rate(1.0);
        while rosrust::is_ok() {
            rosrust::ros_info!("Publisher uris: {:?}", subscriber_info.publisher_uris());
            rate.sleep();
        }
    } else {
        // Block the thread until a shutdown signal is received
        rosrust::spin();
    }
}
