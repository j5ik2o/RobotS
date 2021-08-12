use std::sync::Arc;

use actors::{Actor, Arguments};

/// Public interface of a Props.
pub trait ActorFactory: Send + Sync {
    /// Creates an Actor instance.
    fn create(&self) ->  Arc<dyn Actor>;
}

/// Props is the current only ActorFactory.
///
/// It is used to generate actors in a reliable way, calling `create` will always create the same
/// actor insatance, it is thus the way used to create actors through this crate.
pub struct Props<Args: Arguments, A: Actor> {
    creator: Arc<dyn Fn(Args) -> A + Sync + Send>,
    args: Args,
}

impl<Args: Arguments, A: Actor> Props<Args, A> {
    /// Creates a `Props` which is a factory for `A` with the `creator` function and `args` args.
    pub fn new(creator: Arc<dyn Fn(Args) -> A + Sync + Send>, args: Args) -> Arc<dyn ActorFactory> {
        Arc::new(Props::<Args, A> {
            creator: creator,
            args: args,
        })
    }
}

impl<Args: Arguments, A: Actor> ActorFactory for Props<Args, A> {
    /// Creates an Actor instance with the `creator` function and the `args` args.
    ///
    /// This is meant to allow to respawn an Actor when it fails.
    fn create(&self) -> Arc<dyn Actor> {
        // FIXME(gamazeps): reopen https://github.com/rust-lang/rust/issues/18343 with an example.
        let args = self.args.clone();
        Arc::new((self.creator)(args))
    }
}

impl<Args: Arguments, A: Actor> Clone for Props<Args, A> {
    fn clone(&self) -> Props<Args, A> {
        Props::<Args, A> {
            creator: self.creator.clone(),
            args: self.args.clone(),
        }
    }
}
