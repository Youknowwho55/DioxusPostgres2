// mod.rs
pub mod button;
pub mod input;
pub mod card;
pub mod inline_form;
pub mod steps;
pub mod hero;
pub mod avatar;
pub mod avatar_drop;
pub mod table;
pub mod modal;
pub mod toast;
pub mod accordion;
pub mod loading_spinner;

// Re-export from modules
pub use button::{Button, ButtonSize, ButtonScheme, ButtonType};
pub use input::{Input, InputSize, InputType, InputProps, TextInput, PasswordInput, DateInput, NumberInput,SelectInput};
pub use inline_form::{InlineForm, InlineFormProps};
pub use steps::Steps;
pub use hero::Hero;
pub use avatar::Avatar;
pub use avatar_drop::AvatarDrop;
pub use table::{Table, TableProps, TableHead, TableHeadProps, TableBody, TableBodyProps, TableRow, TableRowProps, TableCell, TableCellProps, TableHeaderCell, TableHeaderCellProps, TableFoot, TableFootProps};
pub use toast::{ToastFrame, ToastFrameProps, ToastManager, ToastInfo, ToastScheme, ToastSize, Position};
pub use loading_spinner::{
    SpinnerSize,
    SpinnerColor,
    SpinnerProps
};
// Optional prelude module for convenient imports
pub mod prelude {
    pub use super::{
        ToastFrame, 
        ToastFrameProps, 
        ToastManager,
        ToastInfo,
        ToastScheme,
        ToastSize,
        Position
    };
    
    // Convenience functions for common toast types
    pub fn simple_toast(text: &str) -> ToastInfo {
        ToastInfo::simple(text)
    }
    
    pub fn success_toast(text: &str, heading: Option<&str>) -> ToastInfo {
        ToastInfo::success(text, heading)
    }
    
    pub fn warning_toast(text: &str, heading: Option<&str>) -> ToastInfo {
        ToastInfo::warning(text, heading)
    }
    
    pub fn error_toast(text: &str, heading: Option<&str>) -> ToastInfo {
        ToastInfo::error(text, heading)
    }
    
    pub fn info_toast(text: &str, heading: Option<&str>) -> ToastInfo {
        ToastInfo::info(text, heading)
    }
}