use use_wasm::{binary, export, import, memory, section, target, value, wasi, wit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert!(binary::looks_like_wasm_binary(b"\0asm\x01\0\0\0"));
    assert_eq!(section::WasmSectionKind::try_from(5)?.to_string(), "memory");

    let limits = memory::MemoryLimits::new(
        memory::WasmPageCount::new(1),
        Some(memory::WasmPageCount::new(2)),
    )?;
    let value_type: value::WasmValueType = "i64".parse()?;
    let import_name = import::ImportName::new("memory")?;
    let export_name = export::ExportName::new("run")?;
    let wasm_target: target::WasmTarget = "wasm32-wasip1".parse()?;
    let world = wit::WitWorldName::new("cli")?;
    let capability = wasi::WasiCapabilityLabel::new("filesystem.read")?;

    assert_eq!(limits.minimum_pages(), 1);
    assert!(value_type.is_numeric());
    assert_eq!(import_name.as_str(), "memory");
    assert_eq!(export_name.as_str(), "run");
    assert_eq!(wasm_target.pointer_width(), 32);
    assert_eq!(world.as_str(), "cli");
    assert_eq!(capability.as_str(), "filesystem.read");

    Ok(())
}
