use iced::widget::{
    Column, button, checkbox, column, container, mouse_area, opaque, pick_list, row, space, stack,
    text,
};
use iced::{Border, Color, Element, Length};

use crate::iced_app::Message;
use crate::iced_app::app::App;
use crate::iced_app::styles::{palette, pick_list_style};

struct PlayerPickListOptions {
    class_opts: Vec<String>,
    race_opts: Vec<String>,
    xp_opts: Vec<String>,
    party_size_opts: Vec<String>,
    rot_opts: Vec<String>,
}

impl App {
    /// Wrap the base view with a modal overlay containing options.
    pub(super) fn wrap_with_modal<'a>(
        &'a self,
        base: Element<'a, Message>,
    ) -> Element<'a, Message> {
        let modal_content = container(
            column![
                self.build_modal_title(),
                self.build_player_config_column(),
                self.build_event_buttons(),
            ]
            .spacing(12)
            .padding(16),
        )
        .width(Length::Shrink)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(palette::BG_PANEL)),
            border: Border {
                color: palette::BORDER_HIGHLIGHT,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        });

        let backdrop = mouse_area(
            container(opaque(modal_content))
                .width(Length::Fill)
                .height(Length::Fill)
                .center(Length::Fill),
        )
        .on_press(Message::CloseOptionsModal);

        stack![base, opaque(backdrop)].into()
    }

    /// Title row for the options modal with close button.
    fn build_modal_title(&self) -> Element<'_, Message> {
        row![
            text("Options").size(16).color(palette::GOLD),
            space::horizontal(),
            button(text("x").size(14))
                .on_press(Message::CloseOptionsModal)
                .padding(2)
                .style(|_, _| button::Style {
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    text_color: palette::TEXT_SECONDARY,
                    ..Default::default()
                }),
        ]
        .align_y(iced::Alignment::Center)
        .into()
    }

    /// Vertical layout of player config options for the modal.
    fn build_player_config_column(&self) -> Element<'_, Message> {
        column![
            self.build_player_pick_lists(),
            self.build_movement_controls(),
        ]
        .spacing(8)
        .into()
    }

    fn build_player_pick_lists(&self) -> Column<'_, Message> {
        let options = player_pick_list_options();

        column![
            class_pick_list(self, options.class_opts),
            race_pick_list(self, options.race_opts),
            xp_pick_list(self, options.xp_opts),
            party_pick_list(self, options.party_size_opts),
            rot_damage_pick_list(self, options.rot_opts),
        ]
        .spacing(8)
    }

    fn build_movement_controls(&self) -> Column<'_, Message> {
        let m = &self.movement;
        column![
            text("Movement").size(12).color(palette::TEXT_SECONDARY),
            labeled_checkbox("Moving", m.moving, |v| Message::MovementToggled(
                "moving", v
            )),
            labeled_checkbox("Mounted", m.mounted, |v| Message::MovementToggled(
                "mounted", v
            )),
            labeled_checkbox("Flying", m.flying, |v| Message::MovementToggled(
                "flying", v
            )),
            labeled_checkbox("Falling", m.falling, |v| Message::MovementToggled(
                "falling", v
            )),
            labeled_checkbox("Swimming", m.swimming, |v| Message::MovementToggled(
                "swimming", v
            )),
        ]
        .spacing(8)
    }
}

fn class_pick_list<'a>(app: &'a App, options: Vec<String>) -> Element<'a, Message> {
    labeled_pick_list(
        "Class:",
        options,
        &app.selected_class,
        Message::PlayerClassChanged,
    )
}

fn race_pick_list<'a>(app: &'a App, options: Vec<String>) -> Element<'a, Message> {
    labeled_pick_list(
        "Race:",
        options,
        &app.selected_race,
        Message::PlayerRaceChanged,
    )
}

fn xp_pick_list<'a>(app: &'a App, options: Vec<String>) -> Element<'a, Message> {
    labeled_pick_list(
        "XP Bar:",
        options,
        &app.selected_xp_level,
        Message::XpLevelChanged,
    )
}

fn party_pick_list<'a>(app: &'a App, options: Vec<String>) -> Element<'a, Message> {
    labeled_pick_list(
        "Party:",
        options,
        &app.selected_party_size,
        Message::PartySizeChanged,
    )
}

fn rot_damage_pick_list<'a>(app: &'a App, options: Vec<String>) -> Element<'a, Message> {
    labeled_pick_list(
        "Rot Damage:",
        options,
        &app.selected_rot_level,
        Message::RotDamageLevelChanged,
    )
}

fn player_pick_list_options() -> PlayerPickListOptions {
    use crate::lua_api::state::{CLASS_LABELS, RACE_DATA, ROT_DAMAGE_LEVELS, XP_LEVELS};

    PlayerPickListOptions {
        class_opts: CLASS_LABELS.iter().map(|label| label.to_string()).collect(),
        race_opts: RACE_DATA
            .iter()
            .map(|(name, _, _)| name.to_string())
            .collect(),
        xp_opts: XP_LEVELS
            .iter()
            .map(|(label, _)| label.to_string())
            .collect(),
        party_size_opts: (0..=4).map(|size| size.to_string()).collect(),
        rot_opts: ROT_DAMAGE_LEVELS
            .iter()
            .map(|(label, _)| label.to_string())
            .collect(),
    }
}

/// A label + pick_list row used in the options modal.
fn labeled_pick_list<'a>(
    label: &'a str,
    options: Vec<String>,
    selected: &str,
    on_select: fn(String) -> Message,
) -> Element<'a, Message> {
    row![
        text(label)
            .size(12)
            .color(palette::TEXT_SECONDARY)
            .width(80),
        pick_list(options, Some(selected.to_string()), on_select)
            .text_size(12)
            .width(Length::Fill)
            .style(pick_list_style),
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center)
    .into()
}

/// A label + checkbox row used in the options modal.
fn labeled_checkbox<'a, F: Fn(bool) -> Message + 'a>(
    label: &'a str,
    checked: bool,
    on_toggle: F,
) -> Element<'a, Message> {
    row![
        checkbox(checked).on_toggle(on_toggle).text_size(12),
        text(label).size(12).color(palette::TEXT_PRIMARY),
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center)
    .into()
}
