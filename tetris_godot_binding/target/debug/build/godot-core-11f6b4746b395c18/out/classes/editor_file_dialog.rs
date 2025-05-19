#![doc = "Sidecar module for class [`EditorFileDialog`][crate::classes::EditorFileDialog].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorFileDialog` enums](https://docs.godotengine.org/en/stable/classes/class_editorfiledialog.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorFileDialog.`\n\nInherits [`ConfirmationDialog`][crate::classes::ConfirmationDialog].\n\nRelated symbols:\n\n* [`editor_file_dialog`][crate::classes::editor_file_dialog]: sidecar module with related enum/flag types\n* [`IEditorFileDialog`][crate::classes::IEditorFileDialog]: virtual methods\n\n\nSee also [Godot docs for `EditorFileDialog`](https://docs.godotengine.org/en/stable/classes/class_editorfiledialog.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorFileDialog::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorFileDialog {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorFileDialog`][crate::classes::EditorFileDialog].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorFileDialog` methods](https://docs.godotengine.org/en/stable/classes/class_editorfiledialog.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorFileDialog: crate::obj::GodotClass < Base = EditorFileDialog > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
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
        fn get_contents_minimum_size(&self,) -> Vector2 {
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
    impl EditorFileDialog {
        pub fn clear_filters(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(36usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "clear_filters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_filter_full(&mut self, filter: CowArg < GString >, description: CowArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (filter, description,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(37usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "add_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_filter_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_filter(&mut self, filter: impl AsArg < GString >,) {
            self.add_filter_ex(filter,) . done()
        }
        #[inline]
        pub fn add_filter_ex < 'a > (&'a mut self, filter: impl AsArg < GString > + 'a,) -> ExAddFilter < 'a > {
            ExAddFilter::new(self, filter,)
        }
        pub fn set_filters(&mut self, filters: &PackedStringArray,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, PackedStringArray >);
            let args = (RefArg::new(filters),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(38usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_filters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_filters(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(39usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_filters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_option_name(&self, option: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(40usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_option_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_option_values(&self, option: i32,) -> PackedStringArray {
            type CallSig = (PackedStringArray, i32);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(41usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_option_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_option_default(&self, option: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(42usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_option_default", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_option_name(&mut self, option: i32, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (option, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(43usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_option_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_option_values(&mut self, option: i32, values: &PackedStringArray,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, PackedStringArray >);
            let args = (option, RefArg::new(values),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(44usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_option_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_option_default(&mut self, option: i32, default_value_index: i32,) {
            type CallSig = ((), i32, i32);
            let args = (option, default_value_index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(45usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_option_default", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_option_count(&mut self, count: i32,) {
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(46usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_option_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_option_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(47usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_option_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_option(&mut self, name: impl AsArg < GString >, values: &PackedStringArray, default_value_index: i32,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, i32);
            let args = (name.into_arg(), RefArg::new(values), default_value_index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(48usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "add_option", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_options(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(49usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_selected_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_dir(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(50usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_current_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_file(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(51usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_current_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_path(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(52usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_current_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_dir(&mut self, dir: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (dir.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(53usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_current_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_file(&mut self, file: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(54usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_current_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_path(&mut self, path: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(55usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_current_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_file_mode(&mut self, mode: crate::classes::editor_file_dialog::FileMode,) {
            type CallSig = ((), crate::classes::editor_file_dialog::FileMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(56usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_file_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file_mode(&self,) -> crate::classes::editor_file_dialog::FileMode {
            type CallSig = (crate::classes::editor_file_dialog::FileMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(57usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_file_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vbox(&mut self,) -> Option < Gd < crate::classes::VBoxContainer > > {
            type CallSig = (Option < Gd < crate::classes::VBoxContainer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(58usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_vbox", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_edit(&mut self,) -> Option < Gd < crate::classes::LineEdit > > {
            type CallSig = (Option < Gd < crate::classes::LineEdit > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(59usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_line_edit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_access(&mut self, access: crate::classes::editor_file_dialog::Access,) {
            type CallSig = ((), crate::classes::editor_file_dialog::Access);
            let args = (access,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(60usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_access", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_access(&self,) -> crate::classes::editor_file_dialog::Access {
            type CallSig = (crate::classes::editor_file_dialog::Access,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(61usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_access", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_show_hidden_files(&mut self, show: bool,) {
            type CallSig = ((), bool);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(62usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_show_hidden_files", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_showing_hidden_files(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(63usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "is_showing_hidden_files", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_display_mode(&mut self, mode: crate::classes::editor_file_dialog::DisplayMode,) {
            type CallSig = ((), crate::classes::editor_file_dialog::DisplayMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(64usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_display_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_display_mode(&self,) -> crate::classes::editor_file_dialog::DisplayMode {
            type CallSig = (crate::classes::editor_file_dialog::DisplayMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(65usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_display_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_overwrite_warning(&mut self, disable: bool,) {
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(66usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_disable_overwrite_warning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_overwrite_warning_disabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(67usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "is_overwrite_warning_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_side_menu_full(&mut self, menu: ObjectArg < crate::classes::Control >, title: CowArg < GString >,) {
            type CallSig < 'a0, > = ((), ObjectArg < crate::classes::Control >, CowArg < 'a0, GString >);
            let args = (menu, title,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(68usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "add_side_menu", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_side_menu_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_side_menu(&mut self, menu: impl AsObjectArg < crate::classes::Control >,) {
            self.add_side_menu_ex(menu,) . done()
        }
        #[inline]
        pub fn add_side_menu_ex < 'a > (&'a mut self, menu: impl AsObjectArg < crate::classes::Control >,) -> ExAddSideMenu < 'a > {
            ExAddSideMenu::new(self, menu,)
        }
        pub fn popup_file_dialog(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(69usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "popup_file_dialog", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn invalidate(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(70usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorFileDialog", "invalidate", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorFileDialog {
        type Base = crate::classes::ConfirmationDialog;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"EditorFileDialog"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorFileDialog {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::ConfirmationDialog > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AcceptDialog > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Window > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorFileDialog {
        
    }
    impl crate::obj::cap::GodotDefault for EditorFileDialog {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorFileDialog {
        type Target = crate::classes::ConfirmationDialog;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorFileDialog {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorFileDialog`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_EditorFileDialog {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorFileDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::ConfirmationDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AcceptDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Window > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Viewport > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorFileDialog::add_filter_ex`][super::EditorFileDialog::add_filter_ex]."]
#[must_use]
pub struct ExAddFilter < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorFileDialog, filter: CowArg < 'a, GString >, description: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddFilter < 'a > {
    fn new(surround_object: &'a mut re_export::EditorFileDialog, filter: impl AsArg < GString > + 'a,) -> Self {
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, filter: filter.into_arg(), description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'a) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, filter, description,
        }
        = self;
        re_export::EditorFileDialog::add_filter_full(surround_object, filter, description,)
    }
}
#[doc = "Default-param extender for [`EditorFileDialog::add_side_menu_ex`][super::EditorFileDialog::add_side_menu_ex]."]
#[must_use]
pub struct ExAddSideMenu < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorFileDialog, menu: ObjectCow < crate::classes::Control >, title: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSideMenu < 'a > {
    fn new(surround_object: &'a mut re_export::EditorFileDialog, menu: impl AsObjectArg < crate::classes::Control >,) -> Self {
        let title = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu: menu.consume_arg(), title: CowArg::Owned(title),
        }
    }
    #[inline]
    pub fn title(self, title: impl AsArg < GString > + 'a) -> Self {
        Self {
            title: title.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, menu, title,
        }
        = self;
        re_export::EditorFileDialog::add_side_menu_full(surround_object, menu.cow_as_object_arg(), title,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileMode {
    ord: i32
}
impl FileMode {
    #[doc(alias = "FILE_MODE_OPEN_FILE")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_FILE`"]
    pub const OPEN_FILE: FileMode = FileMode {
        ord: 0i32
    };
    #[doc(alias = "FILE_MODE_OPEN_FILES")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_FILES`"]
    pub const OPEN_FILES: FileMode = FileMode {
        ord: 1i32
    };
    #[doc(alias = "FILE_MODE_OPEN_DIR")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_DIR`"]
    pub const OPEN_DIR: FileMode = FileMode {
        ord: 2i32
    };
    #[doc(alias = "FILE_MODE_OPEN_ANY")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_ANY`"]
    pub const OPEN_ANY: FileMode = FileMode {
        ord: 3i32
    };
    #[doc(alias = "FILE_MODE_SAVE_FILE")]
    #[doc = "Godot enumerator name: `FILE_MODE_SAVE_FILE`"]
    pub const SAVE_FILE: FileMode = FileMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for FileMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FileMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FileMode {
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
            Self::OPEN_FILE => "OPEN_FILE", Self::OPEN_FILES => "OPEN_FILES", Self::OPEN_DIR => "OPEN_DIR", Self::OPEN_ANY => "OPEN_ANY", Self::SAVE_FILE => "SAVE_FILE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OPEN_FILE => "FILE_MODE_OPEN_FILE", Self::OPEN_FILES => "FILE_MODE_OPEN_FILES", Self::OPEN_DIR => "FILE_MODE_OPEN_DIR", Self::OPEN_ANY => "FILE_MODE_OPEN_ANY", Self::SAVE_FILE => "FILE_MODE_SAVE_FILE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for FileMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FileMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FileMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Access {
    ord: i32
}
impl Access {
    #[doc(alias = "ACCESS_RESOURCES")]
    #[doc = "Godot enumerator name: `ACCESS_RESOURCES`"]
    pub const RESOURCES: Access = Access {
        ord: 0i32
    };
    #[doc(alias = "ACCESS_USERDATA")]
    #[doc = "Godot enumerator name: `ACCESS_USERDATA`"]
    pub const USERDATA: Access = Access {
        ord: 1i32
    };
    #[doc(alias = "ACCESS_FILESYSTEM")]
    #[doc = "Godot enumerator name: `ACCESS_FILESYSTEM`"]
    pub const FILESYSTEM: Access = Access {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Access") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Access {
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
            Self::RESOURCES => "RESOURCES", Self::USERDATA => "USERDATA", Self::FILESYSTEM => "FILESYSTEM", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::RESOURCES => "ACCESS_RESOURCES", Self::USERDATA => "ACCESS_USERDATA", Self::FILESYSTEM => "ACCESS_FILESYSTEM", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Access {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Access {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Access {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DisplayMode {
    ord: i32
}
impl DisplayMode {
    #[doc(alias = "DISPLAY_THUMBNAILS")]
    #[doc = "Godot enumerator name: `DISPLAY_THUMBNAILS`"]
    pub const THUMBNAILS: DisplayMode = DisplayMode {
        ord: 0i32
    };
    #[doc(alias = "DISPLAY_LIST")]
    #[doc = "Godot enumerator name: `DISPLAY_LIST`"]
    pub const LIST: DisplayMode = DisplayMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DisplayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DisplayMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DisplayMode {
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
            Self::THUMBNAILS => "THUMBNAILS", Self::LIST => "LIST", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::THUMBNAILS => "DISPLAY_THUMBNAILS", Self::LIST => "DISPLAY_LIST", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DisplayMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DisplayMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DisplayMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}