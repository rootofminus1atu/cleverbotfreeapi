use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

macro_rules! remove_many {
    ($set:expr, [$($char:literal),*]) => {
      ($set$(.remove($char))*)  
    };
}

/// pythonic safe chars: `!#$%&'()*+,/:;=?@[]~`
const PYTHONIC_NON_ALPHANUMERIC: &AsciiSet = &remove_many!(
    NON_ALPHANUMERIC, 
    [b'!', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'/', b':', b';', b'=', b'?', b'@', b'[', b']', b'~']
);

/// TIL that python's requests' `requests.utils.requote_uri(s)` uses "!#$%&'()*+,/:;=?@[]~" as safe chars, ok i guess
pub fn pythonic_encode(input: &str) -> String {
    utf8_percent_encode(input, PYTHONIC_NON_ALPHANUMERIC).to_string()
}