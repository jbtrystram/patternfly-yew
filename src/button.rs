use crate::Icon;
use yew::prelude::*;
use yew::web_sys::HtmlElement;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Variant {
    None,
    Primary,
    Secondary,
    Tertiary,
    Warning,
    Danger,
    Link,
    InlineLink,
    Control,
    Plain,
}

impl Variant {
    pub fn as_classes(&self) -> Vec<&str> {
        match self {
            Variant::None => vec![],
            Variant::Primary => vec!["pf-m-primary"],
            Variant::Secondary => vec!["pf-m-secondary"],
            Variant::Tertiary => vec!["pf-m-tertiary"],
            Variant::Warning => vec!["pf-m-warning"],
            Variant::Danger => vec!["pf-m-danger"],
            Variant::Link => vec!["pf-m-link"],
            Variant::InlineLink => vec!["pf-m-link", "pf-m-inline"],
            Variant::Control => vec!["pf-m-control"],
            Variant::Plain => vec!["pf-m-plain"],
        }
    }
}

impl Default for Variant {
    fn default() -> Self {
        Variant::None
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Align {
    Start,
    End,
}

impl Default for Align {
    fn default() -> Self {
        Align::Start
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<yew::MouseEvent>,
    #[prop_or_default]
    pub variant: Variant,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub align: Align,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub aria_label: Option<String>,

    #[prop_or("button".into())]
    pub r#type: String,

    #[prop_or_default]
    pub role: Option<String>,

    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub children: Children,
}

pub struct Button {
    props: Props,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node_ref: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(evt) => {
                self.props.onclick.emit(evt);
                // blur the button after a click, otherwise it will continue appear hovered/pressed
                self.blur();
            }
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
        let mut classes = Classes::from(
            self.props
                .class
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or_else(|| "pf-c-button"),
        );

        classes = classes.extend(self.props.variant.as_classes());

        if self.props.expanded {
            classes.push("pf-m-expanded");
        }

        return html! {
            <button
                ref=self.node_ref.clone()
                id=self.props.id
                class=classes
                style=self.props.style.as_ref().cloned().unwrap_or_default()
                disabled=self.props.disabled
                type=self.props.r#type
                onclick=self.link.callback(Msg::Clicked)
                role=self.props.role.clone().unwrap_or_default()
            >

                { self.label() }
                { for self.props.children.iter() }

            </button>
        };
    }
}

impl Button {
    fn icon(&self) -> Html {
        let mut classes = Classes::from("pf-c-button__icon");

        match self.props.align {
            Align::Start => classes.push("pf-m-start"),
            Align::End => classes.push("pf-m-end"),
        }

        match self.props.icon {
            Some(i) => html! {
                <span class=classes>
                    { i }
                </span>
            },
            None => html! {},
        }
    }

    fn label(&self) -> Vec<Html> {
        let label = self.props.label.clone().into();
        match self.props.align {
            Align::Start => vec![self.icon(), label],
            Align::End => vec![label, self.icon()],
        }
    }

    /// Blur (loose focus) on the button element
    fn blur(&self) {
        if let Some(node) = self.node_ref.cast::<HtmlElement>() {
            node.blur().ok();
        }
    }
}
