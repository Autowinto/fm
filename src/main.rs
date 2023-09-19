use bevy::prelude::*;
use bevy_egui::{
    egui::{self, style::default_text_styles},
    EguiContexts, EguiPlugin,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GTNH Production Planner".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        // Debug Utils
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, initialize)
        .add_systems(Update, ui)
        .run();
}

fn initialize(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn ui(commands: Commands, mut ctxs: EguiContexts) {
    egui::Window::new("Hello").show(ctxs.ctx_mut(), |ui| {
        ui.label("world");
        if ui.button("Spawn Process").clicked() {
            spawn_node(commands)
        }
    });
}

#[derive(Component)]
struct ProductionNode {
    pub title: String,
}

impl Default for ProductionNode {
    fn default() -> Self {
        Self {
            title: "New Process Step".to_string(),
        }
    }
}

fn spawn_node(mut commands: Commands) {
    let text_style = TextStyle { ..default() };
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(200.0, 200.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec2::new(0.0, 0.0).extend(0.0)),
                ..default()
            },
            ProductionNode { ..default() },
        ))
        .with_children(|builder| {
            builder
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.25, 0.2),
                        custom_size: Some(Vec2::new(200.0, 20.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 90.0, Vec3::Z.z)),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(Text2dBundle {
                        text: Text::from_section("Test", text_style.clone()),
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    });
                });
        });
}
