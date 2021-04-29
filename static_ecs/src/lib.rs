use member_of_type::declare_member_of_type;

pub type EntityID = u32;

pub trait PushComponent<C> {
    fn push_component(&mut self, component: C);
}
pub trait RemoveComponent {
    fn remove_component(&mut self, entity_id: EntityID);
}

#[derive(Default)]
struct World<CC> {
    next_entity_id: EntityID,
    components: CC,
}

macro_rules! world {
    ( $type_name:ident { $($t:ident),+ $(,)? } ) => {
        type ComponentContainerTuple = ($(CContainer<crate::EntityID, $t>),+);
        member_of_type::declare_member_of_type!(($(CContainer<crate::EntityID, $t>),+,));

        type $type_name = crate::World<ComponentContainerTuple>;
        $(
            impl crate::PushComponent<$t> for $type_name {
                fn push_component(&mut self, component: $t) {
                    component!(self, $t).push(self.next_entity_id, component);
                }
            }
        )+
        impl crate::RemoveComponent for $type_name {
            fn remove_component(&mut self, entity_id: crate::EntityID) {
                $(
                    component!(self, $t).remove(entity_id);
                )+
            }
        }
    };
}

macro_rules! component {
    ($e:expr, $i:ident) => {
        member_of_type!($e.components, CContainer<crate::EntityID, $i>)
    }
}

macro_rules! add_entity {
    ( $i:expr; ( $($e:expr),+ )) => {
        $(
            $i.push_component($e);
        )+
        $i.next_entity_id += 1;// $i.0.next();
    };
}

macro_rules! system {
    ( $world:expr, |$entity_id:ident, $id:ident: &mut $typ:ident| $b:block ) => {
        for (eid, data) in component!($world, $typ).iter_mut() {
            (|$entity_id, $id: &mut $typ| $b)(eid, data);
        }
    };
    ( $world:expr, |$entity_id:ident, $id1:ident: &mut $typ1:ident, $id2:ident: & $typ2:ident| $b:block ) => {
        for (eid, data1, data2) in component!($world, $typ1).iter_mut().zip_entity(&component!($world, $typ2)) {
            (|$entity_id, $id1: &mut $typ1, $id2: & $typ2| $b)(eid, data1, data2);
        }
    };
    ( $world:expr, |$entity_id:ident, $id1:ident: &mut $typ1:ident, $id2:ident: & $typ2:ident, $id3:ident: & $typ3:ident| $b:block ) => {
        for (eid, data1, data2, data3) in component!($world, $typ1).iter_mut().zip_entity2(&component!($world, $typ2), &component!($world, $typ3)) {
            (|$entity_id, $id1: &mut $typ1, $id2: & $typ2, $id3: & $typ3| $b)(eid, data1, data2, data3);
        }
    };
}

mod component;

#[cfg(test)]
mod tests {

    use crate::component::*;
    use crate::PushComponent;
    #[test]
    fn it_works() {
        world!(World { i32, u32, f32 });
        let mut w = World::default();
        add_entity!(w; (1i32, 1u32, 1f32));
        add_entity!(w; (2i32, 2u32, 2f32));
        add_entity!(w; (3i32, 3u32, 3f32));


        system!(w, |eid, data1: &mut i32, data2: &u32, data3: &f32| {
            println!("---{}", data1);
            *data1 += *data2 as i32 + *data3 as i32;
        });
        system!(w, |eid, data: &mut i32| {
            println!("---{}", data);
        });
   }
}