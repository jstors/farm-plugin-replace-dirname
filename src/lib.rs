#![deny(clippy::all)]

use std::path::Path;

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config, Mode},
  module::ModuleType,
  plugin::{Plugin, PluginTransformHookResult},
};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::common::PathFilter;
use regex::Regex;
use serde;

#[farm_plugin]
pub struct FarmPluginReplaceDirname {
  // options: FarmPluginReplaceDirnameOptions,
  // regex: Regex,
}

#[derive(serde::Deserialize)]
pub struct ReplaceDirnameOptions {
  exclude: Vec<ConfigRegex>,
  include: Vec<ConfigRegex>,
}

impl Default for ReplaceDirnameOptions {
  fn default() -> Self {
    Self {
      exclude: vec![ConfigRegex::new("node_modules/")],
      include: vec![],
    }
  }
}

impl FarmPluginReplaceDirname {
  fn new(config: &Config, options: String) -> Self {
    let options: ReplaceDirnameOptions = serde_json::from_str(&options).unwrap_or_default();
    Self {
      // options,
      // regex: Regex::new(r"console\.log\(.*?\)").unwrap(),
    }
  }
}

impl Plugin for FarmPluginReplaceDirname {
  fn name(&self) -> &str {
    "FarmPluginReplaceDirname"
  }

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    // if matches!(context.config.mode, Mode::Production) {
    // let is_support_module_type = matches!(
    //   param.module_type,
    //   ModuleType::Js | ModuleType::Jsx | ModuleType::Ts | ModuleType::Tsx
    // );
    // println!("is_support_module_type: {}", is_support_module_type);
    // let filter = PathFilter::new(&self.options.include, &self.options.exclude);

    // determine if it is a support type
    // is the path is the user-configured include or exclude
    // if is_support_module_type && filter.execute(&param.resolved_path) {
    //   let content = self.regex.replace_all(&param.content, "").into_owned();

    //   return Ok(Some(PluginTransformHookResult {
    //     content,
    //     ..Default::default()
    //   }));
    // }

    let dir_path = Path::new(&param.resolved_path)
      .parent()
      .map_or("", |p| p.to_str().unwrap_or(""));

    let replace_content = param
      .content
      .replace("__dirname", &format!("\"{}\"", dir_path))
      .replace("__filename", &format!("\"{}\"", param.resolved_path))
      .replace("import.meta.url", &format!("\"{}\"", param.resolved_path));

    return Ok(Some(PluginTransformHookResult {
      content: replace_content,
      ..Default::default()
    }));
    // }
    Ok(None)
  }
}
