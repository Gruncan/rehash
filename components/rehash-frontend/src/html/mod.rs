pub(crate) mod html_events;
pub(crate) mod html_video;
pub(crate) mod html_ui;
pub(crate) mod html_callback;

pub use crate::prelude::*;

#[macro_export]
macro_rules! get_element_as {
    ($document:expr, $id:expr, $t:ty) => {
        $document
            .get_element_by_id($id)
            .expect(&format!("Failed to get element with id '{}'", $id))
            .dyn_into::<$t>()
            .expect(&format!("Failed to cast element '{}' to {}", $id, stringify!($t)))
    };
}