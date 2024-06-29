pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// This restricts us to a Screen instance that has a list of components
// all of type Button or all of type TextField.
// If you’ll only ever have homogeneous collections, using generics and trait bounds
// is preferable because the definitions will be monomorphized at compile time
// to use the concrete types.