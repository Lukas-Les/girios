pub trait Command {
    fn try_from(value: &str) -> Self;
}

