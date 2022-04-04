use std::io;

mod generate_ast;

fn main() -> io::Result<()> {
    generate_ast::generate_ast("src")
}
