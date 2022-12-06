use std::marker::PhantomData;
use std::rc::Rc;

pub type ParseResult<I, O, E = String> = Result<(O, I), E>;

/// The common `Parser` trait.
pub trait Parser<I, O, E>
where
    I: ?Sized,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, O, E>;

    fn rc(self) -> RcParser<Self>
    where
        Self: Sized,
    {
        RcParser::new(self)
    }

    fn map<F, O1>(self, f: F) -> MapParser<Self, F, O>
    where
        Self: Sized,
        F: Fn(O) -> O1,
    {
        MapParser {
            parser: self,
            mapper: f,
            _phantom: PhantomData,
        }
    }

    fn and<P2, O2>(self, p2: P2) -> AndParser<Self, P2>
    where
        Self: Sized,
        P2: Parser<I, O2, E>,
    {
        AndParser { p1: self, p2 }
    }

    fn and_l<P2, O2>(self, p2: P2) -> AndLeftParser<Self, P2, O2>
    where
        Self: Sized,
        P2: Parser<I, O2, E>,
    {
        AndLeftParser {
            parser: AndParser { p1: self, p2 },
            _phantom: PhantomData,
        }
    }

    fn and_r<P2, O2>(self, p2: P2) -> AndRightParser<Self, P2, O>
    where
        Self: Sized,
        P2: Parser<I, O2, E>,
    {
        AndRightParser {
            parser: AndParser { p1: self, p2 },
            _phantom: PhantomData,
        }
    }

    fn or<P2, O2>(self, p2: P2) -> OrParser<Self, P2>
    where
        Self: Sized,
        P2: Parser<I, O2, E>,
    {
        OrParser { p1: self, p2 }
    }

    fn repeat(self) -> RepeatParser<Self>
    where
        Self: Sized,
    {
        RepeatParser { parser: self }
    }

    fn repeat1(self) -> RepeatOneParser<Self>
    where
        Self: Sized,
    {
        RepeatOneParser { parser: self }
    }

    fn opt(self) -> OptionParser<Self>
    where
        Self: Sized,
    {
        OptionParser { parser: self }
    }

    fn skip(self) -> SkipParser<Self, O>
    where
        Self: Sized,
    {
        SkipParser {
            parser: self,
            _phantom: PhantomData,
        }
    }
}

pub fn parse<'a, P, I, O, E>(parser: P, input: &'a I) -> ParseResult<&'a I, O, E>
where
    I: ?Sized,
    P: Parser<I, O, E>,
{
    parser.parse(input)
}

/// Implements `Parser` trait for functions & closures.
impl<I, O, E, F> Parser<I, O, E> for F
where
    I: ?Sized,
    F: Fn(&I) -> ParseResult<&I, O, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, O, E> {
        self(input)
    }
}

pub struct MapParser<P, F, O> {
    parser: P,
    mapper: F,
    _phantom: PhantomData<O>,
}

impl<I, O1, E, P, F, O> Parser<I, O1, E> for MapParser<P, F, O>
where
    I: ?Sized,
    P: Parser<I, O, E>,
    F: Fn(O) -> O1,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, O1, E> {
        let (o, i) = self.parser.parse(input)?;
        let o2 = (self.mapper)(o);

        return Ok((o2, i));
    }
}

/// For chaining two parsers.
pub struct AndParser<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<I, O1, O2, E, P1, P2> Parser<I, (O1, O2), E> for AndParser<P1, P2>
where
    I: ?Sized,
    P1: Parser<I, O1, E>,
    P2: Parser<I, O2, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, (O1, O2), E> {
        let (o1, i1) = self.p1.parse(input)?;
        let (o2, i2) = self.p2.parse(i1)?;

        Ok(((o1, o2), i2))
    }
}

pub struct AndLeftParser<P1, P2, O2> {
    parser: AndParser<P1, P2>,
    _phantom: PhantomData<O2>,
}

/// # Example:
/// ```
/// use parcomb::string_parser::lit;
/// use parcomb::parser::*;
///
/// let par = lit("abc").and_l(lit("def"));
///
/// let inp = "abcdefg";
/// let res = par.parse(inp).unwrap();
/// assert_eq!(("abc".to_string(), "g"), res);
///
/// let inp2 = "xxxx";
/// assert!(par.parse(inp2).is_err());
/// ```
impl<I, O1, O2, E, P1, P2> Parser<I, O1, E> for AndLeftParser<P1, P2, O2>
where
    I: ?Sized,
    P1: Parser<I, O1, E>,
    P2: Parser<I, O2, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, O1, E> {
        self.parser.parse(input).map(|((o1, _), i)| (o1, i))
    }
}

pub struct AndRightParser<P1, P2, O1> {
    parser: AndParser<P1, P2>,
    _phantom: PhantomData<O1>,
}

/// # Example:
/// ```
/// use parcomb::string_parser::lit;
/// use parcomb::parser::*;
///
/// let par = lit("abc").and_r(lit("def"));
///
/// let inp = "abcdefg";
/// let res = par.parse(inp).unwrap();
/// assert_eq!(("def".to_string(), "g"), res);
///
/// let inp2 = "xxxx";
/// assert!(par.parse(inp2).is_err());
/// ```
impl<I, O1, O2, E, P1, P2> Parser<I, O2, E> for AndRightParser<P1, P2, O1>
where
    I: ?Sized,
    P1: Parser<I, O1, E>,
    P2: Parser<I, O2, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, O2, E> {
        self.parser.parse(input).map(|((_, o2), i)| (o2, i))
    }
}

pub struct OrParser<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<I, O, E, P1, P2> Parser<I, O, E> for OrParser<P1, P2>
where
    I: ?Sized,
    P1: Parser<I, O, E>,
    P2: Parser<I, O, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, O, E> {
        self.p1.parse(input).or_else(|_| self.p2.parse(input))
    }
}

pub struct RepeatParser<P> {
    parser: P,
}

impl<I, O, E, P> Parser<I, Vec<O>, E> for RepeatParser<P>
where
    I: ?Sized,
    P: Parser<I, O, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, Vec<O>, E> {
        let mut res: Vec<O> = Vec::new();

        let mut i = input;
        loop {
            match self.parser.parse(i) {
                Err(_) => {
                    break;
                }

                Ok((o, i1)) => {
                    res.push(o);
                    i = i1;
                }
            }
        }

        return Ok((res, i));
    }
}

pub struct RepeatOneParser<P> {
    parser: P,
}

impl<I, O, E, P> Parser<I, Vec<O>, E> for RepeatOneParser<P>
where
    I: ?Sized,
    P: Parser<I, O, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, Vec<O>, E> {
        let mut res: Vec<O> = Vec::new();

        let mut i = input;
        let e: E;
        loop {
            match self.parser.parse(i) {
                Err(e0) => {
                    e = e0;
                    break;
                }

                Ok((o, i1)) => {
                    res.push(o);
                    i = i1;
                }
            }
        }

        if res.len() >= 1 {
            return Ok((res, i));
        }

        return Err(e);
    }
}

pub struct OptionParser<P> {
    parser: P,
}

/// # Example:
/// ```
/// use parcomb::string_parser::lit;
/// use parcomb::parser::*;
///
/// let par = lit("abc").opt();
///
/// let inp = "abcd";
/// let res = par.parse(inp).unwrap();
/// assert_eq!((Some("abc".to_string()), "d"), res);
///
/// let inp2 = "xxxx";
/// let res = par.parse(inp2).unwrap();
/// assert_eq!((None, "xxxx"), res);
/// ```
impl<I, O, E, P> Parser<I, Option<O>, E> for OptionParser<P>
where
    I: ?Sized,
    P: Parser<I, O, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, Option<O>, E> {
        match self.parser.parse(input) {
            Ok((r, i)) => Ok((Some(r), i)),
            Err(_) => Ok((None, input)),
        }
    }
}

pub struct SkipParser<P, O> {
    parser: P,
    _phantom: PhantomData<O>,
}

/// # Example:
/// ```
/// use parcomb::string_parser::lit;
/// use parcomb::parser::*;
///
/// let par = lit("abc").skip();
///
/// let inp = "abcd";
/// let res = par.parse(inp).unwrap();
/// assert_eq!(((), "d"), res);
///
/// let inp2 = "xxxx";
/// assert!(par.parse(inp2).is_err());
/// ```
impl<I, O, E, P> Parser<I, (), E> for SkipParser<P, O>
where
    I: ?Sized,
    P: Parser<I, O, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, (), E> {
        self.parser.parse(input).map(|(_, i)| ((), i))
    }
}

pub struct RcParser<P> {
    parser: Rc<P>,
}

impl<P> RcParser<P> {
    pub fn new<I, O, E>(p: P) -> Self
    where
        I: ?Sized,
        P: Parser<I, O, E>,
    {
        Self { parser: Rc::new(p) }
    }
}

impl<I, O, E, P> Parser<I, O, E> for RcParser<P>
where
    I: ?Sized,
    P: Parser<I, O, E>,
{
    fn parse<'a>(&self, input: &'a I) -> ParseResult<&'a I, O, E> {
        self.parser.parse(input)
    }
}

impl<P> Clone for RcParser<P> {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
        }
    }
}
