use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct AngularConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub not_capable_style: &'a str,
    pub detect_package_json: Vec<&'a str>,
    pub detect_angular_json: Vec<&'a str>,
}

impl<'a> Default for AngularConfig<'a> {
    fn default() -> Self {
        AngularConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "󰚿 ",
            style: "bold green",
            disabled: false,
            not_capable_style: "bold red",
            detect_package_json: vec!["package.json"],
            detect_angular_json: vec!["angular.json"],
        }
    }
}
