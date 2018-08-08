use ast::AST;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Var(String),
    Func(Box<Type>, Box<Type>),
}

impl Type {
    pub fn latex_fmt(&self) -> String {
        match *self {
            Type::Var(ref a) => a.to_string(),
            Type::Func(ref optype, ref valtype) => match **optype {
                Type::Var(ref a) => format!("{} \\to {}", a, valtype.latex_fmt()),
                _ => format!(
                    "\\left({}\\right) \\to {}",
                    optype.latex_fmt(),
                    valtype.latex_fmt()
                ),
            },
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match *self {
            Type::Var(ref a) => a.to_string(),
            Type::Func(ref optype, ref valtype) => match **optype {
                Type::Var(ref a) => format!("{}->{}", a, valtype.to_string()),
                _ => format!("({})->{}", optype.to_string(), valtype.to_string()),
            },
        }
    }
}

pub type TypeEnv = HashMap<String, Type>;
pub type Equality = (Type, Type);
pub type Constraints = VecDeque<Equality>;

#[derive(Debug, Clone, PartialEq)]
pub enum TypedAST {
    Var(String),
    App(Box<TypedAST>, Box<TypedAST>),
    Abs(String, Type, Box<TypedAST>),
}

impl ToString for TypedAST {
    fn to_string(&self) -> String {
        match *self {
            TypedAST::Var(ref s) => s.to_string(),
            TypedAST::App(ref e1, ref e2) => {
                let e1str = if let TypedAST::Var(_) = **e1 {
                    e1.to_string()
                } else {
                    format!("({})", e1.to_string())
                };
                let e2str = if let TypedAST::Var(_) = **e2 {
                    e2.to_string()
                } else {
                    format!("({})", e2.to_string())
                };

                format!("{} {}", e1str, e2str)
            }
            TypedAST::Abs(ref s, ref t, ref e) => {
                format!("\\{}:{}.{}", s, t.to_string(), e.to_string())
            }
        }
    }
}

impl TypedAST {
    pub fn construct_type(&self, tenv: &TypeEnv) -> Type {
        match *self {
            TypedAST::Var(ref x) => {
                let t = tenv.get(x).unwrap();
                t.clone()
            }
            TypedAST::App(ref typed_e1, ref typed_e2) => {
                let t1 = typed_e1.construct_type(tenv);
                let _ = typed_e2.construct_type(tenv);
                if let Type::Func(_, ref tv) = t1 {
                    *tv.clone()
                } else {
                    panic!("unexpected");
                }
            }
            TypedAST::Abs(ref x, ref tx, ref typed_e) => {
                let mut tenv_x = tenv.clone();
                tenv_x.insert(x.clone(), tx.clone());
                let t = typed_e.construct_type(&tenv_x);
                Type::Func(Box::new(tx.clone()), Box::new(t))
            }
        }
    }

    pub fn latex_fmt(&self) -> String {
        match *self {
            TypedAST::Var(ref s) => s.to_string(),
            TypedAST::App(ref e1, ref e2) => {
                let e1str = if let TypedAST::Var(_) = **e1 {
                    e1.latex_fmt()
                } else {
                    format!("\\left({}\\right)", e1.latex_fmt())
                };
                let e2str = if let TypedAST::Var(_) = **e2 {
                    e2.latex_fmt()
                } else {
                    format!("\\left({}\\right)", e2.latex_fmt())
                };
                format!("{} \\, {}", e1str, e2str)
            }
            TypedAST::Abs(ref s, _, ref e) => format!("\\lambda {} .{}", s, e.latex_fmt(),),
        }
    }
    pub fn typejudge_latex(&self, t: &Type, tenv: &TypeEnv) -> String {
        let mut tenv_str: Vec<String> = Vec::new();
        for vt in tenv {
            tenv_str.push(format!("{} : {}", vt.0, vt.1.latex_fmt()));
        }
        if tenv.is_empty() {
            format!(" \\vdash {} : {}", self.latex_fmt(), t.latex_fmt())
        } else {
            format!(
                "\\{{ {} \\}} \\vdash {} : {}",
                tenv_str.join(","),
                self.latex_fmt(),
                t.latex_fmt()
            )
        }
    }
    pub fn to_bussproofs(&self, tenv: &TypeEnv) -> Type {
        match *self {
            TypedAST::Var(ref x) => {
                let t = tenv.get(x).unwrap();
                println!("  \\AxiomC{{ $ {} $ }}", self.typejudge_latex(&t, tenv));
                t.clone()
            }
            TypedAST::App(ref typed_e1, ref typed_e2) => {
                let t1 = typed_e1.to_bussproofs(tenv);
                let _ = typed_e2.to_bussproofs(tenv);
                if let Type::Func(_, ref tv) = t1 {
                    println!("  \\BinaryInfC{{ $ {} $ }}", self.typejudge_latex(tv, tenv));
                    *tv.clone()
                } else {
                    panic!("unexpected");
                }
            }
            TypedAST::Abs(ref x, ref tx, ref typed_e) => {
                let mut tenv_x = tenv.clone();
                tenv_x.insert(x.clone(), tx.clone());
                let te = typed_e.to_bussproofs(&tenv_x);
                let t = Type::Func(Box::new(tx.clone()), Box::new(te.clone()));
                println!("  \\UnaryInfC{{ $ {} $ }}", self.typejudge_latex(&t, tenv));
                t
            }
        }
    }

    pub fn unify(&self, unifier: &Unifier) -> Self {
        match *self {
            TypedAST::Var(_) => self.clone(),
            TypedAST::App(ref e1, ref e2) => {
                TypedAST::App(Box::new(e1.unify(unifier)), Box::new(e2.unify(unifier)))
            }
            TypedAST::Abs(ref x, ref t, ref e) => {
                let mut t = t.clone();
                for u in unifier {
                    t = t.unify(u);
                }
                TypedAST::Abs(x.to_string(), t.clone(), Box::new(e.unify(unifier)))
            }
        }
    }
}

fn merge(c_dst: &mut Constraints, mut c_src: Constraints) {
    while !c_src.is_empty() {
        let c = c_src.pop_front().unwrap();
        c_dst.push_back(c);
    }
}

pub struct TypeInf {
    cnt: usize,
}

impl TypeInf {
    pub fn new() -> Self {
        TypeInf { cnt: 0 }
    }

    fn get_type_variable(&mut self) -> Type {
        let a = format!("\\tau_{{{}}}", self.cnt);
        self.cnt = self.cnt + 1;
        Type::Var(a)
    }
    pub fn type_inf(&mut self, tenv: &TypeEnv, e: &AST) -> (TypedAST, Type, Constraints) {
        use ast::AST::*;
        match *e {
            Var(ref x) => match tenv.get(x) {
                Some(t) => (TypedAST::Var(x.to_string()), t.clone(), Constraints::new()),
                None => {
                    panic!("unknown variable:{}", x);
                }
            },
            App(ref e1, ref e2) => {
                let (typed_e1, t1, ce1) = self.type_inf(tenv, e1);
                let (typed_e2, t2, ce2) = self.type_inf(tenv, e2);
                let top = self.get_type_variable();
                let tv = self.get_type_variable();
                let mut c = Constraints::new();
                merge(&mut c, ce1);
                merge(&mut c, ce2);
                c.push_back((t2.clone(), top.clone()));
                c.push_back((t1, Type::Func(Box::new(top), Box::new(tv.clone()))));
                (TypedAST::App(Box::new(typed_e1), Box::new(typed_e2)), tv, c)
            }
            Abs(ref x, ref e) => {
                let tx = self.get_type_variable();
                let mut tenv_x = tenv.clone();
                tenv_x.insert(x.clone(), tx.clone());
                let (typed_e, te, ce) = self.type_inf(&tenv_x, e);
                let t = Type::Func(Box::new(tx.clone()), Box::new(te));
                (TypedAST::Abs(x.to_string(), tx, Box::new(typed_e)), t, ce)
            }
        }
    }
}

pub type VarType = (String, Type);
pub type Unifier = Vec<VarType>;

impl Type {
    pub fn unify(&self, u: &VarType) -> Self {
        use self::Type::*;
        match *self {
            Var(ref s) => if *s == u.0 {
                u.1.clone()
            } else {
                self.clone()
            },
            Func(ref op, ref v) => {
                let unified_op = op.unify(u);
                let unified_v = v.unify(u);
                Func(Box::new(unified_op), Box::new(unified_v))
            }
        }
    }
    pub fn appear(&self, x: &String) -> bool {
        use self::Type::*;
        match *self {
            Var(ref s) => s == x,
            Func(ref op, ref v) => op.appear(x) || v.appear(x),
        }
    }
}

pub fn unify(constraints: &mut Constraints, u: &VarType) -> Constraints {
    let mut unified_constraints = Constraints::new();
    while !constraints.is_empty() {
        let constraint = constraints.pop_front().unwrap();
        unified_constraints.push_back((constraint.0.unify(&u), constraint.1.unify(&u)));
    }
    return unified_constraints;
}

pub fn calculate_mgu(constraints: &Constraints) -> Option<Unifier> {
    use self::Type::*;
    let mut constraints = constraints.clone();
    let mut unifier = Unifier::new();
    while !constraints.is_empty() {
        let constraint = constraints.pop_front().unwrap();
        if constraint.0 == constraint.1 {
            continue;
        }
        match constraint {
            (Var(x), t) | (t, Var(x)) => {
                if t.appear(&x) {
                    println!("{} appear in {}", x, t.to_string());
                    return None;
                } else {
                    let u = (x, t);
                    constraints = unify(&mut constraints, &u);
                    unifier.push(u);
                }
            }
            (Func(op1, v1), Func(op2, v2)) => {
                constraints.push_back((*op1, *op2));
                constraints.push_back((*v1, *v2));
            }
        }
    }
    Some(unifier)
}
