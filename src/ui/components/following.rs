use once_cell::sync::Lazy;
use tui::{Frame, layout::Rect};

use super::utils::SearchWidget;
use crate::{
    handlers::{config::SharedCompleteConfig, user_input::events::Event},
    terminal::TerminalAction,
    twitch::{TwitchAction, channels::Following},
    ui::components::Component,
};

static INCORRECT_SCOPES_ERROR_MESSAGE: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "Failed to get the list of streamers you currently follow.",
        "Either you have incorrect scopes in your token, or the API is down.",
        "To get the correct scopes, see the default config at the link below:",
        "https://github.com/Xithrius/twitch-tui/blob/main/default-config.toml#L8-L13",
        "",
        "Hit ESC to dismiss this error.",
    ]
});

pub struct FollowingWidget {
    #[allow(dead_code)]
    config: SharedCompleteConfig,
    pub search_widget: SearchWidget<String, Following>,
}

impl FollowingWidget {
    pub fn new(config: SharedCompleteConfig) -> Self {
        let item_getter = Following::new(config.borrow().clone());

        let search_widget = SearchWidget::new(
            config.clone(),
            item_getter,
            INCORRECT_SCOPES_ERROR_MESSAGE.to_vec(),
        );

        Self {
            config,
            search_widget,
        }
    }

    pub const fn is_focused(&self) -> bool {
        self.search_widget.is_focused()
    }

    pub async fn toggle_focus(&mut self) {
        self.search_widget.toggle_focus().await;
    }
}

impl Component for FollowingWidget {
    fn draw(&mut self, f: &mut Frame, area: Option<Rect>) {
        self.search_widget.draw(f, area);
    }

    async fn event(&mut self, event: &Event) -> Option<TerminalAction> {
        let action = self.search_widget.event(event).await;

        if let Some(TerminalAction::Enter(TwitchAction::Join(channel))) = &action {
            self.config.borrow_mut().twitch.channel.clone_from(channel);
        }

        action
    }
}
