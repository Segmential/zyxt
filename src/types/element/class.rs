use std::collections::HashMap;

use smol_str::SmolStr;

use crate::{
    types::{
        element::{
            block::Block, declare::Declare, ident::Ident, procedure::Argument, Element,
            ElementData, ElementVariant, PosRaw,
        },
        interpreter_data::FrameType,
        token::Flag,
        typeobj::TypeDefinition,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Class {
    is_struct: bool,
    implementations: HashMap<SmolStr, Element>,
    inst_fields: HashMap<SmolStr, (Element<Ident>, Option<Element>)>,
    content: Element<Block>,
    args: Option<Vec<Argument>>,
}

impl ElementData for Class {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Class(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        typelist.add_frame(None, FrameType::Normal);
        for expr in self.content.data.content.iter_mut() {
            expr.process(typelist)?;
            if let ElementVariant::Declare(Declare {
                variable,
                content,
                flags,
                type_,
                ..
            }) = &*expr.data
            {
                if flags.contains(&Flag::Inst) && self.args != &None {
                    todo!("raise error here")
                }
                if flags.contains(&Flag::Inst) {
                    self.inst_fields.insert(
                        variable.get_name(),
                        (*type_.to_owned(), Some(content.to_owned())),
                    );
                }
            }
        }
        if self.args.is_some() && self.implementations.contains_key("_init") {
            todo!("raise error here")
        }
        for item in self.implementations.values_mut() {
            item.process(typelist)?;
        }
        let new_inst_fields = self
            .inst_fields
            .iter_mut()
            .map(|(ident, (ty, default))| {
                let ty = ty.process(typelist)?;
                if let Some(default) = default {
                    if ty != default.process(typelist)? {
                        todo!("raise error")
                    }
                }
                Ok((
                    ident.to_owned(),
                    (Box::new(ty), default.to_owned().map(|a| *a)),
                ))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        typelist.pop_frame();
        Ok(Type::Definition(TypeDefinition {
            inst_name: None,
            name: Some(if self.is_struct { "struct" } else { "class" }.into()),
            generics: vec![],
            implementations: self.implementations.to_owned(),
            inst_fields: new_inst_fields,
        }))
    }

    fn desugared(
        &self,
        pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariant, ZyxtError> {
        let mut new_self = self.to_owned();
        new_self.content = Element {
            pos_raw: pos_raw.to_owned(),
            data: self.content.desugared(out)?.as_block().unwrap(),
        };
        new_self
            .args
            .map(|args| {
                args.into_iter()
                    .map(|mut arg| {
                        arg.desugar(pos_raw, out)?;
                        Ok(arg)
                    })
                    .collect()
            })
            .transpose()?;
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        Ok(Value::Type(Type::Definition(TypeDefinition {
            name: Some(if self.is_struct { "struct" } else { "class" }.into()),
            inst_name: None,
            generics: vec![],
            implementations: self
                .implementations
                .iter()
                .map(|(k, v)| Ok((k.to_owned(), v.interpret_expr(i_data)?)))
                .collect::<Result<HashMap<_, _>, _>>()?,
            inst_fields: self
                .inst_fields
                .iter()
                .map(|(k, (v1, v2))| {
                    Ok((
                        k.to_owned(),
                        (
                            Box::new(if let Value::Type(value) = v1.interpret_expr(i_data)? {
                                value
                            } else {
                                panic!()
                            }),
                            v2.to_owned()
                                .map(|v2| v2.interpret_expr(i_data))
                                .transpose()?,
                        ),
                    ))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
        })))
    }
}
