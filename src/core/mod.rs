pub mod distance;
pub mod safety;

pub trait Service {
    type Input;
    type Output;

    fn calc(&self, data: Self::Input) -> anyhow::Result<Self::Output>;
}
