/// An implementation of a `cons list`.
pub enum List {
    /// This is the constructor function of a `cons list`.
    Cons(i32, Box<List>),
    /// This is the non-recursive element signaling the end of a `cons list`.
    Nil,
}
