#![doc = "Sidecar module for class [`EditorInterface`][crate::classes::EditorInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorInterface` enums](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorInterface.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`editor_interface`][crate::classes::editor_interface]: sidecar module with related enum/flag types\n* [`IEditorInterface`][crate::classes::IEditorInterface]: virtual methods\n\n\nSee also [Godot docs for `EditorInterface`](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`EditorInterface::singleton()`][EditorInterface::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorInterface`][crate::classes::EditorInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorInterface` methods](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorInterface: crate::obj::GodotClass < Base = EditorInterface > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ObjectNotification) {
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
    }
    impl EditorInterface {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"EditorInterface");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn restart_editor_full(&mut self, save: bool,) {
            type CallSig = ((), bool);
            let args = (save,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "restart_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::restart_editor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn restart_editor(&mut self,) {
            self.restart_editor_ex() . done()
        }
        #[inline]
        pub fn restart_editor_ex < 'a > (&'a mut self,) -> ExRestartEditor < 'a > {
            ExRestartEditor::new(self,)
        }
        pub fn get_command_palette(&self,) -> Option < Gd < crate::classes::EditorCommandPalette > > {
            type CallSig = (Option < Gd < crate::classes::EditorCommandPalette > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_command_palette", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resource_filesystem(&self,) -> Option < Gd < crate::classes::EditorFileSystem > > {
            type CallSig = (Option < Gd < crate::classes::EditorFileSystem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_resource_filesystem", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_paths(&self,) -> Option < Gd < crate::classes::EditorPaths > > {
            type CallSig = (Option < Gd < crate::classes::EditorPaths > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_paths", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resource_previewer(&self,) -> Option < Gd < crate::classes::EditorResourcePreview > > {
            type CallSig = (Option < Gd < crate::classes::EditorResourcePreview > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_resource_previewer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selection(&self,) -> Option < Gd < crate::classes::EditorSelection > > {
            type CallSig = (Option < Gd < crate::classes::EditorSelection > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_settings(&self,) -> Option < Gd < crate::classes::EditorSettings > > {
            type CallSig = (Option < Gd < crate::classes::EditorSettings > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_settings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_mesh_previews(&mut self, meshes: &Array < Gd < crate::classes::Mesh > >, preview_size: i32,) -> Array < Gd < crate::classes::Texture2D > > {
            type CallSig < 'a0, > = (Array < Gd < crate::classes::Texture2D > >, RefArg < 'a0, Array < Gd < crate::classes::Mesh > > >, i32);
            let args = (RefArg::new(meshes), preview_size,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "make_mesh_previews", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_plugin_enabled(&mut self, plugin: impl AsArg < GString >, enabled: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (plugin.into_arg(), enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "set_plugin_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_plugin_enabled(&self, plugin: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "is_plugin_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_theme(&self,) -> Option < Gd < crate::classes::Theme > > {
            type CallSig = (Option < Gd < crate::classes::Theme > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_theme", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_base_control(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallSig = (Option < Gd < crate::classes::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_base_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_main_screen(&self,) -> Option < Gd < crate::classes::VBoxContainer > > {
            type CallSig = (Option < Gd < crate::classes::VBoxContainer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_main_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_editor(&self,) -> Option < Gd < crate::classes::ScriptEditor > > {
            type CallSig = (Option < Gd < crate::classes::ScriptEditor > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_script_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_viewport_2d(&self,) -> Option < Gd < crate::classes::SubViewport > > {
            type CallSig = (Option < Gd < crate::classes::SubViewport > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_viewport_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_editor_viewport_3d_full(&self, idx: i32,) -> Option < Gd < crate::classes::SubViewport > > {
            type CallSig = (Option < Gd < crate::classes::SubViewport > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_viewport_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_editor_viewport_3d_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_editor_viewport_3d(&self,) -> Option < Gd < crate::classes::SubViewport > > {
            self.get_editor_viewport_3d_ex() . done()
        }
        #[inline]
        pub fn get_editor_viewport_3d_ex < 'a > (&'a self,) -> ExGetEditorViewport3d < 'a > {
            ExGetEditorViewport3d::new(self,)
        }
        pub fn set_main_screen_editor(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "set_main_screen_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_distraction_free_mode(&mut self, enter: bool,) {
            type CallSig = ((), bool);
            let args = (enter,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "set_distraction_free_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_distraction_free_mode_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "is_distraction_free_mode_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_multi_window_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "is_multi_window_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_editor_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(120usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn popup_dialog_full(&mut self, dialog: ObjectArg < crate::classes::Window >, rect: Rect2i,) {
            type CallSig = ((), ObjectArg < crate::classes::Window >, Rect2i);
            let args = (dialog, rect,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(121usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_dialog_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_dialog(&mut self, dialog: impl AsObjectArg < crate::classes::Window >,) {
            self.popup_dialog_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_ex < 'a > (&'a mut self, dialog: impl AsObjectArg < crate::classes::Window >,) -> ExPopupDialog < 'a > {
            ExPopupDialog::new(self, dialog,)
        }
        pub(crate) fn popup_dialog_centered_full(&mut self, dialog: ObjectArg < crate::classes::Window >, minsize: Vector2i,) {
            type CallSig = ((), ObjectArg < crate::classes::Window >, Vector2i);
            let args = (dialog, minsize,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(122usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog_centered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_dialog_centered_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_dialog_centered(&mut self, dialog: impl AsObjectArg < crate::classes::Window >,) {
            self.popup_dialog_centered_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_centered_ex < 'a > (&'a mut self, dialog: impl AsObjectArg < crate::classes::Window >,) -> ExPopupDialogCentered < 'a > {
            ExPopupDialogCentered::new(self, dialog,)
        }
        pub(crate) fn popup_dialog_centered_ratio_full(&mut self, dialog: ObjectArg < crate::classes::Window >, ratio: f32,) {
            type CallSig = ((), ObjectArg < crate::classes::Window >, f32);
            let args = (dialog, ratio,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(123usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog_centered_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_dialog_centered_ratio_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_dialog_centered_ratio(&mut self, dialog: impl AsObjectArg < crate::classes::Window >,) {
            self.popup_dialog_centered_ratio_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_centered_ratio_ex < 'a > (&'a mut self, dialog: impl AsObjectArg < crate::classes::Window >,) -> ExPopupDialogCenteredRatio < 'a > {
            ExPopupDialogCenteredRatio::new(self, dialog,)
        }
        pub(crate) fn popup_dialog_centered_clamped_full(&mut self, dialog: ObjectArg < crate::classes::Window >, minsize: Vector2i, fallback_ratio: f32,) {
            type CallSig = ((), ObjectArg < crate::classes::Window >, Vector2i, f32);
            let args = (dialog, minsize, fallback_ratio,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(124usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog_centered_clamped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_dialog_centered_clamped_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_dialog_centered_clamped(&mut self, dialog: impl AsObjectArg < crate::classes::Window >,) {
            self.popup_dialog_centered_clamped_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_centered_clamped_ex < 'a > (&'a mut self, dialog: impl AsObjectArg < crate::classes::Window >,) -> ExPopupDialogCenteredClamped < 'a > {
            ExPopupDialogCenteredClamped::new(self, dialog,)
        }
        pub fn get_current_feature_profile(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(125usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_current_feature_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_feature_profile(&mut self, profile_name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (profile_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(126usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "set_current_feature_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn popup_node_selector_full(&mut self, callback: RefArg < Callable >, valid_types: RefArg < Array < StringName > >,) {
            type CallSig < 'a0, 'a1, > = ((), RefArg < 'a0, Callable >, RefArg < 'a1, Array < StringName > >);
            let args = (callback, valid_types,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(127usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_node_selector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_node_selector_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_node_selector(&mut self, callback: &Callable,) {
            self.popup_node_selector_ex(callback,) . done()
        }
        #[inline]
        pub fn popup_node_selector_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExPopupNodeSelector < 'a > {
            ExPopupNodeSelector::new(self, callback,)
        }
        pub(crate) fn popup_property_selector_full(&mut self, object: ObjectArg < crate::classes::Object >, callback: RefArg < Callable >, type_filter: RefArg < PackedInt32Array >,) {
            type CallSig < 'a0, 'a1, > = ((), ObjectArg < crate::classes::Object >, RefArg < 'a0, Callable >, RefArg < 'a1, PackedInt32Array >);
            let args = (object, callback, type_filter,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(128usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_property_selector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_property_selector_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_property_selector(&mut self, object: impl AsObjectArg < crate::classes::Object >, callback: &Callable,) {
            self.popup_property_selector_ex(object, callback,) . done()
        }
        #[inline]
        pub fn popup_property_selector_ex < 'a > (&'a mut self, object: impl AsObjectArg < crate::classes::Object >, callback: &'a Callable,) -> ExPopupPropertySelector < 'a > {
            ExPopupPropertySelector::new(self, object, callback,)
        }
        pub fn get_file_system_dock(&self,) -> Option < Gd < crate::classes::FileSystemDock > > {
            type CallSig = (Option < Gd < crate::classes::FileSystemDock > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(129usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_file_system_dock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select_file(&mut self, file: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(130usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "select_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_paths(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(131usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_selected_paths", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_path(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(132usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_current_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_directory(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(133usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_current_directory", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inspector(&self,) -> Option < Gd < crate::classes::EditorInspector > > {
            type CallSig = (Option < Gd < crate::classes::EditorInspector > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(134usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_inspector", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn inspect_object_full(&mut self, object: ObjectArg < crate::classes::Object >, for_property: CowArg < GString >, inspector_only: bool,) {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::Object >, CowArg < 'a0, GString >, bool);
            let args = (object, for_property, inspector_only,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(135usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "inspect_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::inspect_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn inspect_object(&mut self, object: impl AsObjectArg < crate::classes::Object >,) {
            self.inspect_object_ex(object,) . done()
        }
        #[inline]
        pub fn inspect_object_ex < 'a > (&'a mut self, object: impl AsObjectArg < crate::classes::Object >,) -> ExInspectObject < 'a > {
            ExInspectObject::new(self, object,)
        }
        pub fn edit_resource(&mut self, resource: impl AsObjectArg < crate::classes::Resource >,) {
            type CallSig = ((), ObjectArg < crate::classes::Resource >);
            let args = (resource.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(136usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "edit_resource", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn edit_node(&mut self, node: impl AsObjectArg < crate::classes::Node >,) {
            type CallSig = ((), ObjectArg < crate::classes::Node >);
            let args = (node.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(137usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "edit_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn edit_script_full(&mut self, script: ObjectArg < crate::classes::Script >, line: i32, column: i32, grab_focus: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::Script >, i32, i32, bool);
            let args = (script, line, column, grab_focus,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(138usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "edit_script", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::edit_script_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn edit_script(&mut self, script: impl AsObjectArg < crate::classes::Script >,) {
            self.edit_script_ex(script,) . done()
        }
        #[inline]
        pub fn edit_script_ex < 'a > (&'a mut self, script: impl AsObjectArg < crate::classes::Script >,) -> ExEditScript < 'a > {
            ExEditScript::new(self, script,)
        }
        pub fn open_scene_from_path(&mut self, scene_filepath: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (scene_filepath.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(139usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "open_scene_from_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reload_scene_from_path(&mut self, scene_filepath: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (scene_filepath.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(140usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "reload_scene_from_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_open_scenes(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_open_scenes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_scene_root(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallSig = (Option < Gd < crate::classes::Node > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_edited_scene_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_scene(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "save_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn save_scene_as_full(&mut self, path: CowArg < GString >, with_preview: bool,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, bool);
            let args = (path, with_preview,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "save_scene_as", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_scene_as_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_scene_as(&mut self, path: impl AsArg < GString >,) {
            self.save_scene_as_ex(path,) . done()
        }
        #[inline]
        pub fn save_scene_as_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExSaveSceneAs < 'a > {
            ExSaveSceneAs::new(self, path,)
        }
        pub fn save_all_scenes(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "save_all_scenes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn mark_scene_as_unsaved(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "mark_scene_as_unsaved", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn play_main_scene(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "play_main_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn play_current_scene(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "play_current_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn play_custom_scene(&mut self, scene_filepath: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (scene_filepath.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "play_custom_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop_playing_scene(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "stop_playing_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_playing_scene(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "is_playing_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playing_scene(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "get_playing_scene", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_movie_maker_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "set_movie_maker_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_movie_maker_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorInterface", "is_movie_maker_enabled", self.object_ptr, self.__checked_id(), args,)
            }
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
    }
    impl crate::obj::GodotClass for EditorInterface {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"EditorInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorInterface {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorInterface {
        
    }
    impl std::ops::Deref for EditorInterface {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorInterface`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_EditorInterface {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorInterface > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorInterface::restart_editor_ex`][super::EditorInterface::restart_editor_ex]."]
#[must_use]
pub struct ExRestartEditor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, save: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRestartEditor < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface,) -> Self {
        let save = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, save: save,
        }
    }
    #[inline]
    pub fn save(self, save: bool) -> Self {
        Self {
            save: save, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, save,
        }
        = self;
        re_export::EditorInterface::restart_editor_full(surround_object, save,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::get_editor_viewport_3d_ex`][super::EditorInterface::get_editor_viewport_3d_ex]."]
#[must_use]
pub struct ExGetEditorViewport3d < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::EditorInterface, idx: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetEditorViewport3d < 'a > {
    fn new(surround_object: &'a re_export::EditorInterface,) -> Self {
        let idx = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx,
        }
    }
    #[inline]
    pub fn idx(self, idx: i32) -> Self {
        Self {
            idx: idx, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::SubViewport > > {
        let Self {
            _phantom, surround_object, idx,
        }
        = self;
        re_export::EditorInterface::get_editor_viewport_3d_full(surround_object, idx,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_ex`][super::EditorInterface::popup_dialog_ex]."]
#[must_use]
pub struct ExPopupDialog < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, dialog: ObjectCow < crate::classes::Window >, rect: Rect2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialog < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: impl AsObjectArg < crate::classes::Window >,) -> Self {
        let rect = Rect2i::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.consume_arg(), rect: rect,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2i) -> Self {
        Self {
            rect: rect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, rect,
        }
        = self;
        re_export::EditorInterface::popup_dialog_full(surround_object, dialog.cow_as_object_arg(), rect,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_ex`][super::EditorInterface::popup_dialog_centered_ex]."]
#[must_use]
pub struct ExPopupDialogCentered < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, dialog: ObjectCow < crate::classes::Window >, minsize: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialogCentered < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: impl AsObjectArg < crate::classes::Window >,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.consume_arg(), minsize: minsize,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, minsize,
        }
        = self;
        re_export::EditorInterface::popup_dialog_centered_full(surround_object, dialog.cow_as_object_arg(), minsize,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_ratio_ex`][super::EditorInterface::popup_dialog_centered_ratio_ex]."]
#[must_use]
pub struct ExPopupDialogCenteredRatio < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, dialog: ObjectCow < crate::classes::Window >, ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialogCenteredRatio < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: impl AsObjectArg < crate::classes::Window >,) -> Self {
        let ratio = 0.8f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.consume_arg(), ratio: ratio,
        }
    }
    #[inline]
    pub fn ratio(self, ratio: f32) -> Self {
        Self {
            ratio: ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, ratio,
        }
        = self;
        re_export::EditorInterface::popup_dialog_centered_ratio_full(surround_object, dialog.cow_as_object_arg(), ratio,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_clamped_ex`][super::EditorInterface::popup_dialog_centered_clamped_ex]."]
#[must_use]
pub struct ExPopupDialogCenteredClamped < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, dialog: ObjectCow < crate::classes::Window >, minsize: Vector2i, fallback_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialogCenteredClamped < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: impl AsObjectArg < crate::classes::Window >,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        let fallback_ratio = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.consume_arg(), minsize: minsize, fallback_ratio: fallback_ratio,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn fallback_ratio(self, fallback_ratio: f32) -> Self {
        Self {
            fallback_ratio: fallback_ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, minsize, fallback_ratio,
        }
        = self;
        re_export::EditorInterface::popup_dialog_centered_clamped_full(surround_object, dialog.cow_as_object_arg(), minsize, fallback_ratio,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_node_selector_ex`][super::EditorInterface::popup_node_selector_ex]."]
#[must_use]
pub struct ExPopupNodeSelector < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, callback: CowArg < 'a, Callable >, valid_types: CowArg < 'a, Array < StringName > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupNodeSelector < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, callback: &'a Callable,) -> Self {
        let valid_types = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), valid_types: CowArg::Owned(valid_types),
        }
    }
    #[inline]
    pub fn valid_types(self, valid_types: &'a Array < StringName >) -> Self {
        Self {
            valid_types: CowArg::Borrowed(valid_types), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, valid_types,
        }
        = self;
        re_export::EditorInterface::popup_node_selector_full(surround_object, callback.cow_as_arg(), valid_types.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_property_selector_ex`][super::EditorInterface::popup_property_selector_ex]."]
#[must_use]
pub struct ExPopupPropertySelector < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, object: ObjectCow < crate::classes::Object >, callback: CowArg < 'a, Callable >, type_filter: CowArg < 'a, PackedInt32Array >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupPropertySelector < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, object: impl AsObjectArg < crate::classes::Object >, callback: &'a Callable,) -> Self {
        let type_filter = PackedInt32Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, object: object.consume_arg(), callback: CowArg::Borrowed(callback), type_filter: CowArg::Owned(type_filter),
        }
    }
    #[inline]
    pub fn type_filter(self, type_filter: &'a PackedInt32Array) -> Self {
        Self {
            type_filter: CowArg::Borrowed(type_filter), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, object, callback, type_filter,
        }
        = self;
        re_export::EditorInterface::popup_property_selector_full(surround_object, object.cow_as_object_arg(), callback.cow_as_arg(), type_filter.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorInterface::inspect_object_ex`][super::EditorInterface::inspect_object_ex]."]
#[must_use]
pub struct ExInspectObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, object: ObjectCow < crate::classes::Object >, for_property: CowArg < 'a, GString >, inspector_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInspectObject < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, object: impl AsObjectArg < crate::classes::Object >,) -> Self {
        let for_property = GString::from("");
        let inspector_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, object: object.consume_arg(), for_property: CowArg::Owned(for_property), inspector_only: inspector_only,
        }
    }
    #[inline]
    pub fn for_property(self, for_property: impl AsArg < GString > + 'a) -> Self {
        Self {
            for_property: for_property.into_arg(), .. self
        }
    }
    #[inline]
    pub fn inspector_only(self, inspector_only: bool) -> Self {
        Self {
            inspector_only: inspector_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, object, for_property, inspector_only,
        }
        = self;
        re_export::EditorInterface::inspect_object_full(surround_object, object.cow_as_object_arg(), for_property, inspector_only,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::edit_script_ex`][super::EditorInterface::edit_script_ex]."]
#[must_use]
pub struct ExEditScript < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, script: ObjectCow < crate::classes::Script >, line: i32, column: i32, grab_focus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEditScript < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, script: impl AsObjectArg < crate::classes::Script >,) -> Self {
        let line = - 1i32;
        let column = 0i32;
        let grab_focus = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, script: script.consume_arg(), line: line, column: column, grab_focus: grab_focus,
        }
    }
    #[inline]
    pub fn line(self, line: i32) -> Self {
        Self {
            line: line, .. self
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn grab_focus(self, grab_focus: bool) -> Self {
        Self {
            grab_focus: grab_focus, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, script, line, column, grab_focus,
        }
        = self;
        re_export::EditorInterface::edit_script_full(surround_object, script.cow_as_object_arg(), line, column, grab_focus,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::save_scene_as_ex`][super::EditorInterface::save_scene_as_ex]."]
#[must_use]
pub struct ExSaveSceneAs < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, path: CowArg < 'a, GString >, with_preview: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveSceneAs < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, path: impl AsArg < GString > + 'a,) -> Self {
        let with_preview = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), with_preview: with_preview,
        }
    }
    #[inline]
    pub fn with_preview(self, with_preview: bool) -> Self {
        Self {
            with_preview: with_preview, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, path, with_preview,
        }
        = self;
        re_export::EditorInterface::save_scene_as_full(surround_object, path, with_preview,)
    }
}