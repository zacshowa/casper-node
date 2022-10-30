use std::time::Duration;

use crate::effect::Effects;
use crate::reactor::main_reactor::MainEvent;

pub(super) enum CatchUpInstruction {
    Do(Duration, Effects<MainEvent>),
    CheckLater(String, Duration),
    Shutdown(String),
    CaughtUp,
    CommitGenesis,
}
