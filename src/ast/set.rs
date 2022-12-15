use crate::{
    ast::{Ast, AstData},
    types::position::{GetSpan, Span},
    SymTable, Type, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Set {
    pub variable: Box<Ast>,
    pub eq_span: Option<Span>,
    pub content: Box<Ast>,
}
impl GetSpan for Set {
    fn span(&self) -> Option<Span> {
        self.variable
            .merge_span(&self.eq_span)
            .merge_span(&self.content)
    }
}

impl AstData for Set {
    fn as_variant(&self) -> Ast {
        Ast::Set(self.to_owned())
    }

    fn process(&mut self, ty_symt: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        if !self.variable.is_pattern() {
            return Err(ZError::t006().with_span(&*self.variable));
        }
        let content_type = self.content.process(ty_symt)?;
        let name = if let Ast::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        let var_type = ty_symt.get_val(
            name,
            &self.variable.span().unwrap_or_else(|| unreachable!()),
        )?;
        if content_type == var_type {
            Ok(var_type)
        } else {
            Err(ZError::t010(&var_type, &content_type).with_span(&*self)) // TODO span
        }
    }

    fn desugared(&self) -> ZResult<Ast> {
        let mut new_self = self.to_owned();
        new_self.content = self.content.desugared()?.into();
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        let var = self.content.interpret_expr(val_symt);
        let name = if let Ast::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        val_symt.set_val(name, &var.to_owned()?, self)?;
        var
    }
}
