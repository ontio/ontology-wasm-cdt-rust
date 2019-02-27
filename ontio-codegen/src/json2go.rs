use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::io::BufWriter;

#[derive(Serialize, Deserialize)]
pub(crate) struct Abi {
    #[serde(rename = "CompilerVersion")]
    compiler_version: String,
    hash:String,
    #[serde(rename = "entrypoint")]
    entry_point:String,
    functions:Vec<Function>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Function {
    name:String,
    parameters:Vec<Parameters>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Parameters {
    name:String,
    #[serde(rename = "type")]
    p_type:String,
}

pub(crate) fn parse_json_to_go(file_path: String) {
    let struct_name = generate_go_struct_name(file_path.clone());
    let abi = read_file(&file_path);
    generate_go_file(struct_name, abi);
}

pub(crate) fn generate_go_file(go_struct_name:String, abi: Abi) -> std::io::Result<()> {
    let buf = include_str!("template.go");
    let buf_new = buf.replace("DemoContract", &go_struct_name);
    let file_new = File::create(format!("{}{}",go_struct_name, ".go".to_string())).unwrap();
    let mut f_out = BufWriter::new(file_new);
    f_out.write(buf_new.as_bytes())?;
    let function_str = "func (this *DemoContract) FunctionName(parameters) (*types.MutableTransaction, error) {
	bs, err := this.buildParams(\"function_name\", []interface{}{parameter_name})
	if err != nil {
		return nil, fmt.Errorf(\"buildparams failed:s%\", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(this.gasPrice, this.gasLimit, bs)
	return tx, nil
}";
    for func in abi.functions {
        let mut function_str_new = function_str
            .replace("FunctionName", &first_char_to_upper(func.name.clone()));
        let params = build_params(func.parameters);
        function_str_new = function_str_new
            .replace("parameters", &params.0)
            .replace("parameter_name", &params.1)
            .replace("function_name", &func.name);
        f_out.write(function_str_new.as_bytes())?;
        f_out.write("\n".as_bytes())?;
    }
    f_out.flush()
}

fn build_params(params:Vec<Parameters>) -> (String, String) {
    let mut res = String::new();
    let mut param_names = String::new();
    for param in params {
        if param_names == "" {
            param_names = format!("{}", param.name);
        } else {
            param_names = format!("{}, {}", param_names, param.name);
        }
        match param.p_type.as_str() {
            "" => {}
            "Address" => {
                if res == "" {
                    res = format!("{} common.Address", param.name);
                } else {
                    res = format!("{}, {} common.Address", res, param.name);
                }
            }
            "String" => {
                if res == "" {
                    res = format!("{} string", param.name);
                } else {
                    res = format!("{}, {} string", res, param.name);
                }
            }
            "U256" => {
                if res == "" {
                    res = format!("{} U256", param.name);
                } else {
                    res = format!("{}, {} U256", res, param.name);
                }
            }
            &_ => {
                panic!("not supported type");
            }
        }
    }
    (res, param_names)
}
pub fn generate_go_struct_name(file_path: String) -> String {
    let v:Vec<&str> = file_path.split(|c|c=='/' || c=='.').collect();
    if v.len() < 2 {
        panic!("file path is wrong:{}", file_path);
    }
    let file_name = v[v.len()-2];
    let res = first_char_to_upper(file_name.to_string());
    format!("{}{}", res, "Contract")
}
pub fn first_char_to_upper(temp:String) ->String {
    let t_upper = temp.to_uppercase();
    let mut t_string = temp.to_string();
    let t_bs;
    unsafe {
        t_bs = t_string.as_bytes_mut();
    }
    t_bs[0] = t_upper.as_bytes()[0];
    let res = match str::from_utf8(t_bs) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    res.to_string()
}
fn read_file(file_path: &str) -> Abi {
    match File::open(&Path::new(file_path)) {
        Ok(mut f) => {
            let mut buf = String::new();
            f.read_to_string(&mut buf).expect("something went wrong reading the file");
            let abi:Abi = serde_json::from_str(&buf).unwrap();
            abi
        }
        Err(e) => {
            panic!("open file failed: {}, err: {}", file_path, e);
        }
    }
}

#[test]
fn it_works() {
    parse_json_to_go("./oep4_abi.json".to_string());
    assert_eq!(2 + 2, 4);
}
