use mir;

pub struct Constraint;

pub trait Typechecker {
    fn generate_constraints(&self) -> Vec<Constraint>;
}

impl Typechecker for mir::Program {
    fn generate_constraints(&self) -> Vec<Constraint> {
        unimplemented!()
    }
}
