pub mod distance;
pub mod safety;

pub trait Handler {
    fn process(input: String) -> anyhow::Result<()>;
}
