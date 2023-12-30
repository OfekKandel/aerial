pub trait Module {
    type Args: clap::Args;

    fn run_with_params(args: &Self::Args);
}
