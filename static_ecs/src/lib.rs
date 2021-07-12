mod component;
use component::*;

pub use derive_world::*;

pub type EntityID = u32;
pub type CContainer<T> = ComponentContainer<EntityID, T>;

// pub trait TypeRef<'a, R>
// where
//     R: 'a,
// {
//     fn type_ref(&'a self) -> R;
// }
// pub trait TypeRefMut<'a, R>
// where
//     R: 'a,
// {
//     fn type_ref_mut(&'a mut self) -> R;
// }

pub trait PushComponent<C> {
    fn push_component(&mut self, entity_id: EntityID, component: C);
}
pub trait RemoveComponent {
    fn remove_component(&mut self, entity_id: EntityID);
}

pub trait GetComponent<'a, R>
where
    R: 'a,
{
    fn get_component(&'a self) -> R;
}
pub trait GetComponentMut<'a, R>
where
    R: 'a,
{
    fn get_component_mut(&'a mut self) -> R;
}

// #[derive(Default)]
// pub struct World<CC> {
//     next_entity_id: EntityID,
//     components: CC,
// }

// impl<'a, R, CC> GetComponent<'a, R> for World<CC>
// where
//     R: 'a,
//     CC: TypeRef<'a, R>,
// {
//     fn get_component(&'a self) -> R {
//         self.components.type_ref()
//     }
// }
// impl<'a, R, CC> GetComponentMut<'a, R> for World<CC>
// where
//     R: 'a,
//     CC: TypeRefMut<'a, R>,
// {
//     fn get_component_mut(&'a mut self) -> R {
//         self.components.type_ref_mut()
//     }
// }
// // impl<CC, C> PushComponent<C> for World<CC>
// // where
// //     CC: TypeRefMut<'a, (&mut CContainer<EntityID, C>)>,
// // {
// //     fn push_component(&mut self, component: C) {
// //         let mut comps: (&mut CContainer<EntityID, C>) = self.components.type_ref_mut();
// //         comps.push(self.next_entity_id, component);
// //     }
// // }

// #[macro_export]
// macro_rules! world {
//     ( $type_name:ident { $($t:ident),+ $(,)? } ) => {

//         use static_ecs::{EntityID,CContainer};
//         use static_ecs::{PushComponent,RemoveComponent};

//         //add next_entity_id
//         #[derive(TypeRef,Default)]
//         struct $type_name( $(CContainer<$t>),+ );

//         $(
//             impl crate::PushComponent<$t> for $type_name {
//                 fn push_component(&mut self, entity_id: EntityID, component: $t) {
//                     let (comp): (&mut CContainer<$t>) = self.type_ref_mut();
//                     comp.push(entity_id, component);
//                 }
//             }
//         )+
//         impl crate::RemoveComponent for $type_name {
//             fn remove_component(&mut self, entity_id: crate::EntityID) {
//                 $(
//                     let (comp): (&mut CContainer<$t>) = self.type_ref_mut();
//                     comp.remove(entity_id);
//                 )+
//             }
//         }
//         //impl GetComponent,GetComponentMut
//         impl<'a, R> GetComponent<'a, R> for $type_name
//         where
//             R: 'a,
//         {
//             fn get_component(&'a self) -> R {
//                 self.type_ref()
//             }
//         }
// };
// }

// macro_rules! add_entity {
//     ( $i:expr; ( $($e:expr),+ )) => {
//         $(
//             $i.push_component($e);
//         )+
//         $i.next_entity_id += 1;// $i.0.next();
//     };
// }

// pub fn system0<T, P>(container: &mut CContainer<T>, pred: P)
// where
//     P: Fn(EntityID, &mut T),
// {
//     for (eid, d) in container.iter_mut() {
//         pred(eid, d);
//     }
// }
// pub fn system1<T1, T2, P>(container: (&mut CContainer<T1>, &CContainer<T2>), pred: P)
// where
//     P: Fn(EntityID, &mut T1, &T2),
// {
//     for (eid, d1, d2) in container.0.iter_mut().zip_entity(container.1) {
//         pred(eid, d1, d2);
//     }
// }
// pub fn system2<T1, T2, T3, P>(
//     container: (&mut CContainer<T1>, &CContainer<T2>, &CContainer<T3>),
//     pred: P,
// ) where
//     P: Fn(EntityID, &mut T1, &T2, &T3),
// {
//     for (eid, d1, d2, d3) in container.0.iter_mut().zip_entity2(container.1, container.2) {
//         pred(eid, d1, d2, d3);
//     }
// }

// mod component;

#[cfg(test)]
mod tests {
    use crate::component::*;
    use crate::system1;
    use crate::CContainer;
    use crate::GetComponent;
    use crate::GetComponentMut;
    use crate::PushComponent;
    use crate::TypeRef;
    use crate::TypeRefMut;
    use impl_type_ref;
    #[test]
    fn it_works() {
        world!(World { i32, f32, u32 });
        let mut w = World::default();

        add_entity!(w; (1i32, 2u32));
        add_entity!(w; (1i32, 3f32));

        // let (c1, c2) = w.components.type_ref_mut();

        system1(
            w.get_component_mut(), // components.type_ref_mut(),
            |entity_id, d1: &mut i32, d2: &u32| *d1 += *d2 as i32,
        );

        let ci32: (&CContainer<i32>) = w.get_component();
        for i in ci32.iter() {
            println!("{}", i.1);
        }
    }
}
