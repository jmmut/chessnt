
pub union ArrayUnion<Container: Copy, Element: Copy, const COUNT: usize> {
    pub named: Container,
    pub array: [Element; COUNT],
}

impl<Container: Copy, Element: Copy, const COUNT: usize> ArrayUnion<Container, Element, COUNT> {
    pub fn named_list(container: Container, names: [&str; COUNT]) -> Vec<(&str, Element)> {
        Self::into_named_iter(container, names).collect()
    }
    pub fn list(container: Container) -> Vec<Element> {
        Self::into_iter(container).collect()
    }
    pub fn set(container: &mut Container, index: usize, element: Element) {
        let mut array = Self::to_array(*container);
        array[index] = element;
        *container = Self::from_array(array);
    }
    pub fn from_array(array: [Element; COUNT]) -> Container {
        unsafe { Self {array}.named}
    }
    pub fn to_array(container: Container) -> [Element; COUNT] {
        unsafe { Self {named: container}.array }
    }
    pub fn into_iter(container: Container) -> impl Iterator<Item = Element> {
        Self::to_array(container).into_iter()
    }
    pub fn into_named_iter(container: Container, names: [&str; COUNT]) -> impl Iterator<Item = (&str, Element)> {
        names.into_iter().zip(Self::into_iter(container))
    }
}