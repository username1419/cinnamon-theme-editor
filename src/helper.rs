use std::collections::VecDeque;

use gtk::{
    Widget,
    glib::object::{Cast, CastNone},
    prelude::WidgetExt,
};

pub struct Helper {} // idk why im doing this but it feels like its right to my oop addicted brain
impl Helper {
    /// Recursively searches the widget tree for a child whose `widget_name()`
    /// matches the given `name`, using a depth-first search.
    ///
    /// This function starts at `root` and traverses its descendants
    /// in *depth-first* order, returning the first matching `Widget`.
    pub fn find_child_df(root: &Widget, name: &str) -> Option<Widget> {
        if root.widget_name() == name {
            return Some(root.clone());
        }
        let mut stack = Vec::new();

        stack.push(root.clone());
        while let Some(widget) = stack.pop() {
            let children = widget.observe_children();
            for child in children.into_iter() {
                if let Ok(inner) = child {
                    let child = inner.downcast::<Widget>();
                    if child.is_err() {
                        continue;
                    }

                    let child = child.unwrap();
                    if child.widget_name() == name {
                        return Some(child);
                    }

                    stack.push(child);
                }
            }
        }

        None
    }

    /// Recursively searches the widget tree for a child whose `widget_name()`
    /// matches the given `name`, using a breadth-first search.
    ///
    /// This function starts at `root` and traverses its descendants
    /// in *breadth-first* order, returning the first matching `Widget`.
    pub fn find_child_bf(root: &Widget, name: &str) -> Option<Widget> {
        if root.widget_name() == name {
            return Some(root.clone());
        }
        let mut queue = VecDeque::new();

        queue.push_front(root.clone());
        while let Some(widget) = queue.pop_back() {
            let children = widget.observe_children();
            for child in children.into_iter() {
                if let Ok(inner) = child {
                    let child = inner.downcast::<Widget>();
                    if child.is_err() {
                        continue;
                    }

                    let child = child.unwrap();
                    if child.widget_name() == name {
                        return Some(child);
                    }

                    queue.push_front(child);
                }
            }
        }

        None
    }

    /// Iteratively find the direct descendant of a widget whose `widget_name()` matches the
    /// provided `name`.
    pub fn find_descendant(root: &Widget, name: &str) -> Option<Widget> {
        // TODO: idk if this is gonna fix the problem with the widget navigation cuz its most likely just gonna result
        // in a tower of hell
        for child in root.observe_children().into_iter() {
            if child.is_err() {
                continue;
            }
            let child = child.ok().and_downcast::<Widget>();

            if let Some(inner) = child {
                if inner.widget_name() == name {
                    return Some(inner);
                }
            }
        }

        None
    }
}
