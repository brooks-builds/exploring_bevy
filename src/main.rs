use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sprite = asset_server.load("item1BIT_gem.png").unwrap();
    let mut scale = Scale::identity();
    scale.0 = 4.0;
    let font_handle = asset_server
        .load("ttf/FiraCode-Regular.ttf".to_owned())
        .unwrap();

    commands
        .spawn(UiCameraComponents::default())
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(sprite.into()),
            scale,
            ..Default::default()
        })
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "Hello World!".to_owned(),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                font: font_handle,
            },
            ..Default::default()
        });
}
