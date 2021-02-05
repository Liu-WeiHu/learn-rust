#![feature(trace_macros)]

/* macro_rules! count_tts {
    () => {
       0 
    };

    ($odd:tt $($a:tt $b:tt)*) => {
        (count_tts!($($a)*) << 1) | 1
    };

    ($($a:tt $even:tt)*) => {
        count_tts!($($a)*) << 1
    };
} */

macro_rules! abacus {
    ((- $($moves:tt)*) -> (+ $($count:tt)*)) => {
        abacus!(($($moves)*) -> ($($count)*))
    };
    ((- $($moves:tt)*) -> ($($count:tt)*)) => {
        abacus!(($($moves)*) -> (- $($count)*))
    };
    ((+ $($moves:tt)*) -> (- $($count:tt)*)) => {
        abacus!(($($moves)*) -> ($($count)*))
    };
    ((+ $($moves:tt)*) -> ($($count:tt)*)) => {
        abacus!(($($moves)*) -> (+ $($count)*))
    };

    // Check if the final result is zero.
    (() -> ()) => { true };
    (() -> ($($count:tt)+)) => { false };
}

macro_rules! function_item_matcher {
    (
        $(#[$meta:meta])*
        $vis:vis fn $name:ident( $($arg:ident: $ty:ty),* $(,)?)
            $( -> $res: ty)?
            {
                $($tt:tt)*
            }
    ) => {
        $( #[$meta])*
        $vis fn $name ( $($arg: $ty),*) $(-> $res: $ty)? {
            $($tt)*
        }
    }
}

macro_rules! struct_item_matcher {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident;
    ) => {
        $(#[$meta])*
        $vis struct $name;
    };
    (
        $( #[$meta:meta])*
        $vis:vis struct $name:ident (
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_ty:ty
            ),*
        )
    ) => {
        $(#[$meta])*
        $vis struct $name (
            $(
                $(#[$field_meta])*
                $field_vis $field_ty
            ),*
        )
    };
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),*
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $(
                $(#[$field_meta])*
                $vis $field_name : $field_ty
            ),*
        }
    }
}

macro_rules! enum_item_matcher {
    (@variant $variant:ident(
        $(
            $(#[$field_meta:meta])*
            $field_vis:vis $field_ty:ty
        ),*
    ) $(, $($tt:tt)*)?) => {
        $(enum_item_matcher! (@variant $($tt)*))?
    };

    (@variant $variant:ident {
        $(
            $(#[field_meta:meta])*
            $field_vis:vis $field_name:ident $field_ty:ty
        ),*
    } $(, $($tt:tt)*)?) => {
        $( enum_item_matcher!(@variant $($tt)*))?
    };
    (@variant) => {};
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($tt:tt)*
        }
    ) => {
        enum_item_matcher!(@variant $($tt)*)
    }
}

fn main() {
    /* trace_macros!(true);
    let a = count_tts!(0 0 0 0 0 0 0 0 0 0);
    trace_macros!(false);
    println!("{}", a); */
    /* let mut i = 4;
    i = i | 1;
    println!("{}",i); */
    let equals_zero = abacus!((++-+-+++--++---++----+) -> ());
    assert_eq!(equals_zero, true);
}
