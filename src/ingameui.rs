use crate::ressources;
use bevy::prelude::*;
use ressources::MudRessource;
use ressources::StoneRessource;

#[derive(Component)]
pub struct StoneCountUi {}
#[derive(Component)]
pub struct MudCountUi {}

pub fn spawn_ingaem_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::GRAY.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Stone: X",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                StoneCountUi {},
            ));

            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Mud: X",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                MudCountUi {},
            ));
        });
}

pub fn update_stone_ui(
    mut texts: Query<&mut Text, With<StoneCountUi>>,
    stone: Res<StoneRessource>,
) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Stone: {}", stone.value);
    }
}
pub fn update_mud_ui(mut texts: Query<&mut Text, With<MudCountUi>>, mud: Res<MudRessource>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Mud: {}", mud.value);
    }
}
