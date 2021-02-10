use std::collections::HashMap;
use std::hash::Hash;

trait ID {
    fn next(&self) -> Self;
}

impl ID for u32 {
    fn next(&self) -> Self {
        self + 1
    }
}

// #[derive(Clone, Copy, Eq, PartialEq, Hash, Default)]
// pub struct EntityID {
//     id: u32,
// }
// impl EntityID {
//     pub fn new() -> Self {
//         Self { id: 0 }
//     }
//     pub fn next(&self) -> Self {
//         Self { id: self.id + 1 }
//     }
// }

pub struct Component<I, T> {
    entity_id: I,
    inner: T,
}

impl<I, T> Component<I, T>
where
    I: Copy,
{
    pub fn entity_id(&self) -> I {
        self.entity_id
    }
}

impl<I, T> Component<I, T> {
    pub fn new(entity_id: I, inner: T) -> Self {
        Self {
            entity_id: entity_id,
            inner: inner,
        }
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

pub type CContainer<I, T> = ComponentContainer<I, T>;

// #[derive(Default)]
pub struct ComponentContainer<I, T> {
    map: HashMap<I, usize>,
    vec: Vec<Component<I, T>>,
}

impl<I, T> Default for ComponentContainer<I, T>
where
    I: Eq + Hash,
{
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }
}

impl<I, T> ComponentContainer<I, T>
where
    I: Copy + Eq + Hash,
{
    pub fn push(&mut self, entity_id: I, item: T) {
        self.vec.push(Component::<I, T>::new(entity_id, item));
        self.map.insert(entity_id, self.vec.len() - 1);
    }
    pub fn get(&self, entity_id: I) -> Option<&T> {
        let index = self.map.get(&entity_id)?;
        Some(self.vec[*index].inner())
    }
    pub fn get_mut(&mut self, entity_id: I) -> Option<&mut T> {
        let index = self.map.get(&entity_id)?;
        Some(self.vec[*index].inner_mut())
    }
    pub fn iter(&self) -> ComponentIter<I, T> {
        ComponentIter {
            iter: self.vec.iter(),
        }
    }
    pub fn iter_mut(&mut self) -> ComponentIterMut<I, T> {
        ComponentIterMut {
            iter: self.vec.iter_mut(),
        }
    }
}

pub struct ComponentIter<'a, I, T>
where
    T: 'a,
{
    iter: std::slice::Iter<'a, Component<I, T>>,
}
impl<'a, I, T> Iterator for ComponentIter<'a, I, T>
where
    I: Copy,
{
    type Item = (I, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        Some((next.entity_id(), next.inner()))
    }
}
impl<'a, I, T> ComponentIter<'a, I, T> {
    pub fn zip_entity<U>(self, other: &'a CContainer<I, U>) -> ZipEntity<'a, I, T, U> {
        ZipEntity {
            base: self,
            other: other,
        }
    }
    pub fn zip_entity2<U, V>(
        self,
        other1: &'a CContainer<I, U>,
        other2: &'a CContainer<I, V>,
    ) -> ZipEntity2<'a, I, T, U, V> {
        ZipEntity2 {
            base: self,
            other1: other1,
            other2: other2,
        }
    }
}
pub struct ComponentIterMut<'a, I, T>
where
    T: 'a,
{
    iter: std::slice::IterMut<'a, Component<I, T>>,
}
impl<'a, I, T> Iterator for ComponentIterMut<'a, I, T>
where
    I: Copy,
{
    type Item = (I, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        Some((next.entity_id(), next.inner_mut()))
    }
}
impl<'a, I, T> ComponentIterMut<'a, I, T> {
    pub fn zip_entity<U>(self, other: &'a CContainer<I, U>) -> ZipEntityMut<'a, I, T, U> {
        ZipEntityMut {
            base: self,
            other: other,
        }
    }
    pub fn zip_entity2<U, V>(
        self,
        other1: &'a CContainer<I, U>,
        other2: &'a CContainer<I, V>,
    ) -> ZipEntity2Mut<'a, I, T, U, V> {
        ZipEntity2Mut {
            base: self,
            other1: other1,
            other2: other2,
        }
    }

    pub fn zip_entity3<U, V, W>(
        self,
        other1: &'a CContainer<I, U>,
        other2: &'a CContainer<I, V>,
        other3: &'a CContainer<I, W>,
    ) -> ZipEntity3Mut<'a, I, T, U, V, W> {
        ZipEntity3Mut {
            base: self,
            other1: other1,
            other2: other2,
            other3: other3,
        }
    }
}

pub struct ZipEntity<'a, I, T, U>
where
    T: 'a,
    U: 'a,
{
    base: ComponentIter<'a, I, T>,
    other: &'a CContainer<I, U>,
}

impl<'a, I, T, U> Iterator for ZipEntity<'a, I, T, U>
where
    I: Copy + Eq + Hash,
{
    type Item = (I, &'a T, &'a U);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entity_id, base)) = self.base.next() {
            if let Some(other_item) = self.other.get(entity_id) {
                return Some((entity_id, base, other_item));
            }
        }
        None
    }
}

pub struct ZipEntityMut<'a, I, T, U>
where
    T: 'a,
    U: 'a,
{
    base: ComponentIterMut<'a, I, T>,
    other: &'a CContainer<I, U>,
}

impl<'a, I, T, U> Iterator for ZipEntityMut<'a, I, T, U>
where
    I: Copy + Eq + Hash,
{
    type Item = (I, &'a mut T, &'a U);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entity_id, base)) = self.base.next() {
            if let Some(other_item) = self.other.get(entity_id) {
                return Some((entity_id, base, other_item));
            }
        }
        None
    }
}
pub struct ZipEntity2<'a, I, T, U, V>
where
    T: 'a,
    U: 'a,
    V: 'a,
{
    base: ComponentIter<'a, I, T>,
    other1: &'a CContainer<I, U>,
    other2: &'a CContainer<I, V>,
}

impl<'a, I, T, U, V> Iterator for ZipEntity2<'a, I, T, U, V>
where
    I: Copy + Eq + Hash,
{
    type Item = (I, &'a T, &'a U, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entity_id, base)) = self.base.next() {
            let other1_item = self.other1.get(entity_id);
            let other2_item = self.other2.get(entity_id);
            if other1_item.is_some() && other2_item.is_some() {
                return Some((entity_id, base, other1_item.unwrap(), other2_item.unwrap()));
            }
        }
        None
    }
}

pub struct ZipEntity2Mut<'a, I, T, U, V>
where
    T: 'a,
    U: 'a,
    V: 'a,
{
    base: ComponentIterMut<'a, I, T>,
    other1: &'a CContainer<I, U>,
    other2: &'a CContainer<I, V>,
}

impl<'a, I, T, U, V> Iterator for ZipEntity2Mut<'a, I, T, U, V>
where
    I: Copy + Eq + Hash,
{
    type Item = (I, &'a mut T, &'a U, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entity_id, base)) = self.base.next() {
            let other1_item = self.other1.get(entity_id);
            let other2_item = self.other2.get(entity_id);
            if other1_item.is_some() && other2_item.is_some() {
                return Some((entity_id, base, other1_item.unwrap(), other2_item.unwrap()));
            }
        }
        None
    }
}

pub struct ZipEntity3Mut<'a, I, T, U, V, W>
where
    T: 'a,
    U: 'a,
    V: 'a,
    W: 'a,
{
    base: ComponentIterMut<'a, I, T>,
    other1: &'a CContainer<I, U>,
    other2: &'a CContainer<I, V>,
    other3: &'a CContainer<I, W>,
}

impl<'a, I, T, U, V, W> Iterator for ZipEntity3Mut<'a, I, T, U, V, W>
where
    I: Copy + Eq + Hash,
{
    type Item = (&'a mut T, &'a U, &'a V, &'a W);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((entity_id, base)) = self.base.next() {
            let other1_item = self.other1.get(entity_id);
            let other2_item = self.other2.get(entity_id);
            let other3_item = self.other3.get(entity_id);
            if other1_item.is_some() && other2_item.is_some() && other3_item.is_some() {
                return Some((
                    base,
                    other1_item.unwrap(),
                    other2_item.unwrap(),
                    other3_item.unwrap(),
                ));
            }
        }
        None
    }
}
