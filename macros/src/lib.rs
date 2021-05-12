#[macro_export]
macro_rules! pipe_fun {
    (($fun:ident($($arg:expr),*)), $ret:expr) => {
        $fun($ret $(,$arg)*);
    };
    ([$fun:ident], $ret:expr) => {
        $ret.$fun();
    };
    ([$fun:ident($($arg:expr),*)], $ret:expr) => {
        $ret.$fun($($arg,)*);
    };
    ($fun:ident, $ret:expr) => {
        $fun($ret);
    }
}

#[macro_export]
macro_rules! pipe {
    ( $expr:expr => $($funs:tt)=>+ ) => {
        {
            let __value = $expr;
            $(
                let __value = pipe_fun!($funs, __value);
            )*
            __value
        }
    };
}
