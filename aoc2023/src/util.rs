use std::io::{stdin, Read};

use nom::{
    character::complete::u64,
    combinator::{all_consuming, map_res},
    Finish,
};

pub fn parse_stdin<T, F>(parser: F) -> color_eyre::Result<T>
where
    F: Fn(&str) -> nom::IResult<&str, T>,
{
    let mut s: String = "".to_owned();
    stdin().read_to_string(&mut s)?;

    let mut full_parser = all_consuming(parser);
    match full_parser(&s).finish() {
        Ok((_rest, items)) => Ok(items),
        Err(err) => Err(color_eyre::eyre::eyre!("{err}")),
    }
}

pub fn parse_usize(s: &str) -> nom::IResult<&str, usize> {
    map_res(u64, usize::try_from)(s)
}
