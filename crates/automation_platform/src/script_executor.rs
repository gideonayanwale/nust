#[derive(Debug, Clone)]
pub enum AutomationCommand {
    Open(String),
    Click(String),
    Type { selector: String, text: String },
    Submit(String),
    WaitFor(String),
}
