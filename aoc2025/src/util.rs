pub trait Half<T> {
    fn half(self) -> T;
}

impl<T> Half<T> for T
where
    T: Sized + Copy + std::ops::Div<Output = T> + From<u8>,
{
    #[inline]
    fn half(self) -> T {
        self / T::from(2u8)
    }
}

pub trait Divmod<T> {
    fn div_mod(self, rhs: T) -> (T, T);
}

impl<T> Divmod<T> for T
where
    T: Sized + Copy + std::ops::Rem<Output = T> + std::ops::Div<Output = T>,
{
    #[inline]
    fn div_mod(self, rhs: Self) -> (T, T) {
        (self / rhs, self % rhs)
    }
}
