#![feature(trace_macros)]

/* macro_rules! as_expr {
    ($e: expr) => {$e}
}

macro_rules! foo {
    ($($tts: tt)*) => {
        as_expr!($($tts)*)
    };
} */

//替代上面的方案
/* macro_rules! foo {
    (@as_ptr $e:expr) => {$e};

    ($($tts:tt)*) => {foo!(@as_ptr $($tts)*)}
} */

//来个复杂的
/* #![feature(trace_macros)]

macro_rules! init_array {
    (@accum (0, $_e:expr) -> ($($body: tt)*)) => {
        {
            init_array!(@as_expr [$($body)*])
        }
    };
    (@accum (1, $e: expr) -> ($($body: tt)*)) => {
        {
            init_array!(@accum(0, $e) -> ($($body)* $e,))
        }
    };
    (@accum (2, $e: expr) -> ($($body: tt)*)) => {
        {
            init_array!(@accum(1, $e) -> ($($body)* $e,))
        }
    };
    (@accum (3, $e: expr) -> ($($body: tt)*)) => {
        {
            init_array!(@accum(2, $e) -> ($($body)* $e,))
        }
    };
    (@as_expr $e: expr) => {
        {
            $e
        }
    };
    [$e: expr; $n: tt] => {
        {
            let e = $e;
            init_array!(@accum ($n, e.clone()) -> ())
        }
    }
} */

//重复替换
macro_rules! replace_expr {
    ($_t: tt $sub: expr) => {
        $sub
    };
}

/* macro_rules! tuple_default {
    ($($tup_ty: ty),*) => {
        {
            $(
                replace_expr!(
                    ($tup_ty)
                    Default::default()
                ),
            )*
        }
    }
} */

/* macro_rules! tuple_default {
    ($($tup_tys: ty),*) => {
        {
            $(
                $tup_tys::default(),
            )*
        }
    }
}  */

macro_rules! call_a_or_b_on_tail {
    ((a: $a:ident, b:$b:ident), call a: $($tail:tt)*) => {
        $a(stringify!($($tail)*))
    };
    ((a: $a:ident, b:$b:ident), call b: $($tail:tt)*) => {
        $b(stringify!($($tail)*))
    };
    ($ab:tt, $_skip:tt $($tail:tt)*) => {
        call_a_or_b_on_tail!($ab, $($tail)*)
    };
}

fn compute_len(s: &str) -> Option<usize> {
    Some(s.len())
}

fn show_tail(s: &str) -> Option<usize> {
    println!("tail: {:?}", s);
    None
}

fn main() {
    /* let a = foo!{
        "sdfsf"
    };
    println!("{}",a); */

    /* trace_macros!(true);
    let s: [String; 3] = init_array![String::from("hi!"); 3];
    trace_macros!(false);
    println!("{:?}", s); */


    call_a_or_b_on_tail!(
        (a: compute_len, b: show_tail),
        the recursive part that skips over all these
        tokens doesn't much care whether we will call a
        or call b: only the terminal rules care.
    );


    trace_macros!(true);
    let s = call_a_or_b_on_tail!(
        (a: compute_len, b: show_tail),
        and now, to justify the existence of two paths
        we will also call a: its input should somehow
        be self-referential, so let's make it return
        some eighty-six!
    );
    trace_macros!(false);
    println!("{}", s.unwrap());
}
