use std::io;

use io::Read;

use infer::Type;

pub fn head2type(head: &[u8]) -> Option<Type> {
    infer::get(head)
}

pub fn type2mime(t: Type) -> &'static str {
    t.mime_type()
}

pub fn head2mime(head: &[u8]) -> Option<&'static str> {
    head2type(head).map(type2mime)
}

pub fn reader2type<R>(rdr: R) -> Result<Type, io::Error>
where
    R: Read,
{
    let limit: u64 = 1024;
    let mut taken = rdr.take(limit);
    let mut buf: [u8; 1024] = [0; 1024];
    let mut wtr: &mut [u8] = &mut buf;
    io::copy(&mut taken, &mut wtr)?;
    let head: &[u8] = &buf;
    let otyp: Option<Type> = head2type(head);
    otyp.ok_or_else(|| io::Error::other("unknown type"))
}

pub fn reader2mime<R>(rdr: R) -> Result<&'static str, io::Error>
where
    R: Read,
{
    let typ: Type = reader2type(rdr)?;
    Ok(type2mime(typ))
}

pub fn stdin2mime() -> Result<&'static str, io::Error> {
    reader2mime(io::stdin().lock())
}
