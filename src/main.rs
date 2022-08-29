mod cursor;

use std::ops::ControlFlow;

use bevy::{
    input::mouse::{MouseMotion},
    prelude::*,
};
use bevy_easings::Lerp;
use bevy_prototype_lyon::{prelude::*};
use cursor::{Cursor, CursorPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Paused(bool);
struct CorePlugins;
#[derive(Component, Clone, Debug, Default)]
struct Orb {
    damage: i8,
    health: i8,
    children: Vec<Orb>,
}
#[derive(Component)]
struct PlayerOrb;
#[derive(Component)]
struct EnemyOrb;

struct ChangeLevel {
    fail: bool,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 1.0)))
        .add_event::<ChangeLevel>()
        .insert_resource(ClosestCircle { data: None })
        .insert_resource(Phase::PREP)
        .insert_resource(CombatStep::LineUp)
        .insert_resource(CurrentLevel(0))
        .insert_resource(TextDetails {
            text_alignment: None,
            text_style: None,
        })
        .add_startup_system(setup_text_details)
        .add_plugins(CorePlugins)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_system(switch_phase_listener)
        .add_startup_system(setup_camera)
        .add_startup_system(setup)
        .add_system(change_level)
        .add_system(modify_camera_scale)
        .add_system(update)
        .add_system(combat_update)
        .add_system(update_level_display)
        .add_startup_system(setup_ui.after(setup_text_details))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection::default(),
        ..default()
    });
}

#[derive(Clone)]
struct TextDetails {
    text_style: Option<TextStyle>,
    text_alignment: Option<TextAlignment>,
}

fn setup_ui(mut commands: Commands, text_details: Res<TextDetails>) {
    let existing_style = text_details.text_style.clone().unwrap();
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "\n",
                    TextStyle {
                        font: existing_style.font.clone(),
                        font_size: 30.,
                        color: Color::BLACK,
                    },
                ),
                TextSection::new(
                "Welcome to Orber! \n\nControls: \n - press SPACE to initiate combat \n - drag orbs over each-other to combine \n - right click an orb to split it",
                TextStyle {
                    font: existing_style.font.clone(),
                    font_size: 30.,
                    color: Color::BLACK,
                },
            ), 
                ])
            .with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }),
        )
        .insert(UiText);
}

fn update_level_display(mut ui_text_query: Query<&mut Text, With<UiText>>, current_level: Res<CurrentLevel>) {
    let mut text = ui_text_query.single_mut();
    if current_level.0 == 5 {
        text.sections[0].value = format!("Game complete! Well done!! | you can press SPACE to restart :)\n\n");
        text.sections[1].value = format!("Enjoy the rest of the competition. \nThis game was made in a few days using the Bevy game engine.\n\n Good luck to all the other contestants! <3 \n -jake");
    } else {
        text.sections[0].value = format!("Current Level: {} \n\n", current_level.0);
        text.sections[1].value = format!("Welcome to Orber! \n\nControls: \n - press SPACE to initiate combat \n - drag orbs over each-other to combine \n - right click an orb to split it");

    }
}

#[derive(Component)]
struct UiText;

fn setup_text_details(mut text_details: ResMut<TextDetails>, asset_server: Res<AssetServer>) {
    let text_alignment = TextAlignment::CENTER;

    text_details.text_style = Some(TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 20.0,
        color: Color::BLACK,
    });
    text_details.text_alignment = Some(text_alignment);
}

fn change_level(
    mut change_level_reader: EventReader<ChangeLevel>,
    mut current_level: ResMut<CurrentLevel>,
    mut commands: Commands,
    // current entities
    orb_query: Query<Entity, With<Orb>>,
    text_details: Res<TextDetails>,
) {
    for ev in change_level_reader.iter() {
        if ev.fail {
            current_level.0 -= 1;
        }
        // despawn current
        orb_query.iter().for_each(|e| {
            commands.entity(e).despawn_recursive();
        });

        let mut player_orb_options = vec![];

        let mut enemy_orbs = vec![];

        match current_level.0 {
            0 => {
                player_orb_options.push(Orb {
                    health: 1,
                    damage: 2,
                    ..default()
                });
                enemy_orbs.push(Orb {
                    health: 2,
                    damage: 1,
                    ..default()
                });
            }
            1 => {
                player_orb_options.push(Orb {
                    health: 4,
                    damage: 1,
                    ..default()
                });
                player_orb_options.push(Orb {
                    health: 1,
                    damage: 4,
                    ..default()
                });

                enemy_orbs.push(Orb {
                    health: 8,
                    damage: 1,
                    ..default()
                });
                enemy_orbs.push(Orb {
                    health: 1,
                    damage: 3,
                    ..default()
                });
            }
            2 => {
                player_orb_options.push(Orb {
                    health: 1,
                    damage: 1,
                    ..default()
                });
                player_orb_options.push(Orb {
                    health: 1,
                    damage: 1,
                    ..default()
                });
                player_orb_options.push(Orb {
                    health: 1,
                    damage: 1,
                    ..default()
                });

                player_orb_options.push(Orb {
                    health: 1,
                    damage: 1,
                    ..default()
                });

                enemy_orbs.push(Orb {
                    health: 1,
                    damage: 3,
                    ..default()
                });
                enemy_orbs.push(Orb {
                    health: 3,
                    damage: 3,
                    ..default()
                });
            }
            3 => {
                player_orb_options.push(Orb {
                    health: 3,
                    damage: 2,
                    ..default()
                });
                player_orb_options.push(Orb {
                    health: -2,
                    damage: 3,
                    ..default()
                });

                enemy_orbs.push(Orb {
                    health: 5,
                    damage: 3,
                    ..default()
                });
            }
            _ => {}
        }

        for (i, orb) in player_orb_options.iter().enumerate() {
            let position = Vec3::new(-150.0 - i as f32 * 80., -200.0, 0.0);
            spawn_orb(&mut commands, position, orb, text_details.clone(), false);
        }

        for (i, orb) in enemy_orbs.iter().enumerate() {
            let position = Vec3::new(150.0 + i as f32 * 120., 0.0, 0.0);
            spawn_orb(&mut commands, position, orb, text_details.clone(), true);
        }

        // finally change the internal level counter
        current_level.0 += 1;
    }
}

#[derive(Component)]
struct HealthDisplay;
#[derive(Component)]
struct DamageDisplay;

fn spawn_orb(
    commands: &mut Commands,
    position: Vec3,
    orb: &Orb,
    text_details: TextDetails,
    is_enemy: bool,
) {
    let parent = if is_enemy {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    center: Vec2::new(0., 0.),
                    radius: 50.,
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::RED),
                    outline_mode: StrokeMode::new(Color::BLACK, 5.0),
                },
                Transform {
                    translation: position,
                    ..default()
                },
            ))
            .insert(orb.clone())
            .insert(EnemyOrb)
            .id()
    } else {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    center: Vec2::new(0., 0.),
                    radius: 30.,
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::rgb(0.4, 0.4, 0.7)),
                    outline_mode: StrokeMode::new(Color::BLACK, 5.0),
                },
                Transform {
                    translation: position,
                    ..default()
                },
            ))
            .insert(orb.clone())
            .insert(PlayerOrb)
            .id()
    };

    let health_child = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                center: Vec2::new(0., 0.),
                radius: 12.,
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::RED),
                outline_mode: StrokeMode::new(Color::BLACK, 5.0),
            },
            Transform {
                translation: if is_enemy {
                    Vec3::new(34., -34., 0.1)
                } else {
                    Vec3::new(22., -22., 0.1)
                },
                ..default()
            },
        ))
        .insert(HealthDisplay)
        .id();
    let damage_child = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                center: Vec2::new(0., 0.),
                radius: 12.,
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::ALICE_BLUE),
                outline_mode: StrokeMode::new(Color::BLACK, 5.0),
            },
            Transform {
                translation: if is_enemy {
                    Vec3::new(-34., -34., 0.1)
                } else {
                    Vec3::new(-22., -22., 0.1)
                },
                ..default()
            },
        ))
        .insert(DamageDisplay)
        .id();
    let health_text = commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                format!("{}", orb.health),
                text_details.text_style.clone().unwrap(),
            )
            .with_alignment(text_details.text_alignment.unwrap()),
            transform: Transform {
                translation: Vec3::new(0., 0., 5.),
                ..default()
            },
            ..default()
        })
        .id();
    commands.entity(health_child).add_child(health_text);
    let damage_text = commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                format!("{}", orb.damage),
                text_details.text_style.clone().unwrap(),
            )
            .with_alignment(text_details.text_alignment.unwrap()),
            transform: Transform {
                translation: Vec3::new(0., 0., 5.),
                ..default()
            },
            ..default()
        })
        .id();
    commands.entity(damage_child).add_child(damage_text);
    commands.entity(parent).add_child(health_child);
    commands.entity(parent).add_child(damage_child);
}

fn setup(mut change_level_writer: EventWriter<ChangeLevel>) {
    change_level_writer.send(ChangeLevel { fail: false });
}

#[derive(Debug)]
struct ClosestCircle {
    data: Option<ClosestCircleData>,
}

#[derive(Debug)]
struct ClosestCircleData {
    entity: Entity,
    distance: f32,
    orb: Orb,
}

#[derive(PartialEq, Default, Debug)]
enum Phase {
    #[default]
    PREP,
    COMBAT,
}

#[derive(PartialEq, Default, Debug)]
enum CombatStep {
    #[default]
    LineUp,
    Attack,
    Next,
}

struct CurrentLevel(i8);

fn update(
    mut player_orb_query: Query<(&mut Transform, &mut DrawMode, Entity, &Orb), With<PlayerOrb>>,
    cursor: Res<Cursor>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut closest_circle: ResMut<ClosestCircle>,
    mut commands: Commands,
    phase: Res<Phase>,
    text_details: Res<TextDetails>,
) {
    let prep_mode = *phase == Phase::PREP;

    let mut distance_to_each_entity: Vec<ClosestCircleData> = vec![];

    for (transform, _, entity, orb) in player_orb_query.iter_mut() {
        let center = transform.translation.truncate();

        let mouse_distance_to_center = cursor.0.distance(center);
        distance_to_each_entity.push(ClosestCircleData {
            entity,
            distance: mouse_distance_to_center,
            orb: orb.clone(),
        })
    }

    distance_to_each_entity.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    let closest = distance_to_each_entity.first();

    if !mouse_buttons.pressed(MouseButton::Left) && prep_mode {
        for (transform, _, entity, orb) in player_orb_query.iter_mut() {
            if let Some(closest) = closest {
                let center = transform.translation.truncate();

                let mouse_distance_to_center = cursor.0.distance(center);

                let radius = 30.0;
                if closest.entity == entity {
                    if mouse_distance_to_center < radius {
                        closest_circle.data = Some(ClosestCircleData {
                            entity,
                            distance: mouse_distance_to_center,
                            orb: orb.clone(),
                        })
                    } else {
                        closest_circle.data = None;
                    }
                } else {
                    if mouse_buttons.just_released(MouseButton::Left) && prep_mode {
                        if mouse_distance_to_center < 30.0 {
                            commands.entity(entity).despawn_recursive();
                            commands.entity(closest.entity).despawn_recursive();
                            let new_orb = Orb {
                                damage: closest.orb.damage + orb.damage,
                                health: closest.orb.health + orb.health,
                                children: vec![orb.clone(), closest.orb.clone()],
                            };

                            spawn_combination_orb(
                                &mut commands,
                                &cursor,
                                new_orb,
                                text_details.clone(),
                            );

                            closest_circle.data = None;
                            return;
                        }
                    }
                }
            }
        }
    }

    for (mut t, mut d, entity, orb) in player_orb_query.iter_mut() {
        if let Some(closest) = &closest_circle.data {
            if closest.entity == entity {
                if let DrawMode::Outlined {
                    ref mut fill_mode,
                    outline_mode: _,
                } = *d
                {
                    modify_color(fill_mode, 0.6, 0.6, 1.0, 0.05)
                }

                t.translation.z = 1.0;

                follow_mouse(
                    &mouse_buttons,
                    &mut t,
                    &cursor,
                    &mut mouse_motion_event_reader,
                    &prep_mode,
                );

                if mouse_buttons.just_pressed(MouseButton::Right) && prep_mode {
                    if let ControlFlow::Break(_) =
                        split_orb(orb, &mut commands, &t, entity, &mut closest_circle, text_details.clone())
                    {
                        return;
                    }
                }

                if prep_mode {
                    continue;
                }
            }
        }

        if let DrawMode::Outlined {
            ref mut fill_mode,
            outline_mode: _,
        } = *d
        {
            modify_color(fill_mode, 0.4, 0.4, 0.7, 0.1)
        }

        t.translation.z = 0.0;
    }

    let mut iter = player_orb_query.iter_combinations_mut();
    while let Some([(t1, _, e1, _), (t2, _, e2, _)]) = iter.fetch_next() {
        if let Some(closest) = closest {
            if (closest.entity == e1 || closest.entity == e2)
                && mouse_buttons.pressed(MouseButton::Left)
                && prep_mode
            {
                continue;
            }
        }

        push_apart(t2, t1);
    }

    if mouse_buttons.pressed(MouseButton::Left) && prep_mode {
        let mut combine_ready = false;
        for (t, mut d, entity, _) in player_orb_query.iter_mut() {
            if let Some(data) = &closest {
                if data.entity != entity {
                    let center = t.translation.truncate();
                    let mouse_distance_to_center = cursor.0.distance(center);

                    if mouse_distance_to_center <= 30.0 {
                        combine_ready = true;
                        brighten(&mut d);
                        increase_line_width(&mut d);
                    } else {
                        decrease_line_width(&mut d);
                    }
                }
            }
        }
        for (_, mut d, entity, _) in player_orb_query.iter_mut() {
            if let Some(data) = &closest {
                if data.entity == entity {
                    if combine_ready {
                        if let DrawMode::Outlined {
                            ref mut fill_mode,
                            outline_mode: _,
                        } = *d
                        {
                            modify_color(fill_mode, 0.8, 0.8, 1.0, 0.05);
                            increase_line_width(&mut d);
                        }
                    } else {
                        if let DrawMode::Outlined {
                            fill_mode: _,
                            outline_mode: _,
                        } = *d
                        {
                            decrease_line_width(&mut d);
                        }
                    }
                }
            }
        }
    } else {
        for (_, mut d, _, _) in player_orb_query.iter_mut() {
            if !prep_mode {
                decrease_line_width(&mut d)
            }
        }
    }
}

fn spawn_combination_orb(
    commands: &mut Commands,
    cursor: &Res<Cursor>,
    new_orb: Orb,
    text_details: TextDetails,
) {
    spawn_orb(
        commands,
        cursor.0.extend(0.0),
        &new_orb,
        text_details,
        false,
    )
}

fn combat_update(
    mut phase: ResMut<Phase>,
    mut combat_step: ResMut<CombatStep>,
    mut player_orb_query: Query<
        (
            &mut Transform,
            &mut DrawMode,
            Entity,
            &mut Orb,
            Option<&Children>,
        ),
        (With<PlayerOrb>, Without<EnemyOrb>),
    >,
    mut enemy_orb_query: Query<
        (
            &mut Transform,
            &mut DrawMode,
            Entity,
            &mut Orb,
            Option<&Children>,
        ),
        (With<EnemyOrb>, Without<PlayerOrb>),
    >,
    mut commands: Commands,
    mut change_level_writer: EventWriter<ChangeLevel>,
    health_display_query: Query<(&Parent, &Children), With<HealthDisplay>>,
    mut text_query: Query<(&mut Text, &Parent), With<Text>>,
) {
    if *phase == Phase::COMBAT {
        if *combat_step == CombatStep::LineUp {
            let mut completed_orbs: Vec<bool> = vec![];
            for (i, (mut t, _, _, _, _)) in player_orb_query.iter_mut().enumerate() {
                t.translation.y = t.translation.y.lerp(&0., &0.2);
                t.translation.x = t.translation.x.lerp(&(-150.0 - i as f32 * 120.), &0.2);

                if t.translation.y.abs() < 0.1
                    && (t.translation.x - (-150.0 - i as f32 * 120.)).abs() < 0.1
                {
                    completed_orbs.push(true)
                } else {
                    completed_orbs.push(false)
                }
            }
            if completed_orbs.iter().all(|o| *o) {
                *combat_step = CombatStep::Attack;
            }
        }

        if *combat_step == CombatStep::Attack {
            let mut all_player_orbs: Vec<(Entity, f32)> = vec![];
            for (t, _, e, _, _) in player_orb_query.iter() {
                all_player_orbs.push((e, t.translation.x));
            }

            let mut closest_enemy_orb: Option<(Entity, f32)> = None;
            for (t, _, e, _, _) in enemy_orb_query.iter() {
                if let Some((_, trans_x)) = closest_enemy_orb {
                    if t.translation.x < trans_x {
                        closest_enemy_orb = Some((e, t.translation.x));
                    }
                } else {
                    closest_enemy_orb = Some((e, t.translation.x))
                }
            }

            let mut player_reached = false;
            let max_player_orb = all_player_orbs.iter().max_by(|a, b| a.1.total_cmp(&b.1));

            if !player_reached {
                for (mut t, _, e, _, _) in player_orb_query.iter_mut() {
                    if let Some(max) = max_player_orb {
                        if e == max.0 {
                            let target_x = -30.;
                            let v = t.translation.x - target_x;

                            t.translation.x -= v / 5.;

                            if v <= 0.5 {
                                player_reached = true;
                            }
                        }
                    }
                }
            }

            let mut enemy_reached = false;
            if !enemy_reached {
                for (mut t, _, e, _, _) in enemy_orb_query.iter_mut() {
                    if let Some((ent, _)) = closest_enemy_orb {
                        if e == ent {
                            let target_x = 50.;
                            let v = t.translation.x - target_x;

                            t.translation.x -= v / 5.;

                            if v <= 0.5 {
                                enemy_reached = true;
                            }
                        }
                    }
                }
            }

            if enemy_reached && player_reached {
                if let Some(player_orb) = max_player_orb {
                    if let Some(enemy_orb) = closest_enemy_orb {
                        let (_, _, player_ent, mut player_orb, player_children) =
                            player_orb_query.get_mut(player_orb.0).unwrap();

                        let (_, _, enemy_ent, mut enemy_orb, enemy_children) =
                            enemy_orb_query.get_mut(enemy_orb.0).unwrap();

                        enemy_orb.health -= player_orb.damage;
                        player_orb.health -= enemy_orb.damage;

                        if let Some(children) = enemy_children {
                            for &child in children.iter() {
                                let health_display = health_display_query.get(child);

                                if let Ok((_, health_c)) = health_display {
                                    for &child in health_c.iter() {
                                        let text = text_query.get_mut(child);

                                        if let Ok((mut text, _)) = text {
                                            text.sections[0].value =
                                                format!("{}", enemy_orb.health);
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(children) = player_children {
                            for &child in children.iter() {
                                let health_display = health_display_query.get(child);

                                if let Ok((_, health_c)) = health_display {
                                    for &child in health_c.iter() {
                                        let text = text_query.get_mut(child);

                                        if let Ok((mut text, _)) = text {
                                            text.sections[0].value =
                                                format!("{}", player_orb.health);
                                        }
                                    }
                                }
                            }
                        }

                        if player_orb.health <= 0 {
                            commands.entity(player_ent).despawn_recursive();
                        }
                        if enemy_orb.health <= 0 {
                            commands.entity(enemy_ent).despawn_recursive();
                        }
                        *combat_step = CombatStep::Next;
                    }
                }
            }

            let enemies_still_alive = closest_enemy_orb.is_some();
            let players_still_alive = max_player_orb.is_some();

            if !enemies_still_alive || !players_still_alive {
                change_level_writer.send(ChangeLevel {
                    fail: enemies_still_alive,
                });
                *combat_step = CombatStep::LineUp;
                *phase = Phase::PREP;
            }
        }

        if *combat_step == CombatStep::Next {
            let mut completed_orbs: Vec<bool> = vec![];

            let mut all_enemy_orbs: Vec<(Entity, f32)> = vec![];
            let mut all_player_orbs: Vec<(Entity, f32)> = vec![];

            for (t, _, e, _, _) in player_orb_query.iter() {
                all_player_orbs.push((e, t.translation.x));
            }

            for (t, _, e, _, _) in enemy_orb_query.iter() {
                all_enemy_orbs.push((e, t.translation.x));
            }

            let max_player_orb = all_player_orbs.iter().max_by(|a, b| a.1.total_cmp(&b.1));
            let max_enemy_orb = all_enemy_orbs.iter().max_by(|b, a| a.1.total_cmp(&b.1));

            for (i, (mut t, _, e, _, _)) in player_orb_query.iter_mut().enumerate() {
                if let Some(max) = max_player_orb {
                    if e != max.0 {
                        continue;
                    }
                }
                let target_x = -150.0 - (i as f32 * 80.);

                t.translation.x -= (t.translation.x - target_x) / 10.0;

                if (t.translation.x - target_x).abs() <= 2.0 {
                    completed_orbs.push(true)
                } else {
                    completed_orbs.push(false)
                }
            }

            for (i, (mut t, _, e, _, _)) in enemy_orb_query.iter_mut().enumerate() {
                if let Some(max) = max_enemy_orb {
                    if e != max.0 {
                        continue;
                    }
                }
                let target_x = 150.0 + (i as f32 * 120.);
                t.translation.x -= (t.translation.x - target_x) / 10.0;
                let diff = t.translation.x - target_x;

                if (diff).abs() <= 2.0 {
                    completed_orbs.push(true)
                } else {
                    completed_orbs.push(false)
                }
            }

            if completed_orbs.iter().all(|o| *o) {
                *combat_step = CombatStep::Attack;
            }
        }
    }
}

fn modify_camera_scale(
    phase: Res<Phase>,
    mut camera: Query<&mut Transform, With<Camera>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let (w, h) = (window.width(), window.height());
    let default_w = 1280.0;
    let default_h = 720.0;

    let mut camera_transform = camera.single_mut();
    let relative_to_default: Vec2 = Vec2::new(default_w / w, default_h / h);
    let zoom_level = if *phase == Phase::COMBAT { 0.6 } else { 1.0 };

    camera_transform.scale.x = camera_transform
        .scale
        .x
        .lerp(&(relative_to_default.x * zoom_level), &0.2);
    camera_transform.scale.y = camera_transform
        .scale
        .y
        .lerp(&(relative_to_default.y * zoom_level), &0.2);
}

fn switch_phase_listener(
    mut phase: ResMut<Phase>,
    input: Res<Input<KeyCode>>,
    mut combat_step: ResMut<CombatStep>,
    mut current_level: ResMut<CurrentLevel>, 
    mut change_level_writer: EventWriter<ChangeLevel>
) {
    if input.just_pressed(KeyCode::Space) {
        if current_level.0 == 5 {
            current_level.0 = 0;
            change_level_writer.send(ChangeLevel {
                fail: false
            })
        } else {
            match *phase {
                Phase::PREP => *phase = Phase::COMBAT,
                Phase::COMBAT => {
                    *phase = Phase::PREP;
                    *combat_step = CombatStep::LineUp;
                }
            }
        }
    }
}

fn split_orb(
    orb: &Orb,
    commands: &mut Commands,
    t: &Mut<Transform>,
    entity: Entity,
    closest_circle: &mut ResMut<ClosestCircle>,
    text_details: TextDetails,
) -> ControlFlow<()> {
    if orb.children.is_empty() {
        return ControlFlow::Break(());
    }
    orb.children.iter().enumerate().for_each(|(i, child_orb)| {
        let position = Vec2::new(
                t.translation.x + i as f32 * 5.,
                t.translation.y + i as f32 * 5.,
            )
            .extend(0.0);
        spawn_orb(commands, position, child_orb, text_details.clone(), false)
        // let parent = commands
        //     .spawn_bundle(GeometryBuilder::build_as(
        //         &shapes::Circle {
        //             center: Vec2::new(0., 0.),
        //             radius: 30.,
        //         },
        //         DrawMode::Outlined {
        //             fill_mode: FillMode::color(Color::rgb(0.4, 0.4, 0.7)),
        //             outline_mode: StrokeMode::new(Color::BLACK, 5.0),
        //         },
        //         Transform {
        //             translation: Vec2::new(
        //                 t.translation.x + i as f32 * 5.,
        //                 t.translation.y + i as f32 * 5.,
        //             )
        //             .extend(0.0),
        //             ..default()
        //         },
        //     ))
        //     .insert(child_orb.clone())
        //     .insert(PlayerOrb)
        //     .id();

        // let health_child = commands
        //     .spawn_bundle(GeometryBuilder::build_as(
        //         &shapes::Circle {
        //             center: Vec2::new(0., 0.),
        //             radius: 12.,
        //         },
        //         DrawMode::Outlined {
        //             fill_mode: FillMode::color(Color::RED),
        //             outline_mode: StrokeMode::new(Color::BLACK, 5.0),
        //         },
        //         Transform {
        //             translation: Vec3::new(22., -22., 0.1),
        //             ..default()
        //         },
        //     ))
        //     .id();

        // let damage_child = commands
        //     .spawn_bundle(GeometryBuilder::build_as(
        //         &shapes::Circle {
        //             center: Vec2::new(0., 0.),
        //             radius: 12.,
        //         },
        //         DrawMode::Outlined {
        //             fill_mode: FillMode::color(Color::ALICE_BLUE),
        //             outline_mode: StrokeMode::new(Color::BLACK, 5.0),
        //         },
        //         Transform {
        //             translation: Vec3::new(-22., -22., 0.1),
        //             ..default()
        //         },
        //     ))
        //     .id();

        // commands.entity(parent).add_child(health_child);
        // commands.entity(parent).add_child(damage_child);
    });
    commands.entity(entity).despawn_recursive();
    closest_circle.data = None;
    return ControlFlow::Break(());
}

fn push_apart(mut t2: Mut<Transform>, mut t1: Mut<Transform>) {
    let delta = t2.translation - t1.translation;
    let force = delta.length();
    if force.abs() < 100. {
        t1.translation.x -=
            ((100.0 - delta.x.abs()) / 2.0) * delta.x.signum() * 1. / (1000.0 / delta.x.abs());
        t2.translation.x +=
            ((100.0 - delta.x.abs()) / 2.0) * delta.x.signum() * 1. / (1000.0 / delta.x.abs());
        t1.translation.y -=
            ((100.0 - delta.y.abs()) / 2.0) * delta.y.signum() * 1. / (1000.0 / delta.y.abs());
        t2.translation.y +=
            ((100.0 - delta.y.abs()) / 2.0) * delta.y.signum() * 1. / (1000.0 / delta.y.abs());
    }
}

fn modify_color(fill_mode: &mut FillMode, r: f32, g: f32, b: f32, rate: f32) {
    fill_mode.color = Color::rgb(
        fill_mode.color.r().lerp(&r, &rate),
        fill_mode.color.g().lerp(&g, &rate),
        fill_mode.color.b().lerp(&b, &rate),
    );
}

fn brighten(d: &mut Mut<DrawMode>) {
    if let DrawMode::Outlined {
        ref mut fill_mode,
        outline_mode: _,
    } = **d
    {
        fill_mode.color = Color::rgb(
            fill_mode.color.r().lerp(&1.0, &0.2),
            fill_mode.color.g().lerp(&1.0, &0.2),
            fill_mode.color.b().lerp(&1.0, &0.2),
        );
    }
}

fn increase_line_width(d: &mut Mut<DrawMode>) {
    if let DrawMode::Outlined {
        fill_mode: _ode,
        ref mut outline_mode,
    } = **d
    {
        outline_mode.options.line_width = outline_mode.options.line_width.lerp(&10.0, &0.2);
    }
}

fn decrease_line_width(d: &mut Mut<DrawMode>) {
    if let DrawMode::Outlined {
        fill_mode: _,
        ref mut outline_mode,
    } = **d
    {
        outline_mode.options.line_width = outline_mode.options.line_width.lerp(&5.0, &0.2);
    }
}

fn follow_mouse(
    mouse_buttons: &Res<Input<MouseButton>>,
    t: &mut Mut<Transform>,
    cursor: &Res<Cursor>,
    mouse_motion_event_reader: &mut EventReader<MouseMotion>,
    controls_disabled: &bool,
) {
    if mouse_buttons.pressed(MouseButton::Left) && *controls_disabled {
        let diff = t.translation.truncate() - cursor.0;

        t.translation -= (diff / 10.0).extend(0.0);

        for mouse_motion_event in mouse_motion_event_reader.iter() {
            t.translation += Vec3::new(mouse_motion_event.delta.x, -mouse_motion_event.delta.y, 0.0)
        }
    }
}


impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(CursorPlugin);
    }
}
