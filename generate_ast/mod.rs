use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &str) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        &[
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal  : Option<Object> value".to_string(),
            "Unary    : Token operator, Box<Expr> right".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    // use modules
    writeln!(file, "use crate::error::*;")?;
    writeln!(file, "use crate::object::*;")?;
    writeln!(file, "use crate::token::*;")?;
    writeln!(file)?;

    // parse the input strings
    for ntype in types {
        let (base_class_name, args) = ntype.split_once(':').unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let arg_split: Vec<&str> = args.trim().split(',').collect();
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(' ').unwrap();
            fields.push(format!("{}: {}", name, t2type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    // enum for top-level production
    writeln!(file, "pub enum {base_name} {{")?;
    for t in &tree_types {
        writeln!(file, "    {}({}),", t.base_class_name, t.class_name)?;
    }
    writeln!(file, "}}")?;
    writeln!(file)?;

    // TODO: generate accept on top-level production
    // impl Expr {
    //     pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
    //         match self {
    //             Expr::Binary(be) => be.accept(expr_visitor),
    //             Expr::Grouping(ge) => ge.accept(expr_visitor),
    //             Expr::Literal(le) => le.accept(expr_visitor),
    //             Expr::Unary(ue) => ue.accept(expr_visitor),
    //         }
    //     }
    // }
    writeln!(file, "impl {} {{", base_name)?;
    let param_name = format!("{}_visitor", base_name.to_lowercase());
    writeln!(
        file,
        "    pub fn accept<T>(&self, {}: &dyn {}Visitor<T>) -> Result<T, LoxError> {{",
        param_name, base_name
    )?;
    writeln!(file, "        match self {{")?;
    for t in &tree_types {
        writeln!(
            file,
            "            {}::{}(x) => x.accept({}),",
            base_name, t.base_class_name, param_name
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;

    // structs for each production rule
    for t in &tree_types {
        writeln!(file, "pub struct {} {{", t.class_name)?;
        for f in t.fields.iter() {
            writeln!(file, "    pub {},", f)?;
        }
        writeln!(file, "}}")?;
        writeln!(file)?;
    }

    // trait for top-level visitor
    writeln!(file, "pub trait {}Visitor<T> {{", base_name)?;
    for t in &tree_types {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, {}: &{}) -> Result<T, LoxError>;",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name
        )?;
    }
    writeln!(file, "}}")?;
    writeln!(file)?;

    // implementation of trait for each production rule
    for t in &tree_types {
        writeln!(file, "impl {} {{", t.class_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{",
            base_name
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file)?;
    }

    Ok(())
}
