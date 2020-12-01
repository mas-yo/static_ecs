// #[macro_export]
macro_rules! inner_typeset {
    ( $t: ty ) => {
        ($t, ())
    };
    ( $th:ty, $( $tt: ty ),+ $(,)? ) => {
        ($th, inner_typeset!($($tt),+))
    };
}

// #[macro_export]
macro_rules! impl_typeset {
    ( $torg:ty, $t: ty ) => {
        pub trait TypeRef<T> {
            fn type_ref(&self) -> &T;
        }
        pub trait TypeRefMut<T> {
            fn type_ref_mut(&mut self) -> &mut T;
        }
        impl TypeRef<$t> for $torg {
            fn type_ref(&self) -> &$t {
                &self.0
            }
        }
        impl TypeRefMut<$t> for $torg {
            fn type_ref_mut(&mut self) -> &mut $t {
                &mut self.0
            }
        }
    };
    ( $torg:ty, $th:ty, $( $tt: ty ),+ $(,)? ) => {
        impl TypeRef<$th> for $torg {
            fn type_ref(&self) -> &$th {
                &self.0
            }
        }
        $(
            impl TypeRef<$tt> for $torg {
                fn type_ref(&self) -> &$tt {
                    self.1.type_ref()
                }
            }
        )+
        impl TypeRefMut<$th> for $torg {
            fn type_ref_mut(&mut self) -> &mut $th {
                &mut self.0
            }
        }
        $(
            impl TypeRefMut<$tt> for $torg {
                fn type_ref_mut(&mut self) -> &mut $tt {
                    self.1.type_ref_mut()
                }
            }
        )+
        impl_typeset!{ inner_typeset!($($tt),+), $($tt),+ }
    };
}
// #[macro_export]
macro_rules! typeset {
    ( $i:ident { $($t:ty),+ $(,)? } ) => {
        type $i = inner_typeset!($($t),+);
        impl_typeset!{$i, $($t),+}
    }
}

// #[macro_export]
macro_rules! typeref {
    ( $i:ident, $t:ty ) => {
        TypeRef::<$t>::type_ref(&$i)
    };
}
// #[macro_export]
macro_rules! typerefmut {
    ( $i:ident, $t:ty ) => {
        TypeRefMut::<$t>::type_ref_mut(&mut $i)
    };
}

// #[test]
// fn test() {

//     typeset!(World { i32, f32, u32 });
//     {
//     let mut w = World::default();
//     let b:&f32 = w.type_ref();
//     let a:&mut i32 = w.type_ref_mut();
//     *a = *b as i32;
//     }
// }
