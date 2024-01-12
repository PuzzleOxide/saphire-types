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
// TODO: Make this type safe. (Maybe use a macro to generate the marker types?)