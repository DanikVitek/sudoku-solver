// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::{iter, num::NonZeroU8, ops::Index};

use bevy::{
    input_focus::InputFocus,
    log::{self, LogPlugin},
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
                    #[cfg(debug_assertions)]
                    level: log::Level::DEBUG,
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
        app.add_systems(Startup, (setup_theme, setup_ui).chain());
        app.add_systems(Update, update_button_style);
    }
}

#[derive(Clone, Copy, Component)]
#[component(immutable)]
#[repr(u8)]
enum ButtonType {
    Slot,
    Keyboard,
}

#[derive(Resource)]
struct ButtonThemes {
    internal: [ButtonTheme; 2],
}

struct ButtonTheme {
    default_color: ButtonColor,
    hovered_color: ButtonColor,
    pressed_color: ButtonColor,
}

struct ButtonColor {
    background: BackgroundColor,
    border: BorderColor,
    text: TextColor,
}

impl Index<ButtonType> for ButtonThemes {
    type Output = ButtonTheme;

    fn index(&self, index: ButtonType) -> &Self::Output {
        self.internal.get(index as usize).unwrap()
    }
}

const ALABASTER_GREY: Color = Color::srgb_u8(220, 220, 221);
const PALE_SLATE: Color = Color::srgb_u8(197, 195, 198);
const PACIFIC_CYAN: Color = Color::srgb_u8(25, 133, 161);
const BLUE_SLATE: Color = Color::srgb_u8(76, 92, 104);
const IRON_GRAY: Color = Color::srgb_u8(0x33, 0x35, 0x37);

fn setup_theme(mut commands: Commands) {
    commands.insert_resource(ButtonThemes {
        internal: [
            // ButtonType::Slot
            ButtonTheme {
                default_color: ButtonColor {
                    background: BLUE_SLATE.into(),
                    border: Default::default(),
                    text: PALE_SLATE.into(),
                },
                hovered_color: ButtonColor {
                    background: BLUE_SLATE.mix(&Color::WHITE, 2. / 14.).into(),
                    border: Default::default(),
                    text: PALE_SLATE.mix(&Color::WHITE, 2. / 14.).into(),
                },
                pressed_color: ButtonColor {
                    background: BLUE_SLATE.mix(&Color::BLACK, 2. / 14.).into(),
                    border: Default::default(),
                    text: PALE_SLATE.mix(&Color::BLACK, 2. / 14.).into(),
                },
            },
            // ButtonType::Keyboard
            ButtonTheme {
                default_color: ButtonColor {
                    background: PACIFIC_CYAN.into(),
                    border: Default::default(),
                    text: ALABASTER_GREY.into(),
                },
                hovered_color: ButtonColor {
                    background: PACIFIC_CYAN.mix(&Color::WHITE, 2. / 14.).into(),
                    border: Default::default(),
                    text: ALABASTER_GREY.mix(&Color::WHITE, 2. / 14.).into(),
                },
                pressed_color: ButtonColor {
                    background: PACIFIC_CYAN.mix(&Color::BLACK, 2. / 14.).into(),
                    border: Default::default(),
                    text: ALABASTER_GREY.mix(&Color::BLACK, 2. / 14.).into(),
                },
            },
        ],
    })
}

fn update_button_style(
    mut input_focus: ResMut<InputFocus>,
    button_themes: Res<ButtonThemes>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            Option<&mut BorderColor>,
            &mut Button,
            &ButtonType,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut text_color_query: Query<&mut TextColor>,
) {
    for (entity, interaction, mut color, border_color, mut button, button_type, children) in
        &mut interaction_query
    {
        let theme = &button_themes[*button_type];
        let mut text_color = text_color_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *color = theme.pressed_color.background;
                if let Some(mut border_color) = border_color {
                    *border_color = theme.pressed_color.border;
                }
                *text_color = theme.pressed_color.text;

                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                *color = theme.hovered_color.background;
                if let Some(mut border_color) = border_color {
                    *border_color = theme.hovered_color.border;
                }
                *text_color = theme.hovered_color.text;

                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *color = theme.default_color.background;
                if let Some(mut border_color) = border_color {
                    *border_color = theme.default_color.border;
                }
                *text_color = theme.default_color.text;
            }
        }
    }
}

fn setup_ui(themes: Res<ButtonThemes>, mut commands: Commands) {
    // UI camera
    commands.spawn(Camera2d);

    let board_entity = board(&themes, &mut commands);
    let keyboard_entity = keyboard(&themes, &mut commands);

    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            BackgroundColor(IRON_GRAY),
        ))
        .add_children(&[board_entity, keyboard_entity]);
}

fn board(themes: &ButtonThemes, commands: &mut Commands) -> Entity {
    commands
        .spawn(Node {
            min_width: vmin(70),
            min_height: vmin(70),
            flex_grow: 1.,
            display: Display::Grid,
            grid_template_columns: iter::chain(
                iter::repeat_n(GridTrack::flex(1.), 3),
                iter::once(GridTrack::auto()),
            )
            .cycle()
            .take(11)
            .collect(),
            grid_template_rows: iter::chain(
                iter::repeat_n(GridTrack::flex(1.), 3),
                iter::once(GridTrack::auto()),
            )
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
                        "({x}, {y}): Spawning a ({}, {}) button",
                        x - x / 4,
                        y - y / 4
                    );
                    commands.spawn(slot_button(themes, x - x / 4, y - y / 4));
                } else if x % 4 == 3 && y.is_multiple_of(4) {
                    log::debug!("({x}, {y}): Spawning vertical line");
                    commands.spawn(vertical_line());
                } else if y % 4 == 3 && x.is_multiple_of(4) {
                    log::debug!("({x}, {y}): Spawning horizontal line");
                    commands.spawn(horizontal_line());
                } else if x % 4 == 3 && y % 4 == 3 {
                    log::debug!("({x}, {y}): Spawning empty cell");
                    commands.spawn(Node::default());
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

fn slot_button(themes: &ButtonThemes, x: u8, y: u8) -> impl Bundle {
    let button_type = ButtonType::Slot;
    let theme_color = &themes[button_type].default_color;
    (
        Button,
        button_type,
        BoardPosition { x, y },
        Node {
            width: percent(90),
            height: percent(90),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        BorderRadius::all(percent(10)),
        theme_color.background,
        children![theme_color.text],
    )
}

fn keyboard(themes: &ButtonThemes, commands: &mut Commands) -> Entity {
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
            for i in 1..=10 {
                commands.spawn(value_button(&themes, NonZeroU8::new(i % 10)));
            }
        })
        .id()
}

#[derive(Debug, Component)]
struct Value(Option<NonZeroU8>);

fn value_button(themes: &ButtonThemes, value: Option<NonZeroU8>) -> impl Bundle {
    let button_type = ButtonType::Keyboard;
    let theme_color = &themes[button_type].default_color;
    (
        Button,
        button_type,
        Value(value),
        Node {
            min_width: percent(90),
            min_height: percent(90),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BorderRadius::all(percent(10)),
        theme_color.background,
        children![(
            Text::new(value.map_or("X".to_string(), |v| v.get().to_string())),
            TextFont {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 15.,
                ..Default::default()
            },
            theme_color.text,
        )],
    )
}
