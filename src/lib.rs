#![deny(clippy::all)]

use std::{path::PathBuf, sync::Arc};

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config},
  context::CompilationContext,
  error::CompilationError,
  plugin::{Plugin, PluginProcessModuleHookParam, PluginTransformHookResult},
  swc_common::{comments::NoopComments, BytePos, Mark, SourceMap, DUMMY_SP},
  swc_ecma_ast::{self, Ident, Module},
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

struct DirnameReplacer {
  new_name: String,
}

impl VisitMut for DirnameReplacer {
  fn visit_mut_ident(&mut self, n: &mut swc_ecma_ast::Ident) {
    if n.sym == *"lib" {
      n.sym = self.new_name.clone().into();
    }
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
    let content = param.content.to_string();
    let input = StringInput::new(&content, BytePos::DUMMY, BytePos::DUMMY);
    let mut parser = Parser::new(
      Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
      }),
      input,
      None,
    );
    let module = parser.parse_module().expect("Failed to parse module");

    let mut replacer = DirnameReplacer {
      new_name: "newVarName".to_string(),
    };
    let mut module = module;
    // module.visit_mut_with(&mut replacer);
    // let cm = Arc::new(SourceMap::default());
    let (cm, _) = create_swc_source_map(Source {
      path: PathBuf::from(&param.module_id.to_string()),
      content: param.content.clone(),
    });
    let mut buf = vec![];
    let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
    let mut emitter = Emitter {
      cfg: {
        let mut cfg = swc_ecma_codegen::Config::default();
        cfg.minify = false;
        cfg
      },
      cm: cm.clone(),
      comments: None,
      wr: writer,
    };
    // println!("Emitting module {:#?}", module);
    // emitter.emit_module(&module).expect("Failed to emit module");
    // let code = String::from_utf8(buf).expect("Failed to convert buffer to string");
    // param.content = Arc::new(code);
    // println!("Module: {:?}", param.module_id.relative_path());
    // let ast = &mut param.meta.as_script_mut().ast;
    println!("AST: {:#?}", param.meta.as_script_mut().ast);
    println!("module: {:#?}", module);
    // param.meta.as_script_mut().ast = module;
    let ast = &mut param.meta.as_script_mut().ast;
    // println!("AST: {:#?}", ast);
    // replace_lib_with_aaa(ast);
    Ok(Some(()))
  }
}

pub fn replace_lib_with_aaa(ast: &mut Module) {
  struct ReplaceLibVisitor;

  impl VisitMut for ReplaceLibVisitor {
    fn visit_mut_ident(&mut self, ident: &mut Ident) {
      println!("Ident: {:?}", ident.sym);
      if ident.sym == *"a" {
        println!("Found a");
        *ident = Ident::new("bbbbbb".into(), DUMMY_SP);
      }
    }
  }
  let mut visitor = ReplaceLibVisitor;
  ast.visit_mut_with(&mut visitor);
}
