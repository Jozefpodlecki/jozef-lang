use anyhow::*;
use object::{write::{Object, StandardSegment, Symbol, SymbolSection}, Architecture, BinaryFormat, Endianness, SymbolKind, SymbolScope};

use crate::{codegen::{lower, x86_64}, parser::ast::Program};

pub fn generate_binary(program: &Program, output_path: &str) -> Result<()> {
    let ir = lower::lower(program);
    let code = x86_64::emit(&ir);

    let mut obj = Object::new(
        BinaryFormat::native_object(),
        Architecture::X86_64,
        Endianness::Little,
    );

    let section_id = obj.add_section(
        obj.segment_name(StandardSegment::Text).to_vec(),
        b".text".to_vec(),
        object::SectionKind::Text,
    );

    obj.append_section_data(section_id, &code, 1);

    let symbol_id = obj.add_symbol(Symbol {
        name: b"main".to_vec(),
        value: 0,
        size: code.len() as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: SymbolSection::Section(section_id),
        flags: object::SymbolFlags::None,
    });

    let file = std::fs::File::create(output_path)?;
    obj.write_stream(file).map_err(|err| anyhow!("test"))?;

    Ok(())
}