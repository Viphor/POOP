use super::super::mir;
use super::ast;

type Output<Out = ()> = Result<Out, ()>;

trait InlineAppend {
    fn inline_append(self, other: &mut Self) -> Self;
}

impl<T> InlineAppend for Vec<T> {
    fn inline_append(mut self, other: &mut Self) -> Self {
        self.append(other);
        self
    }
}

pub fn convert_ast(program: ast::Program) -> Output<mir::Program> {
    Ok(mir::Program {
        declarations: convert_program(program)?,
    })
}

fn convert_program(program: ast::Program) -> Output<Vec<mir::Decl>> {
    match program {
        ast::Program::Decl(decl, rest) => {
            Ok(vec![convert_decl(decl)?].inline_append(&mut convert_program(*rest)?))
        }
        ast::Program::Empty => Ok(Vec::new()),
    }
}

fn convert_decl(_decl: ast::Decl) -> Output<mir::Decl> {
    unimplemented!();
}
