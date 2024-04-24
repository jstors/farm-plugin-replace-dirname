#![deny(clippy::all)]

use std::{path::Path, sync::Arc};

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config, Mode},
  module::ModuleType,
  plugin::{Plugin, PluginTransformHookResult},
};

use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::{common::PathFilter, swc_atoms::Atom};
use serde;
use swc_common::{sync::Lrc, FileName, FilePathMapping, DUMMY_SP};
use swc_common::{BytePos, SourceMap, Span};
use swc_ecma_ast::{EsVersion, Expr, ExprStmt, Ident, Module, ModuleItem, Stmt};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::{Fold, FoldWith, VisitMut, VisitMutWith};

// use swc_common::{
//   errors::{ColorConfig, Handler},
//   SourceMap,
// };
// use swc_ecma_parser::{EsConfig, Syntax};
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

struct VarReplacer {
  dirname: Ident,
  filename: Ident,
  // import_meta_url: Ident,
}

impl Fold for VarReplacer {
  fn fold_expr(&mut self, expr: Expr) -> Expr {
    match &expr {
      Expr::Ident(ident) => {
        if ident.sym == "__dirname" {
          Expr::Ident(self.dirname.clone())
        } else if ident.sym == "__filename" {
          Expr::Ident(self.filename.clone())
        } else {
          expr
        }
        //  else if ident.sym == "import.meta.url" {
        //   Expr::Ident(self.import_meta_url.clone())
        // } else {
      }
      _ => expr.fold_with(self),
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
    let dir_path = Path::new(&param.resolved_path)
      .parent()
      .map_or("", |p| p.to_str().unwrap_or(""));
    // println!("content: {}", param.content);
    let replace_content = param
      .content
      .replace("__dirname", &format!("\"{}\"", dir_path))
      .replace("__filename", &format!("\"{}\"", param.resolved_path))
      .replace("import.meta.url", &format!("\"{}\"", param.resolved_path));
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm
      .load_file(&Path::new("../playground/index.js"))
      .expect("没有找到文件");
    let lexer = Lexer::new(
      Syntax::Es(Default::default()),
      Default::default(),
      StringInput::from(&*fm),
      None,
    );
    let mut parser = Parser::new_from(lexer);
    // 转换出来的ast
    let module = parser.parse_module().expect("转换失败");
    let dirname_var = Ident::new("new_dirname".into(), DUMMY_SP);
    let filename_var = Ident::new("new_filename".into(), DUMMY_SP);
    // let import_meta_url_var = Ident::new("new_import_meta_url".into(), DUMMY_SP);
    // println!("lexer: {:#?}", dirname_var);
    let mut replacer = VarReplacer {
      dirname: dirname_var,
      filename: filename_var,
      // import_meta_url: import_meta_url_var,
    };

    // println!("lexer: {:#?}", replacer.dirname);

    // let new_module = module.fold_with(&mut replacer);
    // println!("lexer: {:#?}", new_module);
    // test

    pub struct MyFold {
      dir_path: Atom,
      resolve_path: Atom,
    }

    impl Fold for MyFold {
      fn fold_ident(&mut self, ident: Ident) -> Ident {
        let new_sym = match ident.sym.as_ref() {
          "__dirname" => self.dir_path.clone(),
          "__filename" => self.resolve_path.clone(),
          "bar" => self.resolve_path.clone(),
          _ => ident.sym.clone(),
        };
        Ident::new(new_sym, ident.span)
      }
    }
    // 创建一个 AST 修改器实例
    println!("dir_path: {:#?}", dir_path);
    println!("dir_path: {:#?}", dir_path.to_string());
    println!("dir_path: {:#?}", param.resolved_path);

    // let mut my_fold = MyFold {
    //   // dir_path: dir_path.to_string(),
    //   // resolve_path: param.resolved_path.to_string(),
    //   dir_path: Expr::Ident(Ident::new(dir_path.into(), DUMMY_SP)),
    //   resolve_path: Expr::Ident(Ident::new(param.resolved_path.into(), DUMMY_SP)),
    // };
    let mut my_fold = MyFold {
      dir_path: Atom::from(dir_path),
      resolve_path: Atom::from(param.resolved_path),
    };
    struct MyModule;

    impl VisitMut for MyModule {}
    let mut buf = vec![];

    // 对 AST 进行遍历和修改
    let mut modified_module = module.fold_with(&mut my_fold);
    let cfg = swc_ecma_codegen::Config::default()
      .with_minify(false)
      .with_target(EsVersion::Es5)
      .with_omit_last_semi(true)
      .with_ascii_only(false);
    let source_map: Lrc<SourceMap> = Default::default();
    modified_module.visit_mut_with(&mut MyModule);
    {
      let writer = JsWriter::new(source_map.clone(), "\n", &mut buf, None);
      let mut emitter = Emitter {
        cfg,
        comments: None,
        cm: source_map,
        wr: Box::new(writer),
      };
      emitter.emit_module(&modified_module).expect("代码转换失败");
    }
    // 将 buffer 转换成字符串
    let code = String::from_utf8(buf).expect("Buffer 包含无效的 UTF-8");
    println!("Modified AST: {:#?}", code);
    return Ok(Some(PluginTransformHookResult {
      content: code,
      ..Default::default()
    }));
  }
}
