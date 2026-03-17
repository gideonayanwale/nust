use crate::script_executor::AutomationCommand;

pub trait AutomationRuntime {
    fn open(&self, url: &str);
    fn click(&self, selector: &str);
    fn r#type(&self, selector: &str, text: &str);
    fn submit(&self, selector: &str);
    fn wait_for(&self, selector: &str);
    fn run_script(&self, script: &[AutomationCommand]);
}
