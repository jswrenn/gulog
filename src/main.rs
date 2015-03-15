#![feature(collections)]
#![feature(core)]
#![feature(non_ascii_idents)]

use std::vec::*;
use std::collections::*;


fn main() {
    let a = Term::Structure(Structure(
        "test".to_string(),
        vec!(
            Term::Variable(Variable("A".to_string())),
            Term::Atom(Atom("b".to_string())),
            Term::Variable(Variable("A".to_string()))
            )));
            
    let b = Term::Structure(Structure(
        "test".to_string(),
        vec!(
            Term::Atom(Atom("a".to_string())),
            Term::Variable(Variable("B".to_string())),
            Term::Variable(Variable("A".to_string()))
            )));
    
    match unify(a,b) {
        Ok(unifier) => {
            for (variable,term) in unifier {
               println!("{:?}={:?}",variable,term);
            }
        },
        Err(n) => { println!("Err:{}",n);}
    }
}

macro_rules! zip {
    ($head:expr) => ($head.iter());
    ($head:expr, $($tail:expr),*) => ($head.iter().zip(zip!($($tail),*)));
}

#[derive(Clone, PartialEq,Debug)]
struct Atom(String);

#[derive(Clone, PartialEq,Debug)]
struct Variable(String);

#[derive(Clone, PartialEq,Debug)]
struct Structure(String, Vec<Term>);

#[derive(Clone, PartialEq,Debug)]
enum Term {
    Atom(Atom),
    Variable(Variable),
    Structure(Structure),
}
   
fn find_substitution(substitutions: &Vec<(Variable,Term)>, query: & Variable) -> Term {
    for (var,term) in substitutions.clone() {
        if var.eq(query) {
            return term;
        }
    }
    return Term::Variable(query.clone());
}

fn apply(substitutions: &Vec<(Variable,Term)>, term: &Term) -> Term {
    match *term {
        Term::Atom(_) => term.clone(),
        Term::Variable(ref variable) => {
            find_substitution(substitutions, variable)
        },
        Term::Structure(Structure(ref f,ref t)) => {
            Term::Structure(Structure(
                f.clone(),
                t.iter().map(|term| apply(substitutions, term)).collect()))
        }
    }
}

fn unify<'a>(s:Term, t:Term) -> Result<Vec<(Variable,Term)>,i32> {
    match (s, t) {
        (Term::Atom(s),Term::Atom(t)) =>
            if s.eq(&t) {
                Ok(vec![])
            } else {
                Err(0)
            },
           
        (Term::Variable(s),t@_) | (t@_,Term::Variable(s)) =>
            Result::Ok(vec![(s,t)]),
           
        (Term::Structure(Structure(fₛ, tₛ)), Term::Structure(Structure(fₜ,tₜ))) => {
            if fₛ == fₜ && tₛ.len() == tₜ.len() {
                let mut substitutions = Vec::new();
                for (tₛᵢ,tₜᵢ) in zip!(tₛ,tₜ) {
                    let tₛᵢ = apply(&substitutions, &tₛᵢ);
                    let tₜᵢ = apply(&substitutions, &tₜᵢ);
                   
                    if let Ok(unifier) = unify(tₛᵢ,tₜᵢ) {
                        substitutions.push_all(unifier.as_slice())
                    } else {
                        return Err(1)
                    }
                }
                Ok(substitutions)
            } else {
                Err(3)
            }
        },
        (Term::Atom(_), Term::Structure(_)) | (Term::Structure(_), Term::Atom(_))
            => Result::Err(4)
    }
}
