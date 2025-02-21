use std::{any::type_name, collections::HashMap, ops::DerefMut};

use refs::{MainLock, Weak};

use crate::{Button, Label, View};

static GLOBAL_STYLES: MainLock<HashMap<&'static str, Vec<Style>>> = MainLock::new();

static ALLOWED_TYPES: &[&str] = &[type_name::<Button>(), type_name::<Label>()];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    action: fn(&mut dyn View),
}

impl Style {
    pub const fn new(action: fn(&mut dyn View)) -> Self {
        Self { action }
    }

    pub fn apply(&self, view: &mut dyn View) {
        (self.action)(view);
    }

    pub(crate) fn apply_global<T: View>(view: Weak<T>) {
        Self::check_allowed::<T>();

        if view.base_view().ignore_global_style {
            return;
        }

        for style in Self::get_global_for::<T>() {
            style.apply(view.weak_view().deref_mut());
        }
    }

    fn get_global_for<T: View>() -> &'static [Style] {
        if let Some(styles) = GLOBAL_STYLES.get(type_name::<T>()) {
            styles
        } else {
            &[]
        }
    }

    pub fn apply_to_all<T: View>(&self) {
        Self::check_allowed::<T>();
        let styles = GLOBAL_STYLES.get_mut().entry(type_name::<T>()).or_default();

        assert!(
            !styles.contains(self),
            "{} already has this global style",
            type_name::<T>()
        );

        styles.push(*self);
    }

    fn check_allowed<T: View>() {
        assert!(
            ALLOWED_TYPES.contains(&type_name::<T>()),
            "Global style for {} is not allowed. Allowed types: {ALLOWED_TYPES:?}",
            type_name::<T>()
        );
    }
}

#[cfg(test)]
mod test {
    use refs::set_current_thread_as_main;

    use crate::{Button, Label, Style, TableView};

    const STYLE: Style = Style::new(|_v| {});
    const STYLE2: Style = Style::new(|_v| {
        dbg!("a");
    });
    const STYLE3: Style = Style::new(|_v| {
        dbg!("b");
    });

    #[test]
    fn valid_global_style_type() {
        set_current_thread_as_main();
        STYLE.apply_to_all::<Button>();
        STYLE2.apply_to_all::<Button>();
        STYLE3.apply_to_all::<Label>();

        assert_eq!(Style::get_global_for::<Button>(), &[STYLE, STYLE2]);
        assert_eq!(Style::get_global_for::<Label>(), &[STYLE3]);
    }

    #[test]
    #[should_panic]
    fn invalid_global_style_type() {
        STYLE.apply_to_all::<TableView>();
    }
}
