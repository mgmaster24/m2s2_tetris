#![doc = "Sidecar module for class [`TreeItem`][crate::classes::TreeItem].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TreeItem` enums](https://docs.godotengine.org/en/stable/classes/class_treeitem.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TreeItem.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`tree_item`][crate::classes::tree_item]: sidecar module with related enum/flag types\n* [`ITreeItem`][crate::classes::ITreeItem]: virtual methods\n\n\nSee also [Godot docs for `TreeItem`](https://docs.godotengine.org/en/stable/classes/class_treeitem.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TreeItem>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TreeItem {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TreeItem`][crate::classes::TreeItem].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TreeItem` methods](https://docs.godotengine.org/en/stable/classes/class_treeitem.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITreeItem: crate::obj::GodotClass < Base = TreeItem > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TreeItem {
        pub fn set_cell_mode(&mut self, column: i32, mode: crate::classes::tree_item::TreeCellMode,) {
            type CallSig = ((), i32, crate::classes::tree_item::TreeCellMode);
            let args = (column, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9521usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_cell_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_cell_mode(&self, column: i32,) -> crate::classes::tree_item::TreeCellMode {
            type CallSig = (crate::classes::tree_item::TreeCellMode, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_cell_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edit_multiline(&mut self, column: i32, multiline: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, multiline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_edit_multiline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_edit_multiline(&self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_edit_multiline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_checked(&mut self, column: i32, checked: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_indeterminate(&mut self, column: i32, indeterminate: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, indeterminate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_indeterminate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_checked(&self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_indeterminate(&self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_indeterminate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn propagate_check_full(&mut self, column: i32, emit_signal: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, emit_signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "propagate_check", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::propagate_check_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn propagate_check(&mut self, column: i32,) {
            self.propagate_check_ex(column,) . done()
        }
        #[inline]
        pub fn propagate_check_ex < 'a > (&'a mut self, column: i32,) -> ExPropagateCheck < 'a > {
            ExPropagateCheck::new(self, column,)
        }
        pub fn set_text(&mut self, column: i32, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (column, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text(&self, column: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_direction(&mut self, column: i32, direction: crate::classes::control::TextDirection,) {
            type CallSig = ((), i32, crate::classes::control::TextDirection);
            let args = (column, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_direction(&self, column: i32,) -> crate::classes::control::TextDirection {
            type CallSig = (crate::classes::control::TextDirection, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_text_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, column: i32, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallSig = ((), i32, crate::classes::text_server::AutowrapMode);
            let args = (column, autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autowrap_mode(&self, column: i32,) -> crate::classes::text_server::AutowrapMode {
            type CallSig = (crate::classes::text_server::AutowrapMode, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_autowrap_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, column: i32, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallSig = ((), i32, crate::classes::text_server::OverrunBehavior);
            let args = (column, overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self, column: i32,) -> crate::classes::text_server::OverrunBehavior {
            type CallSig = (crate::classes::text_server::OverrunBehavior, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_text_overrun_behavior", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, column: i32, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallSig = ((), i32, crate::classes::text_server::StructuredTextParser);
            let args = (column, parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self, column: i32,) -> crate::classes::text_server::StructuredTextParser {
            type CallSig = (crate::classes::text_server::StructuredTextParser, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_structured_text_bidi_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, column: i32, args: &VariantArray,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, VariantArray >);
            let args = (column, RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self, column: i32,) -> VariantArray {
            type CallSig = (VariantArray, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_structured_text_bidi_override_options", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_language(&mut self, column: i32, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (column, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language(&self, column: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_suffix(&mut self, column: i32, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (column, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_suffix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_suffix(&self, column: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_suffix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon(&mut self, column: i32, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Texture2D >);
            let args = (column, texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon(&self, column: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_region(&mut self, column: i32, region: Rect2,) {
            type CallSig = ((), i32, Rect2);
            let args = (column, region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_region(&self, column: i32,) -> Rect2 {
            type CallSig = (Rect2, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_max_width(&mut self, column: i32, width: i32,) {
            type CallSig = ((), i32, i32);
            let args = (column, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_max_width(&self, column: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_max_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_icon_modulate(&mut self, column: i32, modulate: Color,) {
            type CallSig = ((), i32, Color);
            let args = (column, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_icon_modulate(&self, column: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_modulate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_range(&mut self, column: i32, value: f64,) {
            type CallSig = ((), i32, f64);
            let args = (column, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_range(&self, column: i32,) -> f64 {
            type CallSig = (f64, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_range_config_full(&mut self, column: i32, min: f64, max: f64, step: f64, expr: bool,) {
            type CallSig = ((), i32, f64, f64, f64, bool);
            let args = (column, min, max, step, expr,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_range_config", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_range_config_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_range_config(&mut self, column: i32, min: f64, max: f64, step: f64,) {
            self.set_range_config_ex(column, min, max, step,) . done()
        }
        #[inline]
        pub fn set_range_config_ex < 'a > (&'a mut self, column: i32, min: f64, max: f64, step: f64,) -> ExSetRangeConfig < 'a > {
            ExSetRangeConfig::new(self, column, min, max, step,)
        }
        pub fn get_range_config(&mut self, column: i32,) -> Dictionary {
            type CallSig = (Dictionary, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_range_config", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_metadata(&mut self, column: i32, meta: &Variant,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Variant >);
            let args = (column, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_metadata(&self, column: i32,) -> Variant {
            type CallSig = (Variant, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_metadata", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_draw(&mut self, column: i32, object: impl AsObjectArg < crate::classes::Object >, callback: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), i32, ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >);
            let args = (column, object.as_object_arg(), callback.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_draw_callback(&mut self, column: i32, callback: &Callable,) {
            type CallSig < 'a0, > = ((), i32, RefArg < 'a0, Callable >);
            let args = (column, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_draw_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_draw_callback(&self, column: i32,) -> Callable {
            type CallSig = (Callable, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_draw_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collapsed(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_collapsed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collapsed(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_collapsed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collapsed_recursive(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_collapsed_recursive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn is_any_collapsed_full(&mut self, only_visible: bool,) -> bool {
            type CallSig = (bool, bool);
            let args = (only_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_any_collapsed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_any_collapsed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_any_collapsed(&mut self,) -> bool {
            self.is_any_collapsed_ex() . done()
        }
        #[inline]
        pub fn is_any_collapsed_ex < 'a > (&'a mut self,) -> ExIsAnyCollapsed < 'a > {
            ExIsAnyCollapsed::new(self,)
        }
        pub fn set_visible(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_visible_in_tree(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_visible_in_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn uncollapse_tree(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "uncollapse_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_minimum_height(&mut self, height: i32,) {
            type CallSig = ((), i32);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_minimum_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_minimum_height(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_minimum_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_selectable(&mut self, column: i32, selectable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_selectable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_selectable(&self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_selectable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_selected(&mut self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn select(&mut self, column: i32,) {
            type CallSig = ((), i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deselect(&mut self, column: i32,) {
            type CallSig = ((), i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "deselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_editable(&mut self, column: i32, enabled: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_editable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editable(&mut self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_editable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_color(&mut self, column: i32, color: Color,) {
            type CallSig = ((), i32, Color);
            let args = (column, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_color(&self, column: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_custom_color(&mut self, column: i32,) {
            type CallSig = ((), i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "clear_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_font(&mut self, column: i32, font: impl AsObjectArg < crate::classes::Font >,) {
            type CallSig = ((), i32, ObjectArg < crate::classes::Font >);
            let args = (column, font.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9583usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_font(&self, column: i32,) -> Option < Gd < crate::classes::Font > > {
            type CallSig = (Option < Gd < crate::classes::Font > >, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9584usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_font", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_font_size(&mut self, column: i32, font_size: i32,) {
            type CallSig = ((), i32, i32);
            let args = (column, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9585usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_font_size(&self, column: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9586usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_font_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_custom_bg_color_full(&mut self, column: i32, color: Color, just_outline: bool,) {
            type CallSig = ((), i32, Color, bool);
            let args = (column, color, just_outline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9587usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_custom_bg_color_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_custom_bg_color(&mut self, column: i32, color: Color,) {
            self.set_custom_bg_color_ex(column, color,) . done()
        }
        #[inline]
        pub fn set_custom_bg_color_ex < 'a > (&'a mut self, column: i32, color: Color,) -> ExSetCustomBgColor < 'a > {
            ExSetCustomBgColor::new(self, column, color,)
        }
        pub fn clear_custom_bg_color(&mut self, column: i32,) {
            type CallSig = ((), i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9588usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "clear_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_bg_color(&self, column: i32,) -> Color {
            type CallSig = (Color, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9589usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_bg_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_as_button(&mut self, column: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9590usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_as_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_custom_set_as_button(&self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9591usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_custom_set_as_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_button_full(&mut self, column: i32, button: ObjectArg < crate::classes::Texture2D >, id: i32, disabled: bool, tooltip_text: CowArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, ObjectArg < crate::classes::Texture2D >, i32, bool, CowArg < 'a0, GString >);
            let args = (column, button, id, disabled, tooltip_text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9592usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "add_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_button_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_button(&mut self, column: i32, button: impl AsObjectArg < crate::classes::Texture2D >,) {
            self.add_button_ex(column, button,) . done()
        }
        #[inline]
        pub fn add_button_ex < 'a > (&'a mut self, column: i32, button: impl AsObjectArg < crate::classes::Texture2D >,) -> ExAddButton < 'a > {
            ExAddButton::new(self, column, button,)
        }
        pub fn get_button_count(&self, column: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9593usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_tooltip_text(&self, column: i32, button_index: i32,) -> GString {
            type CallSig = (GString, i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9594usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_id(&self, column: i32, button_index: i32,) -> i32 {
            type CallSig = (i32, i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9595usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_by_id(&self, column: i32, id: i32,) -> i32 {
            type CallSig = (i32, i32, i32);
            let args = (column, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9596usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_by_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_color(&self, column: i32, id: i32,) -> Color {
            type CallSig = (Color, i32, i32);
            let args = (column, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9597usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button(&self, column: i32, button_index: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >, i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button_tooltip_text(&mut self, column: i32, button_index: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, i32, CowArg < 'a0, GString >);
            let args = (column, button_index, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button(&mut self, column: i32, button_index: i32, button: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), i32, i32, ObjectArg < crate::classes::Texture2D >);
            let args = (column, button_index, button.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_button(&mut self, column: i32, button_index: i32,) {
            type CallSig = ((), i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "erase_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button_disabled(&mut self, column: i32, button_index: i32, disabled: bool,) {
            type CallSig = ((), i32, i32, bool);
            let args = (column, button_index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_button_color(&mut self, column: i32, button_index: i32, color: Color,) {
            type CallSig = ((), i32, i32, Color);
            let args = (column, button_index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_button_disabled(&self, column: i32, button_index: i32,) -> bool {
            type CallSig = (bool, i32, i32);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_button_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tooltip_text(&mut self, column: i32, tooltip: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (column, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tooltip_text(&self, column: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_tooltip_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_text_alignment(&mut self, column: i32, text_alignment: crate::global::HorizontalAlignment,) {
            type CallSig = ((), i32, crate::global::HorizontalAlignment);
            let args = (column, text_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_text_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_text_alignment(&self, column: i32,) -> crate::global::HorizontalAlignment {
            type CallSig = (crate::global::HorizontalAlignment, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_text_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_expand_right(&mut self, column: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_expand_right", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_expand_right(&self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_expand_right", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_folding(&mut self, disable: bool,) {
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9611usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "set_disable_folding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_folding_disabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9612usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "is_folding_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_child_full(&mut self, index: i32,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "create_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_child_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_child(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.create_child_ex() . done()
        }
        #[inline]
        pub fn create_child_ex < 'a > (&'a mut self,) -> ExCreateChild < 'a > {
            ExCreateChild::new(self,)
        }
        pub fn add_child(&mut self, child: impl AsObjectArg < crate::classes::TreeItem >,) {
            type CallSig = ((), ObjectArg < crate::classes::TreeItem >);
            let args = (child.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "add_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_child(&mut self, child: impl AsObjectArg < crate::classes::TreeItem >,) {
            type CallSig = ((), ObjectArg < crate::classes::TreeItem >);
            let args = (child.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "remove_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tree(&self,) -> Option < Gd < crate::classes::Tree > > {
            type CallSig = (Option < Gd < crate::classes::Tree > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_prev(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_prev", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_first_child(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_first_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_next_in_tree_full(&mut self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >, bool);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_next_in_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_next_in_tree_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_next_in_tree(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_next_in_tree_ex() . done()
        }
        #[inline]
        pub fn get_next_in_tree_ex < 'a > (&'a mut self,) -> ExGetNextInTree < 'a > {
            ExGetNextInTree::new(self,)
        }
        pub(crate) fn get_prev_in_tree_full(&mut self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >, bool);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_prev_in_tree", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_prev_in_tree_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_prev_in_tree(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_prev_in_tree_ex() . done()
        }
        #[inline]
        pub fn get_prev_in_tree_ex < 'a > (&'a mut self,) -> ExGetPrevInTree < 'a > {
            ExGetPrevInTree::new(self,)
        }
        pub(crate) fn get_next_visible_full(&mut self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >, bool);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_next_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_next_visible_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_next_visible(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_next_visible_ex() . done()
        }
        #[inline]
        pub fn get_next_visible_ex < 'a > (&'a mut self,) -> ExGetNextVisible < 'a > {
            ExGetNextVisible::new(self,)
        }
        pub(crate) fn get_prev_visible_full(&mut self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >, bool);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_prev_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_prev_visible_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_prev_visible(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_prev_visible_ex() . done()
        }
        #[inline]
        pub fn get_prev_visible_ex < 'a > (&'a mut self,) -> ExGetPrevVisible < 'a > {
            ExGetPrevVisible::new(self,)
        }
        pub fn get_child(&mut self, index: i32,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_child", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_child_count(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_child_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_children(&mut self,) -> Array < Gd < crate::classes::TreeItem > > {
            type CallSig = (Array < Gd < crate::classes::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_children", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_index(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "get_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_before(&mut self, item: impl AsObjectArg < crate::classes::TreeItem >,) {
            type CallSig = ((), ObjectArg < crate::classes::TreeItem >);
            let args = (item.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "move_before", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn move_after(&mut self, item: impl AsObjectArg < crate::classes::TreeItem >,) {
            type CallSig = ((), ObjectArg < crate::classes::TreeItem >);
            let args = (item.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TreeItem", "move_after", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_recursive(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) {
            Self::try_call_recursive(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_recursive(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < (), crate::meta::error::CallError > {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9631usize);
                < CallSig as VarcallSignatureTuple > ::out_class_varcall(method_bind, "TreeItem", "call_recursive", self.object_ptr, self.__checked_id(), args, varargs)
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
    impl crate::obj::GodotClass for TreeItem {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TreeItem"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TreeItem {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TreeItem {
        
    }
    impl std::ops::Deref for TreeItem {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TreeItem {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TreeItem`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TreeItem {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TreeItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TreeItem::propagate_check_ex`][super::TreeItem::propagate_check_ex]."]
#[must_use]
pub struct ExPropagateCheck < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, column: i32, emit_signal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPropagateCheck < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32,) -> Self {
        let emit_signal = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, emit_signal: emit_signal,
        }
    }
    #[inline]
    pub fn emit_signal(self, emit_signal: bool) -> Self {
        Self {
            emit_signal: emit_signal, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, emit_signal,
        }
        = self;
        re_export::TreeItem::propagate_check_full(surround_object, column, emit_signal,)
    }
}
#[doc = "Default-param extender for [`TreeItem::set_range_config_ex`][super::TreeItem::set_range_config_ex]."]
#[must_use]
pub struct ExSetRangeConfig < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, column: i32, min: f64, max: f64, step: f64, expr: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetRangeConfig < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32, min: f64, max: f64, step: f64,) -> Self {
        let expr = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, min: min, max: max, step: step, expr: expr,
        }
    }
    #[inline]
    pub fn expr(self, expr: bool) -> Self {
        Self {
            expr: expr, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, min, max, step, expr,
        }
        = self;
        re_export::TreeItem::set_range_config_full(surround_object, column, min, max, step, expr,)
    }
}
#[doc = "Default-param extender for [`TreeItem::is_any_collapsed_ex`][super::TreeItem::is_any_collapsed_ex]."]
#[must_use]
pub struct ExIsAnyCollapsed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, only_visible: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsAnyCollapsed < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let only_visible = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, only_visible: only_visible,
        }
    }
    #[inline]
    pub fn only_visible(self, only_visible: bool) -> Self {
        Self {
            only_visible: only_visible, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, only_visible,
        }
        = self;
        re_export::TreeItem::is_any_collapsed_full(surround_object, only_visible,)
    }
}
#[doc = "Default-param extender for [`TreeItem::set_custom_bg_color_ex`][super::TreeItem::set_custom_bg_color_ex]."]
#[must_use]
pub struct ExSetCustomBgColor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, column: i32, color: Color, just_outline: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCustomBgColor < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32, color: Color,) -> Self {
        let just_outline = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, color: color, just_outline: just_outline,
        }
    }
    #[inline]
    pub fn just_outline(self, just_outline: bool) -> Self {
        Self {
            just_outline: just_outline, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, color, just_outline,
        }
        = self;
        re_export::TreeItem::set_custom_bg_color_full(surround_object, column, color, just_outline,)
    }
}
#[doc = "Default-param extender for [`TreeItem::add_button_ex`][super::TreeItem::add_button_ex]."]
#[must_use]
pub struct ExAddButton < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, column: i32, button: ObjectCow < crate::classes::Texture2D >, id: i32, disabled: bool, tooltip_text: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddButton < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32, button: impl AsObjectArg < crate::classes::Texture2D >,) -> Self {
        let id = - 1i32;
        let disabled = false;
        let tooltip_text = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, button: button.consume_arg(), id: id, disabled: disabled, tooltip_text: CowArg::Owned(tooltip_text),
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled: disabled, .. self
        }
    }
    #[inline]
    pub fn tooltip_text(self, tooltip_text: impl AsArg < GString > + 'a) -> Self {
        Self {
            tooltip_text: tooltip_text.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, button, id, disabled, tooltip_text,
        }
        = self;
        re_export::TreeItem::add_button_full(surround_object, column, button.cow_as_object_arg(), id, disabled, tooltip_text,)
    }
}
#[doc = "Default-param extender for [`TreeItem::create_child_ex`][super::TreeItem::create_child_ex]."]
#[must_use]
pub struct ExCreateChild < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateChild < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::TreeItem::create_child_full(surround_object, index,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_next_in_tree_ex`][super::TreeItem::get_next_in_tree_ex]."]
#[must_use]
pub struct ExGetNextInTree < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetNextInTree < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_next_in_tree_full(surround_object, wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_prev_in_tree_ex`][super::TreeItem::get_prev_in_tree_ex]."]
#[must_use]
pub struct ExGetPrevInTree < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPrevInTree < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_prev_in_tree_full(surround_object, wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_next_visible_ex`][super::TreeItem::get_next_visible_ex]."]
#[must_use]
pub struct ExGetNextVisible < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetNextVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_next_visible_full(surround_object, wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_prev_visible_ex`][super::TreeItem::get_prev_visible_ex]."]
#[must_use]
pub struct ExGetPrevVisible < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPrevVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_prev_visible_full(surround_object, wrap,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TreeCellMode {
    ord: i32
}
impl TreeCellMode {
    #[doc(alias = "CELL_MODE_STRING")]
    #[doc = "Godot enumerator name: `CELL_MODE_STRING`"]
    pub const STRING: TreeCellMode = TreeCellMode {
        ord: 0i32
    };
    #[doc(alias = "CELL_MODE_CHECK")]
    #[doc = "Godot enumerator name: `CELL_MODE_CHECK`"]
    pub const CHECK: TreeCellMode = TreeCellMode {
        ord: 1i32
    };
    #[doc(alias = "CELL_MODE_RANGE")]
    #[doc = "Godot enumerator name: `CELL_MODE_RANGE`"]
    pub const RANGE: TreeCellMode = TreeCellMode {
        ord: 2i32
    };
    #[doc(alias = "CELL_MODE_ICON")]
    #[doc = "Godot enumerator name: `CELL_MODE_ICON`"]
    pub const ICON: TreeCellMode = TreeCellMode {
        ord: 3i32
    };
    #[doc(alias = "CELL_MODE_CUSTOM")]
    #[doc = "Godot enumerator name: `CELL_MODE_CUSTOM`"]
    pub const CUSTOM: TreeCellMode = TreeCellMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TreeCellMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TreeCellMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TreeCellMode {
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
            Self::STRING => "STRING", Self::CHECK => "CHECK", Self::RANGE => "RANGE", Self::ICON => "ICON", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STRING => "CELL_MODE_STRING", Self::CHECK => "CELL_MODE_CHECK", Self::RANGE => "CELL_MODE_RANGE", Self::ICON => "CELL_MODE_ICON", Self::CUSTOM => "CELL_MODE_CUSTOM", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TreeCellMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TreeCellMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TreeCellMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}