use godot::{meta::ArrayElement, prelude::*};

pub trait ToGodotArray<T: ArrayElement> {
    fn to_godot_array(&self) -> Array<T>;
}

impl<T, S> ToGodotArray<T> for S
where
    T: ArrayElement,
    S: IntoIterator<Item = T> + Clone,
{
    fn to_godot_array(&self) -> Array<T> {
        self.clone().into_iter().collect()
    }
}
