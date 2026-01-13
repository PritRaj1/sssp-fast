use nalgebra::RealField;

pub trait FloatNumber:
    Copy
    + RealField
    + std::fmt::Debug
    + Send
    + Sync
{
}

impl<T> FloatNumber for T where T: 
    Copy
    + RealField
    + std::fmt::Debug
    + Send
    + Sync
{
}
