#![doc = "Sidecar module for class [`EditorProperty`][crate::classes::EditorProperty].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorProperty` enums](https://docs.godotengine.org/en/stable/classes/class_editorproperty.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorProperty.`\n\nInherits [`Container`][crate::classes::Container].\n\nRelated symbols:\n\n* [`editor_property`][crate::classes::editor_property]: sidecar module with related enum/flag types\n* [`IEditorProperty`][crate::classes::IEditorProperty]: virtual methods\n\n\nSee also [Godot docs for `EditorProperty`](https://docs.godotengine.org/en/stable/classes/class_editorproperty.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorProperty::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorProperty {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorProperty`][crate::classes::EditorProperty].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorProperty` methods](https://docs.godotengine.org/en/stable/classes/class_editorproperty.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorProperty: crate::obj::GodotClass < Base = EditorProperty > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ContainerNotification) {
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
        fn update_property(&mut self,) {
            unimplemented !()
        }
        fn set_read_only(&mut self, read_only: bool,) {
            unimplemented !()
        }
        fn get_allowed_size_flags_horizontal(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_allowed_size_flags_vertical(&self,) -> PackedInt32Array {
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
    impl EditorProperty {
        pub fn set_label(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "set_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_label(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "get_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_read_only(&mut self, read_only: bool,) {
            type CallSig = ((), bool);
            let args = (read_only,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "set_read_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_read_only(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "is_read_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_checkable(&mut self, checkable: bool,) {
            type CallSig = ((), bool);
            let args = (checkable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "set_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_checkable(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "is_checkable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_checked(&mut self, checked: bool,) {
            type CallSig = ((), bool);
            let args = (checked,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "set_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_checked(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "is_checked", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_draw_warning(&mut self, draw_warning: bool,) {
            type CallSig = ((), bool);
            let args = (draw_warning,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "set_draw_warning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_draw_warning(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "is_draw_warning", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_keying(&mut self, keying: bool,) {
            type CallSig = ((), bool);
            let args = (keying,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "set_keying", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_keying(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "is_keying", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deletable(&mut self, deletable: bool,) {
            type CallSig = ((), bool);
            let args = (deletable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "set_deletable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_deletable(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "is_deletable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_property(&self,) -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "get_edited_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edited_object(&mut self,) -> Option < Gd < crate::classes::Object > > {
            type CallSig = (Option < Gd < crate::classes::Object > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "get_edited_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_property(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "update_property", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_focusable(&mut self, control: impl AsObjectArg < crate::classes::Control >,) {
            type CallSig = ((), ObjectArg < crate::classes::Control >);
            let args = (control.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "add_focusable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bottom_editor(&mut self, editor: impl AsObjectArg < crate::classes::Control >,) {
            type CallSig = ((), ObjectArg < crate::classes::Control >);
            let args = (editor.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "set_bottom_editor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn emit_changed_full(&mut self, property: CowArg < StringName >, value: RefArg < Variant >, field: CowArg < StringName >, changing: bool,) {
            type CallSig < 'a0, 'a1, 'a2, > = ((), CowArg < 'a0, StringName >, RefArg < 'a1, Variant >, CowArg < 'a2, StringName >, bool);
            let args = (property, value, field, changing,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "EditorProperty", "emit_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::emit_changed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn emit_changed(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            self.emit_changed_ex(property, value,) . done()
        }
        #[inline]
        pub fn emit_changed_ex < 'a > (&'a mut self, property: impl AsArg < StringName > + 'a, value: &'a Variant,) -> ExEmitChanged < 'a > {
            ExEmitChanged::new(self, property, value,)
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
    impl crate::obj::GodotClass for EditorProperty {
        type Base = crate::classes::Container;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"EditorProperty"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorProperty {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for EditorProperty {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for EditorProperty {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for EditorProperty {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorProperty {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorProperty {
        
    }
    impl crate::obj::cap::GodotDefault for EditorProperty {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorProperty {
        type Target = crate::classes::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorProperty {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorProperty`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_EditorProperty {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorProperty > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Container > for $Class {
                
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
#[doc = "Default-param extender for [`EditorProperty::emit_changed_ex`][super::EditorProperty::emit_changed_ex]."]
#[must_use]
pub struct ExEmitChanged < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorProperty, property: CowArg < 'a, StringName >, value: CowArg < 'a, Variant >, field: CowArg < 'a, StringName >, changing: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEmitChanged < 'a > {
    fn new(surround_object: &'a mut re_export::EditorProperty, property: impl AsArg < StringName > + 'a, value: &'a Variant,) -> Self {
        let field = StringName::from("");
        let changing = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, property: property.into_arg(), value: CowArg::Borrowed(value), field: CowArg::Owned(field), changing: changing,
        }
    }
    #[inline]
    pub fn field(self, field: impl AsArg < StringName > + 'a) -> Self {
        Self {
            field: field.into_arg(), .. self
        }
    }
    #[inline]
    pub fn changing(self, changing: bool) -> Self {
        Self {
            changing: changing, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, property, value, field, changing,
        }
        = self;
        re_export::EditorProperty::emit_changed_full(surround_object, property, value.cow_as_arg(), field, changing,)
    }
}