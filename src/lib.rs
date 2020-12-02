#[macro_use]
pub mod typeset;

pub mod component;

#[macro_export]
macro_rules! world {
    ( $i:ident { $($t:ty),+ $(,)? } ) => {
        typeset!{ Components { $( component::CContainer<$t> ),+ } }
        type $i = (component::EntityID, Components);
        pub trait PushComponent<C> {
            fn push_component(&mut self, component: C);
        }
        $(
            impl PushComponent<$t> for $i {
                fn push_component(&mut self, component: $t) {
                    component_mut!(self, $t).push(self.0, component);
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! component {
    ( $world:expr, $t:ty ) => {
        typeref!($world.1, component::CContainer<$t>)
    };
}

#[macro_export]
macro_rules! component_mut {
    ( $world:expr, $t:ty) => {
        typerefmut!($world.1, component::CContainer<$t>)
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
    ( $world:expr, |$entity_id:ident, $id:ident : & $typ:ty| $b:block ) => {
        let output: Vec<(component::EntityID, $typ)> = component!($world, $typ)
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
        let output: Vec<(component::EntityID, $typ1)> = component!($world, $typ1)
            .iter()
            .zip_entity(component!($world, $typ2))
            .map(
                |(eid, data1, data2): (component::EntityID, &$typ1, &$typ2)| {
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
        let output: Vec<(component::EntityID, $typ1)> = component!($world, $typ1)
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
            u32,
        }
    }

    #[test]
    fn it_works() {
        let mut a = World::default();
        add_entity!(a; 1i32, 1f32);
        let re = component_mut!(a, i32);

        system!(a, |eid, i: &i32, f: &f32, u: &u32| {
            *i;
            1
        });
    }
}
