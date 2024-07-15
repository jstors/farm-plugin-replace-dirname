#![deny(clippy::all)]

use std::{
  env,
  path::{Path, PathBuf},
  sync::Arc,
};

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config},
  context::CompilationContext,
  error::CompilationError,
  plugin::{Plugin, PluginProcessModuleHookParam, PluginTransformHookResult},
  swc_common::{comments::NoopComments, BytePos, Mark, SourceMap, DUMMY_SP},
  swc_ecma_ast::{self, Expr, Ident, Lit, MemberExpr, MemberProp, Module, Str},
  swc_ecma_parser::{EsConfig, Parser, StringInput, Syntax},
};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::{
  common::{create_swc_source_map, PathFilter, Source},
  script::swc_try_with::try_with,
  swc_ecma_codegen::{self, text_writer::JsWriter, Emitter},
  swc_ecma_transforms::{helpers::inject_helpers, typescript::tsx},
  swc_ecma_visit::{VisitMut, VisitMutWith},
};
use serde;

#[farm_plugin]
pub struct FarmPluginReplaceDirname {
  options: ReplaceDirnameOptions,
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
    Self { options }
  }
}

impl Plugin for FarmPluginReplaceDirname {
  fn name(&self) -> &str {
    "FarmPluginReplaceDirname"
  }

  fn process_module(
    &self,
    param: &mut PluginProcessModuleHookParam,
    context: &Arc<CompilationContext>,
  ) -> Result<Option<()>, CompilationError> {
    let filter = PathFilter::new(&self.options.include, &self.options.exclude);
    if !filter.execute(&param.module_id.relative_path()) {
      return Ok(None);
    }

    let file_path = env::current_dir()
      .unwrap()
      .join(param.module_id.relative_path());

    let dir_path: &str = Path::new(&file_path)
      .parent()
      .map_or("", |p| p.to_str().unwrap_or(""));

    let ast = &mut param.meta.as_script_mut().ast;
    replace_dirname_with_ast(ast, dir_path, file_path.to_str().unwrap());
    Ok(Some(()))
  }
}

pub fn replace_dirname_with_ast(ast: &mut Module, dir_path: &str, file_path: &str) {
  struct ReplaceLibVisitor<'a> {
    dir_path: &'a str,
    file_path: &'a str,
  }

  impl<'a> VisitMut for ReplaceLibVisitor<'a> {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
      match expr {
        Expr::Ident(ident) => {
          match &*ident.sym {
            "__dirname" => {
              *expr = Expr::Lit(Lit::Str(Str {
                value: self.dir_path.into(),
                span: DUMMY_SP,
                raw: None,
              }));
            }
            "__filename" => {
              *expr = Expr::Lit(Lit::Str(Str {
                value: self.file_path.into(),
                span: DUMMY_SP,
                raw: None,
              }));
            }
            _ => {}
          }
        }
        Expr::Member(MemberExpr { obj, prop, .. }) => {
          if let Expr::MetaProp(meta_prop) = &**obj {
            if meta_prop.kind == swc_ecma_ast::MetaPropKind::ImportMeta {
              if let MemberProp::Ident(ident) = &prop {
                if ident.sym == "url" {
                  *expr = Expr::Lit(Lit::Str(Str {
                    value: self.file_path.into(),
                    span: DUMMY_SP,
                    raw: None,
                  }));
                }
              }
            }
          }
        }
        _ => {
          // 递归访问子节点
          expr.visit_mut_children_with(self);
        }
      }
    }
  }

  let mut visitor = ReplaceLibVisitor {
    dir_path,
    file_path,
  };

  ast.visit_mut_with(&mut visitor);
}