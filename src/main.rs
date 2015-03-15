#![feature(collections)]
#![feature(core)]
#![feature(non_ascii_idents)]

use std::vec::*;


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
        Some(unifier) => {
            for (variable,term) in unifier {
               println!("{:?}={:?}",variable,term);
            }
        },
        None => { println!("{}","Oh tits");}
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

fn unify(s:Term, t:Term) -> Option<Vec<(Variable,Term)>> {
    match (s, t) {
        // Unify an atom with another atom
        (Term::Atom(s),Term::Atom(t)) =>
            if s.eq(&t) {
                Some(vec![])
            } else {
                None
            },
           
           
        // Unify a variable with anything else
        (Term::Variable(s),t@_) | (t@_,Term::Variable(s)) =>
            Some(vec![(s,t)]),
            
            
        // Unify two structures.
        (Term::Structure(Structure(functorₛ, termsₛ)), 
         Term::Structure(Structure(functorₜ, termsₜ))) =>
            
            if functorₛ == functorₜ && termsₛ.len() == termsₜ.len() {
            
                let mut substitutions = Vec::new();
                
                for (termₛ,termₜ) in zip!(termsₛ,termsₜ) {
                
                    let termₛ = apply(&substitutions, &termₛ);
                    let termₜ = apply(&substitutions, &termₜ);
                   
                    if let Some(unifier) = unify(termₛ,termₜ) {
                        substitutions.push_all(&unifier[..])
                    } else {
                        return None;
                    }
                }
                
                Some(substitutions)
            } else {
                None
            },
        _ => None
    }
}
