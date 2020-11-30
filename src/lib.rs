#[macro_use]
mod typeset;


mod component;

#[macro_export]
macro_rules! world {
    ( $i:ident { $($t:ty),+ $(,)? } ) => {
        use crate::component::{EntityID, ComponentContainer};

        typeset!{ Components { $( ComponentContainer<$t> ),+ } }
        type $i = (EntityID, Components);
        pub trait PushComponent<C> {
            fn push_component(&mut self, component: C);
        }
        $(
            impl PushComponent<$t> for $i {
                fn push_component(&mut self, component: $t) {
                    TypeRefMut::<ComponentContainer<$t>>::type_ref_mut(&mut self.1).push(self.0, component);
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! add_entity {
    ( $i:ident; $($e:expr),+ $(,)?) => {
        $(
            $i.push_component($e);
        )+
        $i.0 += 1;
    };
}

#[macro_export]
macro_rules! system {
    ( $world:expr; $typ:ty ; $lambda:expr) => {
        let mut output = Vec::<(EntityID, $typ)>::new();
        TypeRef::<ComponentContainer<$typ>>::type_ref(&$world.1).iter().map($lambda).for_each(|(eid,data)|{
            output.push((eid, data));
        });
        let update = TypeRefMut::<ComponentContainer<$typ>>::type_ref_mut(&mut $world.1);
        for (eid, data) in output {
            if let Some(d) = update.get_mut(eid) {
                *d = data;
            }
        }
    };
    ( $world:expr; $typ1:ty; $typ2:ty ; $lambda:expr) => {
        let mut output = Vec::<(EntityID, $typ1)>::new();
        TypeRef::<ComponentContainer<$typ1>>::type_ref(&$world.1).iter()
        .zip_entity(TypeRef::<ComponentContainer<$typ2>>::type_ref(&$world.1))
        .map($lambda)
        .for_each(|(eid,data)|{
            output.push((eid, data));
        });
        let update = TypeRefMut::<ComponentContainer<$typ1>>::type_ref_mut(&mut $world.1);
        for (eid, data) in output {
            if let Some(d) = update.get_mut(eid) {
                *d = data;
            }
        }
    };
    ( $world:expr; $typ1:ty; $typ2:ty, $typ3:ty ; $lambda:expr) => {
        let mut output = Vec::<(EntityID, $typ1)>::new();
        TypeRef::<ComponentContainer<$typ1>>::type_ref(&$world.1).iter()
        .zip_entity2(TypeRef::<ComponentContainer<$typ2>>::type_ref(&$world.1), TypeRef::<ComponentContainer<$typ3>>::type_ref(&$world.1))
        .map($lambda)
        .for_each(|(eid,data)|{
            output.push((eid, data));
        });
        let update = TypeRefMut::<ComponentContainer<$typ1>>::type_ref_mut(&mut $world.1);
        for (eid, data) in output {
            if let Some(d) = update.get_mut(eid) {
                *d = data;
            }
        }
    };
}

#[cfg(test)]
mod tests {
    world! {
        World {
            i32,
            f32,
            u32,
        }
    }
    
    #[test]
    fn it_works() {
        let mut a = World::default();
        add_entity!(a; 1i32, 1f32);
        system!( a; i32; f32, u32; |(_,i,f,u)|{ (1,*i) });
    }
}
