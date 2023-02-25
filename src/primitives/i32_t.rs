use std::collections::HashMap;

use half::f16;
use once_cell::sync::Lazy;
use smol_str::SmolStr;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param,
    primitives::*,
    typecast_int, typecast_to_type,
    types::{
        typeobj::TypeDefinition,
        value::{Proc, Value},
    },
    unary, Type,
};

#[allow(clippy::cognitive_complexity, clippy::float_cmp)]
fn i32_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::I32(0));
    concat_vals!(h, I32_T);
    unary!(h, signed default I32_T I32);
    arith_opr_num!(h, default I32_T I32);
    comp_opr_num!(h, default I32_T I32);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.as_type() => typecast_to_type!(I32_T),
            p if p == STR_T.as_type() => typecast_int!(I32 => str, x),
            p if p == BOOL_T.as_type() => typecast_int!(I32 => bool, x),
            p if p == I8_T.as_type() => typecast_int!(I32 => I8, x),
            p if p == I16_T.as_type() => typecast_int!(I32 => I16, x),
            p if p == I32_T.as_type() => x[0].to_owned(),
            p if p == I64_T.as_type() => typecast_int!(I32 => I64, x),
            p if p == I128_T.as_type() => typecast_int!(I32 => I128, x),
            p if p == ISIZE_T.as_type() => typecast_int!(I32 => Isize, x),
            p if p == IBIG_T.as_type() => typecast_int!(I32 => Ibig, x),
            p if p == U8_T.as_type() => typecast_int!(I32 => U8, x),
            p if p == U16_T.as_type() => typecast_int!(I32 => U16, x),
            p if p == U32_T.as_type() => typecast_int!(I32 => U32, x),
            p if p == U64_T.as_type() => typecast_int!(I32 => U64, x),
            p if p == U128_T.as_type() => typecast_int!(I32 => U128, x),
            p if p == USIZE_T.as_type() => typecast_int!(I32 => Usize, x),
            p if p == UBIG_T.as_type() => typecast_int!(I32 => Ubig, x),
            p if p == F16_T.as_type() => typecast_int!(I32 => f16, x),
            p if p == F32_T.as_type() => typecast_int!(I32 => f32, x),
            p if p == F64_T.as_type() => typecast_int!(I32 => f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        I32_T.as_type(),
        "_typecast",
        [TYPE_T.as_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

pub static I32_T: Lazy<TypeDefinition<Value>> = Lazy::new(|| TypeDefinition {
    name: Some("{builtin i32}".into()),
    inst_name: Some("i32".into()),
    generics: vec![],
    implementations: i32_t(),
    inst_fields: HashMap::new(),
});
