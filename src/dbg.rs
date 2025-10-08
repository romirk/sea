use crate::hir::{
    Binding, Decl, Expr, FnDefn, MonoDecl, TopDefn, Type, TypeDefn, VarDefn,
};
use std::fmt::{Debug, Pointer};
use std::fmt::{Display, Formatter};

impl Debug for TopDefn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TopDefn::Fn(dfn) => dfn.fmt(f),
            TopDefn::Var(var) => var.fmt(f),
            TopDefn::Type(ty) => ty.fmt(f),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Char { signed } => Self::write_signed(f, "char", signed),
            Type::Short { signed } => Self::write_signed(f, "short", signed),
            Type::Int { signed } => Self::write_signed(f, "int", signed),
            Type::Long { signed } => Self::write_signed(f, "long", signed),
            Type::LongLong { .. } => write!(f, "long long"),
            Type::Float => write!(f, "float"),
            Type::Double => write!(f, "double"),
            Type::LongDouble => write!(f, "long double"),
            Type::Struct { name, fields, .. } => {
                write!(f, "struct ",)?;
                match name {
                    Some(name) => write!(f, "{} ", name),
                    None => write!(f, ""),
                }?;
                fields.fmt(f)
            }
            Type::Ident(name) => write!(f, "{}", name),
        }
    }
}

impl Debug for Decl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)?;
        write!(f, " ")?;
        self.bindings.fmt(f)
    }
}
impl Debug for MonoDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)?;
        write!(f, " ")?;
        self.binding.fmt(f)
    }
}

impl Debug for Binding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Binding::Fn { inner, params } => {
                write!(f, "{:?}(", inner)?;
                params.fmt(f)?;
                write!(f, ")")
            }
            Binding::Array { inner, size } => {
                write!(f, "{:?}[", inner)?;
                match size {
                    None => {}
                    Some(len) => len.fmt(f)?,
                };
                write!(f, "]")
            }
            Binding::Pointer(b) => write!(f, "*{:?}", b),
            Binding::Paren(b) => b.as_ref().fmt(f),
            Binding::Ident(id) => write!(f, "{}", id),
            Binding::Anonymous => write!(f, "<anonymous>"),
        }
    }
}

impl Debug for VarDefn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.base.fmt(f)?;
        write!(f, " ")?;
        for (binding, expr) in self.bindings.iter() {
            binding.fmt(f)?;
            match expr {
                None => {}
                Some(e) => {
                    write!(f, " = ")?;
                    e.fmt(f)?;
                }
            };
        }
        Ok(())
    }
}

impl Debug for TypeDefn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "typedef {:?} ", self.base)?;
        self.bindings.fmt(f)
    }
}

impl Debug for FnDefn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fn {}{:?} -> {:?} = ",
            self.name, self.params, self.return_type
        )?;
        match &self.body {
            None => write!(f, "{{}}"),
            Some(body) => body.fmt(f),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<expr>")
    }
}

impl Type {
    fn write_signed(
        f: &mut Formatter,
        type_name: &str,
        signed: &Option<bool>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match signed {
                None => "",
                Some(sign) =>
                    if *sign {
                        "signed "
                    } else {
                        "unsigned "
                    },
            },
            type_name
        )
    }
}
