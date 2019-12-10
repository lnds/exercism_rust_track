use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    func: Box<dyn 'a + Fn(T) -> String>,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<'a, T>
    where
        F: 'a + Fn(T) -> bool,
        S: 'a + ToString,
    {
        Matcher {
            func: Box::new(move |x: T|{
                if matcher(x) {
                    subs.to_string()
                } else {
                    String::default()
                }
            }),
        }
    }

}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
#[derive(Default)]
pub struct Fizzy<'a, T>
where
    T: 'a + ToString + Clone,
{
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T> Fizzy<'a, T>
where
    T: ToString + Clone,
{
    pub fn new() -> Self {
        Fizzy { matchers: vec![] }
    }

    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: Iterator<Item = T> + 'a,
    {
        iter.map(move |i| {
            let s = self
                .matchers
                .iter()
                .map(|m| (m.func)(i.clone()))
                .collect::<String>();
            if s.is_empty() {
                i.to_string()
            } else {
                s
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules

pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: ToString + Clone + Rem<Output = T> + From<u8> + PartialEq,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| (n % T::from(3)) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n: T| (n % T::from(5)) == T::from(0), "buzz"))
}
