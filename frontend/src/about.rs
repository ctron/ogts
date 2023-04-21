use patternfly_yew::prelude::*;
use yew::prelude::*;

const COPYRIGHT: &str = "Copyright © 2023 Jens Reimann and others";

#[function_component(About)]
pub fn about() -> Html {
    let client_version = "0.0.0";
    html!(
        <Bullseye plain=true>
            <patternfly_yew::prelude::About
                brand_src="images/logo.png"
                brand_alt="OGTS"
                title="OGTS"
                strapline={html!(COPYRIGHT)}
                hero_style=r#"
--pf-c-about-modal-box__hero--lg--BackgroundImage: url("https://www.patternfly.org/assets/images/pfbg_992@2x.jpg");
--pf-c-about-modal-box__hero--sm--BackgroundImage: url("https://www.patternfly.org/assets/images/pfbg_992.jpg");
"#
            >
                <Content>
                    <dl style="width: 100%">
                        <dt>{ "Version" }</dt>
                        <dd>{ client_version }</dd>
                        <dt>{ "License" }</dt>
                        <dd>{ env!("CARGO_PKG_LICENSE") }</dd>
                    </dl>
                </Content>
            </patternfly_yew::prelude::About>
        </Bullseye>
    )
}
