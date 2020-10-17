use crate::pagesection::PageSection;
use crate::pagesidebar::PageSidebar;
use yew::virtual_dom::VChild;
use yew::{html, Component, ComponentLink, Html};
use yew::{ChildrenWithProps, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenWithProps<PageSection>,
    #[prop_or_default]
    pub sidebar: Option<VChild<PageSidebar>>,
    #[prop_or_default]
    pub tools: Html,
}

pub struct Page {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ToggleSidebar,
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleSidebar => self.toggle_sidebar(),
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="pf-c-page">
                <header class="pf-c-page__header">
                    <div class="pf-c-page__header-brand">
                        { self.sidebar_button() }
                        <a href="#" class="pf-c-page__header-brand-link">{"Logo"}</a>
                    </div>
                    <div class="pf-c-page__header-tools"> { self.props.tools.clone() }</div>
                </header>
                { self.sidebar() }
                <main class="pf-c-page__main" tabindex="-1">
                { for self.props.children.iter().map(|child|child) }
                </main>
            </div>
        }
    }
}

impl Page {
    fn sidebar_button(&self) -> Html {
        let click_callback = self.link.callback(|_| Msg::ToggleSidebar);

        match &self.props.sidebar {
            Some(_) => html! {<div class="pf-c-page__header-brand-toggle">
                <button class="pf-c-button pf-m-plain" type="button" onclick=click_callback>
                    <i class="fas fa-bars" aria-hidden="true"/>
                </button>
            </div>},
            None => html! {},
        }
    }

    fn sidebar(&self) -> Html {
        match &self.props.sidebar {
            Some(sidebar) => Html::from(sidebar.clone()),
            None => html! {},
        }
    }

    fn toggle_sidebar(&mut self) {
        match &mut self.props.sidebar {
            Some(sidebar) => sidebar.props.open = !sidebar.props.open,
            _ => {}
        }
    }
}
