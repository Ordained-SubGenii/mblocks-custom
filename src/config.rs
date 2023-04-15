use crate::block::Block;
#[allow(unused_imports)]
use crate::block::BlockType::{Interval, Once, Signal};
#[allow(unused_imports)]
use crate::block::CommandType::{Function, Shell};

use crate::blocks::cpu::cpu_usage;
use crate::blocks::datetime::current_time;
//use crate::blocks::memory::memory_usage;

pub const SEPARATOR: &str = " | ";
pub const PREFIX: &str = " ";
pub const SUFFIX: &str = " ";

pub const BLOCKS: &[Block] = &[
    Block {
        kind: Interval(30),
        command: Shell(&["cat", "/sys/class/net/wlan0/operstate"]),
        prefix: "Wifi: ",
        suffix: "",
    },
    Block {
        kind: Interval(1),
        command: Function(cpu_usage),
        prefix: "CPU: ",
        suffix: "%",
    },
    //Block {
    //    kind: Interval(1),
    //    command: Function(memory_usage),
    //    prefix: "MEM: ",
    //    suffix: "",
    //},
    Block {
        kind: Signal(5),
        command: Shell(&["wpctl", "get-volume", "@DEFAULT_SINK@"]),
        prefix: "",
        suffix: "",
    },
    Block {
        kind: Interval(1800),
        command: Shell(&["date", "+%a, %b %d %Y"]),
        prefix: "",
        suffix: "",
    },
    Block {
        kind: Interval(30),
        command: Function(current_time),
        prefix: "",
        suffix: "",
    },
    Block {
        kind: Once,
        command: Shell(&["whoami"]),
        prefix: "",
        suffix: "",
    },
    //Block {
    //    kind: Once,
    //    command: Shell(&["dwm", "-v"]),
    //    prefix: "",
    //    suffix: "",
    //},
];
