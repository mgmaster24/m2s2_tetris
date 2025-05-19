#![doc = "Sidecar module for class [`ColorPicker`][crate::classes::ColorPicker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ColorPicker` enums](https://docs.godotengine.org/en/stable/classes/class_colorpicker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ColorPicker.`\n\nInherits [`VBoxContainer`][crate::classes::VBoxContainer].\n\nRelated symbols:\n\n* [`color_picker`][crate::classes::color_picker]: sidecar module with related enum/flag types\n* [`IColorPicker`][crate::classes::IColorPicker]: virtual methods\n\n\nSee also [Godot docs for `ColorPicker`](https://docs.godotengine.org/en/stable/classes/class_colorpicker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ColorPicker::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ColorPicker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ColorPicker`][crate::classes::ColorPicker].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ColorPicker` methods](https://docs.godotengine.org/en/stable/classes/class_colorpicker.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IColorPicker: crate::obj::GodotClass < Base = ColorPicker > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ColorPicker {
        pub fn set_pick_color(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_pick_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pick_color(&self,) -> Color {
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "get_pick_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deferred_mode(&mut self, mode: bool,) {
            type CallSig = ((), bool);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_deferred_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_deferred_mode(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "is_deferred_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_mode(&mut self, color_mode: crate::classes::color_picker::ColorModeType,) {
            type CallSig = ((), crate::classes::color_picker::ColorModeType);
            let args = (color_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_color_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_mode(&self,) -> crate::classes::color_picker::ColorModeType {
            type CallSig = (crate::classes::color_picker::ColorModeType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "get_color_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edit_alpha(&mut self, show: bool,) {
            type CallSig = ((), bool);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_edit_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_editing_alpha(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "is_editing_alpha", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_can_add_swatches(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_can_add_swatches", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_swatches_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "are_swatches_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_presets_visible(&mut self, visible: bool,) {
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_presets_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_presets_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2230usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "are_presets_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_modes_visible(&mut self, visible: bool,) {
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2231usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_modes_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_modes_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2232usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "are_modes_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sampler_visible(&mut self, visible: bool,) {
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2233usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_sampler_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sampler_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2234usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "is_sampler_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sliders_visible(&mut self, visible: bool,) {
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2235usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_sliders_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn are_sliders_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2236usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "are_sliders_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hex_visible(&mut self, visible: bool,) {
            type CallSig = ((), bool);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2237usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_hex_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hex_visible(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2238usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "is_hex_visible", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_preset(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "add_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_preset(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "erase_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_presets(&self,) -> PackedColorArray {
            type CallSig = (PackedColorArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "get_presets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_recent_preset(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "add_recent_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_recent_preset(&mut self, color: Color,) {
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "erase_recent_preset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_recent_presets(&self,) -> PackedColorArray {
            type CallSig = (PackedColorArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "get_recent_presets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_picker_shape(&mut self, shape: crate::classes::color_picker::PickerShapeType,) {
            type CallSig = ((), crate::classes::color_picker::PickerShapeType);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "set_picker_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_picker_shape(&self,) -> crate::classes::color_picker::PickerShapeType {
            type CallSig = (crate::classes::color_picker::PickerShapeType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ColorPicker", "get_picker_shape", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ColorPicker {
        type Base = crate::classes::VBoxContainer;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ColorPicker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ColorPicker {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VBoxContainer > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BoxContainer > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ColorPicker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ColorPicker {
        
    }
    impl crate::obj::cap::GodotDefault for ColorPicker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ColorPicker {
        type Target = crate::classes::VBoxContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ColorPicker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ColorPicker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ColorPicker {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ColorPicker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VBoxContainer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BoxContainer > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ColorModeType {
    ord: i32
}
impl ColorModeType {
    #[doc(alias = "MODE_RGB")]
    #[doc = "Godot enumerator name: `MODE_RGB`"]
    pub const RGB: ColorModeType = ColorModeType {
        ord: 0i32
    };
    #[doc(alias = "MODE_HSV")]
    #[doc = "Godot enumerator name: `MODE_HSV`"]
    pub const HSV: ColorModeType = ColorModeType {
        ord: 1i32
    };
    #[doc(alias = "MODE_RAW")]
    #[doc = "Godot enumerator name: `MODE_RAW`"]
    pub const RAW: ColorModeType = ColorModeType {
        ord: 2i32
    };
    #[doc(alias = "MODE_OKHSL")]
    #[doc = "Godot enumerator name: `MODE_OKHSL`"]
    pub const OKHSL: ColorModeType = ColorModeType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ColorModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ColorModeType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ColorModeType {
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
            Self::RGB => "RGB", Self::HSV => "HSV", Self::RAW => "RAW", Self::OKHSL => "OKHSL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::RGB => "MODE_RGB", Self::HSV => "MODE_HSV", Self::RAW => "MODE_RAW", Self::OKHSL => "MODE_OKHSL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ColorModeType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ColorModeType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ColorModeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PickerShapeType {
    ord: i32
}
impl PickerShapeType {
    #[doc(alias = "SHAPE_HSV_RECTANGLE")]
    #[doc = "Godot enumerator name: `SHAPE_HSV_RECTANGLE`"]
    pub const HSV_RECTANGLE: PickerShapeType = PickerShapeType {
        ord: 0i32
    };
    #[doc(alias = "SHAPE_HSV_WHEEL")]
    #[doc = "Godot enumerator name: `SHAPE_HSV_WHEEL`"]
    pub const HSV_WHEEL: PickerShapeType = PickerShapeType {
        ord: 1i32
    };
    #[doc(alias = "SHAPE_VHS_CIRCLE")]
    #[doc = "Godot enumerator name: `SHAPE_VHS_CIRCLE`"]
    pub const VHS_CIRCLE: PickerShapeType = PickerShapeType {
        ord: 2i32
    };
    #[doc(alias = "SHAPE_OKHSL_CIRCLE")]
    #[doc = "Godot enumerator name: `SHAPE_OKHSL_CIRCLE`"]
    pub const OKHSL_CIRCLE: PickerShapeType = PickerShapeType {
        ord: 3i32
    };
    #[doc(alias = "SHAPE_NONE")]
    #[doc = "Godot enumerator name: `SHAPE_NONE`"]
    pub const NONE: PickerShapeType = PickerShapeType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for PickerShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PickerShapeType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PickerShapeType {
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
            Self::HSV_RECTANGLE => "HSV_RECTANGLE", Self::HSV_WHEEL => "HSV_WHEEL", Self::VHS_CIRCLE => "VHS_CIRCLE", Self::OKHSL_CIRCLE => "OKHSL_CIRCLE", Self::NONE => "NONE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::HSV_RECTANGLE => "SHAPE_HSV_RECTANGLE", Self::HSV_WHEEL => "SHAPE_HSV_WHEEL", Self::VHS_CIRCLE => "SHAPE_VHS_CIRCLE", Self::OKHSL_CIRCLE => "SHAPE_OKHSL_CIRCLE", Self::NONE => "SHAPE_NONE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PickerShapeType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PickerShapeType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PickerShapeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}