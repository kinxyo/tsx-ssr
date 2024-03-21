use std::{path::Path, sync::Arc, fs};
use swc_bundler::{Bundle, Bundler};
use anyhow::Context;
use swc::{self, try_with_handler};
use swc_common::{SourceMap, GLOBALS};
use swc_ecma_codegen::{
    text_writer::{omit_trailing_semi, JsWriter, WriteJs},
    Emitter,
};
use swc_common::sync::Lrc;

pub fn render_out(path: &str) -> String { //render out signifies the template first renders and then it's sent out as a response.
    
    transpile(path);
    
    let mut bundler = Bundler::new(
        globals,
        cm.clone(),
        Loader { cm: cm.clone() },
        CachingResolver::new(
            4096,
            NodeModulesResolver::new(TargetEnv::Node, Default::default(), true),
        ),
        swc_bundler::Config {
            require: false,
            disable_inliner: !inline,
            external_modules: Default::default(),
            disable_fixer: minify,
            disable_hygiene: minify,
            disable_dce: false,
            module: Default::default(),
        },
        Box::new(Hook),
    );

    print_bundles(cm, modules, minify);

    String::new()
    // tokio::fs::read_to_string(path).await.unwrap()
}

fn print_bundles(cm: Lrc<SourceMap>, modules: Vec<Bundle>, minify: bool) {
    for bundled in modules {
        let code = {
            let mut buf = vec![];

            {
                let wr = JsWriter::new(cm.clone(), "\n", &mut buf, None);
                let mut emitter = Emitter {
                    cfg: swc_ecma_codegen::Config::default().with_minify(true),
                    cm: cm.clone(),
                    comments: None,
                    wr: if minify {
                        Box::new(omit_trailing_semi(wr)) as Box<dyn WriteJs>
                    } else {
                        Box::new(wr) as Box<dyn WriteJs>
                    },
                };

                emitter.emit_module(&bundled.module).unwrap();
            }

            String::from_utf8_lossy(&buf).to_string()
        };

        #[cfg(feature = "concurrent")]
        rayon::spawn(move || drop(bundled));

        println!("Created output.js ({}kb)", code.len() / 1024);
        fs::write("output.js", &code).unwrap();
    }
}

fn transpile(path: &str) {
    let cm = Arc::<SourceMap>::default();

    let c = swc::Compiler::new(cm.clone());
    let output = GLOBALS
        .set(&Default::default(), || {
            try_with_handler(cm.clone(), Default::default(), |handler| {
                let fm = cm
                    .load_file(Path::new(path))
                    .expect("failed to load file");

                c.process_js_file(fm, handler, &Default::default())
                    .context("failed to process file")
            })
        })
        .unwrap();

    println!("{}", output.code);
}