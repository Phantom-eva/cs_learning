/* Expressions */
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Exp<Ann> {
    Num(i64, Ann),
    Var(String, Ann),
    Prim1(Prim1, Box<Exp<Ann>>, Ann),
    Let {
        bindings: Vec<(String, Exp<Ann>)>, // new binding declarations
        body: Box<Exp<Ann>>,               // the expression in which the new variables are bound
        ann: Ann,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Prim1 {
    Add1,
    Sub1,
}

impl<Ann> Exp<Ann> {
    pub fn ann(&self) -> Ann
    where
        Ann: Clone,
    {
        match self {
            Exp::Num(_, a) => a.clone(),
            Exp::Var(_, a) => a.clone(),
            Exp::Prim1(_, _, a) => a.clone(),
            Exp::Let { ann: a, .. } => a.clone(),
        }
    }
}

pub type SurfProg<Ann> = Exp<Ann>;
