pub enum Reduced1<T, U> {
    Zero,
    One(T),
    Many(U),
}

pub trait TryReduce
where
    Self: Sized,
{
    type Item;

    fn try_reduce1(self) -> Reduced1<Self::Item, Self>;
}

impl<T> TryReduce for Vec<T> {
    type Item = T;

    #[inline]
    fn try_reduce1(mut self) -> Reduced1<Self::Item, Self> {
        if self.len() > 1 {
            Reduced1::Many(self)
        } else if let Some(t) = self.pop() {
            Reduced1::One(t)
        } else {
            Reduced1::Zero
        }
    }
}
