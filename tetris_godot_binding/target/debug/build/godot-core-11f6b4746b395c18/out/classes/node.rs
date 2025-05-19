#![doc = "Sidecar module for class [`Node`][crate::classes::Node].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node` enums](https://docs.godotengine.org/en/stable/classes/class_node.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, AsObjectArg, ClassName, CowArg, ObjectArg, ObjectCow, PtrcallSignatureTuple, RefArg, VarcallSignatureTuple
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Node.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`node`][crate::classes::node]: sidecar module with related enum/flag types\n* [`INode`][crate::classes::INode]: virtual methods\n* [`NodeNotification`][crate::classes::notify::NodeNotification]: notification type\n\n\nSee also [Godot docs for `Node`](https://docs.godotengine.org/en/stable/classes/class_node.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Node::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Node`][crate::classes::Node].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Node` methods](https://docs.godotengine.org/en/stable/classes/class_node.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode: crate::obj::GodotClass < Base = Node > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: NodeNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn get_property(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn set_property(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        fn get_property_list(&mut self) -> Vec < crate::meta::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        #[cfg(since_api = "4.2")]
        fn validate_property(&self, property: &mut crate::meta::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
    }
    #[doc = "Notification type for class [`Node`][crate::classes::Node]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum NodeNotification {
        ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for NodeNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < NodeNotification > for i32 {
        fn from(notification: NodeNotification) -> i32 {
            match notification {
                NodeNotification::ENTER_TREE => 10i32, NodeNotification::EXIT_TREE => 11i32, NodeNotification::MOVED_IN_PARENT => 12i32, NodeNotification::READY => 13i32, NodeNotification::PAUSED => 14i32, NodeNotification::UNPAUSED => 15i32, NodeNotification::PHYSICS_PROCESS => 16i32, NodeNotification::PROCESS => 17i32, NodeNotification::PARENTED => 18i32, NodeNotification::UNPARENTED => 19i32, NodeNotification::SCENE_INSTANTIATED => 20i32, NodeNotification::DRAG_BEGIN => 21i32, NodeNotification::DRAG_END => 22i32, NodeNotification::PATH_RENAMED => 23i32, NodeNotification::CHILD_ORDER_CHANGED => 24i32, NodeNotification::INTERNAL_PROCESS => 25i32, NodeNotification::INTERNAL_PHYSICS_PROCESS => 26i32, NodeNotification::POST_ENTER_TREE => 27i32, NodeNotification::DISABLED => 28i32, NodeNotification::ENABLED => 29i32, NodeNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, NodeNotification::EDITOR_PRE_SAVE => 9001i32, NodeNotification::EDITOR_POST_SAVE => 9002i32, NodeNotification::WM_MOUSE_ENTER => 1002i32, NodeNotification::WM_MOUSE_EXIT => 1003i32, NodeNotification::WM_WINDOW_FOCUS_IN => 1004i32, NodeNotification::WM_WINDOW_FOCUS_OUT => 1005i32, NodeNotification::WM_CLOSE_REQUEST => 1006i32, NodeNotification::WM_GO_BACK_REQUEST => 1007i32, NodeNotification::WM_SIZE_CHANGED => 1008i32, NodeNotification::WM_DPI_CHANGE => 1009i32, NodeNotification::VP_MOUSE_ENTER => 1010i32, NodeNotification::VP_MOUSE_EXIT => 1011i32, NodeNotification::OS_MEMORY_WARNING => 2009i32, NodeNotification::TRANSLATION_CHANGED => 2010i32, NodeNotification::WM_ABOUT => 2011i32, NodeNotification::CRASH => 2012i32, NodeNotification::OS_IME_UPDATE => 2013i32, NodeNotification::APPLICATION_RESUMED => 2014i32, NodeNotification::APPLICATION_PAUSED => 2015i32, NodeNotification::APPLICATION_FOCUS_IN => 2016i32, NodeNotification::APPLICATION_FOCUS_OUT => 2017i32, NodeNotification::TEXT_SERVER_CHANGED => 2018i32, NodeNotification::POSTINITIALIZE => 0i32, NodeNotification::PREDELETE => 1i32, NodeNotification::EXTENSION_RELOADED => 2i32, NodeNotification::Unknown(int) => int,
            }
        }
    }
    impl Node {
        pub fn print_orphan_nodes() {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "print_orphan_nodes", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn add_sibling_full(&mut self, sibling: ObjectArg < crate::classes::Node >, force_readable_name: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >, bool);
            let args = (sibling, force_readable_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "add_sibling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_sibling_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_sibling(&mut self, sibling: impl AsObjectArg < crate::classes::Node >,) {
            self.add_sibling_ex(sibling,) . done()
        }
        #[inline]
        pub fn add_sibling_ex < 'a > (&'a mut self, sibling: impl AsObjectArg < crate::classes::Node >,) -> ExAddSibling < 'a > {
            ExAddSibling::new(self, sibling,)
        }
        pub fn set_name(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_name(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_child_full(&mut self, node: ObjectArg < crate::classes::Node >, force_readable_name: bool, internal: crate::classes::node::InternalMode,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >, bool, crate::classes::node::InternalMode);
            let args = (node, force_readable_name, internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "add_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_child_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_child(&mut self, node: impl AsObjectArg < crate::classes::Node >,) {
            self.add_child_ex(node,) . done()
        }
        #[inline]
        pub fn add_child_ex < 'a > (&'a mut self, node: impl AsObjectArg < crate::classes::Node >,) -> ExAddChild < 'a > {
            ExAddChild::new(self, node,)
        }
        pub fn remove_child(&mut self, node: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "remove_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn reparent_full(&mut self, new_parent: ObjectArg < crate::classes::Node >, keep_global_transform: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >, bool);
            let args = (new_parent, keep_global_transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "reparent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::reparent_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn reparent(&mut self, new_parent: impl AsObjectArg < crate::classes::Node >,) {
            self.reparent_ex(new_parent,) . done()
        }
        #[inline]
        pub fn reparent_ex < 'a > (&'a mut self, new_parent: impl AsObjectArg < crate::classes::Node >,) -> ExReparent < 'a > {
            ExReparent::new(self, new_parent,)
        }
        pub(crate) fn get_child_count_full(&self, include_internal: bool,) -> i32 {
            type CallSig = (i32, bool);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_child_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_child_count_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_child_count(&self,) -> i32 {
            self.get_child_count_ex() . done()
        }
        #[inline]
        pub fn get_child_count_ex < 'a > (&'a self,) -> ExGetChildCount < 'a > {
            ExGetChildCount::new(self,)
        }
        pub(crate) fn get_children_full(&self, include_internal: bool,) -> Array < Gd < crate::classes::Node > > {
            type CallSig = (Array < Gd < crate::classes::Node > >, bool);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_children", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_children_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_children(&self,) -> Array < Gd < crate::classes::Node > > {
            self.get_children_ex() . done()
        }
        #[inline]
        pub fn get_children_ex < 'a > (&'a self,) -> ExGetChildren < 'a > {
            ExGetChildren::new(self,)
        }
        pub(crate) fn get_child_full(&self, idx: i32, include_internal: bool,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >, i32, bool);
            let args = (idx, include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_child_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_child(&self, idx: i32,) -> Option < Gd < crate::classes::Node > > {
            self.get_child_ex(idx,) . done()
        }
        #[inline]
        pub fn get_child_ex < 'a > (&'a self, idx: i32,) -> ExGetChild < 'a > {
            ExGetChild::new(self, idx,)
        }
        pub fn has_node(&self, path: impl AsArg < NodePath >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "has_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_or_null(&self, path: impl AsArg < NodePath >,) -> Option < Gd < crate::classes::Node > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::Node > >, CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_node_or_null", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn find_child_full(&self, pattern: CowArg < GString >, recursive: bool, owned: bool,) -> Option < Gd < crate::classes::Node > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::Node > >, CowArg < 'a0, GString >, bool, bool);
            let args = (pattern, recursive, owned,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "find_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::find_child_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn find_child(&self, pattern: impl AsArg < GString >,) -> Option < Gd < crate::classes::Node > > {
            self.find_child_ex(pattern,) . done()
        }
        #[inline]
        pub fn find_child_ex < 'a > (&'a self, pattern: impl AsArg < GString > + 'a,) -> ExFindChild < 'a > {
            ExFindChild::new(self, pattern,)
        }
        pub(crate) fn find_children_full(&self, pattern: CowArg < GString >, type_: CowArg < GString >, recursive: bool, owned: bool,) -> Array < Gd < crate::classes::Node > > {
            type CallSig < 'a0, 'a1, > = (Array < Gd < crate::classes::Node > >, CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool, bool);
            let args = (pattern, type_, recursive, owned,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "find_children", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::find_children_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn find_children(&self, pattern: impl AsArg < GString >,) -> Array < Gd < crate::classes::Node > > {
            self.find_children_ex(pattern,) . done()
        }
        #[inline]
        pub fn find_children_ex < 'a > (&'a self, pattern: impl AsArg < GString > + 'a,) -> ExFindChildren < 'a > {
            ExFindChildren::new(self, pattern,)
        }
        pub fn find_parent(&self, pattern: impl AsArg < GString >,) -> Option < Gd < crate::classes::Node > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::Node > >, CowArg < 'a0, GString >);
            let args = (pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "find_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_node_and_resource(&self, path: impl AsArg < NodePath >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "has_node_and_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_node_and_resource(&mut self, path: impl AsArg < NodePath >,) -> VariantArray {
            type CallSig < 'a0, > = (VariantArray, CowArg < 'a0, NodePath >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_node_and_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_inside_tree(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_inside_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_part_of_edited_scene(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_part_of_edited_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ancestor_of(&self, node: impl AsObjectArg < crate::classes::Node >,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::Node >);
            let args = (node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_ancestor_of", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_greater_than(&self, node: impl AsObjectArg < crate::classes::Node >,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::Node >);
            let args = (node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_greater_than", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_path_to_full(&self, node: ObjectArg < crate::classes::Node >, use_unique_path: bool,) -> NodePath {
            type CallSig = (NodePath, ObjectArg < crate::classes::Node >, bool);
            let args = (node, use_unique_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_path_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_path_to_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_path_to(&self, node: impl AsObjectArg < crate::classes::Node >,) -> NodePath {
            self.get_path_to_ex(node,) . done()
        }
        #[inline]
        pub fn get_path_to_ex < 'a > (&'a self, node: impl AsObjectArg < crate::classes::Node >,) -> ExGetPathTo < 'a > {
            ExGetPathTo::new(self, node,)
        }
        pub(crate) fn add_to_group_full(&mut self, group: CowArg < StringName >, persistent: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, bool);
            let args = (group, persistent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "add_to_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_to_group_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_to_group(&mut self, group: impl AsArg < StringName >,) {
            self.add_to_group_ex(group,) . done()
        }
        #[inline]
        pub fn add_to_group_ex < 'a > (&'a mut self, group: impl AsArg < StringName > + 'a,) -> ExAddToGroup < 'a > {
            ExAddToGroup::new(self, group,)
        }
        pub fn remove_from_group(&mut self, group: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "remove_from_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_in_group(&self, group: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_in_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_child(&mut self, child_node: impl AsObjectArg < crate::classes::Node >, to_index: i32,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >, i32);
            let args = (child_node.as_object_arg(), to_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "move_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_groups(&self,) -> Array < StringName > {
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_groups", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_owner(&mut self, owner: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (owner.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_owner(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_index_full(&self, include_internal: bool,) -> i32 {
            type CallSig = (i32, bool);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_index_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_index(&self,) -> i32 {
            self.get_index_ex() . done()
        }
        #[inline]
        pub fn get_index_ex < 'a > (&'a self,) -> ExGetIndex < 'a > {
            ExGetIndex::new(self,)
        }
        pub fn print_tree(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "print_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn print_tree_pretty(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "print_tree_pretty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tree_string(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_tree_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tree_string_pretty(&mut self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_tree_string_pretty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scene_file_path(&mut self, scene_file_path: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (scene_file_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_scene_file_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_file_path(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_scene_file_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn propagate_notification(&mut self, what: i32,) {
            type CallSig = ((), i32);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "propagate_notification", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn propagate_call_full(&mut self, method: CowArg < StringName >, args: RefArg < VariantArray >, parent_first: bool,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, VariantArray >, bool);
            let args = (method, args, parent_first,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "propagate_call", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::propagate_call_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn propagate_call(&mut self, method: impl AsArg < StringName >,) {
            self.propagate_call_ex(method,) . done()
        }
        #[inline]
        pub fn propagate_call_ex < 'a > (&'a mut self, method: impl AsArg < StringName > + 'a,) -> ExPropagateCall < 'a > {
            ExPropagateCall::new(self, method,)
        }
        pub fn set_physics_process(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_physics_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_process_delta_time(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_physics_process_delta_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_physics_processing(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_physics_processing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_delta_time(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_process_delta_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_priority(&mut self, priority: i32,) {
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_priority(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_process_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_process_priority(&mut self, priority: i32,) {
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_physics_process_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_process_priority(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_physics_process_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_processing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_input(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_input(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_processing_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_shortcut_input(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_shortcut_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_shortcut_input(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_processing_shortcut_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_unhandled_input(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5383usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_unhandled_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_unhandled_input(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_processing_unhandled_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_unhandled_key_input(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_unhandled_key_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_unhandled_key_input(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_processing_unhandled_key_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_mode(&mut self, mode: crate::classes::node::ProcessMode,) {
            type CallSig = ((), crate::classes::node::ProcessMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_mode(&self,) -> crate::classes::node::ProcessMode {
            type CallSig = (crate::classes::node::ProcessMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_process_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_process(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "can_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_thread_group(&mut self, mode: crate::classes::node::ProcessThreadGroup,) {
            type CallSig = ((), crate::classes::node::ProcessThreadGroup);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_thread_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_thread_group(&self,) -> crate::classes::node::ProcessThreadGroup {
            type CallSig = (crate::classes::node::ProcessThreadGroup,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_process_thread_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_thread_messages(&mut self, flags: crate::classes::node::ProcessThreadMessages,) {
            type CallSig = ((), crate::classes::node::ProcessThreadMessages);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_thread_messages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_thread_messages(&self,) -> crate::classes::node::ProcessThreadMessages {
            type CallSig = (crate::classes::node::ProcessThreadMessages,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_process_thread_messages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_thread_group_order(&mut self, order: i32,) {
            type CallSig = ((), i32);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_thread_group_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_thread_group_order(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_process_thread_group_order", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_display_folded(&mut self, fold: bool,) {
            type CallSig = ((), bool);
            let args = (fold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_display_folded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_displayed_folded(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_displayed_folded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_process_internal(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_process_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_processing_internal(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_processing_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_process_internal(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_physics_process_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_physics_processing_internal(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_physics_processing_internal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_interpolation_mode(&mut self, mode: crate::classes::node::PhysicsInterpolationMode,) {
            type CallSig = ((), crate::classes::node::PhysicsInterpolationMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_physics_interpolation_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_interpolation_mode(&self,) -> crate::classes::node::PhysicsInterpolationMode {
            type CallSig = (crate::classes::node::PhysicsInterpolationMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_physics_interpolation_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_physics_interpolated(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5404usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_physics_interpolated", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_physics_interpolated_and_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5405usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_physics_interpolated_and_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reset_physics_interpolation(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5406usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "reset_physics_interpolation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_translate_mode(&mut self, mode: crate::classes::node::AutoTranslateMode,) {
            type CallSig = ((), crate::classes::node::AutoTranslateMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_auto_translate_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_translate_mode(&self,) -> crate::classes::node::AutoTranslateMode {
            type CallSig = (crate::classes::node::AutoTranslateMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_auto_translate_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_window(&self,) -> Option < Gd < crate::classes::Window > > {
            type CallSig = (Option < Gd < crate::classes::Window > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_window", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_last_exclusive_window(&self,) -> Option < Gd < crate::classes::Window > > {
            type CallSig = (Option < Gd < crate::classes::Window > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_last_exclusive_window", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tree(&self,) -> Option < Gd < crate::classes::SceneTree > > {
            type CallSig = (Option < Gd < crate::classes::SceneTree > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_tween(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            type CallSig = (Option < Gd < crate::classes::Tween > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "create_tween", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn duplicate_full(&self, flags: i32,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >, i32);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "duplicate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::duplicate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn duplicate(&self,) -> Option < Gd < crate::classes::Node > > {
            self.duplicate_ex() . done()
        }
        #[inline]
        pub fn duplicate_ex < 'a > (&'a self,) -> ExDuplicate < 'a > {
            ExDuplicate::new(self,)
        }
        pub(crate) fn replace_by_full(&mut self, node: ObjectArg < crate::classes::Node >, keep_groups: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >, bool);
            let args = (node, keep_groups,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "replace_by", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::replace_by_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn replace_by(&mut self, node: impl AsObjectArg < crate::classes::Node >,) {
            self.replace_by_ex(node,) . done()
        }
        #[inline]
        pub fn replace_by_ex < 'a > (&'a mut self, node: impl AsObjectArg < crate::classes::Node >,) -> ExReplaceBy < 'a > {
            ExReplaceBy::new(self, node,)
        }
        pub fn set_scene_instance_load_placeholder(&mut self, load_placeholder: bool,) {
            type CallSig = ((), bool);
            let args = (load_placeholder,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_scene_instance_load_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scene_instance_load_placeholder(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_scene_instance_load_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editable_instance(&mut self, node: impl AsObjectArg < crate::classes::Node >, is_editable: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >, bool);
            let args = (node.as_object_arg(), is_editable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_editable_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editable_instance(&self, node: impl AsObjectArg < crate::classes::Node >,) -> bool {
            type CallSig = (bool, ObjectArg < crate::classes::Node >);
            let args = (node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_editable_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_viewport(&self,) -> Option < Gd < crate::classes::Viewport > > {
            type CallSig = (Option < Gd < crate::classes::Viewport > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5419usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_viewport", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn queue_free(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5420usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "queue_free", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn request_ready(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5421usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "request_ready", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_node_ready(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5422usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_node_ready", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_multiplayer_authority_full(&mut self, id: i32, recursive: bool,) {
            type CallSig = ((), i32, bool);
            let args = (id, recursive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5423usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_multiplayer_authority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_multiplayer_authority_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_multiplayer_authority(&mut self, id: i32,) {
            self.set_multiplayer_authority_ex(id,) . done()
        }
        #[inline]
        pub fn set_multiplayer_authority_ex < 'a > (&'a mut self, id: i32,) -> ExSetMultiplayerAuthority < 'a > {
            ExSetMultiplayerAuthority::new(self, id,)
        }
        pub fn get_multiplayer_authority(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5424usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_multiplayer_authority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_multiplayer_authority(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5425usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_multiplayer_authority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_multiplayer(&self,) -> Option < Gd < crate::classes::MultiplayerApi > > {
            type CallSig = (Option < Gd < crate::classes::MultiplayerApi > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5426usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_multiplayer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rpc_config(&mut self, method: impl AsArg < StringName >, config: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (method.into_arg(), RefArg::new(config),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5427usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "rpc_config", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editor_description(&mut self, editor_description: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (editor_description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5428usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_editor_description", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_description(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5429usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "get_editor_description", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unique_name_in_owner(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_unique_name_in_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_unique_name_in_owner(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "is_unique_name_in_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn atr_full(&self, message: CowArg < GString >, context: CowArg < StringName >,) -> GString {
            type CallSig < 'a0, 'a1, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, StringName >);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "atr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::atr_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn atr(&self, message: impl AsArg < GString >,) -> GString {
            self.atr_ex(message,) . done()
        }
        #[inline]
        pub fn atr_ex < 'a > (&'a self, message: impl AsArg < GString > + 'a,) -> ExAtr < 'a > {
            ExAtr::new(self, message,)
        }
        pub(crate) fn atr_n_full(&self, message: CowArg < GString >, plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> GString {
            type CallSig < 'a0, 'a1, 'a2, > = (GString, CowArg < 'a0, GString >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "atr_n", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::atr_n_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn atr_n(&self, message: impl AsArg < GString >, plural_message: impl AsArg < StringName >, n: i32,) -> GString {
            self.atr_n_ex(message, plural_message, n,) . done()
        }
        #[inline]
        pub fn atr_n_ex < 'a > (&'a self, message: impl AsArg < GString > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> ExAtrN < 'a > {
            ExAtrN::new(self, message, plural_message, n,)
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn rpc(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> crate::global::Error {
            Self::try_rpc(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_rpc(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < crate::global::Error, crate::meta::error::CallError > {
            type CallSig < 'a0, > = (crate::global::Error, CowArg < 'a0, StringName >);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5434usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "Node", "rpc", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn rpc_id(&mut self, peer_id: i64, method: impl AsArg < StringName >, varargs: &[Variant]) -> crate::global::Error {
            Self::try_rpc_id(self, peer_id, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_rpc_id(&mut self, peer_id: i64, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < crate::global::Error, crate::meta::error::CallError > {
            type CallSig < 'a0, > = (crate::global::Error, i64, CowArg < 'a0, StringName >);
            let args = (peer_id, method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5435usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "Node", "rpc_id", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn update_configuration_warnings(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5436usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "update_configuration_warnings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_deferred_thread_group(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call_deferred_thread_group(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_deferred_thread_group(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5437usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "Node", "call_deferred_thread_group", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn set_deferred_thread_group(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_deferred_thread_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn notify_deferred_thread_group(&mut self, what: i32,) {
            type CallSig = ((), i32);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "notify_deferred_thread_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_thread_safe(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call_thread_safe(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_thread_safe(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallSig < 'a0, > = (Variant, CowArg < 'a0, StringName >);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5440usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "Node", "call_thread_safe", self.object_ptr, self.__checked_id(), args, varargs)
            }
        }
        pub fn set_thread_safe(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Variant >);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "set_thread_safe", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn notify_thread_safe(&mut self, what: i32,) {
            type CallSig = ((), i32);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Node", "notify_thread_safe", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: NodeNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: NodeNotification) {
            self.notification(i32::from(what), true);
            
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        pub(crate) const NOTIFICATION_ENTER_TREE: i32 = 10i32;
        pub(crate) const NOTIFICATION_EXIT_TREE: i32 = 11i32;
        pub(crate) const NOTIFICATION_MOVED_IN_PARENT: i32 = 12i32;
        pub(crate) const NOTIFICATION_READY: i32 = 13i32;
        pub(crate) const NOTIFICATION_PAUSED: i32 = 14i32;
        pub(crate) const NOTIFICATION_UNPAUSED: i32 = 15i32;
        pub(crate) const NOTIFICATION_PHYSICS_PROCESS: i32 = 16i32;
        pub(crate) const NOTIFICATION_PROCESS: i32 = 17i32;
        pub(crate) const NOTIFICATION_PARENTED: i32 = 18i32;
        pub(crate) const NOTIFICATION_UNPARENTED: i32 = 19i32;
        pub(crate) const NOTIFICATION_SCENE_INSTANTIATED: i32 = 20i32;
        pub(crate) const NOTIFICATION_DRAG_BEGIN: i32 = 21i32;
        pub(crate) const NOTIFICATION_DRAG_END: i32 = 22i32;
        pub(crate) const NOTIFICATION_PATH_RENAMED: i32 = 23i32;
        pub(crate) const NOTIFICATION_CHILD_ORDER_CHANGED: i32 = 24i32;
        pub(crate) const NOTIFICATION_INTERNAL_PROCESS: i32 = 25i32;
        pub(crate) const NOTIFICATION_INTERNAL_PHYSICS_PROCESS: i32 = 26i32;
        pub(crate) const NOTIFICATION_POST_ENTER_TREE: i32 = 27i32;
        pub(crate) const NOTIFICATION_DISABLED: i32 = 28i32;
        pub(crate) const NOTIFICATION_ENABLED: i32 = 29i32;
        pub(crate) const NOTIFICATION_RESET_PHYSICS_INTERPOLATION: i32 = 2001i32;
        pub(crate) const NOTIFICATION_EDITOR_PRE_SAVE: i32 = 9001i32;
        pub(crate) const NOTIFICATION_EDITOR_POST_SAVE: i32 = 9002i32;
        pub(crate) const NOTIFICATION_WM_MOUSE_ENTER: i32 = 1002i32;
        pub(crate) const NOTIFICATION_WM_MOUSE_EXIT: i32 = 1003i32;
        pub(crate) const NOTIFICATION_WM_WINDOW_FOCUS_IN: i32 = 1004i32;
        pub(crate) const NOTIFICATION_WM_WINDOW_FOCUS_OUT: i32 = 1005i32;
        pub(crate) const NOTIFICATION_WM_CLOSE_REQUEST: i32 = 1006i32;
        pub(crate) const NOTIFICATION_WM_GO_BACK_REQUEST: i32 = 1007i32;
        pub(crate) const NOTIFICATION_WM_SIZE_CHANGED: i32 = 1008i32;
        pub(crate) const NOTIFICATION_WM_DPI_CHANGE: i32 = 1009i32;
        pub(crate) const NOTIFICATION_VP_MOUSE_ENTER: i32 = 1010i32;
        pub(crate) const NOTIFICATION_VP_MOUSE_EXIT: i32 = 1011i32;
        pub(crate) const NOTIFICATION_OS_MEMORY_WARNING: i32 = 2009i32;
        pub(crate) const NOTIFICATION_TRANSLATION_CHANGED: i32 = 2010i32;
        pub(crate) const NOTIFICATION_WM_ABOUT: i32 = 2011i32;
        pub(crate) const NOTIFICATION_CRASH: i32 = 2012i32;
        pub(crate) const NOTIFICATION_OS_IME_UPDATE: i32 = 2013i32;
        pub(crate) const NOTIFICATION_APPLICATION_RESUMED: i32 = 2014i32;
        pub(crate) const NOTIFICATION_APPLICATION_PAUSED: i32 = 2015i32;
        pub(crate) const NOTIFICATION_APPLICATION_FOCUS_IN: i32 = 2016i32;
        pub(crate) const NOTIFICATION_APPLICATION_FOCUS_OUT: i32 = 2017i32;
        pub(crate) const NOTIFICATION_TEXT_SERVER_CHANGED: i32 = 2018i32;
        
    }
    impl crate::obj::GodotClass for Node {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Node"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Node {
        
    }
    impl crate::obj::cap::GodotDefault for Node {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Node`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Node {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Node::add_sibling_ex`][super::Node::add_sibling_ex]."]
#[must_use]
pub struct ExAddSibling < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, sibling: ObjectCow < crate::classes::Node >, force_readable_name: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSibling < 'a > {
    fn new(surround_object: &'a mut re_export::Node, sibling: impl AsObjectArg < crate::classes::Node >,) -> Self {
        let force_readable_name = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, sibling: sibling.consume_arg(), force_readable_name: force_readable_name,
        }
    }
    #[inline]
    pub fn force_readable_name(self, force_readable_name: bool) -> Self {
        Self {
            force_readable_name: force_readable_name, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, sibling, force_readable_name,
        }
        = self;
        re_export::Node::add_sibling_full(surround_object, sibling.cow_as_object_arg(), force_readable_name,)
    }
}
#[doc = "Default-param extender for [`Node::add_child_ex`][super::Node::add_child_ex]."]
#[must_use]
pub struct ExAddChild < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, node: ObjectCow < crate::classes::Node >, force_readable_name: bool, internal: crate::classes::node::InternalMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddChild < 'a > {
    fn new(surround_object: &'a mut re_export::Node, node: impl AsObjectArg < crate::classes::Node >,) -> Self {
        let force_readable_name = false;
        let internal = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.consume_arg(), force_readable_name: force_readable_name, internal: internal,
        }
    }
    #[inline]
    pub fn force_readable_name(self, force_readable_name: bool) -> Self {
        Self {
            force_readable_name: force_readable_name, .. self
        }
    }
    #[inline]
    pub fn internal(self, internal: crate::classes::node::InternalMode) -> Self {
        Self {
            internal: internal, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, node, force_readable_name, internal,
        }
        = self;
        re_export::Node::add_child_full(surround_object, node.cow_as_object_arg(), force_readable_name, internal,)
    }
}
#[doc = "Default-param extender for [`Node::reparent_ex`][super::Node::reparent_ex]."]
#[must_use]
pub struct ExReparent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, new_parent: ObjectCow < crate::classes::Node >, keep_global_transform: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReparent < 'a > {
    fn new(surround_object: &'a mut re_export::Node, new_parent: impl AsObjectArg < crate::classes::Node >,) -> Self {
        let keep_global_transform = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, new_parent: new_parent.consume_arg(), keep_global_transform: keep_global_transform,
        }
    }
    #[inline]
    pub fn keep_global_transform(self, keep_global_transform: bool) -> Self {
        Self {
            keep_global_transform: keep_global_transform, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, new_parent, keep_global_transform,
        }
        = self;
        re_export::Node::reparent_full(surround_object, new_parent.cow_as_object_arg(), keep_global_transform,)
    }
}
#[doc = "Default-param extender for [`Node::get_child_count_ex`][super::Node::get_child_count_ex]."]
#[must_use]
pub struct ExGetChildCount < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetChildCount < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, include_internal,
        }
        = self;
        re_export::Node::get_child_count_full(surround_object, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::get_children_ex`][super::Node::get_children_ex]."]
#[must_use]
pub struct ExGetChildren < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetChildren < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, include_internal,
        }
        = self;
        re_export::Node::get_children_full(surround_object, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::get_child_ex`][super::Node::get_child_ex]."]
#[must_use]
pub struct ExGetChild < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, idx: i32, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetChild < 'a > {
    fn new(surround_object: &'a re_export::Node, idx: i32,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, idx, include_internal,
        }
        = self;
        re_export::Node::get_child_full(surround_object, idx, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::find_child_ex`][super::Node::find_child_ex]."]
#[must_use]
pub struct ExFindChild < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, pattern: CowArg < 'a, GString >, recursive: bool, owned: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFindChild < 'a > {
    fn new(surround_object: &'a re_export::Node, pattern: impl AsArg < GString > + 'a,) -> Self {
        let recursive = true;
        let owned = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.into_arg(), recursive: recursive, owned: owned,
        }
    }
    #[inline]
    pub fn recursive(self, recursive: bool) -> Self {
        Self {
            recursive: recursive, .. self
        }
    }
    #[inline]
    pub fn owned(self, owned: bool) -> Self {
        Self {
            owned: owned, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, pattern, recursive, owned,
        }
        = self;
        re_export::Node::find_child_full(surround_object, pattern, recursive, owned,)
    }
}
#[doc = "Default-param extender for [`Node::find_children_ex`][super::Node::find_children_ex]."]
#[must_use]
pub struct ExFindChildren < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, pattern: CowArg < 'a, GString >, type_: CowArg < 'a, GString >, recursive: bool, owned: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFindChildren < 'a > {
    fn new(surround_object: &'a re_export::Node, pattern: impl AsArg < GString > + 'a,) -> Self {
        let type_ = GString::from("");
        let recursive = true;
        let owned = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.into_arg(), type_: CowArg::Owned(type_), recursive: recursive, owned: owned,
        }
    }
    #[inline]
    pub fn type_(self, type_: impl AsArg < GString > + 'a) -> Self {
        Self {
            type_: type_.into_arg(), .. self
        }
    }
    #[inline]
    pub fn recursive(self, recursive: bool) -> Self {
        Self {
            recursive: recursive, .. self
        }
    }
    #[inline]
    pub fn owned(self, owned: bool) -> Self {
        Self {
            owned: owned, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, pattern, type_, recursive, owned,
        }
        = self;
        re_export::Node::find_children_full(surround_object, pattern, type_, recursive, owned,)
    }
}
#[doc = "Default-param extender for [`Node::get_path_to_ex`][super::Node::get_path_to_ex]."]
#[must_use]
pub struct ExGetPathTo < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, node: ObjectCow < crate::classes::Node >, use_unique_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPathTo < 'a > {
    fn new(surround_object: &'a re_export::Node, node: impl AsObjectArg < crate::classes::Node >,) -> Self {
        let use_unique_path = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.consume_arg(), use_unique_path: use_unique_path,
        }
    }
    #[inline]
    pub fn use_unique_path(self, use_unique_path: bool) -> Self {
        Self {
            use_unique_path: use_unique_path, .. self
        }
    }
    #[inline]
    pub fn done(self) -> NodePath {
        let Self {
            _phantom, surround_object, node, use_unique_path,
        }
        = self;
        re_export::Node::get_path_to_full(surround_object, node.cow_as_object_arg(), use_unique_path,)
    }
}
#[doc = "Default-param extender for [`Node::add_to_group_ex`][super::Node::add_to_group_ex]."]
#[must_use]
pub struct ExAddToGroup < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, group: CowArg < 'a, StringName >, persistent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddToGroup < 'a > {
    fn new(surround_object: &'a mut re_export::Node, group: impl AsArg < StringName > + 'a,) -> Self {
        let persistent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, group: group.into_arg(), persistent: persistent,
        }
    }
    #[inline]
    pub fn persistent(self, persistent: bool) -> Self {
        Self {
            persistent: persistent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, group, persistent,
        }
        = self;
        re_export::Node::add_to_group_full(surround_object, group, persistent,)
    }
}
#[doc = "Default-param extender for [`Node::get_index_ex`][super::Node::get_index_ex]."]
#[must_use]
pub struct ExGetIndex < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetIndex < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, include_internal,
        }
        = self;
        re_export::Node::get_index_full(surround_object, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::propagate_call_ex`][super::Node::propagate_call_ex]."]
#[must_use]
pub struct ExPropagateCall < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, method: CowArg < 'a, StringName >, args: CowArg < 'a, VariantArray >, parent_first: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPropagateCall < 'a > {
    fn new(surround_object: &'a mut re_export::Node, method: impl AsArg < StringName > + 'a,) -> Self {
        let args = Array::new();
        let parent_first = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, method: method.into_arg(), args: CowArg::Owned(args), parent_first: parent_first,
        }
    }
    #[inline]
    pub fn args(self, args: &'a VariantArray) -> Self {
        Self {
            args: CowArg::Borrowed(args), .. self
        }
    }
    #[inline]
    pub fn parent_first(self, parent_first: bool) -> Self {
        Self {
            parent_first: parent_first, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, method, args, parent_first,
        }
        = self;
        re_export::Node::propagate_call_full(surround_object, method, args.cow_as_arg(), parent_first,)
    }
}
#[doc = "Default-param extender for [`Node::duplicate_ex`][super::Node::duplicate_ex]."]
#[must_use]
pub struct ExDuplicate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDuplicate < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        let flags = 15i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: i32) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, flags,
        }
        = self;
        re_export::Node::duplicate_full(surround_object, flags,)
    }
}
#[doc = "Default-param extender for [`Node::replace_by_ex`][super::Node::replace_by_ex]."]
#[must_use]
pub struct ExReplaceBy < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, node: ObjectCow < crate::classes::Node >, keep_groups: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReplaceBy < 'a > {
    fn new(surround_object: &'a mut re_export::Node, node: impl AsObjectArg < crate::classes::Node >,) -> Self {
        let keep_groups = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.consume_arg(), keep_groups: keep_groups,
        }
    }
    #[inline]
    pub fn keep_groups(self, keep_groups: bool) -> Self {
        Self {
            keep_groups: keep_groups, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, node, keep_groups,
        }
        = self;
        re_export::Node::replace_by_full(surround_object, node.cow_as_object_arg(), keep_groups,)
    }
}
#[doc = "Default-param extender for [`Node::set_multiplayer_authority_ex`][super::Node::set_multiplayer_authority_ex]."]
#[must_use]
pub struct ExSetMultiplayerAuthority < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, id: i32, recursive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetMultiplayerAuthority < 'a > {
    fn new(surround_object: &'a mut re_export::Node, id: i32,) -> Self {
        let recursive = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, recursive: recursive,
        }
    }
    #[inline]
    pub fn recursive(self, recursive: bool) -> Self {
        Self {
            recursive: recursive, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, recursive,
        }
        = self;
        re_export::Node::set_multiplayer_authority_full(surround_object, id, recursive,)
    }
}
#[doc = "Default-param extender for [`Node::atr_ex`][super::Node::atr_ex]."]
#[must_use]
pub struct ExAtr < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, message: CowArg < 'a, GString >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAtr < 'a > {
    fn new(surround_object: &'a re_export::Node, message: impl AsArg < GString > + 'a,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, context,
        }
        = self;
        re_export::Node::atr_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`Node::atr_n_ex`][super::Node::atr_n_ex]."]
#[must_use]
pub struct ExAtrN < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, message: CowArg < 'a, GString >, plural_message: CowArg < 'a, StringName >, n: i32, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAtrN < 'a > {
    fn new(surround_object: &'a re_export::Node, message: impl AsArg < GString > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), plural_message: plural_message.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, plural_message, n, context,
        }
        = self;
        re_export::Node::atr_n_full(surround_object, message, plural_message, n, context,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessMode {
    ord: i32
}
impl ProcessMode {
    #[doc(alias = "PROCESS_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_INHERIT`"]
    pub const INHERIT: ProcessMode = ProcessMode {
        ord: 0i32
    };
    #[doc(alias = "PROCESS_MODE_PAUSABLE")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_PAUSABLE`"]
    pub const PAUSABLE: ProcessMode = ProcessMode {
        ord: 1i32
    };
    #[doc(alias = "PROCESS_MODE_WHEN_PAUSED")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_WHEN_PAUSED`"]
    pub const WHEN_PAUSED: ProcessMode = ProcessMode {
        ord: 2i32
    };
    #[doc(alias = "PROCESS_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_ALWAYS`"]
    pub const ALWAYS: ProcessMode = ProcessMode {
        ord: 3i32
    };
    #[doc(alias = "PROCESS_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_DISABLED`"]
    pub const DISABLED: ProcessMode = ProcessMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ProcessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProcessMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProcessMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERIT => "INHERIT", Self::PAUSABLE => "PAUSABLE", Self::WHEN_PAUSED => "WHEN_PAUSED", Self::ALWAYS => "ALWAYS", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERIT => "PROCESS_MODE_INHERIT", Self::PAUSABLE => "PROCESS_MODE_PAUSABLE", Self::WHEN_PAUSED => "PROCESS_MODE_WHEN_PAUSED", Self::ALWAYS => "PROCESS_MODE_ALWAYS", Self::DISABLED => "PROCESS_MODE_DISABLED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ProcessMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ProcessMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessThreadGroup {
    ord: i32
}
impl ProcessThreadGroup {
    #[doc(alias = "PROCESS_THREAD_GROUP_INHERIT")]
    #[doc = "Godot enumerator name: `PROCESS_THREAD_GROUP_INHERIT`"]
    pub const INHERIT: ProcessThreadGroup = ProcessThreadGroup {
        ord: 0i32
    };
    #[doc(alias = "PROCESS_THREAD_GROUP_MAIN_THREAD")]
    #[doc = "Godot enumerator name: `PROCESS_THREAD_GROUP_MAIN_THREAD`"]
    pub const MAIN_THREAD: ProcessThreadGroup = ProcessThreadGroup {
        ord: 1i32
    };
    #[doc(alias = "PROCESS_THREAD_GROUP_SUB_THREAD")]
    #[doc = "Godot enumerator name: `PROCESS_THREAD_GROUP_SUB_THREAD`"]
    pub const SUB_THREAD: ProcessThreadGroup = ProcessThreadGroup {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ProcessThreadGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProcessThreadGroup") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProcessThreadGroup {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERIT => "INHERIT", Self::MAIN_THREAD => "MAIN_THREAD", Self::SUB_THREAD => "SUB_THREAD", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERIT => "PROCESS_THREAD_GROUP_INHERIT", Self::MAIN_THREAD => "PROCESS_THREAD_GROUP_MAIN_THREAD", Self::SUB_THREAD => "PROCESS_THREAD_GROUP_SUB_THREAD", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ProcessThreadGroup {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ProcessThreadGroup {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessThreadGroup {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ProcessThreadMessages {
    ord: u64
}
impl ProcessThreadMessages {
    #[doc(alias = "FLAG_PROCESS_THREAD_MESSAGES")]
    #[doc = "Godot enumerator name: `FLAG_PROCESS_THREAD_MESSAGES`"]
    pub const MESSAGES: ProcessThreadMessages = ProcessThreadMessages {
        ord: 1u64
    };
    #[doc(alias = "FLAG_PROCESS_THREAD_MESSAGES_PHYSICS")]
    #[doc = "Godot enumerator name: `FLAG_PROCESS_THREAD_MESSAGES_PHYSICS`"]
    pub const MESSAGES_PHYSICS: ProcessThreadMessages = ProcessThreadMessages {
        ord: 2u64
    };
    #[doc(alias = "FLAG_PROCESS_THREAD_MESSAGES_ALL")]
    #[doc = "Godot enumerator name: `FLAG_PROCESS_THREAD_MESSAGES_ALL`"]
    pub const MESSAGES_ALL: ProcessThreadMessages = ProcessThreadMessages {
        ord: 3u64
    };
    
}
impl std::fmt::Debug for ProcessThreadMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::MESSAGES => "MESSAGES", Self::MESSAGES_PHYSICS => "MESSAGES_PHYSICS", Self::MESSAGES_ALL => "MESSAGES_ALL", _ => {
                f.debug_struct("ProcessThreadMessages") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for ProcessThreadMessages {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for ProcessThreadMessages {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for ProcessThreadMessages {
    type Via = u64;
    
}
impl crate::meta::ToGodot for ProcessThreadMessages {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessThreadMessages {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PhysicsInterpolationMode {
    ord: i32
}
impl PhysicsInterpolationMode {
    #[doc(alias = "PHYSICS_INTERPOLATION_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `PHYSICS_INTERPOLATION_MODE_INHERIT`"]
    pub const INHERIT: PhysicsInterpolationMode = PhysicsInterpolationMode {
        ord: 0i32
    };
    #[doc(alias = "PHYSICS_INTERPOLATION_MODE_ON")]
    #[doc = "Godot enumerator name: `PHYSICS_INTERPOLATION_MODE_ON`"]
    pub const ON: PhysicsInterpolationMode = PhysicsInterpolationMode {
        ord: 1i32
    };
    #[doc(alias = "PHYSICS_INTERPOLATION_MODE_OFF")]
    #[doc = "Godot enumerator name: `PHYSICS_INTERPOLATION_MODE_OFF`"]
    pub const OFF: PhysicsInterpolationMode = PhysicsInterpolationMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PhysicsInterpolationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PhysicsInterpolationMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PhysicsInterpolationMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERIT => "INHERIT", Self::ON => "ON", Self::OFF => "OFF", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERIT => "PHYSICS_INTERPOLATION_MODE_INHERIT", Self::ON => "PHYSICS_INTERPOLATION_MODE_ON", Self::OFF => "PHYSICS_INTERPOLATION_MODE_OFF", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PhysicsInterpolationMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PhysicsInterpolationMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PhysicsInterpolationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DuplicateFlags {
    ord: i32
}
impl DuplicateFlags {
    #[doc(alias = "DUPLICATE_SIGNALS")]
    #[doc = "Godot enumerator name: `DUPLICATE_SIGNALS`"]
    pub const SIGNALS: DuplicateFlags = DuplicateFlags {
        ord: 1i32
    };
    #[doc(alias = "DUPLICATE_GROUPS")]
    #[doc = "Godot enumerator name: `DUPLICATE_GROUPS`"]
    pub const GROUPS: DuplicateFlags = DuplicateFlags {
        ord: 2i32
    };
    #[doc(alias = "DUPLICATE_SCRIPTS")]
    #[doc = "Godot enumerator name: `DUPLICATE_SCRIPTS`"]
    pub const SCRIPTS: DuplicateFlags = DuplicateFlags {
        ord: 4i32
    };
    #[doc(alias = "DUPLICATE_USE_INSTANTIATION")]
    #[doc = "Godot enumerator name: `DUPLICATE_USE_INSTANTIATION`"]
    pub const USE_INSTANTIATION: DuplicateFlags = DuplicateFlags {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for DuplicateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DuplicateFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DuplicateFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SIGNALS => "SIGNALS", Self::GROUPS => "GROUPS", Self::SCRIPTS => "SCRIPTS", Self::USE_INSTANTIATION => "USE_INSTANTIATION", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SIGNALS => "DUPLICATE_SIGNALS", Self::GROUPS => "DUPLICATE_GROUPS", Self::SCRIPTS => "DUPLICATE_SCRIPTS", Self::USE_INSTANTIATION => "DUPLICATE_USE_INSTANTIATION", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DuplicateFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DuplicateFlags {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DuplicateFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InternalMode {
    ord: i32
}
impl InternalMode {
    #[doc(alias = "INTERNAL_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `INTERNAL_MODE_DISABLED`"]
    pub const DISABLED: InternalMode = InternalMode {
        ord: 0i32
    };
    #[doc(alias = "INTERNAL_MODE_FRONT")]
    #[doc = "Godot enumerator name: `INTERNAL_MODE_FRONT`"]
    pub const FRONT: InternalMode = InternalMode {
        ord: 1i32
    };
    #[doc(alias = "INTERNAL_MODE_BACK")]
    #[doc = "Godot enumerator name: `INTERNAL_MODE_BACK`"]
    pub const BACK: InternalMode = InternalMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for InternalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InternalMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InternalMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DISABLED", Self::FRONT => "FRONT", Self::BACK => "BACK", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "INTERNAL_MODE_DISABLED", Self::FRONT => "INTERNAL_MODE_FRONT", Self::BACK => "INTERNAL_MODE_BACK", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for InternalMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for InternalMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InternalMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AutoTranslateMode {
    ord: i32
}
impl AutoTranslateMode {
    #[doc(alias = "AUTO_TRANSLATE_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `AUTO_TRANSLATE_MODE_INHERIT`"]
    pub const INHERIT: AutoTranslateMode = AutoTranslateMode {
        ord: 0i32
    };
    #[doc(alias = "AUTO_TRANSLATE_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `AUTO_TRANSLATE_MODE_ALWAYS`"]
    pub const ALWAYS: AutoTranslateMode = AutoTranslateMode {
        ord: 1i32
    };
    #[doc(alias = "AUTO_TRANSLATE_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `AUTO_TRANSLATE_MODE_DISABLED`"]
    pub const DISABLED: AutoTranslateMode = AutoTranslateMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AutoTranslateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AutoTranslateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AutoTranslateMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERIT => "INHERIT", Self::ALWAYS => "ALWAYS", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERIT => "AUTO_TRANSLATE_MODE_INHERIT", Self::ALWAYS => "AUTO_TRANSLATE_MODE_ALWAYS", Self::DISABLED => "AUTO_TRANSLATE_MODE_DISABLED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for AutoTranslateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AutoTranslateMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AutoTranslateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}