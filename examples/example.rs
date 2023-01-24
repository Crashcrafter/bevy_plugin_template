use bevy_app::App;
use bevy_plugin_template::MyPlugin;

fn main() {
    App::new()
        .add_plugin(MyPlugin)
        .run();
}