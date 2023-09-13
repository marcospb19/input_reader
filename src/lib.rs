use std::{
    array,
    fmt::Debug,
    io::{self, stdin, Read},
    str::FromStr,
};

use tupleops::concat_tuples;

pub struct InputReader<'a> {
    input: &'a str,
    counter: usize,
}

impl<'a> InputReader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, counter: 0 }
    }

    pub fn from_leaked_stdin() -> io::Result<Self> {
        let mut string = String::with_capacity(2048);
        stdin().read_to_string(&mut string)?;
        string.shrink_to_fit();

        let stdin = Box::leak(string.into_boxed_str());

        Ok(Self::new(stdin))
    }

    pub fn read<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        let piece = self.advance_slice();
        piece.parse().unwrap()
    }

    pub fn read_option<T: FromStr>(&mut self) -> Option<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let piece = self.advance_slice();
        piece.parse().ok()
    }

    pub fn read_n<T: FromStr, const N: usize>(&mut self) -> [T; N]
    where
        <T as FromStr>::Err: Debug,
    {
        array::from_fn(|_| self.read::<T>())
    }

    pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        (0..n.into()).map(|_| self.read::<T>()).collect()
    }

    // Update counter and returns the slice, much like a parser
    pub fn advance_slice(&mut self) -> &'a str {
        let piece_position = self.input[self.counter..]
            .find(|x: char| !x.is_whitespace())
            .expect("Pediu para ler mas acabou já");

        let piece = self.input[self.counter + piece_position..]
            .split_whitespace()
            .next()
            .unwrap();

        self.counter += piece_position + piece.len();

        piece
    }

    pub fn read_tuple<T>(&mut self) -> Option<T>
    where
        T: TupleFromStr,
    {
        T::tuple_parse(self)
    }
}

pub trait TupleFromStr {
    fn tuple_parse(reader: &mut InputReader) -> Option<Self>
    where
        Self: Sized;
}

impl<A> TupleFromStr for (A,)
where
    A: FromStr,
{
    fn tuple_parse(reader: &mut InputReader) -> Option<(A,)> {
        reader.advance_slice().parse().ok().map(|x| (x,))
    }
}

impl<A, B> TupleFromStr for (A, B)
where
    A: FromStr,
    B: FromStr,
{
    fn tuple_parse(reader: &mut InputReader) -> Option<(A, B)> {
        reader.advance_slice().parse().ok().and_then(|head| {
            let tail = <(B,)>::tuple_parse(reader)?;
            Some(concat_tuples((head,), tail))
        })
    }
}

impl<A, B, C> TupleFromStr for (A, B, C)
where
    A: FromStr,
    B: FromStr,
    C: FromStr,
{
    fn tuple_parse(reader: &mut InputReader) -> Option<(A, B, C)> {
        reader.advance_slice().parse().ok().and_then(|head| {
            let tail = <(B, C)>::tuple_parse(reader)?;
            Some(concat_tuples((head,), tail))
        })
    }
}

impl<A, B, C, D> TupleFromStr for (A, B, C, D)
where
    A: FromStr,
    B: FromStr,
    C: FromStr,
    D: FromStr,
{
    fn tuple_parse(reader: &mut InputReader) -> Option<(A, B, C, D)> {
        reader.advance_slice().parse().ok().and_then(|head| {
            let tail = <(B, C, D)>::tuple_parse(reader)?;
            Some(concat_tuples((head,), tail))
        })
    }
}

impl<A, B, C, D, E> TupleFromStr for (A, B, C, D, E)
where
    A: FromStr,
    B: FromStr,
    C: FromStr,
    D: FromStr,
    E: FromStr,
{
    fn tuple_parse(reader: &mut InputReader) -> Option<(A, B, C, D, E)> {
        reader.advance_slice().parse().ok().and_then(|head| {
            let tail = <(B, C, D, E)>::tuple_parse(reader)?;
            Some(concat_tuples((head,), tail))
        })
    }
}

impl<A, B, C, D, E, F> TupleFromStr for (A, B, C, D, E, F)
where
    A: FromStr,
    B: FromStr,
    C: FromStr,
    D: FromStr,
    E: FromStr,
    F: FromStr,
{
    fn tuple_parse(reader: &mut InputReader) -> Option<(A, B, C, D, E, F)> {
        reader.advance_slice().parse().ok().and_then(|head| {
            let tail = <(B, C, D, E, F)>::tuple_parse(reader)?;
            Some(concat_tuples((head,), tail))
        })
    }
}

impl<A, B, C, D, E, F, G> TupleFromStr for (A, B, C, D, E, F, G)
where
    A: FromStr,
    B: FromStr,
    C: FromStr,
    D: FromStr,
    E: FromStr,
    F: FromStr,
    G: FromStr,
{
    fn tuple_parse(reader: &mut InputReader) -> Option<(A, B, C, D, E, F, G)> {
        reader.advance_slice().parse().ok().and_then(|head| {
            let tail = <(B, C, D, E, F, G)>::tuple_parse(reader)?;
            Some(concat_tuples((head,), tail))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            InputReader::new("a b c").read_tuple::<(String, String, String)>(),
            Some(("a".into(), "b".into(), "c".into()))
        );

        assert_eq!(
            InputReader::new("1 hi false").read_tuple::<(i32, String, bool)>(),
            Some((1, "hi".into(), false))
        );

        assert_eq!(
            InputReader::new("123 135 159").read_tuple::<(i32, u64, u8)>(),
            Some((123, 135, 159))
        );

        assert_eq!(
            InputReader::new("1 2 3 4 5 6").read_tuple::<(u8, u8, u8, u8, u8, u8)>(),
            Some((1, 2, 3, 4, 5, 6))
        );
    }
}
