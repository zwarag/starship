use super::{Context, Module, ModuleConfig};

use crate::configs::angular::AngularConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

use semver::Version;
use serde_json as json;

/// Creates a module with the current Angular Core version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    log::warn!("eyy");

    let mut module = context.new_module("angular");
    let config = AngularConfig::try_load(module.config);

    // TODO: continue from here
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&config.detect_package_json)
        .is_match();

    let is_angular_project = context
        .try_begin_scan()?
        .set_files(&config.detect_angular_json)
        .is_match();

    if !is_js_project || !is_angular_project {
        return None;
    }

    let package_json: json::Value = json::from_str(&config.detect_package_json[0]).ok()?;

    let angular_version = package_json["dependencies"]["@angular/core"]
        .as_str()
        .or_else(|| package_json["devDependencies"]["@angular/core"].as_str())?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => {
                    //if in_engines_range {
                    Some(Ok(config.style))
                    //} else {
                    //    Some(Ok(config.not_capable_style))
                    //}
                }
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let angular_ver = Version::parse(
                        angular_version
                            .strip_prefix('^')
                            .or_else(|| angular_version.strip_prefix('~'))?,
                    )
                    .ok()?;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &angular_ver.to_string(),
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `angular`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_node_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json_and_esy_lock() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;
        let esy_lock = dir.path().join("esy.lock");
        fs::create_dir_all(esy_lock)?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".node-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nvmrc() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".nvmrc"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_js_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.js"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mjs_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.mjs"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cjs_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.cjs"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ts_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.ts"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_modules() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let node_modules = dir.path().join("node_modules");
        fs::create_dir_all(node_modules)?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn engines_node_version_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\">=12.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn engines_node_version_not_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\"<12.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn show_expected_version_when_engines_does_not_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\"<=11.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .config(toml::toml! {
                [nodejs]
                format = "via [$symbol($version )($engines_version )]($style)"
            })
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Red.bold().paint(" v12.0.0 <=11.0.0 ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn do_not_show_expected_version_if_engines_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\">=12.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .config(toml::toml! [
                [nodejs]
                format = "via [$symbol($version )($engines_version )]($style)"
            ])
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn do_not_show_expected_version_if_no_set_engines_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .config(toml::toml! {
                [nodejs]
                format = "via [$symbol($version )($engines_version )]($style)"
            })
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn no_node_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.js"))?.sync_all()?;
        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .cmd("node --version", None)
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
