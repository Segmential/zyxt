use std::collections::HashMap;

use half::f16;
use lazy_static::lazy_static;
use smol_str::SmolStr;

use crate::{
    arith_opr_num, binary, comp_opr_num, concat_vals, get_param, typecast_int, typecast_to_type,
    types::{
        typeobj::{
            bool_t::BOOL_T, f16_t::F16_T, f32_t::F32_T, f64_t::F64_T, i128_t::I128_T, i16_t::I16_T,
            i32_t::I32_T, i64_t::I64_T, i8_t::I8_T, ibig_t::IBIG_T, isize_t::ISIZE_T, str_t::STR_T,
            type_t::TYPE_T, u128_t::U128_T, u16_t::U16_T, u32_t::U32_T, u64_t::U64_T,
            ubig_t::UBIG_T, usize_t::USIZE_T, TypeDefinition,
        },
        value::{Proc, Value},
    },
    unary, Type,
};

fn u8_t() -> HashMap<SmolStr, Value> {
    let mut h = HashMap::new();
    h.insert("_default", Value::U8(0));
    concat_vals!(h, U8_T);
    unary!(h, signed default U8_T U8);
    arith_opr_num!(h, default U8_T U8);
    comp_opr_num!(h, default U8_T U8);

    let typecast = |x: &Vec<Value>| {
        Some(match get_param!(x, 1, Type) {
            p if p == TYPE_T.to_type() => typecast_to_type!(U8_T),
            p if p == STR_T.to_type() => typecast_int!(U8 => str, x),
            p if p == BOOL_T.to_type() => typecast_int!(U8 => bool, x),
            p if p == I8_T.to_type() => typecast_int!(U8 => I8, x),
            p if p == I16_T.to_type() => typecast_int!(U8 => I16, x),
            p if p == I32_T.to_type() => typecast_int!(U8 => I32, x),
            p if p == I64_T.to_type() => typecast_int!(U8 => I64, x),
            p if p == I128_T.to_type() => typecast_int!(U8 => I128, x),
            p if p == ISIZE_T.to_type() => typecast_int!(U8 => Isize, x),
            p if p == IBIG_T.to_type() => typecast_int!(U8 => Ibig, x),
            p if p == U8_T.to_type() => x[0].to_owned(),
            p if p == U16_T.to_type() => typecast_int!(U8 => U16, x),
            p if p == U32_T.to_type() => typecast_int!(U8 => U32, x),
            p if p == U64_T.to_type() => typecast_int!(U8 => U64, x),
            p if p == U128_T.to_type() => typecast_int!(U8 => U128, x),
            p if p == USIZE_T.to_type() => typecast_int!(U8 => Usize, x),
            p if p == UBIG_T.to_type() => typecast_int!(U8 => Ubig, x),
            p if p == F16_T.to_type() => typecast_int!(U8 => f16, x),
            p if p == F32_T.to_type() => typecast_int!(U8 => f32, x),
            p if p == F64_T.to_type() => typecast_int!(U8 => f64, x),
            _ => return None,
        })
    };
    binary!(
        h,
        U8_T.to_type(),
        "_typecast",
        [TYPE_T.to_type()],
        Type::Any,
        typecast
    );

    h.drain().map(|(k, v)| (k.into(), v)).collect()
}

lazy_static! {
    pub static ref U8_T: TypeDefinition<Value> = TypeDefinition {
        name: Some("{builtin u8}".into()),
        inst_name: Some("u8".into()),
        generics: vec![],
        implementations: u8_t(),
        inst_fields: HashMap::new(),
    };
}
