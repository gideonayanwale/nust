//! Session manager with normal and incognito mode.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionMode {
    Normal,
    Incognito,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionState {
    pub mode: SessionMode,
    pub restore_on_startup: bool,
}

#[derive(Debug)]
pub struct SessionManagerService {
    state: SessionState,
}

impl Default for SessionManagerService {
    fn default() -> Self {
        Self {
            state: SessionState {
                mode: SessionMode::Normal,
                restore_on_startup: true,
            },
        }
    }
}

impl SessionManagerService {
    pub fn set_mode(&mut self, mode: SessionMode) {
        self.state.mode = mode;
        match mode {
            SessionMode::Incognito => self.state.restore_on_startup = false,
            SessionMode::Normal => self.state.restore_on_startup = true,
        }
    }

    pub fn state(&self) -> &SessionState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::{SessionManagerService, SessionMode};

    #[test]
    fn incognito_disables_restore() {
        let mut session = SessionManagerService::default();
        session.set_mode(SessionMode::Incognito);
        assert_eq!(session.state().mode, SessionMode::Incognito);
        assert!(!session.state().restore_on_startup);
    }

    #[test]
    fn returning_to_normal_reenables_restore() {
        let mut session = SessionManagerService::default();
        session.set_mode(SessionMode::Incognito);
        session.set_mode(SessionMode::Normal);

        assert_eq!(session.state().mode, SessionMode::Normal);
        assert!(session.state().restore_on_startup);
    }
}
