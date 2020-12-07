#[macro_export]
macro_rules! recursive_tuple {
    ( $t: ty ) => {
        ($t, ())
    };
    ( $th:ty, $( $tt: ty ),+ $(,)? ) => {
        ($th, recursive_tuple!($($tt),+))
    };
}

#[macro_export]
macro_rules! impl_typeref {
    ( $torg:ty, $t: ty ) => {
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
        impl_typeref!{ recursive_tuple!($($tt),+), $($tt),+ }
    };
}

#[macro_export]
macro_rules! typeset {
    ( $i:ident { $($t:ty),+ $(,)? } ) => {
        pub trait TypeRef<T> {
            fn type_ref(&self) -> &T;
        }
        pub trait TypeRefMut<T> {
            fn type_ref_mut(&mut self) -> &mut T;
        }
        type $i = recursive_tuple!($($t),+);
        impl_typeref!{$i, $($t),+}
    }
}

#[macro_export]
macro_rules! typeref {
    ( $e:expr, $t:ty ) => {
        TypeRef::<$t>::type_ref(&$e)
    };
}
#[macro_export]
macro_rules! typerefmut {
    ( $e:expr, $t:ty ) => {
        TypeRefMut::<$t>::type_ref_mut(&mut $e)
    };
}
