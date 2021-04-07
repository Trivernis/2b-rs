mod current;
mod join;
mod leave;
mod play;
mod queue;
mod shuffle;
mod skip;
mod utils;

use serenity::framework::standard::macros::group;

use current::CURRENT_COMMAND;
use join::JOIN_COMMAND;
use leave::LEAVE_COMMAND;
use play::PLAY_COMMAND;
use queue::QUEUE_COMMAND;
use shuffle::SHUFFLE_COMMAND;
use skip::SKIP_COMMAND;

#[group]
#[commands(join, leave, play, queue, skip, shuffle, current)]
#[prefix("m")]
pub struct Music;
