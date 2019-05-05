extern crate parity_wasm;
extern crate wasmi;
use super::common;
use parity_wasm::elements::{
    CodeSection, DataSection, ElementSection, External, FuncBody, FunctionType, GlobalEntry,
    GlobalSection, ImportEntry, ImportSection, InitExpr, Instruction, Local, Module, Type,
    ValueType,
};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::vec::Vec;

fn check_code_section(code_section: &CodeSection) -> Result<(), String> {
    let bodies: &[FuncBody] = code_section.bodies();
    for body in bodies {
        let locals = body.locals();
        for local in locals {
            if common::is_invalid_value_type(&local.value_type()) {
                return Err(format!("invalid value type: {}", local.value_type()));
            }
        }
        //        let instructions = body.code();
        //        for instruction in instructions.elements() {
        //            if common::is_invalid_instruction(&instruction) {
        //                return Err(format!("invalid instruction: {}", instruction));
        //            }
        //        }
    }
    return Ok(());
}
fn check_global_section(global_section: &GlobalSection) -> Result<(), String> {
    let entries = global_section.entries();
    for entry in entries {
        if common::is_invalid_value_type(&entry.global_type().content_type()) {
            return Err(format!(
                "global type content type is invalid: {}",
                &entry.global_type().content_type()
            ));
        }
        let res = common::is_invalid_init_expr(entry.init_expr());
        if res.is_err() {
            return Err(format!("global type content type is invalid: {}", &entry.init_expr()));
        }
    }
    return Ok(());
}
fn check_import_section(import_section: &ImportSection, module: &Module) -> Result<(), String> {
    let import_sections = import_section.entries();
    for import_entry in import_sections {
        match import_entry.external() {
            External::Global(external_gloal_type) => {
                if common::is_invalid_value_type(&external_gloal_type.content_type()) {
                    return Err(format!(
                        "import section use invalid value type: {}",
                        external_gloal_type.content_type()
                    ));
                }
            }
            _ => {}
        };
        let func_index_temp = common::get_import_func_index(import_entry);
        match func_index_temp {
            Ok(func_index) => {
                if let Some(type_section) = module.type_section() {
                    let types = type_section.types();
                    let mut index = 0u32;
                    for ty in types {
                        match ty {
                            Type::Function(t) => {
                                let param = t.params();
                                for value_type in param {
                                    if common::is_invalid_value_type(value_type) {
                                        return Err(format!(
                                            "invalid function parameter type: {}",
                                            value_type
                                        ));
                                    }
                                }
                                if let Some(ret_type) = t.return_type() {
                                    if common::is_invalid_value_type(&ret_type) {
                                        return Err(format!(
                                            "invalid function return type: {}",
                                            ret_type
                                        ));
                                    }
                                }
                                if let Some(func_name) = func_index.get(&index) {
                                    let res = common::check_type(func_name, t);
                                    match res {
                                        Ok(_) => {}
                                        Err(e) => {
                                            return Err(format!("check_type failed: {}", e));
                                        }
                                    }
                                }
                                index += 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => return Err(format!("get_import_func_index failed: {}", e)),
        }
    }
    return Ok(());
}

fn check_data_section(data_section: &DataSection) -> Result<(), String> {
    let data_segments = data_section.entries();
    for data_segment in data_segments {
        if let init_expr = data_segment.offset() {
            let res = common::is_invalid_init_expr(init_expr);
            if res.is_err() {
                return res;
            }
        }
    }
    return Ok(());
}
fn check_element_section(elements_section: &ElementSection) -> Result<(), String> {
    let entries = elements_section.entries();
    for entry in entries {
        if let init_expr = entry.offset() {
            let res = common::is_invalid_init_expr(init_expr);
            if res.is_err() {
                return res;
            }
        }
    }
    return Ok(());
}
pub fn check_module(module: &Module) -> Result<(), String> {
    let wasmi_module = wasmi::Module::from_parity_wasm_module(module.into())
        .expect("wasmi from_parity_wasm_module failed");
    let float_check = wasmi_module.deny_floating_point();
    if float_check.is_err() {
        return Err(format!("float check failed"));
    }
    if let Some(code_section) = module.code_section() {
        let res = check_code_section(code_section);
        if res.is_err() {
            return res;
        }
    }
    if let Some(global_section) = module.global_section() {
        let res = check_global_section(global_section);
        if res.is_err() {
            return res;
        }
    }
    if let Some(import_section) = module.import_section() {
        let res = check_import_section(import_section, module);
        if res.is_err() {
            return res;
        }
    }
    if let Some(data_section) = module.data_section() {
        let res = check_data_section(data_section);
        if res.is_err() {
            return res;
        }
    }
    if let Some(elements_section) = module.elements_section() {
        let res = check_element_section(elements_section);
        if res.is_err() {
            return res;
        }
    }
    return Ok(());
}

pub fn check(p: &str) -> Result<(), String> {
    let modul = parity_wasm::deserialize_file(p);
    if modul.is_err() {
        return Err("parity_wasm::deserialize_file error".to_string());
    }
    return check_module(&modul.unwrap());
}
