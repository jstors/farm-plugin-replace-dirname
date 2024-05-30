use swc_ecmascript::ast::*;
use swc_ecmascript::visit::{VisitMut, VisitMutWith};
use swc_common::DUMMY_SP;
use std::env;
use std::path::Path;

pub fn replace_dirname_with_ast(param: &mut PluginProcessModuleHookParam) {
    let absolute_path = env::current_dir()
        .unwrap()
        .join(param.module_id.relative_path());

    let dir_path: &str = Path::new(&absolute_path)
        .parent()
        .map_or("", |p| p.to_str().unwrap_or(""));

    println!("Dir path: {:?}", dir_path);
    println!("absolute_path: {:?}", absolute_path);

    let ast = &mut param.meta.as_script_mut().ast;

    struct ReplaceLibVisitor<'a> {
        dir_path: &'a str,
        absolute_path: &'a str,
    }

    impl<'a> VisitMut for ReplaceLibVisitor<'a> {
        fn visit_mut_ident(&mut self, ident: &mut Ident) {
            match &*ident.sym {
                "__dirname" => {
                    *ident = Ident::new(self.dir_path.into(), DUMMY_SP);
                }
                "__filename" => {
                    *ident = Ident::new(self.absolute_path.into(), DUMMY_SP);
                }
                _ => {}
            }
        }
    }

    let mut visitor = ReplaceLibVisitor {
        dir_path,
        absolute_path: absolute_path.to_str().unwrap(),
    };
    
    ast.visit_mut_with(&mut visitor);
}
