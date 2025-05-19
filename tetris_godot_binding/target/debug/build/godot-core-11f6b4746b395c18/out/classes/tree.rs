#![doc = "Sidecar module for class [`Tree`][crate::classes::Tree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Tree` enums](https://docs.godotengine.org/en/stable/classes/class_tree.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Tree.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`tree`][crate::classes::tree]: sidecar module with related enum/flag types\n* [`ITree`][crate::classes::ITree]: virtual methods\n\n\nSee also [Godot docs for `Tree`](https://docs.godotengine.org/en/stable/classes/class_tree.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Tree::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Tree {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Tree`][crate::classes::Tree].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Tree` methods](https://docs.godotengine.org/en/stable/classes/class_tree.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITree: crate::obj::GodotClass < Base = Tree > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Tree {
        pub fn clear(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_item_full(&mut self, parent: ObjectArg < crate::classes::TreeItem >, index: i32,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >, ObjectArg < crate::classes::TreeItem >, i32);
            let args = (parent, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "create_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_item(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.create_item_ex() . done()
        }
        #[inline]
        pub fn create_item_ex < 'a > (&'a mut self,) -> ExCreateItem < 'a > {
            ExCreateItem::new(self,)
        }
        pub fn get_root(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_custom_minimum_width(&mut self, column: i32, min_width: i32,) {
            type CallSig = ((), i32, i32);
            let args = (column, min_width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_column_custom_minimum_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_expand(&mut self, column: i32, expand: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, expand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9464usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_column_expand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_expand_ratio(&mut self, column: i32, ratio: i32,) {
            type CallSig = ((), i32, i32);
            let args = (column, ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9465usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_column_expand_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_clip_content(&mut self, column: i32, enable: bool,) {
            type CallSig = ((), i32, bool);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9466usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_column_clip_content", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_column_expanding(&self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9467usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "is_column_expanding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_column_clipping_content(&self, column: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9468usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "is_column_clipping_content", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_expand_ratio(&self, column: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9469usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_column_expand_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_width(&self, column: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9470usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_column_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hide_root(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9471usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_hide_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_root_hidden(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "is_root_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next_selected(&mut self, from: impl AsObjectArg < crate::classes::TreeItem >,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >, ObjectArg < crate::classes::TreeItem >);
            let args = (from.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_next_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_selected(&mut self, item: impl AsObjectArg < crate::classes::TreeItem >, column: i32,) {
            type CallSig = ((), ObjectArg < crate::classes::TreeItem >, i32);
            let args = (item.as_object_arg(), column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_selected_column(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_selected_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pressed_button(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_pressed_button", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_select_mode(&mut self, mode: crate::classes::tree::SelectMode,) {
            type CallSig = ((), crate::classes::tree::SelectMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_select_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_select_mode(&self,) -> crate::classes::tree::SelectMode {
            type CallSig = (crate::classes::tree::SelectMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_select_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deselect_all(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "deselect_all", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_columns(&mut self, amount: i32,) {
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_columns(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_columns", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_edited", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_column(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_edited_column", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn edit_selected_full(&mut self, force_edit: bool,) -> bool {
            type CallSig = (bool, bool);
            let args = (force_edit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "edit_selected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::edit_selected_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn edit_selected(&mut self,) -> bool {
            self.edit_selected_ex() . done()
        }
        #[inline]
        pub fn edit_selected_ex < 'a > (&'a mut self,) -> ExEditSelected < 'a > {
            ExEditSelected::new(self,)
        }
        pub fn get_custom_popup_rect(&self,) -> Rect2 {
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_custom_popup_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_item_area_rect_full(&self, item: ObjectArg < crate::classes::TreeItem >, column: i32, button_index: i32,) -> Rect2 {
            type CallSig = (Rect2, ObjectArg < crate::classes::TreeItem >, i32, i32);
            let args = (item, column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_item_area_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_item_area_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_item_area_rect(&self, item: impl AsObjectArg < crate::classes::TreeItem >,) -> Rect2 {
            self.get_item_area_rect_ex(item,) . done()
        }
        #[inline]
        pub fn get_item_area_rect_ex < 'a > (&'a self, item: impl AsObjectArg < crate::classes::TreeItem >,) -> ExGetItemAreaRect < 'a > {
            ExGetItemAreaRect::new(self, item,)
        }
        pub fn get_item_at_position(&self, position: Vector2,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallSig = (Option < Gd < crate::classes::TreeItem > >, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_item_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_at_position(&self, position: Vector2,) -> i32 {
            type CallSig = (i32, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_column_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drop_section_at_position(&self, position: Vector2,) -> i32 {
            type CallSig = (i32, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_drop_section_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_button_id_at_position(&self, position: Vector2,) -> i32 {
            type CallSig = (i32, Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_button_id_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ensure_cursor_is_visible(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "ensure_cursor_is_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_titles_visible(&mut self, visible: bool,) {
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_column_titles_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_column_titles_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "are_column_titles_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_title(&mut self, column: i32, title: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (column, title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_column_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_title(&self, column: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_column_title", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_title_alignment(&mut self, column: i32, title_alignment: crate::global::HorizontalAlignment,) {
            type CallSig = ((), i32, crate::global::HorizontalAlignment);
            let args = (column, title_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_title_alignment(&self, column: i32,) -> crate::global::HorizontalAlignment {
            type CallSig = (crate::global::HorizontalAlignment, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_alignment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_title_direction(&mut self, column: i32, direction: crate::classes::control::TextDirection,) {
            type CallSig = ((), i32, crate::classes::control::TextDirection);
            let args = (column, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_title_direction(&self, column: i32,) -> crate::classes::control::TextDirection {
            type CallSig = (crate::classes::control::TextDirection, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_column_title_language(&mut self, column: i32, language: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), i32, CowArg < 'a0, GString >);
            let args = (column, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_column_title_language(&self, column: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_language", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scroll(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_scroll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn scroll_to_item_full(&mut self, item: ObjectArg < crate::classes::TreeItem >, center_on_item: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::TreeItem >, bool);
            let args = (item, center_on_item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "scroll_to_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::scroll_to_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn scroll_to_item(&mut self, item: impl AsObjectArg < crate::classes::TreeItem >,) {
            self.scroll_to_item_ex(item,) . done()
        }
        #[inline]
        pub fn scroll_to_item_ex < 'a > (&'a mut self, item: impl AsObjectArg < crate::classes::TreeItem >,) -> ExScrollToItem < 'a > {
            ExScrollToItem::new(self, item,)
        }
        pub fn set_h_scroll_enabled(&mut self, h_scroll: bool,) {
            type CallSig = ((), bool);
            let args = (h_scroll,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_h_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_h_scroll_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "is_h_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_v_scroll_enabled(&mut self, h_scroll: bool,) {
            type CallSig = ((), bool);
            let args = (h_scroll,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_v_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_v_scroll_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9508usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "is_v_scroll_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hide_folding(&mut self, hide: bool,) {
            type CallSig = ((), bool);
            let args = (hide,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9509usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_hide_folding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_folding_hidden(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9510usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "is_folding_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_recursive_folding(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9511usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_enable_recursive_folding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_recursive_folding_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9512usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "is_recursive_folding_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_drop_mode_flags(&mut self, flags: i32,) {
            type CallSig = ((), i32);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9513usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_drop_mode_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_drop_mode_flags(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9514usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_drop_mode_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_rmb_select(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9515usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_allow_rmb_select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_rmb_select(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9516usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_allow_rmb_select", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9517usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9518usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_allow_reselect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type CallSig = ((), bool);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9519usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "set_allow_search", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9520usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Tree", "get_allow_search", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Tree {
        type Base = crate::classes::Control;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Tree"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Tree {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Tree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Tree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Tree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Tree {
        
    }
    impl crate::obj::cap::GodotDefault for Tree {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Tree {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Tree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Tree`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Tree {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Tree > for $Class {
                
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
#[doc = "Default-param extender for [`Tree::create_item_ex`][super::Tree::create_item_ex]."]
#[must_use]
pub struct ExCreateItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tree, parent: ObjectCow < crate::classes::TreeItem >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateItem < 'a > {
    fn new(surround_object: &'a mut re_export::Tree,) -> Self {
        let parent = Gd::null_arg();
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parent: parent.consume_arg(), index: index,
        }
    }
    #[inline]
    pub fn parent(self, parent: impl AsObjectArg < crate::classes::TreeItem >) -> Self {
        Self {
            parent: parent.consume_arg(), .. self
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
            _phantom, surround_object, parent, index,
        }
        = self;
        re_export::Tree::create_item_full(surround_object, parent.cow_as_object_arg(), index,)
    }
}
#[doc = "Default-param extender for [`Tree::edit_selected_ex`][super::Tree::edit_selected_ex]."]
#[must_use]
pub struct ExEditSelected < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tree, force_edit: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEditSelected < 'a > {
    fn new(surround_object: &'a mut re_export::Tree,) -> Self {
        let force_edit = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force_edit: force_edit,
        }
    }
    #[inline]
    pub fn force_edit(self, force_edit: bool) -> Self {
        Self {
            force_edit: force_edit, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, force_edit,
        }
        = self;
        re_export::Tree::edit_selected_full(surround_object, force_edit,)
    }
}
#[doc = "Default-param extender for [`Tree::get_item_area_rect_ex`][super::Tree::get_item_area_rect_ex]."]
#[must_use]
pub struct ExGetItemAreaRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Tree, item: ObjectCow < crate::classes::TreeItem >, column: i32, button_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetItemAreaRect < 'a > {
    fn new(surround_object: &'a re_export::Tree, item: impl AsObjectArg < crate::classes::TreeItem >,) -> Self {
        let column = - 1i32;
        let button_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item.consume_arg(), column: column, button_index: button_index,
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn button_index(self, button_index: i32) -> Self {
        Self {
            button_index: button_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2 {
        let Self {
            _phantom, surround_object, item, column, button_index,
        }
        = self;
        re_export::Tree::get_item_area_rect_full(surround_object, item.cow_as_object_arg(), column, button_index,)
    }
}
#[doc = "Default-param extender for [`Tree::scroll_to_item_ex`][super::Tree::scroll_to_item_ex]."]
#[must_use]
pub struct ExScrollToItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tree, item: ObjectCow < crate::classes::TreeItem >, center_on_item: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScrollToItem < 'a > {
    fn new(surround_object: &'a mut re_export::Tree, item: impl AsObjectArg < crate::classes::TreeItem >,) -> Self {
        let center_on_item = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item.consume_arg(), center_on_item: center_on_item,
        }
    }
    #[inline]
    pub fn center_on_item(self, center_on_item: bool) -> Self {
        Self {
            center_on_item: center_on_item, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, center_on_item,
        }
        = self;
        re_export::Tree::scroll_to_item_full(surround_object, item.cow_as_object_arg(), center_on_item,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SelectMode {
    ord: i32
}
impl SelectMode {
    #[doc(alias = "SELECT_SINGLE")]
    #[doc = "Godot enumerator name: `SELECT_SINGLE`"]
    pub const SINGLE: SelectMode = SelectMode {
        ord: 0i32
    };
    #[doc(alias = "SELECT_ROW")]
    #[doc = "Godot enumerator name: `SELECT_ROW`"]
    pub const ROW: SelectMode = SelectMode {
        ord: 1i32
    };
    #[doc(alias = "SELECT_MULTI")]
    #[doc = "Godot enumerator name: `SELECT_MULTI`"]
    pub const MULTI: SelectMode = SelectMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SelectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SelectMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SelectMode {
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
            Self::SINGLE => "SINGLE", Self::ROW => "ROW", Self::MULTI => "MULTI", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SINGLE => "SELECT_SINGLE", Self::ROW => "SELECT_ROW", Self::MULTI => "SELECT_MULTI", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SelectMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SelectMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SelectMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DropModeFlags {
    ord: i32
}
impl DropModeFlags {
    #[doc(alias = "DROP_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `DROP_MODE_DISABLED`"]
    pub const DISABLED: DropModeFlags = DropModeFlags {
        ord: 0i32
    };
    #[doc(alias = "DROP_MODE_ON_ITEM")]
    #[doc = "Godot enumerator name: `DROP_MODE_ON_ITEM`"]
    pub const ON_ITEM: DropModeFlags = DropModeFlags {
        ord: 1i32
    };
    #[doc(alias = "DROP_MODE_INBETWEEN")]
    #[doc = "Godot enumerator name: `DROP_MODE_INBETWEEN`"]
    pub const INBETWEEN: DropModeFlags = DropModeFlags {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DropModeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DropModeFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DropModeFlags {
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
            Self::DISABLED => "DISABLED", Self::ON_ITEM => "ON_ITEM", Self::INBETWEEN => "INBETWEEN", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DROP_MODE_DISABLED", Self::ON_ITEM => "DROP_MODE_ON_ITEM", Self::INBETWEEN => "DROP_MODE_INBETWEEN", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DropModeFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DropModeFlags {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DropModeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}