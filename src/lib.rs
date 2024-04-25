#![deny(clippy::all)]

use std::path::Path;

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config},
  plugin::{Plugin, PluginTransformHookResult},
};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::{common::PathFilter, swc_atoms::Atom};
use serde;
use swc_common::SourceMap;
use swc_common::{sync::Lrc, FileName, DUMMY_SP};
use swc_ecma_ast::Ident;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::{Fold, FoldWith, VisitMut, VisitMutWith};

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

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    let filter = PathFilter::new(&self.options.include, &self.options.exclude);
    if !filter.execute(&param.resolved_path) {
      return Ok(None);
    }
    let dir_path: &str = Path::new(&param.resolved_path)
      .parent()
      .map_or("", |p| p.to_str().unwrap_or(""));
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(FileName::Anon, param.content.to_string());
    let lexer = Lexer::new(
      Syntax::Es(Default::default()),
      Default::default(),
      StringInput::from(&*fm),
      None,
    );
    let mut parser = Parser::new_from(lexer);
    // Converted ast
    let module = parser.parse_module().expect("ast conversion failed");

    pub struct ReplaceFold {
      dir_path: Atom,
      resolve_path: Atom,
    }

    impl Fold for ReplaceFold {
      fn fold_ident(&mut self, ident: Ident) -> Ident {
        let new_sym = match ident.sym.as_ref() {
          "__dirname" => self.dir_path.clone(),
          "__filename" => self.resolve_path.clone(),
          _ => ident.sym.clone(),
        };
        Ident::new(new_sym, DUMMY_SP)
      }
    }

    // Create an instance of the AST modifier
    let mut replace_fold = ReplaceFold {
      dir_path: Atom::from(format!("\"{}\"", dir_path)),
      resolve_path: Atom::from(format!("\"{}\"", param.resolved_path)),
    };
    struct ReplaceModule;

    impl VisitMut for ReplaceModule {}
    let mut buf = vec![];

    // Traversing and modifying the AST
    let mut modified_module = module.fold_with(&mut replace_fold);
    let cfg = swc_ecma_codegen::Config::default();
    let source_map: Lrc<SourceMap> = Default::default();
    modified_module.visit_mut_with(&mut ReplaceModule);
    {
      let writer = JsWriter::new(source_map.clone(), "\n", &mut buf, None);
      let mut emitter = Emitter {
        cfg,
        comments: None,
        cm: source_map,
        wr: Box::new(writer),
      };
      emitter
        .emit_module(&modified_module)
        .expect("Code conversion failed");
    }
    // Convert buffer to string
    let code = String::from_utf8(buf).expect("Buffer contains invalid UTF-8");
    return Ok(Some(PluginTransformHookResult {
      content: code,
      ..Default::default()
    }));
  }
}
