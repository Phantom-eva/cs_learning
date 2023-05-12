pub mod arith;

/* First, let's write some basic functions in Rust to get used to it.
 * Then, complete the exercises in arith.rs
 *
 * You are encouraged to test your functions.
 * */

/* EXERCISE 1: Implement a function that returns the ith fibonacci number, starting with 0, 1
* https://en.wikipedia.org/wiki/Fibonacci_number.
*/
pub fn fibonacci(i: u32) -> u32 {
    panic!("Not yet implemented: fibonacci");
}

#[cfg(test)]
mod fib_tests {
    use super::*; // This imports all functions from the top-level module of this file.
}

/* EXERCISE 2 */
/* Return the string slice with the smallest length of those passed in
 * If none are passed in, you should use panic!()
 * If multiple have the same length, return the one occurring earliest in the input
 */
pub fn smallest_str<'a, 'b>(strings: &'b [&'a str]) -> &'a str {
    panic!("Not yet implemented: smallest_str")
}

#[cfg(test)]
mod smallest_string_tests {
    use super::*; // This imports all functions from the top-level module of this file.
}

/* EXERCISE 3: Write your response in a comment below this
 * Look at the type of smallest_str
 *
 *   smallest_str<'a, 'b>(strings: &'b[&'a str]) -> &'a str
 *
 * Consider the following alternative lifetime annotations for the same function:
 *
 *   1. smallest_str<'a>(strings: &'a[&'a str]) -> &'a str
 *   2. smallest_str<'a, 'b>(strings: &'a[&'a str]) -> &'b str
 *   3. smallest_str<'a, 'b>(strings: &'b[&'a str]) -> &'b str
 *
 * Explain what each of these differing lifetime annotations would
 * mean, and why the original is the best one for our function.
 *
 * Now consider some other choices of whether or not inputs or outputs are borrowed:
 *   1. smallest_str(strings: Vec<&str>) -> &str
 *   2. smallest_str(strings: Vec<String>) -> String
 *
 * Why is the original type preferable to each of these types for this
 * function?
 */

/* EXERCISE 4
 * Implement a function that adds the 2nd and 4th elements of a slice together (0 indexed of course).
 * Return None if the vector doesn't have enough elements instead of using panic!
 *
 * To get full credit, don't directly use array indexing, instead use
 * the `get` method:
 * https://doc.rust-lang.org/std/primitive.slice.html#method.get on
 * slices to get an Option and either use pattern matching or the `?`
 * operator to implement the correct behavior.
 */
pub fn add_them_up(v: &[i32]) -> Option<i32> {
    panic!("NYI: add_them_up")
}

#[cfg(test)]
mod add_them_up_tests {
    use super::*;
}
