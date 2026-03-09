use syn::parse::{ParseStream, Result};
use syn::{Ident, LitFloat, LitInt, LitStr, Token};

use crate::parsers::var::parse_indices;
use crate::types::{OpArg, OpAttrValue, OpSetting, VarRef};

pub(crate) fn parse_op_arg(input: ParseStream) -> Result<OpArg> {
    let name: Ident = input.parse()?;
    if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        let value = parse_op_attr_value(input)?;
        Ok(OpArg::Setting(OpSetting { name, value }))
    } else if input.peek(syn::token::Bracket) {
        let indices = parse_indices(input)?;
        Ok(OpArg::Input(VarRef { name, indices }))
    } else {
        Ok(OpArg::Input(VarRef {
            name,
            indices: Vec::new(),
        }))
    }
}

fn is_dtype_ident(name: &str) -> bool {
    matches!(
        name,
        "i4" | "i8" | "i16" | "i32" | "i64" | "u4" | "u8" | "u16" | "u32" | "u64"
            | "f8" | "bf16" | "f16" | "f32" | "f64" | "bool"
    )
}

pub(crate) fn parse_op_attr_value(input: ParseStream) -> Result<OpAttrValue> {
    if input.peek(syn::token::Bracket) {
        let content;
        syn::bracketed!(content in input);
        if content.is_empty() {
            return Ok(OpAttrValue::IntList(Vec::new()));
        }
        // Peek first token: DTypeList starts with ident (i32, f32) or LitFloat; IntList with number or minus
        let fork = content.fork();
        let is_dtype_list = fork.peek(Ident) || fork.peek(LitFloat);
        drop(fork);

        if !is_dtype_list {
            let mut values = Vec::new();
            while !content.is_empty() {
                let negative = if content.peek(Token![-]) {
                    content.parse::<Token![-]>()?;
                    true
                } else {
                    false
                };
                let lit: LitInt = content.parse()?;
                let mut value: i64 = lit.base10_parse()?;
                if negative {
                    value = -value;
                }
                values.push(value);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
            return Ok(OpAttrValue::IntList(values));
        }
        // Parse as DTypeList (acc=[i32, i64] or acc=[f32, f32] etc)
        let mut dtypes = Vec::new();
        while !content.is_empty() {
            let ident = if content.peek(Ident) {
                content.parse::<Ident>()?
            } else if content.peek(LitFloat) {
                let lit: LitFloat = content.parse()?;
                let s = lit.to_string();
                // "f32" can be lexed as float literal 32f32 -> "32f32"; "f64" -> "64f64" or similar
                let dtype = if s.ends_with("f32") {
                    "f32"
                } else if s.ends_with("f64") {
                    "f64"
                } else if s == "f32" || s == "f64" {
                    s.as_str()
                } else {
                    return Err(syn::Error::new(
                        lit.span(),
                        format!("expected dtype (e.g. i32, f32), got {}", s),
                    ));
                };
                Ident::new(dtype, lit.span())
            } else {
                return Err(content.error("expected dtype identifier (e.g. i32, f32)"));
            };
            let name = ident.to_string();
            if !is_dtype_ident(&name) {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("expected dtype (e.g. i32, f32), got {}", name),
                ));
            }
            dtypes.push(ident);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }
        return Ok(OpAttrValue::DTypeList(dtypes));
    }
    let negative = if input.peek(Token![-]) {
        input.parse::<Token![-]>()?;
        true
    } else {
        false
    };

    if input.peek(syn::LitFloat) {
        let lit: syn::LitFloat = input.parse()?;
        let mut value: f64 = lit.base10_parse()?;
        if negative {
            value = -value;
        }
        return Ok(OpAttrValue::Double(value));
    }
    if input.peek(syn::LitBool) {
        let lit: syn::LitBool = input.parse()?;
        if negative {
            return Err(input.error("unexpected '-' before bool literal"));
        }
        return Ok(OpAttrValue::Bool(lit.value));
    }
    if input.peek(LitInt) {
        let lit: LitInt = input.parse()?;
        let mut value: i64 = lit.base10_parse()?;
        if negative {
            value = -value;
        }
        return Ok(OpAttrValue::Int(value));
    }
    if input.peek(LitStr) {
        let lit: LitStr = input.parse()?;
        if negative {
            return Err(input.error("unexpected '-' before string literal"));
        }
        return Ok(OpAttrValue::String(lit.value()));
    }
    if input.peek(Ident) {
        let ident: Ident = input.parse()?;
        if negative {
            return Err(input.error("unexpected '-' before identifier"));
        }
        let name = ident.to_string();
        if name == "true" {
            return Ok(OpAttrValue::Bool(true));
        }
        if name == "false" {
            return Ok(OpAttrValue::Bool(false));
        }
        if name == "inf" {
            return Ok(OpAttrValue::Double(f64::INFINITY));
        }
        return Ok(OpAttrValue::Var(ident));
    }

    Err(input.error("expected literal or identifier for op setting"))
}
