use crate::ast::Rule;

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_integer))]
pub enum IntegerType {
    U8Type(U8Type),
    U16Type(U16Type),
    U32Type(U32Type),
    U64Type(U64Type),
    U128Type(U128Type),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u8))]
pub struct U8Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u16))]
pub struct U16Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u32))]
pub struct U32Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u64))]
pub struct U64Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u128))]
pub struct U128Type {}