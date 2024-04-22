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

use std::sync::Arc;

use swc_ecma_ast::{CallExpr, Expr, Lit, Module, ModuleItem};
use swc_ecma_visit::VisitWith;
#[farm_plugin]
pub struct FarmPluginReplaceDirname {}

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
    // let filter = PathFilter::new(&self.options.include, &self.options.exclude);
    // if !filter.is_match(&param.resolved_path) {
    //   return Ok(None);
    // }
    let dir_path = Path::new(&param.resolved_path)
      .parent()
      .map_or("", |p| p.to_str().unwrap_or(""));
      println!("dir_path: {}", dir_path);
      println!("content: {}", param.content);
    let replace_content = param
      .content
      .replace("__dirname", &format!("\"{}\"", dir_path))
      .replace("__filename", &format!("\"{}\"", param.resolved_path))
      .replace("import.meta.url", &format!("\"{}\"", param.resolved_path));

    return Ok(Some(PluginTransformHookResult {
      content: replace_content,
      ..Default::default()
    }));
  }
}

struct DirNameTransformer {
  resolved_path: String,
}

impl VisitMut  for DirNameTransformer {
  fn visit_mut_module_item(&mut self, item: &mut ModuleItem) {
      item.visit_mut_with(&mut swc::EcmaVisitor::new(self, swc::ecma::visit::VisitMutEcmascriptRootExt));
  }

  fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
      let dir_path = Path::new(&self.resolved_path)
          .parent()
          .map_or("", |p| p.to_str().unwrap_or(""));
      
      // 处理 __dirname
      if let Expr::Ident(ident) = &call.callee {
          if ident.sym.as_ref() == "__dirname" {
              call.args = vec![Lit::Str(dir_path.into()).into()];
          }
      }

      // 处理 __filename
      if let Expr::Ident(ident) = &call.callee {
          if ident.sym.as_ref() == "__filename" {
              call.args = vec![Lit::Str(self.resolved_path.clone().into()).into()];
          }
      }

      // 处理 import.meta.url
      if let Expr::Member(member) = &call.callee {
          if member.obj.is_ident_ref().map(|ident| ident.sym.as_ref() == "import").unwrap_or(false)
              && member.prop.is_ident_ref().map(|ident| ident.sym.as_ref() == "meta").unwrap_or(false)
              && member.computed
          {
              if let Expr::Member(meta) = &*member.obj {
                  if meta.prop.is_ident_ref().map(|ident| ident.sym.as_ref() == "url").unwrap_or(false) {
                      call.args = vec![Lit::Str(self.resolved_path.clone().into()).into()];
                  }
              }
          }
      }
  }
}