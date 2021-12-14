extern crate protoc_rust;

// use protoc_rust::Customize;
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::path::Path;

/// Version # of package sent out on requests to help with debugging
#[allow(dead_code)]
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// name of package
#[allow(dead_code)]
const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
#[allow(dead_code)]
fn visit_dir(dir: &Path) -> io::Result<Vec<String>> {
    visit_dirs(dir, dir)
}
#[allow(dead_code)]
fn visit_dirs(dir: &Path, initial: &Path) -> io::Result<Vec<String>> {
    let mut files: Vec<String> = Default::default();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut sub_files = visit_dirs(&path, initial)?;
                files.append(&mut sub_files);
            } else {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".proto") {
                        if let Some(fullname) = path.to_str() {
                            files.push(fullname.replace("\\", "/").into())
                        }
                    }
                }
            }
        }
    }
    Ok(files)
}
#[allow(dead_code)]
fn gen_module_file_for_dir(module: &str, dir: &str, files: Vec<String>) -> io::Result<()> {
    let lib_name = format!("{}.rs", dir);
    let mut out_file = File::create(&lib_name)?;
    out_file.write(b"// auto-generated\n")?;
    out_file.write(
        format!(
            "// Version {}/{}\n",
            NAME.unwrap_or("PFC"),
            VERSION.unwrap_or("dev")
        )
        .as_bytes(),
    )?;
    let len_proto = ".proto".len();
    for name in files {
        let index = name.rfind("/").unwrap_or_default();
        let end = name.len() - len_proto;
        let rs_name = &name[index + 1..end];

        out_file.write(b"pub mod ")?;
        out_file.write(rs_name.as_bytes())?;
        out_file.write(b";\n")?;
        if rs_name == "types" {
            out_file
                .write(format!("pub use {} as {}_{};\n", rs_name, module, rs_name).as_bytes())?;
            //  out_file.write(rs_name.as_bytes())?;
            //  out_file.write(b";\n")?;
        }
    }
    /*
    if !lib_name.ends_with("coin.rs") {
        out_file.write(b"pub use crate::protos::coin::coin;\n")?;
    }

     */
    Ok(())
}
fn main() -> Result<(), io::Error> {
    //return Ok(());
    // this stuff needs tweaking to get it to compile.
    /*
    fs::remove_dir_all("src/tendermint").ok();
    fs::create_dir("src/tendermint")?;

    let tendermint_modules = vec!["abci", "p2p", "crypto", "types", "version"];

    let mut out_module = File::create("src/tendermint.rs")?;
    out_module.write(b"// auto-generated\n")?;
    out_module.write(
        format!(
            "// Version {}/{}\n",
            NAME.unwrap_or("PFC"),
            VERSION.unwrap_or("dev")
        )
        .as_bytes(),
    )?;
    for module in tendermint_modules {
        let output_dir = format!("src/tendermint/{}", module);
        fs::create_dir(&output_dir)?;
        match visit_dir(Path::new(&format!(
            "tendermint/proto/tendermint/{}",
            module
        ))) {
            Ok(files) => {
                eprintln!("Module {} Files:\n{}", module, files.join("\t\n"));
                protoc_rust::Codegen::new()
                    .out_dir(&output_dir)
                    .inputs(&files)
                    .include("protos")
                    .include("tendermint/proto")
                    .include("googleapis")
                    .include("inc")
                    .run()
                    .expect("protoc");
                gen_module_file_for_dir(module, &output_dir, files)?;
                out_module.write(b"pub mod ")?;
                out_module.write(module.as_bytes())?;
                out_module.write(b";\n")?;
            }
            Err(e) => eprintln!("Err:{}", e),
        }
    }

    */
    // return Ok(());
    /*
    fs::remove_dir_all("src/cosmos").ok();
    fs::create_dir("src/cosmos")?;

    //  let cosmos_base_modules = vec!["coin"];
    let cosmos_tx_modules = vec![
        "tx",
        "base",
        "crypto",
        "auth",
        "authz",
        "distribution",
        "staking",
        "slashing",
        "vesting",
    ];

    let mut out_module = File::create("src/cosmos.rs")?;
    out_module.write(b"// auto-generated\n")?;
    out_module.write(
        format!(
            "// Version {}/{}\n",
            NAME.unwrap_or("PFC"),
            VERSION.unwrap_or("dev")
        )
        .as_bytes(),
    )?;

    for module in cosmos_tx_modules {
        let output_dir = format!("src/cosmos/{}", module);
        fs::create_dir(&output_dir)?;
        match visit_dir(Path::new(&format!("cosmos-sdk/proto/cosmos/{}", module))) {
            Ok(files) => {
                eprintln!("Module:{} Files:\n\t{}", module, files.join("\n\t"));
                protoc_rust::Codegen::new()
                    .out_dir(&output_dir)
                    .inputs(&files)
                    .include("protos")
                    .include("googleapis")
                    .include("tendermint/proto")
                    .include("cosmos-sdk/proto")
                    .include("inc")
                    .run()
                    .expect("protoc");
                gen_module_file_for_dir(module, &output_dir, files)?;
                out_module.write(b"pub mod ")?;
                out_module.write(module.as_bytes())?;
                out_module.write(b";\n")?;
            }
            Err(e) => eprintln!("Err:{}", e),
        }
    }

    //   */
    return Ok(());
    /*
       let modules = vec!["market", "oracle", "treasury", "tx", "wasm"];
       //let modules = vec!["market", "oracle", "treasury", "tx", "vesting", "wasm"];

       fs::remove_dir_all("src/protos").ok();
       fs::create_dir("src/protos")?;

       let mut out_module = File::create("src/protos.rs")?;
       out_module.write(b"// auto-generated\n")?;
       out_module.write(
           format!(
               "// Version {}/{}\n",
               NAME.unwrap_or("PFC"),
               VERSION.unwrap_or("dev")
           )
           .as_bytes(),
       )?;

       for module in modules {
           match visit_dir(Path::new(&format!("terrad/proto/terra/{}", module))) {
               Ok(files) => {
                   eprintln!("Module: {} Files:\n\t{}", module, files.join("\n\t"));
                   let output_dir = format!("src/protos/{}", module);
                   fs::create_dir(&output_dir)?;
                   protoc_rust::Codegen::new()
                       .out_dir(&output_dir)
                       .inputs(&files)
                       .include("protos")
                       .include("googleapis")
                       .include("cosmos-sdk/proto")
                       .include("inc")
                       .include("terrad/proto/")
                       .run()
                       .expect("protoc");
                   gen_module_file_for_dir(module, &output_dir, files)?
               }
               Err(e) => eprintln!("Err:{}", e),
           }
           out_module.write(b"pub mod ")?;
           out_module.write(module.as_bytes())?;
           out_module.write(b";\n")?;
       }

       Ok(())

    */
}
