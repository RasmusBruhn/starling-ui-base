/// The state for a widget describing whether it is modified, active, or alive
/// etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct State {
    /// If false, then the widget is not displayed, cannot capture events, and
    /// will not be updated when parent geometry changes
    active: bool,
    /// The currently set state for active, if enabled != active then in the
    /// next update cycle active is updated
    enabled: bool,
    /// When set to false the widget will be deleted in the next update cycle
    alive: bool,
    /// If true then something in the widget or one of its children has changed
    /// and they must be updated
    modified: bool,
}

impl State {
    /// Constructs a new initial state
    ///
    /// # Parameters
    ///
    /// active: The initial value for active
    pub(crate) fn new(active: bool) -> Self {
        return Self {
            active,
            enabled: active,
            alive: true,
            modified: false,
        };
    }
}
