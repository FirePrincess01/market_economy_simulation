use crate::ecs::component::Component;

use super::Entity;

pub struct EntityIterator<'a, T1, T2>
{
    entities: &'a Vec<Entity>, 
    data: &'a mut Vec<T1>,
    filter1: &'a Vec<T2>,

    index: usize,
}

impl<'a, T1, T2> EntityIterator<'a, T1, T2> {
    pub fn new(entities: &'a Vec<Entity>, 
    data: &'a mut Vec<T1>,
    filter1: &'a Vec<T2>,) -> Self 
    {
        Self {
            entities,
            data,
            filter1,
            index: 0,
        }
    }
}

impl<'a, T1: Component, T2: Component> Iterator for EntityIterator<'a, T1, T2>
{
    // type Item;
    type Item = (&'a mut T1, &'a T2);

    fn next(&mut self) -> Option<Self::Item> {

        if self.index < self.data.len()
        {
            let data1 = &mut self.data[self.index];
            let entity_index = data1.get_entity_index();
            let entity = &self.entities[entity_index];

            let filter1_index = T1::get_value_index(entity);

            match filter1_index {
                Some(filter1_index) => {
                    // https://stackoverflow.com/questions/27118398/simple-as-possible-example-of-returning-a-mutable-reference-from-your-own-iterat
                    // I copied this code from Stack Overflow without paying attention to
                    // the prose which described why this code is actually safe.
                    let data1 = unsafe { &mut *(data1 as *mut T1) };
            
                    let data2 = &self.filter1[filter1_index];
                    
                    self.index += 1;
                    return Some((data1, data2));
                }
                None => {},
            }
        }

        None
    }

}