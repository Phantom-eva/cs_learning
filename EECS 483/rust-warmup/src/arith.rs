/*
This following is a datatype that describes simple arithmetic expressions.
*/

#[derive(Debug, Clone)]
pub enum Arith {
    Plus(Box<Arith>, Box<Arith>),
    Times(Box<Arith>, Box<Arith>),
    Variable(String),
    Num(i32),
}

/* For example, the expression

   5 * 3 + y

 would be parsed as (suppressing the Arith:: and Box::new noise)

   Plus(Times(Num(5), Num(3)), Variable("y"))

 Note that the addition is at the top of the tree because parser
 understands order of operations

 Here are some other examples:

   1 + x               Plus(Num(1), Variable("x"))
   x + x               Plus(Variable("x"), Variable("x"))
   5 * (y + 10)           Times(Num(5), Plus(Variable("y"), Num(10)))

*/

/**
 Because these expressions have variables in them, in order to write
 an interpreter, we need to know what the values of the variables
 should be.

 To represent the set of known variables, we're going to use a
 simple representation as an slice of pairs of variable names with their associated value.

 Env = &[(&str, i32)]

 We use a slice because our usage is read-only, so we have no need
 for ownership or mutability.

*/

/**
To use such an environment, we need to be able to `get` the values
of variables in an environment.  We may also want to check whether
an environment `contains` a particular variable.

Notice that I described this as a *set*, which implies no duplicate
variable names, but a slice can certainly have such duplicates, so
we need to choose which value in such a situation is correct. For
reasons that will become clear at a later date, we will consider the
occurrence of a string with the *highest* index to have the correct
value for that string.

So for instance, if the slice is [("x", 1), ("x", 2)], then the
correct value for x is 2.
 */
/* EXERCISE 1: Implement get */
pub fn get(env: &[(&str, i32)], x: &str) -> Option<i32> {
    panic!("NYI: get");
}

#[cfg(test)]
mod get_tests {
    // Unit tests go here
}

/*
 Next, write evaluate, which takes an arithmetic expression and
 an environment mapping from strings to integers, and evaluates the expression,
 using the given integer value for the variable.

 For example

    let t = Arith::Times(
              Box::new(Arith::Plus(
                Box::new(Arith::Variable(String::from("x"))),
                Box::new(Arith::Variable(String::from("y"))))),
              Box::new(Arith::Num(5)));
    evaluate(t, &[("x", 5), ("y", 7)])

 should produce 60, and

    let t = Arith::Plus(Box::new(Arith::Num(4)), Box::new(Arith::Num(5)))
    evaluate t (&[])

 should produce 9.

 If there is a variable not contained in vars in the expression,
 return an Err with a helpful error message that includes the name of
 the variable. If there are multiple variables not containd in vars,
 you need mention only any one of them.

*/
/* EXERCISE 2: Implement evaluate */
pub fn evaluate(t: &Arith, vars: &[(&str, i32)]) -> Result<i32, String> {
    panic!("NYI: evaluate")
}

#[cfg(test)]
mod evaluate_tests {
    // Unit tests go here
}

/*
 Next, write pretty, which takes an arithmetic expression and renders it in
 mathematical notation.

 It should print with minimal parentheses, assuming standard order of
 operations where multiplication binds more tightly than addition.

 To keep things simple, we will always use * to write multiplication
 rather than allow things like (1 * 3)x.

   pretty(Plus(Plus(Times(Times(Plus(Num(5), Variable("y")), Variable("x")), Variable("z")), Num(2)), Num(1)))

 should pretty-print as

   (5 + y) * x * z + 2 + 1

 HINT: it will be easiest to implement this recursively using a
 function that keeps track of whether the current expression is part
 of a plus or times expression using an additional argument.
*/
/* EXERCISE 3: implement pretty */
pub fn pretty(t: &Arith) -> String {
    panic!("NYI: pretty")
}

#[cfg(test)]
mod pretty_tests {
    // Unit tests go here
}
