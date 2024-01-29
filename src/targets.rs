use std::fmt::Display;

/// Represents a player target. See variant documentation for more information.
pub enum PlayerTarget {
    /// The player who triggered the event.
    Default,
    /// The Killer in an event.
    Killer,
    /// The Damager in an event.
    Damager,
    /// The Victim in an event.
    Victim,
    /// All Selected Players.
    Selection,
    /// All Players
    All,
}

impl Display for PlayerTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerTarget::Default => write!(f, "Default"),
            PlayerTarget::Killer => write!(f, "Killer"),
            PlayerTarget::Damager => write!(f, "Damager"),
            PlayerTarget::Victim => write!(f, "Victim"),
            PlayerTarget::Selection => write!(f, "Selection"),
            PlayerTarget::All => write!(f, "All"),
        }
    }
}

pub enum EntityTarget {
    /// The entity who triggered the event.
    Default,
    /// The Killer in an event.
    Killer,
    /// The Damager in an event.
    Damager,
    /// The Victim in an event.
    Victim,
    /// All Selected Entities.
    Selected,
    /// All Entities
    All,
    /// The last entity that was spawned.
    Last,
}

impl Display for EntityTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityTarget::Default => write!(f, "Default"),
            EntityTarget::Killer => write!(f, "Killer"),
            EntityTarget::Damager => write!(f, "Damager"),
            EntityTarget::Victim => write!(f, "Victim"),
            EntityTarget::Selected => write!(f, "Selected"),
            EntityTarget::All => write!(f, "All"),
            EntityTarget::Last => write!(f, "Last"),
        }
    }
}
// TODO: Make this type safe. (Maybe use a macro to generate the marker types?)