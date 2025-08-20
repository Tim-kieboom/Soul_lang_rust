use serde::{Deserialize, Serialize};
use crate::errors::soul_error::{new_soul_error, Result, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::Statement;
use crate::{steps::step_interfaces::i_parser::abstract_syntax_tree::statement::Block, utils::multi_ref::rc::MultiRef};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractSyntacTree {
    pub root: Block,
}

#[derive(Debug, Clone)]
pub struct BlockBuilder {
    pub block: MultiRef<Spanned<Block>>,
}

impl BlockBuilder {
    pub fn new(span: SoulSpan) -> Self {
        Self { block: MultiRef::new(Spanned::new(Block::new(), span)) }
    }

    pub fn push_global(&mut self, statment: Statement) -> Result<()> {
        match &statment.node {
            super::statement::StatementKind::CloseBlock => return Err(new_soul_error(SoulErrorKind::InvalidInContext, statment.span, "can not have CloseBlock in global scope")),
            super::statement::StatementKind::Assignment(_) => return Err(new_soul_error(SoulErrorKind::InvalidInContext, statment.span, "can not have Assignment in global scope")),
            super::statement::StatementKind::Expression(spanned) => return Err(new_soul_error(SoulErrorKind::InvalidInContext, statment.span, format!("can not have '{}' expression in global scope", spanned.node.get_variant_name()))),
            _ => (),
        }

        self.push(statment);
        
        Ok(())
    }

    pub fn push(&mut self, statment: Statement) {
        let mut block = self.block.borrow_mut();
        
        block.span.combine(&statment.span);
       
        block.node
            .statments
            .push(statment);
    }

    pub fn into_block(self) -> Spanned<Block> {
        match self.block.try_consume() {
            Ok(val) => val,
            Err(multi_ref) => multi_ref.borrow().clone(),
        }
    }
}

impl AbstractSyntacTree {
    pub fn new(root: Block) -> Self {
        Self{root}
    }
}







