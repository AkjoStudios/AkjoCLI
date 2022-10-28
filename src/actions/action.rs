use crate::action_def::Actions;

pub trait Action {
    fn on_action(&self, action: Actions) {
        match action {
            Actions::OnProject => self.on_project(),
        }
    }

    fn on_project(&self);
}