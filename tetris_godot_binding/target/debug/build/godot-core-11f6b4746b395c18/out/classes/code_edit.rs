#![doc = "Sidecar module for class [`CodeEdit`][crate::classes::CodeEdit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CodeEdit` enums](https://docs.godotengine.org/en/stable/classes/class_codeedit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CodeEdit.`\n\nInherits [`TextEdit`][crate::classes::TextEdit].\n\nRelated symbols:\n\n* [`code_edit`][crate::classes::code_edit]: sidecar module with related enum/flag types\n* [`ICodeEdit`][crate::classes::ICodeEdit]: virtual methods\n\n\nSee also [Godot docs for `CodeEdit`](https://docs.godotengine.org/en/stable/classes/class_codeedit.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CodeEdit::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CodeEdit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CodeEdit`][crate::classes::CodeEdit].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CodeEdit` methods](https://docs.godotengine.org/en/stable/classes/class_codeedit.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICodeEdit: crate::obj::GodotClass < Base = CodeEdit > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn confirm_code_completion(&mut self, replace: bool,) {
            unimplemented !()
        }
        fn request_code_completion(&mut self, force: bool,) {
            unimplemented !()
        }
        fn filter_code_completion_candidates(&self, candidates: Array < Dictionary >,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn handle_unicode_input(&mut self, unicode_char: i32, caret_index: i32,) {
            unimplemented !()
        }
        fn backspace(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn cut(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn copy(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn paste(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn paste_primary_clipboard(&mut self, caret_index: i32,) {
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
    impl CodeEdit {
        pub fn set_indent_size(&mut self, size: i32,) {
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_indent_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_indent_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_indent_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_indent_using_spaces(&mut self, use_spaces: bool,) {
            type CallSig = ((), bool);
            let args = (use_spaces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_indent_using_spaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_indent_using_spaces(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_indent_using_spaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_indent_enabled(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_indent_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_auto_indent_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_auto_indent_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_indent_prefixes(&mut self, prefixes: &Array < GString >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < GString > >);
            let args = (RefArg::new(prefixes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_indent_prefixes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_indent_prefixes(&self,) -> Array < GString > {
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_auto_indent_prefixes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn do_indent(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "do_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn indent_lines(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "indent_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unindent_lines(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "unindent_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn convert_indent_full(&mut self, from_line: i32, to_line: i32,) {
            type CallSig = ((), i32, i32);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "convert_indent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::convert_indent_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn convert_indent(&mut self,) {
            self.convert_indent_ex() . done()
        }
        #[inline]
        pub fn convert_indent_ex < 'a > (&'a mut self,) -> ExConvertIndent < 'a > {
            ExConvertIndent::new(self,)
        }
        pub fn set_auto_brace_completion_enabled(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_brace_completion_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_auto_brace_completion_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_auto_brace_completion_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_highlight_matching_braces_enabled(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_highlight_matching_braces_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_highlight_matching_braces_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_highlight_matching_braces_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_auto_brace_completion_pair(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (start_key.into_arg(), end_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "add_auto_brace_completion_pair", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_brace_completion_pairs(&mut self, pairs: &Dictionary,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Dictionary >);
            let args = (RefArg::new(pairs),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_brace_completion_pairs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_brace_completion_pairs(&self,) -> Dictionary {
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_auto_brace_completion_pairs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_auto_brace_completion_open_key(&self, open_key: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (open_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "has_auto_brace_completion_open_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_auto_brace_completion_close_key(&self, close_key: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (close_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "has_auto_brace_completion_close_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_brace_completion_close_key(&self, open_key: impl AsArg < GString >,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >);
            let args = (open_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_auto_brace_completion_close_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_breakpoints_gutter(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_breakpoints_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_breakpoints_gutter(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_breakpoints_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_bookmarks_gutter(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_bookmarks_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_bookmarks_gutter(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_bookmarks_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_executing_lines_gutter(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_executing_lines_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_executing_lines_gutter(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_executing_lines_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_as_breakpoint(&mut self, line: i32, breakpointed: bool,) {
            type CallSig = ((), i32, bool);
            let args = (line, breakpointed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_as_breakpoint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_breakpointed(&self, line: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_breakpointed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_breakpointed_lines(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_breakpointed_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_breakpointed_lines(&self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_breakpointed_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_as_bookmarked(&mut self, line: i32, bookmarked: bool,) {
            type CallSig = ((), i32, bool);
            let args = (line, bookmarked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_as_bookmarked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_bookmarked(&self, line: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_bookmarked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_bookmarked_lines(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_bookmarked_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bookmarked_lines(&self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_bookmarked_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_as_executing(&mut self, line: i32, executing: bool,) {
            type CallSig = ((), i32, bool);
            let args = (line, executing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_as_executing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_executing(&self, line: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2019usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_executing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_executing_lines(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2020usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_executing_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_executing_lines(&self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2021usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_executing_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_line_numbers(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_line_numbers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_draw_line_numbers_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_draw_line_numbers_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_numbers_zero_padded(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_numbers_zero_padded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_numbers_zero_padded(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_numbers_zero_padded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_fold_gutter(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_fold_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_drawing_fold_gutter(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_fold_gutter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_folding_enabled(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_folding_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_folding_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_folding_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn can_fold_line(&self, line: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "can_fold_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fold_line(&mut self, line: i32,) {
            type CallSig = ((), i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "fold_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unfold_line(&mut self, line: i32,) {
            type CallSig = ((), i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "unfold_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn fold_all_lines(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "fold_all_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unfold_all_lines(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "unfold_all_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn toggle_foldable_line(&mut self, line: i32,) {
            type CallSig = ((), i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "toggle_foldable_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn toggle_foldable_lines_at_carets(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "toggle_foldable_lines_at_carets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_folded(&self, line: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_folded", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_folded_lines(&self,) -> Array < i64 > {
            type CallSig = (Array < i64 >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_folded_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_code_region(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "create_code_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_region_start_tag(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_region_start_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_region_end_tag(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_region_end_tag", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_code_region_tags_full(&mut self, start: CowArg < GString >, end: CowArg < GString >,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >);
            let args = (start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_region_tags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_code_region_tags_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_code_region_tags(&mut self,) {
            self.set_code_region_tags_ex() . done()
        }
        #[inline]
        pub fn set_code_region_tags_ex < 'a > (&'a mut self,) -> ExSetCodeRegionTags < 'a > {
            ExSetCodeRegionTags::new(self,)
        }
        pub fn is_line_code_region_start(&self, line: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_code_region_start", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_line_code_region_end(&self, line: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_code_region_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_string_delimiter_full(&mut self, start_key: CowArg < GString >, end_key: CowArg < GString >, line_only: bool,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool);
            let args = (start_key, end_key, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "add_string_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_string_delimiter_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_string_delimiter(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >,) {
            self.add_string_delimiter_ex(start_key, end_key,) . done()
        }
        #[inline]
        pub fn add_string_delimiter_ex < 'a > (&'a mut self, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a,) -> ExAddStringDelimiter < 'a > {
            ExAddStringDelimiter::new(self, start_key, end_key,)
        }
        pub fn remove_string_delimiter(&mut self, start_key: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "remove_string_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_string_delimiter(&self, start_key: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "has_string_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_string_delimiters(&mut self, string_delimiters: &Array < GString >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < GString > >);
            let args = (RefArg::new(string_delimiters),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_string_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_string_delimiters(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_string_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_string_delimiters(&self,) -> Array < GString > {
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_string_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_in_string_full(&self, line: i32, column: i32,) -> i32 {
            type CallSig = (i32, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_in_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_in_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_in_string(&self, line: i32,) -> i32 {
            self.is_in_string_ex(line,) . done()
        }
        #[inline]
        pub fn is_in_string_ex < 'a > (&'a self, line: i32,) -> ExIsInString < 'a > {
            ExIsInString::new(self, line,)
        }
        pub(crate) fn add_comment_delimiter_full(&mut self, start_key: CowArg < GString >, end_key: CowArg < GString >, line_only: bool,) {
            type CallSig < 'a0, 'a1, > = ((), CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool);
            let args = (start_key, end_key, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "add_comment_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_comment_delimiter_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_comment_delimiter(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >,) {
            self.add_comment_delimiter_ex(start_key, end_key,) . done()
        }
        #[inline]
        pub fn add_comment_delimiter_ex < 'a > (&'a mut self, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a,) -> ExAddCommentDelimiter < 'a > {
            ExAddCommentDelimiter::new(self, start_key, end_key,)
        }
        pub fn remove_comment_delimiter(&mut self, start_key: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "remove_comment_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_comment_delimiter(&self, start_key: impl AsArg < GString >,) -> bool {
            type CallSig < 'a0, > = (bool, CowArg < 'a0, GString >);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "has_comment_delimiter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_comment_delimiters(&mut self, comment_delimiters: &Array < GString >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < GString > >);
            let args = (RefArg::new(comment_delimiters),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_comment_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_comment_delimiters(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_comment_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_comment_delimiters(&self,) -> Array < GString > {
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_comment_delimiters", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_in_comment_full(&self, line: i32, column: i32,) -> i32 {
            type CallSig = (i32, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_in_comment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_in_comment_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_in_comment(&self, line: i32,) -> i32 {
            self.is_in_comment_ex(line,) . done()
        }
        #[inline]
        pub fn is_in_comment_ex < 'a > (&'a self, line: i32,) -> ExIsInComment < 'a > {
            ExIsInComment::new(self, line,)
        }
        pub fn get_delimiter_start_key(&self, delimiter_index: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (delimiter_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_start_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_delimiter_end_key(&self, delimiter_index: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (delimiter_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_end_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_delimiter_start_position(&self, line: i32, column: i32,) -> Vector2 {
            type CallSig = (Vector2, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_start_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_delimiter_end_position(&self, line: i32, column: i32,) -> Vector2 {
            type CallSig = (Vector2, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_end_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_hint(&mut self, code_hint: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (code_hint.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_hint_draw_below(&mut self, draw_below: bool,) {
            type CallSig = ((), bool);
            let args = (draw_below,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_hint_draw_below", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_for_code_completion(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_text_for_code_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn request_code_completion_full(&mut self, force: bool,) {
            type CallSig = ((), bool);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "request_code_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::request_code_completion_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn request_code_completion(&mut self,) {
            self.request_code_completion_ex() . done()
        }
        #[inline]
        pub fn request_code_completion_ex < 'a > (&'a mut self,) -> ExRequestCodeCompletion < 'a > {
            ExRequestCodeCompletion::new(self,)
        }
        pub(crate) fn add_code_completion_option_full(&mut self, type_: crate::classes::code_edit::CodeCompletionKind, display_text: CowArg < GString >, insert_text: CowArg < GString >, text_color: Color, icon: ObjectArg < crate::classes::Resource >, value: RefArg < Variant >, location: i32,) {
            type CallSig < 'a0, 'a1, 'a2, > = ((), crate::classes::code_edit::CodeCompletionKind, CowArg < 'a0, GString >, CowArg < 'a1, GString >, Color, ObjectArg < crate::classes::Resource >, RefArg < 'a2, Variant >, i32);
            let args = (type_, display_text, insert_text, text_color, icon, value, location,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "add_code_completion_option", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_code_completion_option_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_code_completion_option(&mut self, type_: crate::classes::code_edit::CodeCompletionKind, display_text: impl AsArg < GString >, insert_text: impl AsArg < GString >,) {
            self.add_code_completion_option_ex(type_, display_text, insert_text,) . done()
        }
        #[inline]
        pub fn add_code_completion_option_ex < 'a > (&'a mut self, type_: crate::classes::code_edit::CodeCompletionKind, display_text: impl AsArg < GString > + 'a, insert_text: impl AsArg < GString > + 'a,) -> ExAddCodeCompletionOption < 'a > {
            ExAddCodeCompletionOption::new(self, type_, display_text, insert_text,)
        }
        pub fn update_code_completion_options(&mut self, force: bool,) {
            type CallSig = ((), bool);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "update_code_completion_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_completion_options(&self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_completion_option(&self, index: i32,) -> Dictionary {
            type CallSig = (Dictionary, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_option", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_completion_selected_index(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_selected_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_completion_selected_index(&mut self, index: i32,) {
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_completion_selected_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn confirm_code_completion_full(&mut self, replace: bool,) {
            type CallSig = ((), bool);
            let args = (replace,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "confirm_code_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::confirm_code_completion_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn confirm_code_completion(&mut self,) {
            self.confirm_code_completion_ex() . done()
        }
        #[inline]
        pub fn confirm_code_completion_ex < 'a > (&'a mut self,) -> ExConfirmCodeCompletion < 'a > {
            ExConfirmCodeCompletion::new(self,)
        }
        pub fn cancel_code_completion(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "cancel_code_completion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_completion_enabled(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_completion_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_code_completion_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_code_completion_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_code_completion_prefixes(&mut self, prefixes: &Array < GString >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < GString > >);
            let args = (RefArg::new(prefixes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_completion_prefixes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_code_completion_prefixes(&self,) -> Array < GString > {
            type CallSig = (Array < GString >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_prefixes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_line_length_guidelines(&mut self, guideline_columns: &Array < i64 >,) {
            type CallSig < 'a0, > = ((), RefArg < 'a0, Array < i64 > >);
            let args = (RefArg::new(guideline_columns),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_length_guidelines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line_length_guidelines(&self,) -> Array < i64 > {
            type CallSig = (Array < i64 >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_line_length_guidelines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_symbol_lookup_on_click_enabled(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_symbol_lookup_on_click_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_symbol_lookup_on_click_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "is_symbol_lookup_on_click_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_for_symbol_lookup(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_text_for_symbol_lookup", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_with_cursor_char(&self, line: i32, column: i32,) -> GString {
            type CallSig = (GString, i32, i32);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "get_text_with_cursor_char", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_symbol_lookup_word_as_valid(&mut self, valid: bool,) {
            type CallSig = ((), bool);
            let args = (valid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "set_symbol_lookup_word_as_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_lines_up(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "move_lines_up", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_lines_down(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "move_lines_down", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn delete_lines(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "delete_lines", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn duplicate_selection(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "duplicate_selection", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn duplicate_lines(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "CodeEdit", "duplicate_lines", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for CodeEdit {
        type Base = crate::classes::TextEdit;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"CodeEdit"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CodeEdit {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TextEdit > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CodeEdit {
        
    }
    impl crate::obj::cap::GodotDefault for CodeEdit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CodeEdit {
        type Target = crate::classes::TextEdit;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CodeEdit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CodeEdit`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_CodeEdit {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CodeEdit > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextEdit > for $Class {
                
            }
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
#[doc = "Default-param extender for [`CodeEdit::convert_indent_ex`][super::CodeEdit::convert_indent_ex]."]
#[must_use]
pub struct ExConvertIndent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, from_line: i32, to_line: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConvertIndent < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        let from_line = - 1i32;
        let to_line = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_line: from_line, to_line: to_line,
        }
    }
    #[inline]
    pub fn from_line(self, from_line: i32) -> Self {
        Self {
            from_line: from_line, .. self
        }
    }
    #[inline]
    pub fn to_line(self, to_line: i32) -> Self {
        Self {
            to_line: to_line, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_line, to_line,
        }
        = self;
        re_export::CodeEdit::convert_indent_full(surround_object, from_line, to_line,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::set_code_region_tags_ex`][super::CodeEdit::set_code_region_tags_ex]."]
#[must_use]
pub struct ExSetCodeRegionTags < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, start: CowArg < 'a, GString >, end: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCodeRegionTags < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        let start = GString::from("region");
        let end = GString::from("endregion");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start: CowArg::Owned(start), end: CowArg::Owned(end),
        }
    }
    #[inline]
    pub fn start(self, start: impl AsArg < GString > + 'a) -> Self {
        Self {
            start: start.into_arg(), .. self
        }
    }
    #[inline]
    pub fn end(self, end: impl AsArg < GString > + 'a) -> Self {
        Self {
            end: end.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start, end,
        }
        = self;
        re_export::CodeEdit::set_code_region_tags_full(surround_object, start, end,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_string_delimiter_ex`][super::CodeEdit::add_string_delimiter_ex]."]
#[must_use]
pub struct ExAddStringDelimiter < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, start_key: CowArg < 'a, GString >, end_key: CowArg < 'a, GString >, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddStringDelimiter < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a,) -> Self {
        let line_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_key: start_key.into_arg(), end_key: end_key.into_arg(), line_only: line_only,
        }
    }
    #[inline]
    pub fn line_only(self, line_only: bool) -> Self {
        Self {
            line_only: line_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_key, end_key, line_only,
        }
        = self;
        re_export::CodeEdit::add_string_delimiter_full(surround_object, start_key, end_key, line_only,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::is_in_string_ex`][super::CodeEdit::is_in_string_ex]."]
#[must_use]
pub struct ExIsInString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CodeEdit, line: i32, column: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsInString < 'a > {
    fn new(surround_object: &'a re_export::CodeEdit, line: i32,) -> Self {
        let column = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, column: column,
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, column,
        }
        = self;
        re_export::CodeEdit::is_in_string_full(surround_object, line, column,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_comment_delimiter_ex`][super::CodeEdit::add_comment_delimiter_ex]."]
#[must_use]
pub struct ExAddCommentDelimiter < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, start_key: CowArg < 'a, GString >, end_key: CowArg < 'a, GString >, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCommentDelimiter < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a,) -> Self {
        let line_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_key: start_key.into_arg(), end_key: end_key.into_arg(), line_only: line_only,
        }
    }
    #[inline]
    pub fn line_only(self, line_only: bool) -> Self {
        Self {
            line_only: line_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_key, end_key, line_only,
        }
        = self;
        re_export::CodeEdit::add_comment_delimiter_full(surround_object, start_key, end_key, line_only,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::is_in_comment_ex`][super::CodeEdit::is_in_comment_ex]."]
#[must_use]
pub struct ExIsInComment < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CodeEdit, line: i32, column: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsInComment < 'a > {
    fn new(surround_object: &'a re_export::CodeEdit, line: i32,) -> Self {
        let column = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, column: column,
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, column,
        }
        = self;
        re_export::CodeEdit::is_in_comment_full(surround_object, line, column,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::request_code_completion_ex`][super::CodeEdit::request_code_completion_ex]."]
#[must_use]
pub struct ExRequestCodeCompletion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, force: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRequestCodeCompletion < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        let force = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: bool) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force,
        }
        = self;
        re_export::CodeEdit::request_code_completion_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_code_completion_option_ex`][super::CodeEdit::add_code_completion_option_ex]."]
#[must_use]
pub struct ExAddCodeCompletionOption < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, type_: crate::classes::code_edit::CodeCompletionKind, display_text: CowArg < 'a, GString >, insert_text: CowArg < 'a, GString >, text_color: Color, icon: ObjectCow < crate::classes::Resource >, value: CowArg < 'a, Variant >, location: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCodeCompletionOption < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit, type_: crate::classes::code_edit::CodeCompletionKind, display_text: impl AsArg < GString > + 'a, insert_text: impl AsArg < GString > + 'a,) -> Self {
        let text_color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let icon = Gd::null_arg();
        let value = Variant::nil();
        let location = 1024i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, type_: type_, display_text: display_text.into_arg(), insert_text: insert_text.into_arg(), text_color: text_color, icon: icon.consume_arg(), value: CowArg::Owned(value), location: location,
        }
    }
    #[inline]
    pub fn text_color(self, text_color: Color) -> Self {
        Self {
            text_color: text_color, .. self
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsObjectArg < crate::classes::Resource >) -> Self {
        Self {
            icon: icon.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn value(self, value: &'a Variant) -> Self {
        Self {
            value: CowArg::Borrowed(value), .. self
        }
    }
    #[inline]
    pub fn location(self, location: i32) -> Self {
        Self {
            location: location, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, type_, display_text, insert_text, text_color, icon, value, location,
        }
        = self;
        re_export::CodeEdit::add_code_completion_option_full(surround_object, type_, display_text, insert_text, text_color, icon.cow_as_object_arg(), value.cow_as_arg(), location,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::confirm_code_completion_ex`][super::CodeEdit::confirm_code_completion_ex]."]
#[must_use]
pub struct ExConfirmCodeCompletion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, replace: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConfirmCodeCompletion < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        let replace = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, replace: replace,
        }
    }
    #[inline]
    pub fn replace(self, replace: bool) -> Self {
        Self {
            replace: replace, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, replace,
        }
        = self;
        re_export::CodeEdit::confirm_code_completion_full(surround_object, replace,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CodeCompletionKind {
    ord: i32
}
impl CodeCompletionKind {
    #[doc(alias = "KIND_CLASS")]
    #[doc = "Godot enumerator name: `KIND_CLASS`"]
    pub const CLASS: CodeCompletionKind = CodeCompletionKind {
        ord: 0i32
    };
    #[doc(alias = "KIND_FUNCTION")]
    #[doc = "Godot enumerator name: `KIND_FUNCTION`"]
    pub const FUNCTION: CodeCompletionKind = CodeCompletionKind {
        ord: 1i32
    };
    #[doc(alias = "KIND_SIGNAL")]
    #[doc = "Godot enumerator name: `KIND_SIGNAL`"]
    pub const SIGNAL: CodeCompletionKind = CodeCompletionKind {
        ord: 2i32
    };
    #[doc(alias = "KIND_VARIABLE")]
    #[doc = "Godot enumerator name: `KIND_VARIABLE`"]
    pub const VARIABLE: CodeCompletionKind = CodeCompletionKind {
        ord: 3i32
    };
    #[doc(alias = "KIND_MEMBER")]
    #[doc = "Godot enumerator name: `KIND_MEMBER`"]
    pub const MEMBER: CodeCompletionKind = CodeCompletionKind {
        ord: 4i32
    };
    #[doc(alias = "KIND_ENUM")]
    #[doc = "Godot enumerator name: `KIND_ENUM`"]
    pub const ENUM: CodeCompletionKind = CodeCompletionKind {
        ord: 5i32
    };
    #[doc(alias = "KIND_CONSTANT")]
    #[doc = "Godot enumerator name: `KIND_CONSTANT`"]
    pub const CONSTANT: CodeCompletionKind = CodeCompletionKind {
        ord: 6i32
    };
    #[doc(alias = "KIND_NODE_PATH")]
    #[doc = "Godot enumerator name: `KIND_NODE_PATH`"]
    pub const NODE_PATH: CodeCompletionKind = CodeCompletionKind {
        ord: 7i32
    };
    #[doc(alias = "KIND_FILE_PATH")]
    #[doc = "Godot enumerator name: `KIND_FILE_PATH`"]
    pub const FILE_PATH: CodeCompletionKind = CodeCompletionKind {
        ord: 8i32
    };
    #[doc(alias = "KIND_PLAIN_TEXT")]
    #[doc = "Godot enumerator name: `KIND_PLAIN_TEXT`"]
    pub const PLAIN_TEXT: CodeCompletionKind = CodeCompletionKind {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for CodeCompletionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CodeCompletionKind") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CodeCompletionKind {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::CLASS => "CLASS", Self::FUNCTION => "FUNCTION", Self::SIGNAL => "SIGNAL", Self::VARIABLE => "VARIABLE", Self::MEMBER => "MEMBER", Self::ENUM => "ENUM", Self::CONSTANT => "CONSTANT", Self::NODE_PATH => "NODE_PATH", Self::FILE_PATH => "FILE_PATH", Self::PLAIN_TEXT => "PLAIN_TEXT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CLASS => "KIND_CLASS", Self::FUNCTION => "KIND_FUNCTION", Self::SIGNAL => "KIND_SIGNAL", Self::VARIABLE => "KIND_VARIABLE", Self::MEMBER => "KIND_MEMBER", Self::ENUM => "KIND_ENUM", Self::CONSTANT => "KIND_CONSTANT", Self::NODE_PATH => "KIND_NODE_PATH", Self::FILE_PATH => "KIND_FILE_PATH", Self::PLAIN_TEXT => "KIND_PLAIN_TEXT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CodeCompletionKind {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CodeCompletionKind {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CodeCompletionKind {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CodeCompletionLocation {
    ord: i32
}
impl CodeCompletionLocation {
    #[doc(alias = "LOCATION_LOCAL")]
    #[doc = "Godot enumerator name: `LOCATION_LOCAL`"]
    pub const LOCAL: CodeCompletionLocation = CodeCompletionLocation {
        ord: 0i32
    };
    #[doc(alias = "LOCATION_PARENT_MASK")]
    #[doc = "Godot enumerator name: `LOCATION_PARENT_MASK`"]
    pub const PARENT_MASK: CodeCompletionLocation = CodeCompletionLocation {
        ord: 256i32
    };
    #[doc(alias = "LOCATION_OTHER_USER_CODE")]
    #[doc = "Godot enumerator name: `LOCATION_OTHER_USER_CODE`"]
    pub const OTHER_USER_CODE: CodeCompletionLocation = CodeCompletionLocation {
        ord: 512i32
    };
    #[doc(alias = "LOCATION_OTHER")]
    #[doc = "Godot enumerator name: `LOCATION_OTHER`"]
    pub const OTHER: CodeCompletionLocation = CodeCompletionLocation {
        ord: 1024i32
    };
    
}
impl std::fmt::Debug for CodeCompletionLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CodeCompletionLocation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CodeCompletionLocation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 => Some(Self {
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
            Self::LOCAL => "LOCAL", Self::PARENT_MASK => "PARENT_MASK", Self::OTHER_USER_CODE => "OTHER_USER_CODE", Self::OTHER => "OTHER", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LOCAL => "LOCATION_LOCAL", Self::PARENT_MASK => "LOCATION_PARENT_MASK", Self::OTHER_USER_CODE => "LOCATION_OTHER_USER_CODE", Self::OTHER => "LOCATION_OTHER", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for CodeCompletionLocation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CodeCompletionLocation {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CodeCompletionLocation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}