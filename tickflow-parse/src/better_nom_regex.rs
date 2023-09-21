use nom::{
    error::{ErrorKind, ParseError},
    Err, IResult, Slice,
};
use regex::Regex;

pub fn re_find<'a, E>(re: Regex) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    move |i| {
        if let Some(m) = re.find(i) {
            Ok((i.slice(m.end()..), i.slice(m.start()..m.end())))
        } else {
            Err(Err::Error(E::from_error_kind(i, ErrorKind::RegexpFind)))
        }
    }
}

pub fn re_capture<'a, E>(re: Regex) -> impl Fn(&'a str) -> IResult<&'a str, Vec<&'a str>, E>
where
    E: ParseError<&'a str>,
{
    move |i| {
        if let Some(c) = re.captures(i) {
            let v: Vec<_> = c
                .iter()
                .map(|m| m.map(|m| i.slice(m.start()..m.end())).unwrap_or(""))
                .collect();
            let offset = {
                let end = v.last().unwrap();
                end.as_ptr() as usize + end.len() - i.as_ptr() as usize + 1
            };
            Ok((i.slice(offset..), v))
        } else {
            Err(Err::Error(E::from_error_kind(i, ErrorKind::RegexpCapture)))
        }
    }
}
