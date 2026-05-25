use use_wasm::{
    binary, component, export, feature, function, import, memory, module, section, target, text,
    value, wasi, wit,
};

#[test]
fn facade_exposes_child_crate_aliases() -> Result<(), Box<dyn std::error::Error>> {
    assert!(binary::looks_like_wasm_binary(b"\0asm\x01\0\0\0"));
    assert_eq!(
        section::WasmSectionKind::try_from(7)?,
        section::WasmSectionKind::Export
    );
    assert!(text::looks_like_wat_module("(module)"));

    let module_name = module::ModuleName::new("example")?;
    let import_name = import::ImportName::new("memory")?;
    let export_name = export::ExportName::new("run")?;
    let limits = memory::MemoryLimits::new(memory::WasmPageCount::new(1), None)?;
    let signature = function::FunctionSignature::new(
        function::ParameterList::new(vec![function::FunctionValueType::I32]),
        function::ResultList::empty(),
    );
    let value_type: value::WasmValueType = "externref".parse()?;
    let wasm_target: target::WasmTarget = "wasm32-unknown-unknown".parse()?;
    let wasm_feature: feature::WasmFeature = "component model".parse()?;
    let component_world = component::WorldName::new("cli")?;
    let wit_world = wit::WitWorldName::new("cli")?;
    let wasi_version: wasi::WasiVersion = "wasip1".parse()?;

    assert_eq!(module_name.as_str(), "example");
    assert_eq!(import_name.as_str(), "memory");
    assert_eq!(export_name.as_str(), "run");
    assert_eq!(limits.minimum_pages(), 1);
    assert_eq!(signature.params().len(), 1);
    assert!(value_type.is_reference());
    assert_eq!(wasm_target.family(), "wasm32");
    assert_eq!(wasm_feature.to_string(), "component-model");
    assert_eq!(component_world.as_str(), "cli");
    assert_eq!(wit_world.as_str(), "cli");
    assert_eq!(wasi_version, wasi::WasiVersion::Preview1);

    Ok(())
}
