// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::iter;
use std::num::NonZeroU8;

use bevy::{
    color::palettes::basic::*,
    input_focus::InputFocus,
    log::{self, Level, LogPlugin},
    prelude::*,
};
use sudoku_solver::Board;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Index(1)),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..Default::default()
                }),
        )
        .add_plugins(SudokuPlugin)
        .run();
}

struct SudokuPlugin;

impl Plugin for SudokuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>();
        app.init_resource::<Board>();
        app.add_systems(Startup, setup_ui);
        // app.add_systems(Update, button_system);
    }
}

// const NORMAL_KEYBOARD_BUTTON: Color = Color::oklab(lightness, a, b);

// const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
// const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
// const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// fn button_system(
//     mut input_focus: ResMut<InputFocus>,
//     mut interaction_query: Query<
//         (
//             Entity,
//             &Interaction,
//             &mut BackgroundColor,
//             &mut BorderColor,
//             &mut Button,
//             &Children,
//         ),
//         Changed<Interaction>,
//     >,
//     mut text_query: Query<&mut Text>,
// ) {
//     for (entity, interaction, mut color, mut border_color, mut button, children) in
//         &mut interaction_query
//     {
//         let mut text = text_query.get_mut(children[0]).unwrap();

//         match *interaction {
//             Interaction::Pressed => {
//                 input_focus.set(entity);
//                 **text = "Press".to_string();
//                 *color = PRESSED_BUTTON.into();
//                 *border_color = BorderColor::all(RED);

//                 // The accessibility system's only update the button's state when the `Button` component is marked as changed.
//                 button.set_changed();
//             }
//             Interaction::Hovered => {
//                 input_focus.set(entity);
//                 **text = "Hover".to_string();
//                 *color = HOVERED_BUTTON.into();
//                 *border_color = BorderColor::all(Color::WHITE);
//                 button.set_changed();
//             }
//             Interaction::None => {
//                 input_focus.clear();
//                 **text = "Button".to_string();
//                 *color = NORMAL_BUTTON.into();
//                 *border_color = BorderColor::all(Color::BLACK);
//             }
//         }
//     }
// }

fn setup_ui(mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2d);

    let board_entity = board(&mut commands);
    let keyboard_entity = keyboard(&mut commands);
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        })
        .with_children(|commands| {
            commands
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                })
                .add_child(board_entity)
                .add_child(keyboard_entity);
        });
}

fn board(commands: &mut Commands) -> Entity {
    commands
        .spawn(Node {
            width: vmin(70),
            height: vmin(70),
            display: Display::Grid,
            grid_template_columns: iter::repeat_n(GridTrack::flex(1.), 3)
                .chain(iter::once(GridTrack::auto()))
                .cycle()
                .take(11)
                .collect(),
            grid_template_rows: iter::repeat_n(GridTrack::flex(1.), 3)
                .chain(iter::once(GridTrack::auto()))
                .cycle()
                .take(11)
                .collect(),
            grid_auto_flow: GridAutoFlow::Row,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            row_gap: px(2),
            column_gap: px(2),
            ..Default::default()
        })
        .with_children(|commands| {
            // 000|000|000
            // 000|000|000
            // 000|000|000
            // --- --- ---
            // 000|000|000
            // 000|000|000
            // 000|000|000
            // --- --- ---
            // 000|000|000
            // 000|000|000
            // 000|000|000

            let mut x = 0_u8;
            let mut y = 0_u8;
            loop {
                if x % 4 < 3 && y % 4 < 3 {
                    log::debug!(
                        "({x}, {y}): Spawning button at {}, {}",
                        x - x / 4,
                        y - y / 4
                    );
                    commands.spawn(slot_button(x - x / 4, y - y / 4));
                } else if x % 4 == 3 && y.is_multiple_of(4) {
                    log::debug!("({x}, {y}): Spawning vertical line");
                    commands.spawn(vertical_line());
                } else if y % 4 == 3 && x.is_multiple_of(4) {
                    log::debug!("({x}, {y}): Spawning horizontal line");
                    commands.spawn(horizontal_line());
                } else if x % 4 == 3 && y % 4 == 3 {
                    log::debug!("({x}, {y}): Spawning empty cell");
                    commands.spawn(Node::default()); // empty cell
                } else {
                    log::debug!("({x}, {y}): Skipping");
                }

                x += 1;
                if x >= 11 {
                    x = 0;
                    y += 1;
                    if y >= 11 {
                        break;
                    }
                }
            }
        })
        .id()
}

fn vertical_line() -> impl Bundle {
    (
        Node {
            grid_row: GridPlacement::span(3),
            width: px(2),
            height: percent(100),
            ..Default::default()
        },
        BackgroundColor(Color::WHITE),
        BorderRadius::MAX,
    )
}

fn horizontal_line() -> impl Bundle {
    (
        Node {
            grid_column: GridPlacement::span(3),
            width: percent(100),
            height: px(2),
            ..Default::default()
        },
        BackgroundColor(Color::WHITE),
        BorderRadius::MAX,
    )
}

#[derive(Debug, Component)]
struct BoardPosition {
    x: u8,
    y: u8,
}

fn slot_button(x: u8, y: u8) -> impl Bundle {
    (
        Button,
        BoardPosition { x, y },
        Node {
            width: percent(90),
            height: percent(90),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        BorderRadius::all(percent(10)),
        BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
    )
}

fn keyboard(commands: &mut Commands) -> Entity {
    commands
        .spawn(Node {
            width: vmin(70),
            height: vmin(30),
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::flex(1.); 5],
            grid_template_rows: vec![GridTrack::flex(1.); 2],
            grid_auto_flow: GridAutoFlow::Row,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..Default::default()
        })
        .with_children(|commands| {
            for i in 1..=9 {
                commands.spawn(value_button(NonZeroU8::new(i)));
            }
            commands.spawn(value_button(NonZeroU8::new(0))); // 0
        })
        .id()
}

#[derive(Debug, Component)]
struct Value(Option<NonZeroU8>);

fn value_button(value: Option<NonZeroU8>) -> impl Bundle {
    (
        Button,
        Value(value),
        Node {
            min_width: percent(90),
            min_height: percent(90),
            flex_grow: 1.,
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BorderRadius::all(percent(10)),
        BackgroundColor(Color::BLACK),
        children![(
            Text::new(value.map_or("X".to_string(), |v| v.get().to_string())),
            TextFont {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 15.,
                ..Default::default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}
