// Licensed under the MIT license. See LICENSE.md


/// Types representing line must implement this trait
pub trait Line<'a> :
    From<&'a [u8]> +
    Into<&'a [u8]> +
    Default +
    PartialEq +
    Eq +
    Copy +
    Send +
    Sync
{}

// `&[u8]` fulfills everything we need to work as `Line`, so lets mark it
impl<'a> Line<'a> for &'a [u8] {}