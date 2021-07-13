mod component;
use component::*;

pub use derive_world::*;

pub type EntityID = u32;
pub type CContainer<T> = ComponentContainer<EntityID, T>;

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

pub fn system0<T, P>(container: &mut CContainer<T>, pred: P)
where
    P: Fn(EntityID, &mut T),
{
    for (eid, d) in container.iter_mut() {
        pred(eid, d);
    }
}
pub fn system1<T1, T2, P>(container: (&mut CContainer<T1>, &CContainer<T2>), pred: P)
where
    P: Fn(EntityID, &mut T1, &T2),
{
    for (eid, d1, d2) in container.0.iter_mut().zip_entity(container.1) {
        pred(eid, d1, d2);
    }
}
pub fn system2<T1, T2, T3, P>(
    container: (&mut CContainer<T1>, &CContainer<T2>, &CContainer<T3>),
    pred: P,
) where
    P: Fn(EntityID, &mut T1, &T2, &T3),
{
    for (eid, d1, d2, d3) in container.0.iter_mut().zip_entity2(container.1, container.2) {
        pred(eid, d1, d2, d3);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
