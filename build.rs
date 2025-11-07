use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    println!("cargo:rustc-check-cfg=cfg(build_with_asm)");
    println!("cargo:rustc-check-cfg=cfg(build_without_asm)");

    // Flag opcional para montar assembly. Evita falha quando ferramentas (ml64.exe) não estão instaladas.
    let build_asm = env::var("BUILD_ASM")
        .ok()
        .map(|v| v == "1")
        .unwrap_or(false);

    let mut asm_enabled = false;

    if target.contains("x86_64") && build_asm {
        println!("cargo:rerun-if-changed=src/asm/xml_parser_x86_64.asm");
        if cfg!(target_os = "windows") {
            // Tenta compilar para MSVC; se falhar, emite warning e segue sem abortar.
            let res = std::panic::catch_unwind(|| {
                cc::Build::new()
                    .file("src/asm/xml_parser_x86_64_msvc.asm")
                    .flag("/nologo")
                    .compile("xml_parser_asm");
            });
            if res.is_err() {
                println!("cargo:warning=Falha ao compilar assembly MSVC. Continuando sem otimização ASM.");
            } else {
                asm_enabled = true;
            }
        } else {
            let res = std::panic::catch_unwind(|| {
                cc::Build::new()
                    .file("src/asm/xml_parser_x86_64_gas.s")
                    .compile("xml_parser_asm");
            });
            if res.is_err() {
                println!(
                    "cargo:warning=Falha ao compilar assembly GAS. Continuando sem otimização ASM."
                );
            } else {
                asm_enabled = true;
            }
        }
    } else {
        println!("cargo:warning=Assembly desativado (defina BUILD_ASM=1 para habilitar ou instale toolchain). Skipping ASM build.");
    }

    if asm_enabled {
        println!("cargo:rustc-cfg=build_with_asm");
    } else {
        println!("cargo:rustc-cfg=build_without_asm");
    }

    println!("cargo:rerun-if-changed=build.rs");
    // Dica para fallback GNU se estiver sem MSVC
    if target.contains("windows") && std::env::var("BUILD_ASM").unwrap_or_default() == "1" {
        println!("cargo:warning=Se ocorrer erro de linkagem MSVC, considere instalar Visual Studio Build Tools ou usar target x86_64-pc-windows-gnu em rust-toolchain.toml.");
    }
}
