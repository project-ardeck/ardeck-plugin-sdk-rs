use plugin_lib::ardeck_plugin::ArdeckPlugin;

#[tokio::main]
async fn main() {
    let mut plugin = ArdeckPlugin::new().await;

    plugin.add_action_handler("hello", |action| {
        println!("Hello Ardeck!");
    }).await;

    plugin.start_listening().await;
}
