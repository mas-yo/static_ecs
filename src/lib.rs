#[macro_use]
pub mod typeset;

pub mod component;
use component::ID;

impl component::ID for u32 {
    fn next(&self) -> Self {
        self + 1
    }
}

pub type EntityID = u32;
pub type CC<T> = component::CContainer<EntityID, T>;

#[macro_export]
macro_rules! world {
    ( $i:ident { $($t:ty),+ $(,)? } ) => {
        typeset!{ Components { $( CC<$t> ),+ } }
        type $i = (EntityID, Components);
        pub trait PushComponent<C> {
            fn push_component(&mut self, component: C);
        }
        pub trait RemoveComponent {
            fn remove_component(&mut self, entity_id: EntityID);
        }
        $(
            impl PushComponent<$t> for $i {
                fn push_component(&mut self, component: $t) {
                    component_mut!(self, $t).push(self.0, component);
                }
            }
        )+
        impl RemoveComponent for $i {
            fn remove_component(&mut self, entity_id: EntityID) {
                $(
                    component_mut!(self, $t).remove(entity_id);
                )+
            }
        }
    };
}

#[macro_export]
macro_rules! component {
    ( $world:expr, $t:ty ) => {
        typeref!($world.1, CC<$t>)
    };
}

#[macro_export]
macro_rules! component_mut {
    ( $world:expr, $t:ty) => {
        typerefmut!($world.1, CC<$t>)
    };
}

#[macro_export]
macro_rules! add_entity {
    ( $i:ident; $($e:expr),+ $(,)?) => {
        $(
            $i.push_component($e);
        )+
        $i.0 = $i.0.next();
    };
}

#[macro_export]
macro_rules! system {
    ( $world:expr, |$entity_id:ident, $id:ident : & $typ:ty| $b:block ) => {
        let output: Vec<(EntityID, $typ)> = component!($world, $typ)
            .iter()
            .map(|(eid, data)| {
                let new_data: $typ = (|$entity_id, $id: &$typ| $b)(eid, data);
                (eid, new_data)
            })
            .collect();

        let update = component_mut!($world, $typ);
        for (eid, data) in output {
            if let Some(d) = update.get_mut(eid) {
                *d = data;
            }
        }
    };
    ( $world:expr, | $entity_id:ident, $id1:ident : & $typ1:ty, $id2:ident : & $typ2:ty | $b:block ) => {
        let output: Vec<(EntityID, $typ1)> = component!($world, $typ1)
            .iter()
            .zip_entity(component!($world, $typ2))
            .map(
                |(eid, data1, data2): (EntityID, &$typ1, &$typ2)| {
                    let new_data: $typ1 =
                        (|$entity_id, $id1: &$typ1, $id2: &$typ2| $b)(eid, data1, data2);
                    (eid, new_data)
                },
            )
            .collect();

        let update = component_mut!($world, $typ1);
        for (eid, new_data) in output {
            if let Some(d) = update.get_mut(eid) {
                *d = new_data;
            }
        }
    };
    ( $world:expr, |$entity_id:ident, $id1:ident : & $typ1:ty, $id2:ident : & $typ2:ty, $id3:ident : & $typ3:ty | $b:block) => {
        let output: Vec<(EntityID, $typ1)> = component!($world, $typ1)
            .iter()
            .zip_entity2(component!($world, $typ2), component!($world, $typ3))
            .map(|(eid, data1, data2, data3)| {
                let new_data: $typ1 = (|$entity_id, $id1: &$typ1, $id2: &$typ2, $id3: &$typ3| $b)(
                    eid, data1, data2, data3,
                );
                (eid, new_data)
            })
            .collect();

        let update = component_mut!($world, $typ1);
        for (eid, new_data) in output {
            if let Some(d) = update.get_mut(eid) {
                *d = new_data;
            }
        }
    };
}

#[cfg(test)]
mod tests {

    use crate::*;

    world! {
        World {
            i32,
            f32,
            // u32,
        }
    }

    #[test]
    fn it_works() {
        let mut a = World::default();
        add_entity!(a; 1i32, 1f32);
        add_entity!(a; 1i32, 1f32);
        add_entity!(a; 1i32, 1f32);
        add_entity!(a; 1i32, 1f32);
        add_entity!(a; 1i32, 1f32);
        
        system!(a, |eid, i: &i32, f: &f32| {
            println!("{}", eid);
            1
        });

        a.remove_component(2);
        system!(a, |eid, i: &i32, f: &f32| {
            println!("{}", eid);
            1
        });

    }
}
