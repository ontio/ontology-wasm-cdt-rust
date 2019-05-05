use parity_wasm::elements::opcodes::*;
use parity_wasm::elements::{
    External, FunctionType, ImportEntry, InitExpr, Instruction, ValueType,
};
use std::collections::btree_map::BTreeMap;

const VALID_FIELD: [&str; 19] = [
    "timestamp",
    "block_height",
    "self_address",
    "caller_address",
    "entry_address",
    "check_witness",
    "ret",
    "notify",
    "input_length",
    "get_input",
    "call_contract",
    "call_output_length",
    "get_output",
    "current_blockhash",
    "current_txhash",
    "contract_migrate",
    "storage_read",
    "storage_write",
    "storage_delete",
];

pub fn get_import_func_index(import_entry: &ImportEntry) -> Result<BTreeMap<&u32, &str>, String> {
    if import_entry.module() == "env" {
        if !VALID_FIELD.contains(&import_entry.field()) {
            return Err(format!("import section use invalid field: {}", import_entry.field()));
        }
        let mut func_index: BTreeMap<&u32, &str> = BTreeMap::new();
        match import_entry.external() {
            External::Function(index) => {
                func_index.insert(index, import_entry.field());
            }
            _ => {}
        }
        return Ok(func_index);
    }
    return Ok(BTreeMap::new());
}
pub fn check_type(func_name: &str, func_type: &FunctionType) -> Result<(), String> {
    match func_name {
        "timestamp" => {
            let expected_params: &[ValueType] = &[];
            check_signature(func_name, expected_params, Some(ValueType::I64), func_type)?;
        }
        "block_height" | "input_length" | "call_output_length" => {
            let expected_params: &[ValueType] = &[];
            check_signature(func_name, expected_params, Some(ValueType::I32), func_type)?;
        }
        "self_address" | "caller_address" | "entry_address" | "get_input" | "get_output" => {
            let expected_params: &[ValueType] = &[ValueType::I32];
            check_signature(func_name, expected_params, None, func_type)?;
        }
        "check_witness" | "current_blockhash" | "current_txhash" => {
            let expected_params: &[ValueType] = &[ValueType::I32];
            check_signature(func_name, expected_params, Some(ValueType::I32), func_type)?;
        }
        "ret" => {
            let expected_params: &[ValueType] = &[ValueType::I32, ValueType::I32];
            check_signature(func_name, expected_params, None, func_type)?;
        }
        "call_contract" => {
            let expected_params: &[ValueType] = &[ValueType::I32, ValueType::I32, ValueType::I32];
            check_signature(func_name, expected_params, Some(ValueType::I32), func_type)?;
        }
        "contract_migrate" => {
            let expected_params: &[ValueType] = &[
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
                ValueType::I32,
            ];
            check_signature(func_name, expected_params, Some(ValueType::I32), func_type)?;
        }
        "storage_read" => {
            let expected_params: &[ValueType] =
                &[ValueType::I32, ValueType::I32, ValueType::I32, ValueType::I32, ValueType::I32];
            check_signature(func_name, expected_params, Some(ValueType::I32), func_type)?;
        }
        "storage_write" => {
            let expected_params: &[ValueType] =
                &[ValueType::I32, ValueType::I32, ValueType::I32, ValueType::I32];
            check_signature(func_name, expected_params, None, func_type)?;
        }
        "storage_delete" => {
            let expected_params: &[ValueType] = &[ValueType::I32, ValueType::I32];
            check_signature(func_name, expected_params, None, func_type)?;
        }
        _ => {
            return Ok(());
        }
    }
    return Ok(());
}

fn check_signature(
    func_name: &str, expected_param: &[ValueType], expected_ret: Option<ValueType>,
    func_type: &FunctionType,
) -> Result<(), String> {
    if expected_param != func_type.params() {
        return Err(format!(
            "function name:{}, parameter expected {:?}, got {:?}",
            func_name,
            expected_param,
            func_type.params()
        ));
    } else if expected_ret != func_type.return_type() {
        return Err(format!(
            "function name: {}, return value type expected {}, got {}",
            func_name,
            expected_ret.unwrap(),
            func_type.return_type().unwrap()
        ));
    } else {
        return Ok(());
    }
}

pub fn is_invalid_instruction(instruction: &Instruction) -> bool {
    let instruction_str = format!("{}", instruction);
    if instruction_str.contains("f32") || instruction_str.contains("f64") {
        return true;
    }
    false
}

pub fn is_invalid_value_type(value_type: &ValueType) -> bool {
    match value_type {
        ValueType::F32 | ValueType::F64 => return true,
        _ => return false,
    }
}
pub fn is_invalid_init_expr(init_expr: &InitExpr) -> Result<(), String> {
    for expr in init_expr.code() {
        if is_invalid_instruction(&expr) {
            return Err(format!("invalid expr: {}", expr));
        }
    }
    Ok(())
}
