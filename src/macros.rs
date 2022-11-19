/// Create a [`Transcriber`] succinctly, with optional configuration.
///
/// # Examples
///
/// The basic syntax is as follows:
/// ```
/// use tengwar::{Quenya, Transcriber, tscr};
///
/// let ts: Transcriber<_> = tscr!(Quenya; "input");
/// let _s: String = ts.collect();
/// ```
///
/// The advanced form of the syntax is as follows:
/// ```
/// use tengwar::{Quenya, tscr};
///
/// let _s = tscr!(Quenya[alt_a=true, alt_rince=true]; "input", as String);
/// ```
///
/// The Mode may also be inferred:
/// ```
/// use tengwar::{Quenya, Transcriber, tscr};
///
/// let ts: Transcriber<Quenya> = tscr!("input");
/// let _s: String = ts.collect();
/// ```
///
/// [`Transcriber`]: crate::Transcriber
//  TODO: Decide whether to leave this public. If it is left public, bring the
//      docs up to standard.
#[macro_export]
macro_rules! tscr {
    //  Collection via "as" keyword. Primary case.
    //  let ts = tscr!(Quenya[k=v, k=v]; "input", as String);
    (
        $mode:ty // Mode to use.
        $([$($key:ident $($op:tt $value:expr)?),* $(,)?])? // Settings.
        ; $input:expr // Input. Must implement `AsRef<str>`.
        $(, $(as $target:ty)?)? // Output. Must implement `FromIterator<Token>`.
    ) => {{
        let mut t = <$mode as $crate::TengwarMode>::default_transcriber($input);
        $($(t.settings.$key $($op $value)?;)*)?
        t $($(.collect::<$target>())?)?
    }};

    //  Implicit Mode.
    //  let ts: Transcriber<Quenya> = tscr!("input");
    ($input:expr $(, $(as $target:ty)?)?) => {{
        let t = <_ as $crate::TengwarMode>::default_transcriber($input);
        t $($(.collect::<$target>())?)?
    }};
}
