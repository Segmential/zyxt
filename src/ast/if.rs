use crate::{
    ast::{Ast, AstData, Condition},
    types::position::{GetSpan, Span},
    SymTable, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct If {
    pub conditions: Vec<Condition>,
}
impl GetSpan for If {
    fn span(&self) -> Option<Span> {
        self.conditions.span()
    }
}

impl AstData for If {
    fn as_variant(&self) -> Ast {
        Ast::If(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn process(&mut self, ty_symt: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        Ok(self.conditions[0].if_true.block_type(ty_symt, true)?.0)
        // TODO consider all returns
    }

    fn desugared(&self) -> ZResult<Ast> {
        Ok(Self {
            conditions: self
                .conditions
                .iter()
                .map(|a| {
                    let mut a = a.to_owned();
                    a.desugar()?;
                    Ok(a)
                })
                .collect::<Result<_, _>>()?,
        }
        .as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        for cond in &self.conditions {
            if cond.condition.is_none()
                || cond
                    .condition
                    .as_ref()
                    .map(|cond| cond.interpret_expr(val_symt))
                    .transpose()?
                    == Some(Value::Bool(true))
            {
                return cond.if_true.interpret_block(val_symt, false, true);
            }
        }
        Ok(Value::Unit)
    }
}
