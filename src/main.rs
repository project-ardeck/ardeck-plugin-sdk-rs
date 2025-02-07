mod client;

#[tokio::main]
async fn main() {
    let mut plugin = client::ardeck_plugin::ArdeckPlugin::new().await;

    // TODO: event handlerをactionとmessageに分ける
    plugin.add_action_handler("hello", |msg| {
        println!("Hello! plugin: {:?}", msg);
    }).await;

    plugin.add_action_handler("ping", |msg| {
        println!("pong from plugin");
    }).await;

    plugin.start_listening().await;
}
