use yaxi::ewmh::EwmhWindowType;
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};

const DOCK: [EwmhWindowType; 3] = [EwmhWindowType::Dock, EwmhWindowType::Toolbar, EwmhWindowType::Menu];
const FLOAT: [EwmhWindowType; 3] = [EwmhWindowType::Splash, EwmhWindowType::Utility, EwmhWindowType::Dialog];


#[derive(Debug, Clone, Copy, PartialEq, Subcommand, Serialize, Deserialize)]
pub enum State {
    Float,
    Dock,
    Tiled,
}

impl State {
    pub fn from(types: &[EwmhWindowType]) -> State {
        DOCK.iter()
            .any(|type_| types.contains(type_))
            .then(|| State::Dock)
            .or_else(|| {
                FLOAT.iter()
                    .any(|type_| types.contains(type_))
                    .then(|| State::Float)
            })
            .unwrap_or(State::Tiled)
    }

    pub fn toggle(self) -> State {
        match self {
            State::Float => State::Tiled,
            State::Tiled => State::Float,
            State::Dock => State::Dock,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Subcommand, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Subcommand, Serialize, Deserialize)]
pub enum Change {
    Add {
        #[arg(value_name = "VALUE")]
        value: i8,
    },
    Sub {
        #[arg(value_name = "VALUE")]
        value: i8,
    },
    Set {
        #[arg(value_name = "VALUE")]
        value: i8,
    },
}

#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum NodeCommand {
    Insert {
        #[command(subcommand)]
        dir: Direction,

        #[arg(short, long)]
        ratio: Option<i8>,

        #[arg(short, long)]
        toggle: bool,
    },
    State {
        #[command(subcommand)]
        state: State,

        #[arg(short, long)]
        toggle: bool,
    },
    Desktop {
        #[arg(value_name = "DESKTOP")]
        desktop: usize,
    },
    Move {
        #[arg(short, long)]
        dx: i32,

        #[arg(short, long)]
        dy: i32,
    },
    Ratio {
        #[command(subcommand)]
        change: Change,
    },

    Reverse,
    Close,
    Kill,
}

#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum DesktopCommand {
    Focus {
        #[arg(short, long)]
        desktop: usize,
    },
}

#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum ConfigCommand {
    Desktops {
        #[arg(short, long)]
        names: Vec<String>,

        #[arg(short, long)]
        pinned: bool,
    },
    Window {
        #[arg(short, long)]
        gaps: u8,
    },
    Border {
        #[arg(short, long)]
        normal: String,

        #[arg(short, long)]
        focused: String,

        #[arg(short, long)]
        width: u16,
    },
    Padding {
        #[arg(short, long)]
        top: u16,

        #[arg(short, long)]
        bottom: u16,

        #[arg(short, long)]
        left: u16,

        #[arg(short, long)]
        right: u16,
    },

    PointerFollowsFocus,
    FocusFollowsPointer,
}

#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum Command {
    #[command(subcommand)]
    Node(NodeCommand),

    #[command(subcommand)]
    Desktop(DesktopCommand),

    #[command(subcommand)]
    Config(ConfigCommand),

    Exit,
}

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}


