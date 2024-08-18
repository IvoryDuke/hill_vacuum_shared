//=======================================================================//
// IMPORTS
//
//=======================================================================//

use std::{ops::RangeInclusive, path::{Path, PathBuf}};

//=======================================================================//
// CONSTANTS
//
//=======================================================================//

/// The range of the possible draw heights of a texture.
pub const TEXTURE_HEIGHT_RANGE: RangeInclusive<i8> = 0..=20;
/// The file extension of the main HillVacuum file format.
#[allow(clippy::doc_markdown)]
pub const FILE_EXTENSION: &str = "hv";

//=======================================================================//
// TRAITS
//
//=======================================================================//

/// A trait for iterators to get the next value and immediately unwrap it.
pub trait NextValue<T>
where
    Self: Iterator<Item = T>
{
    /// Returns the next unwrapped value.
    /// # Panics
    /// Panic occurs if the next value is None.
    #[inline]
    #[must_use]
    fn next_value(&mut self) -> T { self.next().unwrap() }
}

impl<T, U: Iterator<Item = T>> NextValue<T> for U {}

//=======================================================================//
// MACROS
//
//=======================================================================//

/// Iterates a slice in triplets.
#[macro_export]
macro_rules! iterate_slice_in_triplets {
    ($i:ident, $j:ident, $k:ident, $max: expr, $f:block) => (
		let (mut $i, mut $j, mut $k) = ($max - 2, $max - 1, 0);

		while $k < $max
		{
			$f

			$i = $j;
            $j = $k;
            $k += 1;
		}
	);
}

//=======================================================================//

/// Ends the function call if `$value` is [`None`]. Otherwise it returns the contained value.
#[macro_export]
macro_rules! return_if_none {
    ($value:expr) => {
        match $value
        {
            Some(value) => value,
            None => return
        }
    };

    ($value:expr, $return_value:expr) => {
        match $value
        {
            Some(value) => value,
            None => return $return_value
        }
    };
}

//=======================================================================//

/// Ends the function call if `$value` does not match `$pattern`. Otherwise it returns `$f`
#[macro_export]
macro_rules! return_if_no_match {
    ($value:expr, $pattern:pat, $f:expr) => {
        match $value
        {
            $pattern => $f,
            _ => return
        }
    };

    ($value:expr, $pattern:pat, $f:expr, $return_value:expr) => {
        match $value
        {
            $pattern => $f,
            _ => return $return_value
        }
    };
}

//=======================================================================//

/// Ends the function call if `$value` is [`Err`]. Otherwise it returns the contained value.
#[macro_export]
macro_rules! return_if_err {
    ($value:expr) => {
        match $value
        {
            Ok(value) => value,
            Err(_) => return
        }
    };

    ($value:expr, $return_value:expr) => {
        match $value
        {
            Ok(value) => value,
            Err(_) => return $return_value
        }
    };
}

//=======================================================================//

/// Continues the loop if `$value` is [`None`]. Otherwise it returns the contained value.
#[macro_export]
macro_rules! continue_if_none {
    ($value:expr) => (
		match $value
        {
            Some(value) => value,
            None => continue
        }
	);

    ($value:expr, $label:tt) => (
		match $value
        {
            Some(value) => value,
            None => continue $label
        }
	);
}

//=======================================================================//

/// Continues the loop if `$value` is [`Err`]. Otherwise it returns the contained value.
#[macro_export]
macro_rules! continue_if_err {
    ($value:expr) => {
        match $value
        {
            Ok(value) => value,
            Err(_) => continue
        }
    };
}

//=======================================================================//

/// Continues the loop if `$value` does not match `$pattern`. Otherwise it returns `$f`.
#[macro_export]
macro_rules! continue_if_no_match {
    ($value:expr, $pattern:pat, $f:expr) => {
        match $value
        {
            $pattern => $f,
            _ => continue
        }
    };
}

//=======================================================================//

/// Panics if `$value` does not match `$pattern`. Otherwise it returns `$f`.
#[macro_export]
macro_rules! match_or_panic {
    ($value:expr, $pattern:pat, $f:expr) => {
        match $value
        {
            $pattern => $f,
            _ => panic!("Pattern does not match.")
        }
    };

    ($value:expr, $pattern:pat) => {
        match $value
        {
            $pattern => (),
            _ => panic!("Pattern does not match.")
        }
    };
}

//=======================================================================//
// TYPES
//
//=======================================================================//

pub enum ManualItem
{
    Regular,
    Tool,
    Texture
}

//=======================================================================//
// FUNCTIONS
//
//=======================================================================//

#[allow(clippy::missing_panics_doc)]
#[inline]
pub fn process_manual<
    S: FnMut(&mut String, bool),
    N: FnMut(&mut String, &str, ManualItem),
    P: Fn(&mut String, &str, &PathBuf, ManualItem),
    E: FnMut(&mut String)
>(
    start_string: &str,
    mut section_start: S,
    mut section_name: N,
    process_file: P,
    mut section_end: E
) -> String
{
    impl From<char> for ManualItem
    {
        #[inline]
        fn from(value: char) -> Self
        {
            if value == 'S' || value == 'T'
            {
                Self::Tool
            }
            else if value == 'X'
            {
                Self::Texture
            }
            else
            {
                Self::Regular
            }
        }
    }

    #[inline]
    fn stem_chars(path: &Path) -> (impl Iterator<Item = char> + '_, ManualItem)
    {
        let mut chars = path.file_stem().unwrap().to_str().unwrap().chars();
        let first = chars.next_value();
        (chars.skip_while(|c| !c.is_alphabetic()), first.into())
    }

    let mut string = start_string.to_owned();
    let mut dirs = std::fs::read_dir(PathBuf::from("docs/manual/"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    dirs.sort_unstable();
    let last_index = dirs.len() - 1;

    for (i, dir) in dirs.into_iter().enumerate()
    {
        section_start(&mut string, i == last_index);

        let (mut chars, item) = stem_chars(&dir);
        let mut name = String::from(chars.next_value().to_ascii_uppercase());

        while let Some(mut c) = chars.by_ref().next()
        {
            if c == '_'
            {
                c = ' ';
            }

            name.push(c);
        }

        section_name(&mut string, &name, item);

        let mut paths = std::fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        paths.sort_unstable();

        for path in paths
        {
            let (chars, item) = stem_chars(&path);

            process_file(
                &mut string,
                &chars.collect::<String>(),
                &path,
                item
            );
        }

        section_end(&mut string);
    }

    string
}
