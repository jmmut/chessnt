pub union ArrayUnion<Container: Copy, Element: Copy, const COUNT: usize> {
    pub named: Container,
    pub array: [Element; COUNT],
}

pub trait ArrayUnionTrait<C: Copy + ArrayUnionTrait<C, E, COUNT>, E: Copy, const COUNT: usize>:
    Sized
{
    type Delegate: ExternalArrayUnion<C, E, COUNT>;
    const NAMES: [&'static str; COUNT];

    fn myself(self) -> C;
    fn myself_mut(&mut self) -> &mut C;

    fn array(self) -> [E; COUNT] {
        Self::Delegate::array(Self::myself(self))
    }

    fn named_vec(self) -> Vec<(&'static str, E)> {
        Self::Delegate::named_vec(Self::myself(self), Self::NAMES)
    }
    fn vec(self) -> Vec<E> {
        Self::Delegate::vec(Self::myself(self))
    }
    fn set(&mut self, index: usize, element: E) {
        Self::Delegate::set(Self::myself_mut(self), index, element)
    }
    fn iter(self) -> impl Iterator<Item = E> {
        Self::Delegate::iter(Self::myself(self))
    }
    fn named_iter(self) -> impl Iterator<Item = (&'static str, E)> {
        Self::Delegate::named_iter(Self::myself(self), Self::NAMES)
    }
}

impl<Container: Copy, Element: Copy, const COUNT: usize>
    ExternalArrayUnion<Container, Element, COUNT> for ArrayUnion<Container, Element, COUNT>
{
    fn container(array: [Element; COUNT]) -> Container {
        unsafe { Self { array }.named }
    }
    fn array(container: Container) -> [Element; COUNT] {
        unsafe { Self { named: container }.array }
    }
}
pub trait ExternalArrayUnion<Container: Copy, Element: Copy, const COUNT: usize> {
    fn container(array: [Element; COUNT]) -> Container;
    fn array(container: Container) -> [Element; COUNT];

    fn named_vec(container: Container, names: [&str; COUNT]) -> Vec<(&str, Element)> {
        Self::named_iter(container, names).collect()
    }
    fn vec(container: Container) -> Vec<Element> {
        Self::array(container).to_vec()
    }
    fn set(container: &mut Container, index: usize, element: Element) {
        let mut array = Self::array(*container);
        array[index] = element;
        *container = Self::container(array);
    }
    fn iter(container: Container) -> impl Iterator<Item = Element> {
        Self::array(container).into_iter()
    }
    fn named_iter(
        container: Container,
        names: [&str; COUNT],
    ) -> impl Iterator<Item = (&str, Element)> {
        names.into_iter().zip(Self::iter(container))
    }
}
