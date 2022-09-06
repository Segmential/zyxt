use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    binary, concat_vals, get_param, typecast_to_type,
    types::{
        typeobj::{bool_t::BOOL_T, str_t::STR_T, type_t::TYPE_T},
        value::{Proc, Value},
    },
    Type,
};

macro_rules! comp_opr_unit {
    ($h:ident, $fn_name:literal, $res:literal) => {
        binary!($h, UNIT_T, $fn_name, [UNIT_T], BOOL_T, |x: &Vec<Value>| {
            Some(Value::Bool($res))
        });
    };
}

const fn unit_t() -> HashMap<&'static str, Value> {
    let mut h = HashMap::new();
    concat_vals!(h, UNIT_T);
    comp_opr_unit!(h, "_eq", true);
    comp_opr_unit!(h, "_neq", false);
    comp_opr_unit!(h, "_gt", false);
    comp_opr_unit!(h, "_ge", true);
    comp_opr_unit!(h, "_lt", false);
    comp_opr_unit!(h, "_le", true);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            Type::Instance { name, .. } => match &*name {
                "type" => typecast_to_type!(UNIT_T),
                "str" => Value::Str("()".into()),
                _ => return None,
            },
            _ => unimplemented!(),
        })
    };
    binary!(h, UNIT_T, "_typecast", [TYPE_T], Type::Any, typecast);

    h.drain().map(|(k, v)| (k, Value::Proc(v))).collect()
}

lazy_static! {
    pub static ref UNIT_T: Type<Value> = Type::Definition {
        inst_name: Some("_unit".into()),
        generics: vec![],
        implementations: unit_t(),
        inst_fields: HashMap::new(),
    };
}
