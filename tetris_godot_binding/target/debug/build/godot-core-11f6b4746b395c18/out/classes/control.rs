#![doc = "Sidecar module for class [`Control`][crate::classes::Control].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Control` enums](https://docs.godotengine.org/en/stable/classes/class_control.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Control.`\n\nInherits [`CanvasItem`][crate::classes::CanvasItem].\n\nRelated symbols:\n\n* [`control`][crate::classes::control]: sidecar module with related enum/flag types\n* [`IControl`][crate::classes::IControl]: virtual methods\n* [`ControlNotification`][crate::classes::notify::ControlNotification]: notification type\n\n\nSee also [Godot docs for `Control`](https://docs.godotengine.org/en/stable/classes/class_control.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Control::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Control {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Control`][crate::classes::Control].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Control` methods](https://docs.godotengine.org/en/stable/classes/class_control.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IControl: crate::obj::GodotClass < Base = Control > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    #[doc = "Notification type for class [`Control`][crate::classes::Control]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum ControlNotification {
        RESIZED = 40i32, MOUSE_ENTER = 41i32, MOUSE_EXIT = 42i32, MOUSE_ENTER_SELF = 60i32, MOUSE_EXIT_SELF = 61i32, FOCUS_ENTER = 43i32, FOCUS_EXIT = 44i32, THEME_CHANGED = 45i32, SCROLL_BEGIN = 47i32, SCROLL_END = 48i32, LAYOUT_DIRECTION_CHANGED = 49i32, TRANSFORM_CHANGED = 2000i32, LOCAL_TRANSFORM_CHANGED = 35i32, DRAW = 30i32, VISIBILITY_CHANGED = 31i32, ENTER_CANVAS = 32i32, EXIT_CANVAS = 33i32, WORLD_2D_CHANGED = 36i32, ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for ControlNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                40i32 => Self::RESIZED, 41i32 => Self::MOUSE_ENTER, 42i32 => Self::MOUSE_EXIT, 60i32 => Self::MOUSE_ENTER_SELF, 61i32 => Self::MOUSE_EXIT_SELF, 43i32 => Self::FOCUS_ENTER, 44i32 => Self::FOCUS_EXIT, 45i32 => Self::THEME_CHANGED, 47i32 => Self::SCROLL_BEGIN, 48i32 => Self::SCROLL_END, 49i32 => Self::LAYOUT_DIRECTION_CHANGED, 2000i32 => Self::TRANSFORM_CHANGED, 35i32 => Self::LOCAL_TRANSFORM_CHANGED, 30i32 => Self::DRAW, 31i32 => Self::VISIBILITY_CHANGED, 32i32 => Self::ENTER_CANVAS, 33i32 => Self::EXIT_CANVAS, 36i32 => Self::WORLD_2D_CHANGED, 10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < ControlNotification > for i32 {
        fn from(notification: ControlNotification) -> i32 {
            match notification {
                ControlNotification::RESIZED => 40i32, ControlNotification::MOUSE_ENTER => 41i32, ControlNotification::MOUSE_EXIT => 42i32, ControlNotification::MOUSE_ENTER_SELF => 60i32, ControlNotification::MOUSE_EXIT_SELF => 61i32, ControlNotification::FOCUS_ENTER => 43i32, ControlNotification::FOCUS_EXIT => 44i32, ControlNotification::THEME_CHANGED => 45i32, ControlNotification::SCROLL_BEGIN => 47i32, ControlNotification::SCROLL_END => 48i32, ControlNotification::LAYOUT_DIRECTION_CHANGED => 49i32, ControlNotification::TRANSFORM_CHANGED => 2000i32, ControlNotification::LOCAL_TRANSFORM_CHANGED => 35i32, ControlNotification::DRAW => 30i32, ControlNotification::VISIBILITY_CHANGED => 31i32, ControlNotification::ENTER_CANVAS => 32i32, ControlNotification::EXIT_CANVAS => 33i32, ControlNotification::WORLD_2D_CHANGED => 36i32, ControlNotification::ENTER_TREE => 10i32, ControlNotification::EXIT_TREE => 11i32, ControlNotification::MOVED_IN_PARENT => 12i32, ControlNotification::READY => 13i32, ControlNotification::PAUSED => 14i32, ControlNotification::UNPAUSED => 15i32, ControlNotification::PHYSICS_PROCESS => 16i32, ControlNotification::PROCESS => 17i32, ControlNotification::PARENTED => 18i32, ControlNotification::UNPARENTED => 19i32, ControlNotification::SCENE_INSTANTIATED => 20i32, ControlNotification::DRAG_BEGIN => 21i32, ControlNotification::DRAG_END => 22i32, ControlNotification::PATH_RENAMED => 23i32, ControlNotification::CHILD_ORDER_CHANGED => 24i32, ControlNotification::INTERNAL_PROCESS => 25i32, ControlNotification::INTERNAL_PHYSICS_PROCESS => 26i32, ControlNotification::POST_ENTER_TREE => 27i32, ControlNotification::DISABLED => 28i32, ControlNotification::ENABLED => 29i32, ControlNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, ControlNotification::EDITOR_PRE_SAVE => 9001i32, ControlNotification::EDITOR_POST_SAVE => 9002i32, ControlNotification::WM_MOUSE_ENTER => 1002i32, ControlNotification::WM_MOUSE_EXIT => 1003i32, ControlNotification::WM_WINDOW_FOCUS_IN => 1004i32, ControlNotification::WM_WINDOW_FOCUS_OUT => 1005i32, ControlNotification::WM_CLOSE_REQUEST => 1006i32, ControlNotification::WM_GO_BACK_REQUEST => 1007i32, ControlNotification::WM_SIZE_CHANGED => 1008i32, ControlNotification::WM_DPI_CHANGE => 1009i32, ControlNotification::VP_MOUSE_ENTER => 1010i32, ControlNotification::VP_MOUSE_EXIT => 1011i32, ControlNotification::OS_MEMORY_WARNING => 2009i32, ControlNotification::TRANSLATION_CHANGED => 2010i32, ControlNotification::WM_ABOUT => 2011i32, ControlNotification::CRASH => 2012i32, ControlNotification::OS_IME_UPDATE => 2013i32, ControlNotification::APPLICATION_RESUMED => 2014i32, ControlNotification::APPLICATION_PAUSED => 2015i32, ControlNotification::APPLICATION_FOCUS_IN => 2016i32, ControlNotification::APPLICATION_FOCUS_OUT => 2017i32, ControlNotification::TEXT_SERVER_CHANGED => 2018i32, ControlNotification::POSTINITIALIZE => 0i32, ControlNotification::PREDELETE => 1i32, ControlNotification::EXTENSION_RELOADED => 2i32, ControlNotification::Unknown(int) => int,
            }
        }
    }
    impl Control {
        pub fn accept_event(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2291usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "accept_event", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_minimum_size(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2292usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_combined_minimum_size(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2293usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_combined_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_anchors_preset_full(&mut self, preset: crate::classes::control::LayoutPreset, keep_offsets: bool,) {
            type CallSig = ((), crate::classes::control::LayoutPreset, bool);
            let args = (preset, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2294usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_anchors_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_anchors_preset_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_anchors_preset(&mut self, preset: crate::classes::control::LayoutPreset,) {
            self.set_anchors_preset_ex(preset,) . done()
        }
        #[inline]
        pub fn set_anchors_preset_ex < 'a > (&'a mut self, preset: crate::classes::control::LayoutPreset,) -> ExSetAnchorsPreset < 'a > {
            ExSetAnchorsPreset::new(self, preset,)
        }
        pub(crate) fn set_offsets_preset_full(&mut self, preset: crate::classes::control::LayoutPreset, resize_mode: crate::classes::control::LayoutPresetMode, margin: i32,) {
            type CallSig = ((), crate::classes::control::LayoutPreset, crate::classes::control::LayoutPresetMode, i32);
            let args = (preset, resize_mode, margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2295usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_offsets_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_offsets_preset_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_offsets_preset(&mut self, preset: crate::classes::control::LayoutPreset,) {
            self.set_offsets_preset_ex(preset,) . done()
        }
        #[inline]
        pub fn set_offsets_preset_ex < 'a > (&'a mut self, preset: crate::classes::control::LayoutPreset,) -> ExSetOffsetsPreset < 'a > {
            ExSetOffsetsPreset::new(self, preset,)
        }
        pub(crate) fn set_anchors_and_offsets_preset_full(&mut self, preset: crate::classes::control::LayoutPreset, resize_mode: crate::classes::control::LayoutPresetMode, margin: i32,) {
            type CallSig = ((), crate::classes::control::LayoutPreset, crate::classes::control::LayoutPresetMode, i32);
            let args = (preset, resize_mode, margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2296usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_anchors_and_offsets_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_anchors_and_offsets_preset_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_anchors_and_offsets_preset(&mut self, preset: crate::classes::control::LayoutPreset,) {
            self.set_anchors_and_offsets_preset_ex(preset,) . done()
        }
        #[inline]
        pub fn set_anchors_and_offsets_preset_ex < 'a > (&'a mut self, preset: crate::classes::control::LayoutPreset,) -> ExSetAnchorsAndOffsetsPreset < 'a > {
            ExSetAnchorsAndOffsetsPreset::new(self, preset,)
        }
        pub(crate) fn set_anchor_full(&mut self, side: crate::global::Side, anchor: f32, keep_offset: bool, push_opposite_anchor: bool,) {
            type CallSig = ((), crate::global::Side, f32, bool, bool);
            let args = (side, anchor, keep_offset, push_opposite_anchor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2297usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_anchor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_anchor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_anchor(&mut self, side: crate::global::Side, anchor: f32,) {
            self.set_anchor_ex(side, anchor,) . done()
        }
        #[inline]
        pub fn set_anchor_ex < 'a > (&'a mut self, side: crate::global::Side, anchor: f32,) -> ExSetAnchor < 'a > {
            ExSetAnchor::new(self, side, anchor,)
        }
        pub fn get_anchor(&self, side: crate::global::Side,) -> f32 {
            type CallSig = (f32, crate::global::Side);
            let args = (side,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2298usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_anchor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_offset(&mut self, side: crate::global::Side, offset: f32,) {
            type CallSig = ((), crate::global::Side, f32);
            let args = (side, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2299usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_offset(&self, offset: crate::global::Side,) -> f32 {
            type CallSig = (f32, crate::global::Side);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2300usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_anchor_and_offset_full(&mut self, side: crate::global::Side, anchor: f32, offset: f32, push_opposite_anchor: bool,) {
            type CallSig = ((), crate::global::Side, f32, f32, bool);
            let args = (side, anchor, offset, push_opposite_anchor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2301usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_anchor_and_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_anchor_and_offset_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_anchor_and_offset(&mut self, side: crate::global::Side, anchor: f32, offset: f32,) {
            self.set_anchor_and_offset_ex(side, anchor, offset,) . done()
        }
        #[inline]
        pub fn set_anchor_and_offset_ex < 'a > (&'a mut self, side: crate::global::Side, anchor: f32, offset: f32,) -> ExSetAnchorAndOffset < 'a > {
            ExSetAnchorAndOffset::new(self, side, anchor, offset,)
        }
        pub fn set_begin(&mut self, position: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2302usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_end(&mut self, position: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2303usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_position_full(&mut self, position: Vector2, keep_offsets: bool,) {
            type CallSig = ((), Vector2, bool);
            let args = (position, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2304usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_position(&mut self, position: Vector2,) {
            self.set_position_ex(position,) . done()
        }
        #[inline]
        pub fn set_position_ex < 'a > (&'a mut self, position: Vector2,) -> ExSetPosition < 'a > {
            ExSetPosition::new(self, position,)
        }
        pub(crate) fn set_size_full(&mut self, size: Vector2, keep_offsets: bool,) {
            type CallSig = ((), Vector2, bool);
            let args = (size, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2305usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_size(&mut self, size: Vector2,) {
            self.set_size_ex(size,) . done()
        }
        #[inline]
        pub fn set_size_ex < 'a > (&'a mut self, size: Vector2,) -> ExSetSize < 'a > {
            ExSetSize::new(self, size,)
        }
        pub fn reset_size(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2306usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "reset_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_minimum_size(&mut self, size: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2307usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_custom_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_global_position_full(&mut self, position: Vector2, keep_offsets: bool,) {
            type CallSig = ((), Vector2, bool);
            let args = (position, keep_offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2308usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_global_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_global_position(&mut self, position: Vector2,) {
            self.set_global_position_ex(position,) . done()
        }
        #[inline]
        pub fn set_global_position_ex < 'a > (&'a mut self, position: Vector2,) -> ExSetGlobalPosition < 'a > {
            ExSetGlobalPosition::new(self, position,)
        }
        pub fn set_rotation(&mut self, radians: f32,) {
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2309usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_degrees(&mut self, degrees: f32,) {
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2310usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2311usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pivot_offset(&mut self, pivot_offset: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (pivot_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2312usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_pivot_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_begin(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2313usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_end(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2314usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2315usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2316usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_degrees(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pivot_offset(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_pivot_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_minimum_size(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_custom_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_area_size(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_parent_area_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_position(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_screen_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rect(&self,) -> Rect2 {
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rect(&self,) -> Rect2 {
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_global_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focus_mode(&mut self, mode: crate::classes::control::FocusMode,) {
            type CallSig = ((), crate::classes::control::FocusMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_focus_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focus_mode(&self,) -> crate::classes::control::FocusMode {
            type CallSig = (crate::classes::control::FocusMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_focus_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_focus(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn grab_focus(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "grab_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn release_focus(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "release_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_prev_valid_focus(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallSig = (Option < Gd < crate::classes::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "find_prev_valid_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_next_valid_focus(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallSig = (Option < Gd < crate::classes::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "find_next_valid_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_valid_focus_neighbor(&self, side: crate::global::Side,) -> Option < Gd < crate::classes::Control > > {
            type CallSig = (Option < Gd < crate::classes::Control > >, crate::global::Side);
            let args = (side,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "find_valid_focus_neighbor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_h_size_flags(&mut self, flags: crate::classes::control::SizeFlags,) {
            type CallSig = ((), crate::classes::control::SizeFlags);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_h_size_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_size_flags(&self,) -> crate::classes::control::SizeFlags {
            type CallSig = (crate::classes::control::SizeFlags,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_h_size_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stretch_ratio(&mut self, ratio: f32,) {
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_stretch_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stretch_ratio(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2338usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_stretch_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_size_flags(&mut self, flags: crate::classes::control::SizeFlags,) {
            type CallSig = ((), crate::classes::control::SizeFlags);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_v_size_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_size_flags(&self,) -> crate::classes::control::SizeFlags {
            type CallSig = (crate::classes::control::SizeFlags,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_v_size_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_theme(&mut self, theme: impl AsObjectArg < crate::classes::Theme >,) {
            type CallSig = ((), ObjectArg < crate::classes::Theme >);
            let args = (theme.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_theme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme(&self,) -> Option < Gd < crate::classes::Theme > > {
            type CallSig = (Option < Gd < crate::classes::Theme > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_theme_type_variation(&mut self, theme_type: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_theme_type_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_type_variation(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_type_variation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn begin_bulk_theme_override(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "begin_bulk_theme_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn end_bulk_theme_override(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "end_bulk_theme_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_icon_override(&mut self, name: impl AsArg < StringName >, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, ObjectArg < crate::classes::Texture2D >);
            let args = (name.into_arg(), texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "add_theme_icon_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_stylebox_override(&mut self, name: impl AsArg < StringName >, stylebox: impl AsObjectArg < crate::classes::StyleBox >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, ObjectArg < crate::classes::StyleBox >);
            let args = (name.into_arg(), stylebox.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "add_theme_stylebox_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_font_override(&mut self, name: impl AsArg < StringName >, font: impl AsObjectArg < crate::classes::Font >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, ObjectArg < crate::classes::Font >);
            let args = (name.into_arg(), font.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "add_theme_font_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_font_size_override(&mut self, name: impl AsArg < StringName >, font_size: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, i32);
            let args = (name.into_arg(), font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "add_theme_font_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_color_override(&mut self, name: impl AsArg < StringName >, color: Color,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, Color);
            let args = (name.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "add_theme_color_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_theme_constant_override(&mut self, name: impl AsArg < StringName >, constant: i32,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >, i32);
            let args = (name.into_arg(), constant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "add_theme_constant_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_icon_override(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "remove_theme_icon_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_stylebox_override(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "remove_theme_stylebox_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_font_override(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "remove_theme_font_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_font_size_override(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "remove_theme_font_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_color_override(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "remove_theme_color_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_theme_constant_override(&mut self, name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "remove_theme_constant_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_theme_icon_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::Texture2D > >, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_icon_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_icon(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Texture2D > > {
            self.get_theme_icon_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_icon_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeIcon < 'a > {
            ExGetThemeIcon::new(self, name,)
        }
        pub(crate) fn get_theme_stylebox_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Option < Gd < crate::classes::StyleBox > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::StyleBox > >, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_stylebox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_stylebox_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_stylebox(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::StyleBox > > {
            self.get_theme_stylebox_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_stylebox_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeStylebox < 'a > {
            ExGetThemeStylebox::new(self, name,)
        }
        pub(crate) fn get_theme_font_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Option < Gd < crate::classes::Font > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::Font > >, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_font_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_font(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Font > > {
            self.get_theme_font_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_font_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeFont < 'a > {
            ExGetThemeFont::new(self, name,)
        }
        pub(crate) fn get_theme_font_size_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_font_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_font_size(&self, name: impl AsArg < StringName >,) -> i32 {
            self.get_theme_font_size_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_font_size_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeFontSize < 'a > {
            ExGetThemeFontSize::new(self, name,)
        }
        pub(crate) fn get_theme_color_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Color {
            type CallSig < 'a0, 'a1, > = (Color, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_color_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_color(&self, name: impl AsArg < StringName >,) -> Color {
            self.get_theme_color_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_color_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeColor < 'a > {
            ExGetThemeColor::new(self, name,)
        }
        pub(crate) fn get_theme_constant_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> i32 {
            type CallSig < 'a0, 'a1, > = (i32, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_constant_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_constant(&self, name: impl AsArg < StringName >,) -> i32 {
            self.get_theme_constant_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_constant_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeConstant < 'a > {
            ExGetThemeConstant::new(self, name,)
        }
        pub fn has_theme_icon_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_icon_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_stylebox_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2366usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_stylebox_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_font_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2367usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_font_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_font_size_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2368usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_font_size_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_color_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2369usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_color_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_theme_constant_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, StringName >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2370usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_constant_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn has_theme_icon_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2371usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_icon_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_icon(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_icon_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_icon_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeIcon < 'a > {
            ExHasThemeIcon::new(self, name,)
        }
        pub(crate) fn has_theme_stylebox_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2372usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_stylebox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_stylebox_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_stylebox(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_stylebox_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_stylebox_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeStylebox < 'a > {
            ExHasThemeStylebox::new(self, name,)
        }
        pub(crate) fn has_theme_font_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2373usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_font_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_font(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_font_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_font_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeFont < 'a > {
            ExHasThemeFont::new(self, name,)
        }
        pub(crate) fn has_theme_font_size_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2374usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_font_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_font_size(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_font_size_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_font_size_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeFontSize < 'a > {
            ExHasThemeFontSize::new(self, name,)
        }
        pub(crate) fn has_theme_color_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2375usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_color_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_color(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_color_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_color_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeColor < 'a > {
            ExHasThemeColor::new(self, name,)
        }
        pub(crate) fn has_theme_constant_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2376usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "has_theme_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_constant_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_constant(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_constant_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_constant_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeConstant < 'a > {
            ExHasThemeConstant::new(self, name,)
        }
        pub fn get_theme_default_base_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2377usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_default_base_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_default_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallSig = (Option < Gd < crate::classes::Font > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2378usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_default_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_theme_default_font_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2379usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_theme_default_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent_control(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallSig = (Option < Gd < crate::classes::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2380usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_parent_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_h_grow_direction(&mut self, direction: crate::classes::control::GrowDirection,) {
            type CallSig = ((), crate::classes::control::GrowDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2381usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_h_grow_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_h_grow_direction(&self,) -> crate::classes::control::GrowDirection {
            type CallSig = (crate::classes::control::GrowDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2382usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_h_grow_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_grow_direction(&mut self, direction: crate::classes::control::GrowDirection,) {
            type CallSig = ((), crate::classes::control::GrowDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2383usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_v_grow_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_v_grow_direction(&self,) -> crate::classes::control::GrowDirection {
            type CallSig = (crate::classes::control::GrowDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2384usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_v_grow_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tooltip_text(&mut self, hint: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (hint.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2385usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tooltip_text(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2386usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_tooltip_full(&self, at_position: Vector2,) -> GString {
            type CallSig = (GString, Vector2);
            let args = (at_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2387usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_tooltip", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_tooltip_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_tooltip(&self,) -> GString {
            self.get_tooltip_ex() . done()
        }
        #[inline]
        pub fn get_tooltip_ex < 'a > (&'a self,) -> ExGetTooltip < 'a > {
            ExGetTooltip::new(self,)
        }
        pub fn set_default_cursor_shape(&mut self, shape: crate::classes::control::CursorShape,) {
            type CallSig = ((), crate::classes::control::CursorShape);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2388usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_default_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_cursor_shape(&self,) -> crate::classes::control::CursorShape {
            type CallSig = (crate::classes::control::CursorShape,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2389usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_default_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_cursor_shape_full(&self, position: Vector2,) -> crate::classes::control::CursorShape {
            type CallSig = (crate::classes::control::CursorShape, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2390usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_cursor_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_cursor_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_cursor_shape(&self,) -> crate::classes::control::CursorShape {
            self.get_cursor_shape_ex() . done()
        }
        #[inline]
        pub fn get_cursor_shape_ex < 'a > (&'a self,) -> ExGetCursorShape < 'a > {
            ExGetCursorShape::new(self,)
        }
        pub fn set_focus_neighbor(&mut self, side: crate::global::Side, neighbor: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), crate::global::Side, CowArg < 'a0, NodePath >);
            let args = (side, neighbor.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_focus_neighbor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focus_neighbor(&self, side: crate::global::Side,) -> NodePath {
            type CallSig = (NodePath, crate::global::Side);
            let args = (side,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_focus_neighbor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focus_next(&mut self, next: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (next.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_focus_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focus_next(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_focus_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_focus_previous(&mut self, previous: impl AsArg < NodePath >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, NodePath >);
            let args = (previous.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_focus_previous", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_focus_previous(&self,) -> NodePath {
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_focus_previous", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_drag(&mut self, data: &Variant, preview: impl AsObjectArg < crate::classes::Control >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Variant >, ObjectArg < crate::classes::Control >);
            let args = (RefArg::new(data), preview.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "force_drag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mouse_filter(&mut self, filter: crate::classes::control::MouseFilter,) {
            type CallSig = ((), crate::classes::control::MouseFilter);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_mouse_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mouse_filter(&self,) -> crate::classes::control::MouseFilter {
            type CallSig = (crate::classes::control::MouseFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_mouse_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_force_pass_scroll_events(&mut self, force_pass_scroll_events: bool,) {
            type CallSig = ((), bool);
            let args = (force_pass_scroll_events,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_force_pass_scroll_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_force_pass_scroll_events(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "is_force_pass_scroll_events", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_clip_contents(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_clip_contents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_clipping_contents(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "is_clipping_contents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn grab_click_focus(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2404usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "grab_click_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_forwarding(&mut self, drag_func: &Callable, can_drop_func: &Callable, drop_func: &Callable,) {
            type CallSig < 'a0, 'a1, 'a2, > = ((), RefArg < 'a0, Callable >, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >);
            let args = (RefArg::new(drag_func), RefArg::new(can_drop_func), RefArg::new(drop_func),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2405usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_drag_forwarding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drag_preview(&mut self, control: impl AsObjectArg < crate::classes::Control >,) {
            type CallSig = ((), ObjectArg < crate::classes::Control >);
            let args = (control.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2406usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_drag_preview", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drag_successful(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "is_drag_successful", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "warp_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shortcut_context(&mut self, node: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_shortcut_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shortcut_context(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_shortcut_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_minimum_size(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "update_minimum_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_layout_direction(&mut self, direction: crate::classes::control::LayoutDirection,) {
            type CallSig = ((), crate::classes::control::LayoutDirection);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_layout_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_layout_direction(&self,) -> crate::classes::control::LayoutDirection {
            type CallSig = (crate::classes::control::LayoutDirection,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "get_layout_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_layout_rtl(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "is_layout_rtl", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_translate(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_auto_translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_auto_translating(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "is_auto_translating", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_localize_numeral_system(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2417usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "set_localize_numeral_system", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_localizing_numeral_system(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2418usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Control", "is_localizing_numeral_system", self.object_ptr, self.__checked_id(), args,)
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
        pub fn notify(&mut self, what: ControlNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ControlNotification) {
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
        pub(crate) const NOTIFICATION_RESIZED: i32 = 40i32;
        pub(crate) const NOTIFICATION_MOUSE_ENTER: i32 = 41i32;
        pub(crate) const NOTIFICATION_MOUSE_EXIT: i32 = 42i32;
        pub(crate) const NOTIFICATION_MOUSE_ENTER_SELF: i32 = 60i32;
        pub(crate) const NOTIFICATION_MOUSE_EXIT_SELF: i32 = 61i32;
        pub(crate) const NOTIFICATION_FOCUS_ENTER: i32 = 43i32;
        pub(crate) const NOTIFICATION_FOCUS_EXIT: i32 = 44i32;
        pub(crate) const NOTIFICATION_THEME_CHANGED: i32 = 45i32;
        pub(crate) const NOTIFICATION_SCROLL_BEGIN: i32 = 47i32;
        pub(crate) const NOTIFICATION_SCROLL_END: i32 = 48i32;
        pub(crate) const NOTIFICATION_LAYOUT_DIRECTION_CHANGED: i32 = 49i32;
        
    }
    impl crate::obj::GodotClass for Control {
        type Base = crate::classes::CanvasItem;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Control"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Control {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Control {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Control {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Control {
        
    }
    impl crate::obj::cap::GodotDefault for Control {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Control {
        type Target = crate::classes::CanvasItem;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Control {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Control`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Control {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Control::set_anchors_preset_ex`][super::Control::set_anchors_preset_ex]."]
#[must_use]
pub struct ExSetAnchorsPreset < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Control, preset: crate::classes::control::LayoutPreset, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetAnchorsPreset < 'a > {
    fn new(surround_object: &'a mut re_export::Control, preset: crate::classes::control::LayoutPreset,) -> Self {
        let keep_offsets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset, keep_offsets: keep_offsets,
        }
    }
    #[inline]
    pub fn keep_offsets(self, keep_offsets: bool) -> Self {
        Self {
            keep_offsets: keep_offsets, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, preset, keep_offsets,
        }
        = self;
        re_export::Control::set_anchors_preset_full(surround_object, preset, keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::set_offsets_preset_ex`][super::Control::set_offsets_preset_ex]."]
#[must_use]
pub struct ExSetOffsetsPreset < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Control, preset: crate::classes::control::LayoutPreset, resize_mode: crate::classes::control::LayoutPresetMode, margin: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetOffsetsPreset < 'a > {
    fn new(surround_object: &'a mut re_export::Control, preset: crate::classes::control::LayoutPreset,) -> Self {
        let resize_mode = crate::obj::EngineEnum::from_ord(0);
        let margin = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset, resize_mode: resize_mode, margin: margin,
        }
    }
    #[inline]
    pub fn resize_mode(self, resize_mode: crate::classes::control::LayoutPresetMode) -> Self {
        Self {
            resize_mode: resize_mode, .. self
        }
    }
    #[inline]
    pub fn margin(self, margin: i32) -> Self {
        Self {
            margin: margin, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, preset, resize_mode, margin,
        }
        = self;
        re_export::Control::set_offsets_preset_full(surround_object, preset, resize_mode, margin,)
    }
}
#[doc = "Default-param extender for [`Control::set_anchors_and_offsets_preset_ex`][super::Control::set_anchors_and_offsets_preset_ex]."]
#[must_use]
pub struct ExSetAnchorsAndOffsetsPreset < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Control, preset: crate::classes::control::LayoutPreset, resize_mode: crate::classes::control::LayoutPresetMode, margin: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetAnchorsAndOffsetsPreset < 'a > {
    fn new(surround_object: &'a mut re_export::Control, preset: crate::classes::control::LayoutPreset,) -> Self {
        let resize_mode = crate::obj::EngineEnum::from_ord(0);
        let margin = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset, resize_mode: resize_mode, margin: margin,
        }
    }
    #[inline]
    pub fn resize_mode(self, resize_mode: crate::classes::control::LayoutPresetMode) -> Self {
        Self {
            resize_mode: resize_mode, .. self
        }
    }
    #[inline]
    pub fn margin(self, margin: i32) -> Self {
        Self {
            margin: margin, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, preset, resize_mode, margin,
        }
        = self;
        re_export::Control::set_anchors_and_offsets_preset_full(surround_object, preset, resize_mode, margin,)
    }
}
#[doc = "Default-param extender for [`Control::set_anchor_ex`][super::Control::set_anchor_ex]."]
#[must_use]
pub struct ExSetAnchor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Control, side: crate::global::Side, anchor: f32, keep_offset: bool, push_opposite_anchor: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetAnchor < 'a > {
    fn new(surround_object: &'a mut re_export::Control, side: crate::global::Side, anchor: f32,) -> Self {
        let keep_offset = false;
        let push_opposite_anchor = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, side: side, anchor: anchor, keep_offset: keep_offset, push_opposite_anchor: push_opposite_anchor,
        }
    }
    #[inline]
    pub fn keep_offset(self, keep_offset: bool) -> Self {
        Self {
            keep_offset: keep_offset, .. self
        }
    }
    #[inline]
    pub fn push_opposite_anchor(self, push_opposite_anchor: bool) -> Self {
        Self {
            push_opposite_anchor: push_opposite_anchor, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, side, anchor, keep_offset, push_opposite_anchor,
        }
        = self;
        re_export::Control::set_anchor_full(surround_object, side, anchor, keep_offset, push_opposite_anchor,)
    }
}
#[doc = "Default-param extender for [`Control::set_anchor_and_offset_ex`][super::Control::set_anchor_and_offset_ex]."]
#[must_use]
pub struct ExSetAnchorAndOffset < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Control, side: crate::global::Side, anchor: f32, offset: f32, push_opposite_anchor: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetAnchorAndOffset < 'a > {
    fn new(surround_object: &'a mut re_export::Control, side: crate::global::Side, anchor: f32, offset: f32,) -> Self {
        let push_opposite_anchor = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, side: side, anchor: anchor, offset: offset, push_opposite_anchor: push_opposite_anchor,
        }
    }
    #[inline]
    pub fn push_opposite_anchor(self, push_opposite_anchor: bool) -> Self {
        Self {
            push_opposite_anchor: push_opposite_anchor, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, side, anchor, offset, push_opposite_anchor,
        }
        = self;
        re_export::Control::set_anchor_and_offset_full(surround_object, side, anchor, offset, push_opposite_anchor,)
    }
}
#[doc = "Default-param extender for [`Control::set_position_ex`][super::Control::set_position_ex]."]
#[must_use]
pub struct ExSetPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Control, position: Vector2, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetPosition < 'a > {
    fn new(surround_object: &'a mut re_export::Control, position: Vector2,) -> Self {
        let keep_offsets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, keep_offsets: keep_offsets,
        }
    }
    #[inline]
    pub fn keep_offsets(self, keep_offsets: bool) -> Self {
        Self {
            keep_offsets: keep_offsets, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, keep_offsets,
        }
        = self;
        re_export::Control::set_position_full(surround_object, position, keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::set_size_ex`][super::Control::set_size_ex]."]
#[must_use]
pub struct ExSetSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Control, size: Vector2, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetSize < 'a > {
    fn new(surround_object: &'a mut re_export::Control, size: Vector2,) -> Self {
        let keep_offsets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size: size, keep_offsets: keep_offsets,
        }
    }
    #[inline]
    pub fn keep_offsets(self, keep_offsets: bool) -> Self {
        Self {
            keep_offsets: keep_offsets, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, size, keep_offsets,
        }
        = self;
        re_export::Control::set_size_full(surround_object, size, keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::set_global_position_ex`][super::Control::set_global_position_ex]."]
#[must_use]
pub struct ExSetGlobalPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Control, position: Vector2, keep_offsets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetGlobalPosition < 'a > {
    fn new(surround_object: &'a mut re_export::Control, position: Vector2,) -> Self {
        let keep_offsets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, keep_offsets: keep_offsets,
        }
    }
    #[inline]
    pub fn keep_offsets(self, keep_offsets: bool) -> Self {
        Self {
            keep_offsets: keep_offsets, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, keep_offsets,
        }
        = self;
        re_export::Control::set_global_position_full(surround_object, position, keep_offsets,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_icon_ex`][super::Control::get_theme_icon_ex]."]
#[must_use]
pub struct ExGetThemeIcon < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeIcon < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Texture2D > > {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_icon_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_stylebox_ex`][super::Control::get_theme_stylebox_ex]."]
#[must_use]
pub struct ExGetThemeStylebox < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeStylebox < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::StyleBox > > {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_stylebox_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_font_ex`][super::Control::get_theme_font_ex]."]
#[must_use]
pub struct ExGetThemeFont < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeFont < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Font > > {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_font_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_font_size_ex`][super::Control::get_theme_font_size_ex]."]
#[must_use]
pub struct ExGetThemeFontSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeFontSize < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_font_size_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_color_ex`][super::Control::get_theme_color_ex]."]
#[must_use]
pub struct ExGetThemeColor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeColor < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Color {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_color_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_theme_constant_ex`][super::Control::get_theme_constant_ex]."]
#[must_use]
pub struct ExGetThemeConstant < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeConstant < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::get_theme_constant_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_icon_ex`][super::Control::has_theme_icon_ex]."]
#[must_use]
pub struct ExHasThemeIcon < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeIcon < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_icon_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_stylebox_ex`][super::Control::has_theme_stylebox_ex]."]
#[must_use]
pub struct ExHasThemeStylebox < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeStylebox < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_stylebox_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_font_ex`][super::Control::has_theme_font_ex]."]
#[must_use]
pub struct ExHasThemeFont < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeFont < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_font_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_font_size_ex`][super::Control::has_theme_font_size_ex]."]
#[must_use]
pub struct ExHasThemeFontSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeFontSize < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_font_size_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_color_ex`][super::Control::has_theme_color_ex]."]
#[must_use]
pub struct ExHasThemeColor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeColor < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_color_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::has_theme_constant_ex`][super::Control::has_theme_constant_ex]."]
#[must_use]
pub struct ExHasThemeConstant < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeConstant < 'a > {
    fn new(surround_object: &'a re_export::Control, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Control::has_theme_constant_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Control::get_tooltip_ex`][super::Control::get_tooltip_ex]."]
#[must_use]
pub struct ExGetTooltip < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, at_position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTooltip < 'a > {
    fn new(surround_object: &'a re_export::Control,) -> Self {
        let at_position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, at_position: at_position,
        }
    }
    #[inline]
    pub fn at_position(self, at_position: Vector2) -> Self {
        Self {
            at_position: at_position, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, at_position,
        }
        = self;
        re_export::Control::get_tooltip_full(surround_object, at_position,)
    }
}
#[doc = "Default-param extender for [`Control::get_cursor_shape_ex`][super::Control::get_cursor_shape_ex]."]
#[must_use]
pub struct ExGetCursorShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Control, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCursorShape < 'a > {
    fn new(surround_object: &'a re_export::Control,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::control::CursorShape {
        let Self {
            _phantom, surround_object, position,
        }
        = self;
        re_export::Control::get_cursor_shape_full(surround_object, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FocusMode {
    ord: i32
}
impl FocusMode {
    #[doc(alias = "FOCUS_NONE")]
    #[doc = "Godot enumerator name: `FOCUS_NONE`"]
    pub const NONE: FocusMode = FocusMode {
        ord: 0i32
    };
    #[doc(alias = "FOCUS_CLICK")]
    #[doc = "Godot enumerator name: `FOCUS_CLICK`"]
    pub const CLICK: FocusMode = FocusMode {
        ord: 1i32
    };
    #[doc(alias = "FOCUS_ALL")]
    #[doc = "Godot enumerator name: `FOCUS_ALL`"]
    pub const ALL: FocusMode = FocusMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FocusMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FocusMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FocusMode {
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
            Self::NONE => "NONE", Self::CLICK => "CLICK", Self::ALL => "ALL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "FOCUS_NONE", Self::CLICK => "FOCUS_CLICK", Self::ALL => "FOCUS_ALL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FocusMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FocusMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FocusMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CursorShape {
    ord: i32
}
impl CursorShape {
    #[doc(alias = "CURSOR_ARROW")]
    #[doc = "Godot enumerator name: `CURSOR_ARROW`"]
    pub const ARROW: CursorShape = CursorShape {
        ord: 0i32
    };
    #[doc(alias = "CURSOR_IBEAM")]
    #[doc = "Godot enumerator name: `CURSOR_IBEAM`"]
    pub const IBEAM: CursorShape = CursorShape {
        ord: 1i32
    };
    #[doc(alias = "CURSOR_POINTING_HAND")]
    #[doc = "Godot enumerator name: `CURSOR_POINTING_HAND`"]
    pub const POINTING_HAND: CursorShape = CursorShape {
        ord: 2i32
    };
    #[doc(alias = "CURSOR_CROSS")]
    #[doc = "Godot enumerator name: `CURSOR_CROSS`"]
    pub const CROSS: CursorShape = CursorShape {
        ord: 3i32
    };
    #[doc(alias = "CURSOR_WAIT")]
    #[doc = "Godot enumerator name: `CURSOR_WAIT`"]
    pub const WAIT: CursorShape = CursorShape {
        ord: 4i32
    };
    #[doc(alias = "CURSOR_BUSY")]
    #[doc = "Godot enumerator name: `CURSOR_BUSY`"]
    pub const BUSY: CursorShape = CursorShape {
        ord: 5i32
    };
    #[doc(alias = "CURSOR_DRAG")]
    #[doc = "Godot enumerator name: `CURSOR_DRAG`"]
    pub const DRAG: CursorShape = CursorShape {
        ord: 6i32
    };
    #[doc(alias = "CURSOR_CAN_DROP")]
    #[doc = "Godot enumerator name: `CURSOR_CAN_DROP`"]
    pub const CAN_DROP: CursorShape = CursorShape {
        ord: 7i32
    };
    #[doc(alias = "CURSOR_FORBIDDEN")]
    #[doc = "Godot enumerator name: `CURSOR_FORBIDDEN`"]
    pub const FORBIDDEN: CursorShape = CursorShape {
        ord: 8i32
    };
    #[doc(alias = "CURSOR_VSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_VSIZE`"]
    pub const VSIZE: CursorShape = CursorShape {
        ord: 9i32
    };
    #[doc(alias = "CURSOR_HSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_HSIZE`"]
    pub const HSIZE: CursorShape = CursorShape {
        ord: 10i32
    };
    #[doc(alias = "CURSOR_BDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_BDIAGSIZE`"]
    pub const BDIAGSIZE: CursorShape = CursorShape {
        ord: 11i32
    };
    #[doc(alias = "CURSOR_FDIAGSIZE")]
    #[doc = "Godot enumerator name: `CURSOR_FDIAGSIZE`"]
    pub const FDIAGSIZE: CursorShape = CursorShape {
        ord: 12i32
    };
    #[doc(alias = "CURSOR_MOVE")]
    #[doc = "Godot enumerator name: `CURSOR_MOVE`"]
    pub const MOVE: CursorShape = CursorShape {
        ord: 13i32
    };
    #[doc(alias = "CURSOR_VSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_VSPLIT`"]
    pub const VSPLIT: CursorShape = CursorShape {
        ord: 14i32
    };
    #[doc(alias = "CURSOR_HSPLIT")]
    #[doc = "Godot enumerator name: `CURSOR_HSPLIT`"]
    pub const HSPLIT: CursorShape = CursorShape {
        ord: 15i32
    };
    #[doc(alias = "CURSOR_HELP")]
    #[doc = "Godot enumerator name: `CURSOR_HELP`"]
    pub const HELP: CursorShape = CursorShape {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for CursorShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CursorShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CursorShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 => Some(Self {
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
            Self::ARROW => "ARROW", Self::IBEAM => "IBEAM", Self::POINTING_HAND => "POINTING_HAND", Self::CROSS => "CROSS", Self::WAIT => "WAIT", Self::BUSY => "BUSY", Self::DRAG => "DRAG", Self::CAN_DROP => "CAN_DROP", Self::FORBIDDEN => "FORBIDDEN", Self::VSIZE => "VSIZE", Self::HSIZE => "HSIZE", Self::BDIAGSIZE => "BDIAGSIZE", Self::FDIAGSIZE => "FDIAGSIZE", Self::MOVE => "MOVE", Self::VSPLIT => "VSPLIT", Self::HSPLIT => "HSPLIT", Self::HELP => "HELP", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ARROW => "CURSOR_ARROW", Self::IBEAM => "CURSOR_IBEAM", Self::POINTING_HAND => "CURSOR_POINTING_HAND", Self::CROSS => "CURSOR_CROSS", Self::WAIT => "CURSOR_WAIT", Self::BUSY => "CURSOR_BUSY", Self::DRAG => "CURSOR_DRAG", Self::CAN_DROP => "CURSOR_CAN_DROP", Self::FORBIDDEN => "CURSOR_FORBIDDEN", Self::VSIZE => "CURSOR_VSIZE", Self::HSIZE => "CURSOR_HSIZE", Self::BDIAGSIZE => "CURSOR_BDIAGSIZE", Self::FDIAGSIZE => "CURSOR_FDIAGSIZE", Self::MOVE => "CURSOR_MOVE", Self::VSPLIT => "CURSOR_VSPLIT", Self::HSPLIT => "CURSOR_HSPLIT", Self::HELP => "CURSOR_HELP", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CursorShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CursorShape {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CursorShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LayoutPreset {
    ord: i32
}
impl LayoutPreset {
    #[doc(alias = "PRESET_TOP_LEFT")]
    #[doc = "Godot enumerator name: `PRESET_TOP_LEFT`"]
    pub const TOP_LEFT: LayoutPreset = LayoutPreset {
        ord: 0i32
    };
    #[doc(alias = "PRESET_TOP_RIGHT")]
    #[doc = "Godot enumerator name: `PRESET_TOP_RIGHT`"]
    pub const TOP_RIGHT: LayoutPreset = LayoutPreset {
        ord: 1i32
    };
    #[doc(alias = "PRESET_BOTTOM_LEFT")]
    #[doc = "Godot enumerator name: `PRESET_BOTTOM_LEFT`"]
    pub const BOTTOM_LEFT: LayoutPreset = LayoutPreset {
        ord: 2i32
    };
    #[doc(alias = "PRESET_BOTTOM_RIGHT")]
    #[doc = "Godot enumerator name: `PRESET_BOTTOM_RIGHT`"]
    pub const BOTTOM_RIGHT: LayoutPreset = LayoutPreset {
        ord: 3i32
    };
    #[doc(alias = "PRESET_CENTER_LEFT")]
    #[doc = "Godot enumerator name: `PRESET_CENTER_LEFT`"]
    pub const CENTER_LEFT: LayoutPreset = LayoutPreset {
        ord: 4i32
    };
    #[doc(alias = "PRESET_CENTER_TOP")]
    #[doc = "Godot enumerator name: `PRESET_CENTER_TOP`"]
    pub const CENTER_TOP: LayoutPreset = LayoutPreset {
        ord: 5i32
    };
    #[doc(alias = "PRESET_CENTER_RIGHT")]
    #[doc = "Godot enumerator name: `PRESET_CENTER_RIGHT`"]
    pub const CENTER_RIGHT: LayoutPreset = LayoutPreset {
        ord: 6i32
    };
    #[doc(alias = "PRESET_CENTER_BOTTOM")]
    #[doc = "Godot enumerator name: `PRESET_CENTER_BOTTOM`"]
    pub const CENTER_BOTTOM: LayoutPreset = LayoutPreset {
        ord: 7i32
    };
    #[doc(alias = "PRESET_CENTER")]
    #[doc = "Godot enumerator name: `PRESET_CENTER`"]
    pub const CENTER: LayoutPreset = LayoutPreset {
        ord: 8i32
    };
    #[doc(alias = "PRESET_LEFT_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_LEFT_WIDE`"]
    pub const LEFT_WIDE: LayoutPreset = LayoutPreset {
        ord: 9i32
    };
    #[doc(alias = "PRESET_TOP_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_TOP_WIDE`"]
    pub const TOP_WIDE: LayoutPreset = LayoutPreset {
        ord: 10i32
    };
    #[doc(alias = "PRESET_RIGHT_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_RIGHT_WIDE`"]
    pub const RIGHT_WIDE: LayoutPreset = LayoutPreset {
        ord: 11i32
    };
    #[doc(alias = "PRESET_BOTTOM_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_BOTTOM_WIDE`"]
    pub const BOTTOM_WIDE: LayoutPreset = LayoutPreset {
        ord: 12i32
    };
    #[doc(alias = "PRESET_VCENTER_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_VCENTER_WIDE`"]
    pub const VCENTER_WIDE: LayoutPreset = LayoutPreset {
        ord: 13i32
    };
    #[doc(alias = "PRESET_HCENTER_WIDE")]
    #[doc = "Godot enumerator name: `PRESET_HCENTER_WIDE`"]
    pub const HCENTER_WIDE: LayoutPreset = LayoutPreset {
        ord: 14i32
    };
    #[doc(alias = "PRESET_FULL_RECT")]
    #[doc = "Godot enumerator name: `PRESET_FULL_RECT`"]
    pub const FULL_RECT: LayoutPreset = LayoutPreset {
        ord: 15i32
    };
    
}
impl std::fmt::Debug for LayoutPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LayoutPreset") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LayoutPreset {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 => Some(Self {
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
            Self::TOP_LEFT => "TOP_LEFT", Self::TOP_RIGHT => "TOP_RIGHT", Self::BOTTOM_LEFT => "BOTTOM_LEFT", Self::BOTTOM_RIGHT => "BOTTOM_RIGHT", Self::CENTER_LEFT => "CENTER_LEFT", Self::CENTER_TOP => "CENTER_TOP", Self::CENTER_RIGHT => "CENTER_RIGHT", Self::CENTER_BOTTOM => "CENTER_BOTTOM", Self::CENTER => "CENTER", Self::LEFT_WIDE => "LEFT_WIDE", Self::TOP_WIDE => "TOP_WIDE", Self::RIGHT_WIDE => "RIGHT_WIDE", Self::BOTTOM_WIDE => "BOTTOM_WIDE", Self::VCENTER_WIDE => "VCENTER_WIDE", Self::HCENTER_WIDE => "HCENTER_WIDE", Self::FULL_RECT => "FULL_RECT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TOP_LEFT => "PRESET_TOP_LEFT", Self::TOP_RIGHT => "PRESET_TOP_RIGHT", Self::BOTTOM_LEFT => "PRESET_BOTTOM_LEFT", Self::BOTTOM_RIGHT => "PRESET_BOTTOM_RIGHT", Self::CENTER_LEFT => "PRESET_CENTER_LEFT", Self::CENTER_TOP => "PRESET_CENTER_TOP", Self::CENTER_RIGHT => "PRESET_CENTER_RIGHT", Self::CENTER_BOTTOM => "PRESET_CENTER_BOTTOM", Self::CENTER => "PRESET_CENTER", Self::LEFT_WIDE => "PRESET_LEFT_WIDE", Self::TOP_WIDE => "PRESET_TOP_WIDE", Self::RIGHT_WIDE => "PRESET_RIGHT_WIDE", Self::BOTTOM_WIDE => "PRESET_BOTTOM_WIDE", Self::VCENTER_WIDE => "PRESET_VCENTER_WIDE", Self::HCENTER_WIDE => "PRESET_HCENTER_WIDE", Self::FULL_RECT => "PRESET_FULL_RECT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for LayoutPreset {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LayoutPreset {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LayoutPreset {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LayoutPresetMode {
    ord: i32
}
impl LayoutPresetMode {
    #[doc(alias = "PRESET_MODE_MINSIZE")]
    #[doc = "Godot enumerator name: `PRESET_MODE_MINSIZE`"]
    pub const MINSIZE: LayoutPresetMode = LayoutPresetMode {
        ord: 0i32
    };
    #[doc(alias = "PRESET_MODE_KEEP_WIDTH")]
    #[doc = "Godot enumerator name: `PRESET_MODE_KEEP_WIDTH`"]
    pub const KEEP_WIDTH: LayoutPresetMode = LayoutPresetMode {
        ord: 1i32
    };
    #[doc(alias = "PRESET_MODE_KEEP_HEIGHT")]
    #[doc = "Godot enumerator name: `PRESET_MODE_KEEP_HEIGHT`"]
    pub const KEEP_HEIGHT: LayoutPresetMode = LayoutPresetMode {
        ord: 2i32
    };
    #[doc(alias = "PRESET_MODE_KEEP_SIZE")]
    #[doc = "Godot enumerator name: `PRESET_MODE_KEEP_SIZE`"]
    pub const KEEP_SIZE: LayoutPresetMode = LayoutPresetMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for LayoutPresetMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LayoutPresetMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LayoutPresetMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::MINSIZE => "MINSIZE", Self::KEEP_WIDTH => "KEEP_WIDTH", Self::KEEP_HEIGHT => "KEEP_HEIGHT", Self::KEEP_SIZE => "KEEP_SIZE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::MINSIZE => "PRESET_MODE_MINSIZE", Self::KEEP_WIDTH => "PRESET_MODE_KEEP_WIDTH", Self::KEEP_HEIGHT => "PRESET_MODE_KEEP_HEIGHT", Self::KEEP_SIZE => "PRESET_MODE_KEEP_SIZE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for LayoutPresetMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LayoutPresetMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LayoutPresetMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct SizeFlags {
    ord: u64
}
impl SizeFlags {
    #[doc(alias = "SIZE_SHRINK_BEGIN")]
    #[doc = "Godot enumerator name: `SIZE_SHRINK_BEGIN`"]
    pub const SHRINK_BEGIN: SizeFlags = SizeFlags {
        ord: 0u64
    };
    #[doc(alias = "SIZE_FILL")]
    #[doc = "Godot enumerator name: `SIZE_FILL`"]
    pub const FILL: SizeFlags = SizeFlags {
        ord: 1u64
    };
    #[doc(alias = "SIZE_EXPAND")]
    #[doc = "Godot enumerator name: `SIZE_EXPAND`"]
    pub const EXPAND: SizeFlags = SizeFlags {
        ord: 2u64
    };
    #[doc(alias = "SIZE_EXPAND_FILL")]
    #[doc = "Godot enumerator name: `SIZE_EXPAND_FILL`"]
    pub const EXPAND_FILL: SizeFlags = SizeFlags {
        ord: 3u64
    };
    #[doc(alias = "SIZE_SHRINK_CENTER")]
    #[doc = "Godot enumerator name: `SIZE_SHRINK_CENTER`"]
    pub const SHRINK_CENTER: SizeFlags = SizeFlags {
        ord: 4u64
    };
    #[doc(alias = "SIZE_SHRINK_END")]
    #[doc = "Godot enumerator name: `SIZE_SHRINK_END`"]
    pub const SHRINK_END: SizeFlags = SizeFlags {
        ord: 8u64
    };
    
}
impl std::fmt::Debug for SizeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::SHRINK_BEGIN => "SHRINK_BEGIN", Self::FILL => "FILL", Self::EXPAND => "EXPAND", Self::EXPAND_FILL => "EXPAND_FILL", Self::SHRINK_CENTER => "SHRINK_CENTER", Self::SHRINK_END => "SHRINK_END", _ => {
                f.debug_struct("SizeFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for SizeFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for SizeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for SizeFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for SizeFlags {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SizeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MouseFilter {
    ord: i32
}
impl MouseFilter {
    #[doc(alias = "MOUSE_FILTER_STOP")]
    #[doc = "Godot enumerator name: `MOUSE_FILTER_STOP`"]
    pub const STOP: MouseFilter = MouseFilter {
        ord: 0i32
    };
    #[doc(alias = "MOUSE_FILTER_PASS")]
    #[doc = "Godot enumerator name: `MOUSE_FILTER_PASS`"]
    pub const PASS: MouseFilter = MouseFilter {
        ord: 1i32
    };
    #[doc(alias = "MOUSE_FILTER_IGNORE")]
    #[doc = "Godot enumerator name: `MOUSE_FILTER_IGNORE`"]
    pub const IGNORE: MouseFilter = MouseFilter {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MouseFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MouseFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MouseFilter {
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
            Self::STOP => "STOP", Self::PASS => "PASS", Self::IGNORE => "IGNORE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STOP => "MOUSE_FILTER_STOP", Self::PASS => "MOUSE_FILTER_PASS", Self::IGNORE => "MOUSE_FILTER_IGNORE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for MouseFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MouseFilter {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MouseFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GrowDirection {
    ord: i32
}
impl GrowDirection {
    #[doc(alias = "GROW_DIRECTION_BEGIN")]
    #[doc = "Godot enumerator name: `GROW_DIRECTION_BEGIN`"]
    pub const BEGIN: GrowDirection = GrowDirection {
        ord: 0i32
    };
    #[doc(alias = "GROW_DIRECTION_END")]
    #[doc = "Godot enumerator name: `GROW_DIRECTION_END`"]
    pub const END: GrowDirection = GrowDirection {
        ord: 1i32
    };
    #[doc(alias = "GROW_DIRECTION_BOTH")]
    #[doc = "Godot enumerator name: `GROW_DIRECTION_BOTH`"]
    pub const BOTH: GrowDirection = GrowDirection {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for GrowDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GrowDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GrowDirection {
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
            Self::BEGIN => "BEGIN", Self::END => "END", Self::BOTH => "BOTH", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BEGIN => "GROW_DIRECTION_BEGIN", Self::END => "GROW_DIRECTION_END", Self::BOTH => "GROW_DIRECTION_BOTH", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for GrowDirection {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GrowDirection {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GrowDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Anchor {
    ord: i32
}
impl Anchor {
    #[doc(alias = "ANCHOR_BEGIN")]
    #[doc = "Godot enumerator name: `ANCHOR_BEGIN`"]
    pub const BEGIN: Anchor = Anchor {
        ord: 0i32
    };
    #[doc(alias = "ANCHOR_END")]
    #[doc = "Godot enumerator name: `ANCHOR_END`"]
    pub const END: Anchor = Anchor {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Anchor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Anchor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::BEGIN => "BEGIN", Self::END => "END", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BEGIN => "ANCHOR_BEGIN", Self::END => "ANCHOR_END", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Anchor {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Anchor {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Anchor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LayoutDirection {
    ord: i32
}
impl LayoutDirection {
    #[doc(alias = "LAYOUT_DIRECTION_INHERITED")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_INHERITED`"]
    pub const INHERITED: LayoutDirection = LayoutDirection {
        ord: 0i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_LOCALE")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_LOCALE`"]
    pub const LOCALE: LayoutDirection = LayoutDirection {
        ord: 1i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_LTR")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_LTR`"]
    pub const LTR: LayoutDirection = LayoutDirection {
        ord: 2i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_RTL")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_RTL`"]
    pub const RTL: LayoutDirection = LayoutDirection {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for LayoutDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LayoutDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LayoutDirection {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::INHERITED => "INHERITED", Self::LOCALE => "LOCALE", Self::LTR => "LTR", Self::RTL => "RTL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERITED => "LAYOUT_DIRECTION_INHERITED", Self::LOCALE => "LAYOUT_DIRECTION_LOCALE", Self::LTR => "LAYOUT_DIRECTION_LTR", Self::RTL => "LAYOUT_DIRECTION_RTL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for LayoutDirection {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LayoutDirection {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LayoutDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextDirection {
    ord: i32
}
impl TextDirection {
    #[doc(alias = "TEXT_DIRECTION_INHERITED")]
    #[doc = "Godot enumerator name: `TEXT_DIRECTION_INHERITED`"]
    pub const INHERITED: TextDirection = TextDirection {
        ord: 3i32
    };
    #[doc(alias = "TEXT_DIRECTION_AUTO")]
    #[doc = "Godot enumerator name: `TEXT_DIRECTION_AUTO`"]
    pub const AUTO: TextDirection = TextDirection {
        ord: 0i32
    };
    #[doc(alias = "TEXT_DIRECTION_LTR")]
    #[doc = "Godot enumerator name: `TEXT_DIRECTION_LTR`"]
    pub const LTR: TextDirection = TextDirection {
        ord: 1i32
    };
    #[doc(alias = "TEXT_DIRECTION_RTL")]
    #[doc = "Godot enumerator name: `TEXT_DIRECTION_RTL`"]
    pub const RTL: TextDirection = TextDirection {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TextDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextDirection {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::INHERITED => "INHERITED", Self::AUTO => "AUTO", Self::LTR => "LTR", Self::RTL => "RTL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INHERITED => "TEXT_DIRECTION_INHERITED", Self::AUTO => "TEXT_DIRECTION_AUTO", Self::LTR => "TEXT_DIRECTION_LTR", Self::RTL => "TEXT_DIRECTION_RTL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TextDirection {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextDirection {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}