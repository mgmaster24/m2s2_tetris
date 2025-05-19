#![doc = "Sidecar module for class [`Viewport`][crate::classes::Viewport].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Viewport` enums](https://docs.godotengine.org/en/stable/classes/class_viewport.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Viewport.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`viewport`][crate::classes::viewport]: sidecar module with related enum/flag types\n* [`IViewport`][crate::classes::IViewport]: virtual methods\n\n\nSee also [Godot docs for `Viewport`](https://docs.godotengine.org/en/stable/classes/class_viewport.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Viewport>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Viewport {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Viewport`][crate::classes::Viewport].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Viewport` methods](https://docs.godotengine.org/en/stable/classes/class_viewport.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IViewport: crate::obj::GodotClass < Base = Viewport > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Viewport {
        pub fn set_world_2d(&mut self, world_2d: impl AsObjectArg < crate::classes::World2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::World2D >);
            let args = (world_2d.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9807usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_world_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_world_2d(&self,) -> Option < Gd < crate::classes::World2D > > {
            type CallSig = (Option < Gd < crate::classes::World2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9808usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_world_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_world_2d(&self,) -> Option < Gd < crate::classes::World2D > > {
            type CallSig = (Option < Gd < crate::classes::World2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9809usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "find_world_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_canvas_transform(&mut self, xform: Transform2D,) {
            type CallSig = ((), Transform2D);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9810usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_canvas_transform(&self,) -> Transform2D {
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9811usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_canvas_transform(&mut self, xform: Transform2D,) {
            type CallSig = ((), Transform2D);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9812usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_global_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_canvas_transform(&self,) -> Transform2D {
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9813usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_global_canvas_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_final_transform(&self,) -> Transform2D {
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9814usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_final_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_transform(&self,) -> Transform2D {
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9815usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_screen_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visible_rect(&self,) -> Rect2 {
            type CallSig = (Rect2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9816usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_visible_rect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transparent_background(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9817usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_transparent_background", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_transparent_background(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9818usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "has_transparent_background", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_hdr_2d(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9819usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_use_hdr_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_hdr_2d(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9820usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_using_hdr_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msaa_2d(&mut self, msaa: crate::classes::viewport::Msaa,) {
            type CallSig = ((), crate::classes::viewport::Msaa);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9821usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_msaa_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msaa_2d(&self,) -> crate::classes::viewport::Msaa {
            type CallSig = (crate::classes::viewport::Msaa,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9822usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_msaa_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_msaa_3d(&mut self, msaa: crate::classes::viewport::Msaa,) {
            type CallSig = ((), crate::classes::viewport::Msaa);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9823usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_msaa_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msaa_3d(&self,) -> crate::classes::viewport::Msaa {
            type CallSig = (crate::classes::viewport::Msaa,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9824usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_msaa_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_screen_space_aa(&mut self, screen_space_aa: crate::classes::viewport::ScreenSpaceAa,) {
            type CallSig = ((), crate::classes::viewport::ScreenSpaceAa);
            let args = (screen_space_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9825usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_screen_space_aa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_space_aa(&self,) -> crate::classes::viewport::ScreenSpaceAa {
            type CallSig = (crate::classes::viewport::ScreenSpaceAa,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9826usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_screen_space_aa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_taa(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_use_taa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_taa(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_using_taa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_debanding(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_use_debanding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_debanding(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_using_debanding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_occlusion_culling(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_use_occlusion_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_occlusion_culling(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_using_occlusion_culling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_debug_draw(&mut self, debug_draw: crate::classes::viewport::DebugDraw,) {
            type CallSig = ((), crate::classes::viewport::DebugDraw);
            let args = (debug_draw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_debug_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_debug_draw(&self,) -> crate::classes::viewport::DebugDraw {
            type CallSig = (crate::classes::viewport::DebugDraw,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_debug_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_info(&mut self, type_: crate::classes::viewport::RenderInfoType, info: crate::classes::viewport::RenderInfo,) -> i32 {
            type CallSig = (i32, crate::classes::viewport::RenderInfoType, crate::classes::viewport::RenderInfo);
            let args = (type_, info,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_render_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::ViewportTexture > > {
            type CallSig = (Option < Gd < crate::classes::ViewportTexture > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_object_picking(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_physics_object_picking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_object_picking(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_physics_object_picking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_object_picking_sort(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_physics_object_picking_sort", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_object_picking_sort(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_physics_object_picking_sort", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_object_picking_first_only(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_physics_object_picking_first_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_object_picking_first_only(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_physics_object_picking_first_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_viewport_rid(&self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_viewport_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn push_text_input(&mut self, text: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "push_text_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn push_input_full(&mut self, event: ObjectArg < crate::classes::InputEvent >, in_local_coords: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::InputEvent >, bool);
            let args = (event, in_local_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "push_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_input_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_input(&mut self, event: impl AsObjectArg < crate::classes::InputEvent >,) {
            self.push_input_ex(event,) . done()
        }
        #[inline]
        pub fn push_input_ex < 'a > (&'a mut self, event: impl AsObjectArg < crate::classes::InputEvent >,) -> ExPushInput < 'a > {
            ExPushInput::new(self, event,)
        }
        pub(crate) fn push_unhandled_input_full(&mut self, event: ObjectArg < crate::classes::InputEvent >, in_local_coords: bool,) {
            type CallSig = ((), ObjectArg < crate::classes::InputEvent >, bool);
            let args = (event, in_local_coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "push_unhandled_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_unhandled_input_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_unhandled_input(&mut self, event: impl AsObjectArg < crate::classes::InputEvent >,) {
            self.push_unhandled_input_ex(event,) . done()
        }
        #[inline]
        pub fn push_unhandled_input_ex < 'a > (&'a mut self, event: impl AsObjectArg < crate::classes::InputEvent >,) -> ExPushUnhandledInput < 'a > {
            ExPushUnhandledInput::new(self, event,)
        }
        pub fn get_mouse_position(&self,) -> Vector2 {
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_mouse_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2,) {
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "warp_mouse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn update_mouse_cursor_state(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "update_mouse_cursor_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_get_drag_data(&self,) -> Variant {
            type CallSig = (Variant,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "gui_get_drag_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_is_dragging(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "gui_is_dragging", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_is_drag_successful(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "gui_is_drag_successful", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_release_focus(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "gui_release_focus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_get_focus_owner(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallSig = (Option < Gd < crate::classes::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "gui_get_focus_owner", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn gui_get_hovered_control(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallSig = (Option < Gd < crate::classes::Control > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "gui_get_hovered_control", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_input(&mut self, disable: bool,) {
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_disable_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_input_disabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_input_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_positional_shadow_atlas_size(&mut self, size: i32,) {
            type CallSig = ((), i32);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_positional_shadow_atlas_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_positional_shadow_atlas_size(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_positional_shadow_atlas_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_positional_shadow_atlas_16_bits(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_positional_shadow_atlas_16_bits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_positional_shadow_atlas_16_bits(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_positional_shadow_atlas_16_bits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_snap_controls_to_pixels(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_snap_controls_to_pixels", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_snap_controls_to_pixels_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_snap_controls_to_pixels_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_snap_2d_transforms_to_pixel(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_snap_2d_transforms_to_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_snap_2d_transforms_to_pixel_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_snap_2d_transforms_to_pixel_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_snap_2d_vertices_to_pixel(&mut self, enabled: bool,) {
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_snap_2d_vertices_to_pixel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_snap_2d_vertices_to_pixel_enabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_snap_2d_vertices_to_pixel_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_positional_shadow_atlas_quadrant_subdiv(&mut self, quadrant: i32, subdiv: crate::classes::viewport::PositionalShadowAtlasQuadrantSubdiv,) {
            type CallSig = ((), i32, crate::classes::viewport::PositionalShadowAtlasQuadrantSubdiv);
            let args = (quadrant, subdiv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_positional_shadow_atlas_quadrant_subdiv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_positional_shadow_atlas_quadrant_subdiv(&self, quadrant: i32,) -> crate::classes::viewport::PositionalShadowAtlasQuadrantSubdiv {
            type CallSig = (crate::classes::viewport::PositionalShadowAtlasQuadrantSubdiv, i32);
            let args = (quadrant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_positional_shadow_atlas_quadrant_subdiv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input_as_handled(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_input_as_handled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_input_handled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_input_handled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_handle_input_locally(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_handle_input_locally", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_handling_input_locally(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_handling_input_locally", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_canvas_item_texture_filter(&mut self, mode: crate::classes::viewport::DefaultCanvasItemTextureFilter,) {
            type CallSig = ((), crate::classes::viewport::DefaultCanvasItemTextureFilter);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_default_canvas_item_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_canvas_item_texture_filter(&self,) -> crate::classes::viewport::DefaultCanvasItemTextureFilter {
            type CallSig = (crate::classes::viewport::DefaultCanvasItemTextureFilter,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_default_canvas_item_texture_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_embedding_subwindows(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_embedding_subwindows", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_embedding_subwindows(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_embedding_subwindows", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_embedded_subwindows(&self,) -> Array < Gd < crate::classes::Window > > {
            type CallSig = (Array < Gd < crate::classes::Window > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_embedded_subwindows", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_canvas_cull_mask(&mut self, mask: u32,) {
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_canvas_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_canvas_cull_mask(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_canvas_cull_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_canvas_cull_mask_bit(&mut self, layer: u32, enable: bool,) {
            type CallSig = ((), u32, bool);
            let args = (layer, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_canvas_cull_mask_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_canvas_cull_mask_bit(&self, layer: u32,) -> bool {
            type CallSig = (bool, u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_canvas_cull_mask_bit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_canvas_item_texture_repeat(&mut self, mode: crate::classes::viewport::DefaultCanvasItemTextureRepeat,) {
            type CallSig = ((), crate::classes::viewport::DefaultCanvasItemTextureRepeat);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_default_canvas_item_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_canvas_item_texture_repeat(&self,) -> crate::classes::viewport::DefaultCanvasItemTextureRepeat {
            type CallSig = (crate::classes::viewport::DefaultCanvasItemTextureRepeat,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_default_canvas_item_texture_repeat", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdf_oversize(&mut self, oversize: crate::classes::viewport::SdfOversize,) {
            type CallSig = ((), crate::classes::viewport::SdfOversize);
            let args = (oversize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_sdf_oversize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdf_oversize(&self,) -> crate::classes::viewport::SdfOversize {
            type CallSig = (crate::classes::viewport::SdfOversize,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_sdf_oversize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sdf_scale(&mut self, scale: crate::classes::viewport::SdfScale,) {
            type CallSig = ((), crate::classes::viewport::SdfScale);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_sdf_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sdf_scale(&self,) -> crate::classes::viewport::SdfScale {
            type CallSig = (crate::classes::viewport::SdfScale,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_sdf_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mesh_lod_threshold(&mut self, pixels: f32,) {
            type CallSig = ((), f32);
            let args = (pixels,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_mesh_lod_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh_lod_threshold(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_mesh_lod_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_audio_listener_2d(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_as_audio_listener_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_audio_listener_2d(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_audio_listener_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_2d(&self,) -> Option < Gd < crate::classes::Camera2D > > {
            type CallSig = (Option < Gd < crate::classes::Camera2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_camera_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_world_3d(&mut self, world_3d: impl AsObjectArg < crate::classes::World3D >,) {
            type CallSig = ((), ObjectArg < crate::classes::World3D >);
            let args = (world_3d.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_world_3d(&self,) -> Option < Gd < crate::classes::World3D > > {
            type CallSig = (Option < Gd < crate::classes::World3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_world_3d(&self,) -> Option < Gd < crate::classes::World3D > > {
            type CallSig = (Option < Gd < crate::classes::World3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "find_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_own_world_3d(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_use_own_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_own_world_3d(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_using_own_world_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_3d(&self,) -> Option < Gd < crate::classes::Camera3D > > {
            type CallSig = (Option < Gd < crate::classes::Camera3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_camera_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_as_audio_listener_3d(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_as_audio_listener_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_audio_listener_3d(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_audio_listener_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_disable_3d(&mut self, disable: bool,) {
            type CallSig = ((), bool);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_disable_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_3d_disabled(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_3d_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_xr(&mut self, use_: bool,) {
            type CallSig = ((), bool);
            let args = (use_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_use_xr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_xr(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "is_using_xr", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scaling_3d_mode(&mut self, scaling_3d_mode: crate::classes::viewport::Scaling3DMode,) {
            type CallSig = ((), crate::classes::viewport::Scaling3DMode);
            let args = (scaling_3d_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_scaling_3d_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scaling_3d_mode(&self,) -> crate::classes::viewport::Scaling3DMode {
            type CallSig = (crate::classes::viewport::Scaling3DMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_scaling_3d_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scaling_3d_scale(&mut self, scale: f32,) {
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_scaling_3d_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scaling_3d_scale(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_scaling_3d_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fsr_sharpness(&mut self, fsr_sharpness: f32,) {
            type CallSig = ((), f32);
            let args = (fsr_sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_fsr_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fsr_sharpness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_fsr_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_mipmap_bias(&mut self, texture_mipmap_bias: f32,) {
            type CallSig = ((), f32);
            let args = (texture_mipmap_bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_texture_mipmap_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_mipmap_bias(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_texture_mipmap_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vrs_mode(&mut self, mode: crate::classes::viewport::VrsMode,) {
            type CallSig = ((), crate::classes::viewport::VrsMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_vrs_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vrs_mode(&self,) -> crate::classes::viewport::VrsMode {
            type CallSig = (crate::classes::viewport::VrsMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_vrs_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vrs_update_mode(&mut self, mode: crate::classes::viewport::VrsUpdateMode,) {
            type CallSig = ((), crate::classes::viewport::VrsUpdateMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_vrs_update_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vrs_update_mode(&self,) -> crate::classes::viewport::VrsUpdateMode {
            type CallSig = (crate::classes::viewport::VrsUpdateMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_vrs_update_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vrs_texture(&mut self, texture: impl AsObjectArg < crate::classes::Texture2D >,) {
            type CallSig = ((), ObjectArg < crate::classes::Texture2D >);
            let args = (texture.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "set_vrs_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vrs_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallSig = (Option < Gd < crate::classes::Texture2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Viewport", "get_vrs_texture", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Viewport {
        type Base = crate::classes::Node;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Viewport"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Viewport {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Viewport {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Viewport {
        
    }
    impl std::ops::Deref for Viewport {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Viewport {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Viewport`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Viewport {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Viewport > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Viewport::push_input_ex`][super::Viewport::push_input_ex]."]
#[must_use]
pub struct ExPushInput < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Viewport, event: ObjectCow < crate::classes::InputEvent >, in_local_coords: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushInput < 'a > {
    fn new(surround_object: &'a mut re_export::Viewport, event: impl AsObjectArg < crate::classes::InputEvent >,) -> Self {
        let in_local_coords = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.consume_arg(), in_local_coords: in_local_coords,
        }
    }
    #[inline]
    pub fn in_local_coords(self, in_local_coords: bool) -> Self {
        Self {
            in_local_coords: in_local_coords, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, event, in_local_coords,
        }
        = self;
        re_export::Viewport::push_input_full(surround_object, event.cow_as_object_arg(), in_local_coords,)
    }
}
#[doc = "Default-param extender for [`Viewport::push_unhandled_input_ex`][super::Viewport::push_unhandled_input_ex]."]
#[must_use]
pub struct ExPushUnhandledInput < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Viewport, event: ObjectCow < crate::classes::InputEvent >, in_local_coords: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushUnhandledInput < 'a > {
    fn new(surround_object: &'a mut re_export::Viewport, event: impl AsObjectArg < crate::classes::InputEvent >,) -> Self {
        let in_local_coords = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.consume_arg(), in_local_coords: in_local_coords,
        }
    }
    #[inline]
    pub fn in_local_coords(self, in_local_coords: bool) -> Self {
        Self {
            in_local_coords: in_local_coords, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, event, in_local_coords,
        }
        = self;
        re_export::Viewport::push_unhandled_input_full(surround_object, event.cow_as_object_arg(), in_local_coords,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PositionalShadowAtlasQuadrantSubdiv {
    ord: i32
}
impl PositionalShadowAtlasQuadrantSubdiv {
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED`"]
    pub const DISABLED: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_1")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_1`"]
    pub const SUBDIV_1: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_4")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_4`"]
    pub const SUBDIV_4: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 2i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_16")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_16`"]
    pub const SUBDIV_16: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 3i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_64")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_64`"]
    pub const SUBDIV_64: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 4i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_256")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_256`"]
    pub const SUBDIV_256: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 5i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_1024")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_1024`"]
    pub const SUBDIV_1024: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 6i32
    };
    #[doc(alias = "SHADOW_ATLAS_QUADRANT_SUBDIV_MAX")]
    #[doc = "Godot enumerator name: `SHADOW_ATLAS_QUADRANT_SUBDIV_MAX`"]
    pub const MAX: PositionalShadowAtlasQuadrantSubdiv = PositionalShadowAtlasQuadrantSubdiv {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for PositionalShadowAtlasQuadrantSubdiv {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PositionalShadowAtlasQuadrantSubdiv") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PositionalShadowAtlasQuadrantSubdiv {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::SUBDIV_1 => "SUBDIV_1", Self::SUBDIV_4 => "SUBDIV_4", Self::SUBDIV_16 => "SUBDIV_16", Self::SUBDIV_64 => "SUBDIV_64", Self::SUBDIV_256 => "SUBDIV_256", Self::SUBDIV_1024 => "SUBDIV_1024", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "SHADOW_ATLAS_QUADRANT_SUBDIV_DISABLED", Self::SUBDIV_1 => "SHADOW_ATLAS_QUADRANT_SUBDIV_1", Self::SUBDIV_4 => "SHADOW_ATLAS_QUADRANT_SUBDIV_4", Self::SUBDIV_16 => "SHADOW_ATLAS_QUADRANT_SUBDIV_16", Self::SUBDIV_64 => "SHADOW_ATLAS_QUADRANT_SUBDIV_64", Self::SUBDIV_256 => "SHADOW_ATLAS_QUADRANT_SUBDIV_256", Self::SUBDIV_1024 => "SHADOW_ATLAS_QUADRANT_SUBDIV_1024", Self::MAX => "SHADOW_ATLAS_QUADRANT_SUBDIV_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for PositionalShadowAtlasQuadrantSubdiv {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for PositionalShadowAtlasQuadrantSubdiv {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PositionalShadowAtlasQuadrantSubdiv {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PositionalShadowAtlasQuadrantSubdiv {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Scaling3DMode {
    ord: i32
}
impl Scaling3DMode {
    #[doc(alias = "SCALING_3D_MODE_BILINEAR")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_BILINEAR`"]
    pub const BILINEAR: Scaling3DMode = Scaling3DMode {
        ord: 0i32
    };
    #[doc(alias = "SCALING_3D_MODE_FSR")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_FSR`"]
    pub const FSR: Scaling3DMode = Scaling3DMode {
        ord: 1i32
    };
    #[doc(alias = "SCALING_3D_MODE_FSR2")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_FSR2`"]
    pub const FSR2: Scaling3DMode = Scaling3DMode {
        ord: 2i32
    };
    #[doc(alias = "SCALING_3D_MODE_MAX")]
    #[doc = "Godot enumerator name: `SCALING_3D_MODE_MAX`"]
    pub const MAX: Scaling3DMode = Scaling3DMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Scaling3DMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Scaling3DMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Scaling3DMode {
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
            Self::BILINEAR => "BILINEAR", Self::FSR => "FSR", Self::FSR2 => "FSR2", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BILINEAR => "SCALING_3D_MODE_BILINEAR", Self::FSR => "SCALING_3D_MODE_FSR", Self::FSR2 => "SCALING_3D_MODE_FSR2", Self::MAX => "SCALING_3D_MODE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Scaling3DMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for Scaling3DMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Scaling3DMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Scaling3DMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `MSAA`."]
pub struct Msaa {
    ord: i32
}
impl Msaa {
    #[doc(alias = "MSAA_DISABLED")]
    #[doc = "Godot enumerator name: `MSAA_DISABLED`"]
    pub const DISABLED: Msaa = Msaa {
        ord: 0i32
    };
    pub const MSAA_2X: Msaa = Msaa {
        ord: 1i32
    };
    pub const MSAA_4X: Msaa = Msaa {
        ord: 2i32
    };
    pub const MSAA_8X: Msaa = Msaa {
        ord: 3i32
    };
    #[doc(alias = "MSAA_MAX")]
    #[doc = "Godot enumerator name: `MSAA_MAX`"]
    pub const MAX: Msaa = Msaa {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Msaa {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Msaa") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Msaa {
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
            Self::DISABLED => "DISABLED", Self::MSAA_2X => "MSAA_2X", Self::MSAA_4X => "MSAA_4X", Self::MSAA_8X => "MSAA_8X", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "MSAA_DISABLED", Self::MAX => "MSAA_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for Msaa {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for Msaa {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Msaa {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Msaa {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ScreenSpaceAA`."]
pub struct ScreenSpaceAa {
    ord: i32
}
impl ScreenSpaceAa {
    #[doc(alias = "SCREEN_SPACE_AA_DISABLED")]
    #[doc = "Godot enumerator name: `SCREEN_SPACE_AA_DISABLED`"]
    pub const DISABLED: ScreenSpaceAa = ScreenSpaceAa {
        ord: 0i32
    };
    #[doc(alias = "SCREEN_SPACE_AA_FXAA")]
    #[doc = "Godot enumerator name: `SCREEN_SPACE_AA_FXAA`"]
    pub const FXAA: ScreenSpaceAa = ScreenSpaceAa {
        ord: 1i32
    };
    #[doc(alias = "SCREEN_SPACE_AA_MAX")]
    #[doc = "Godot enumerator name: `SCREEN_SPACE_AA_MAX`"]
    pub const MAX: ScreenSpaceAa = ScreenSpaceAa {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ScreenSpaceAa {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ScreenSpaceAa") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ScreenSpaceAa {
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
            Self::DISABLED => "DISABLED", Self::FXAA => "FXAA", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "SCREEN_SPACE_AA_DISABLED", Self::FXAA => "SCREEN_SPACE_AA_FXAA", Self::MAX => "SCREEN_SPACE_AA_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for ScreenSpaceAa {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for ScreenSpaceAa {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ScreenSpaceAa {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ScreenSpaceAa {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderInfo {
    ord: i32
}
impl RenderInfo {
    #[doc(alias = "RENDER_INFO_OBJECTS_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDER_INFO_OBJECTS_IN_FRAME`"]
    pub const OBJECTS_IN_FRAME: RenderInfo = RenderInfo {
        ord: 0i32
    };
    #[doc(alias = "RENDER_INFO_PRIMITIVES_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDER_INFO_PRIMITIVES_IN_FRAME`"]
    pub const PRIMITIVES_IN_FRAME: RenderInfo = RenderInfo {
        ord: 1i32
    };
    #[doc(alias = "RENDER_INFO_DRAW_CALLS_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDER_INFO_DRAW_CALLS_IN_FRAME`"]
    pub const DRAW_CALLS_IN_FRAME: RenderInfo = RenderInfo {
        ord: 2i32
    };
    #[doc(alias = "RENDER_INFO_MAX")]
    #[doc = "Godot enumerator name: `RENDER_INFO_MAX`"]
    pub const MAX: RenderInfo = RenderInfo {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for RenderInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderInfo") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderInfo {
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
            Self::OBJECTS_IN_FRAME => "OBJECTS_IN_FRAME", Self::PRIMITIVES_IN_FRAME => "PRIMITIVES_IN_FRAME", Self::DRAW_CALLS_IN_FRAME => "DRAW_CALLS_IN_FRAME", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OBJECTS_IN_FRAME => "RENDER_INFO_OBJECTS_IN_FRAME", Self::PRIMITIVES_IN_FRAME => "RENDER_INFO_PRIMITIVES_IN_FRAME", Self::DRAW_CALLS_IN_FRAME => "RENDER_INFO_DRAW_CALLS_IN_FRAME", Self::MAX => "RENDER_INFO_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for RenderInfo {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for RenderInfo {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RenderInfo {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderInfoType {
    ord: i32
}
impl RenderInfoType {
    #[doc(alias = "RENDER_INFO_TYPE_VISIBLE")]
    #[doc = "Godot enumerator name: `RENDER_INFO_TYPE_VISIBLE`"]
    pub const VISIBLE: RenderInfoType = RenderInfoType {
        ord: 0i32
    };
    #[doc(alias = "RENDER_INFO_TYPE_SHADOW")]
    #[doc = "Godot enumerator name: `RENDER_INFO_TYPE_SHADOW`"]
    pub const SHADOW: RenderInfoType = RenderInfoType {
        ord: 1i32
    };
    #[doc(alias = "RENDER_INFO_TYPE_CANVAS")]
    #[doc = "Godot enumerator name: `RENDER_INFO_TYPE_CANVAS`"]
    pub const CANVAS: RenderInfoType = RenderInfoType {
        ord: 2i32
    };
    #[doc(alias = "RENDER_INFO_TYPE_MAX")]
    #[doc = "Godot enumerator name: `RENDER_INFO_TYPE_MAX`"]
    pub const MAX: RenderInfoType = RenderInfoType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for RenderInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderInfoType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderInfoType {
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
            Self::VISIBLE => "VISIBLE", Self::SHADOW => "SHADOW", Self::CANVAS => "CANVAS", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::VISIBLE => "RENDER_INFO_TYPE_VISIBLE", Self::SHADOW => "RENDER_INFO_TYPE_SHADOW", Self::CANVAS => "RENDER_INFO_TYPE_CANVAS", Self::MAX => "RENDER_INFO_TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for RenderInfoType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for RenderInfoType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RenderInfoType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderInfoType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DebugDraw {
    ord: i32
}
impl DebugDraw {
    #[doc(alias = "DEBUG_DRAW_DISABLED")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_DISABLED`"]
    pub const DISABLED: DebugDraw = DebugDraw {
        ord: 0i32
    };
    #[doc(alias = "DEBUG_DRAW_UNSHADED")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_UNSHADED`"]
    pub const UNSHADED: DebugDraw = DebugDraw {
        ord: 1i32
    };
    #[doc(alias = "DEBUG_DRAW_LIGHTING")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_LIGHTING`"]
    pub const LIGHTING: DebugDraw = DebugDraw {
        ord: 2i32
    };
    #[doc(alias = "DEBUG_DRAW_OVERDRAW")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_OVERDRAW`"]
    pub const OVERDRAW: DebugDraw = DebugDraw {
        ord: 3i32
    };
    #[doc(alias = "DEBUG_DRAW_WIREFRAME")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_WIREFRAME`"]
    pub const WIREFRAME: DebugDraw = DebugDraw {
        ord: 4i32
    };
    #[doc(alias = "DEBUG_DRAW_NORMAL_BUFFER")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_NORMAL_BUFFER`"]
    pub const NORMAL_BUFFER: DebugDraw = DebugDraw {
        ord: 5i32
    };
    #[doc(alias = "DEBUG_DRAW_VOXEL_GI_ALBEDO")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_VOXEL_GI_ALBEDO`"]
    pub const VOXEL_GI_ALBEDO: DebugDraw = DebugDraw {
        ord: 6i32
    };
    #[doc(alias = "DEBUG_DRAW_VOXEL_GI_LIGHTING")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_VOXEL_GI_LIGHTING`"]
    pub const VOXEL_GI_LIGHTING: DebugDraw = DebugDraw {
        ord: 7i32
    };
    #[doc(alias = "DEBUG_DRAW_VOXEL_GI_EMISSION")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_VOXEL_GI_EMISSION`"]
    pub const VOXEL_GI_EMISSION: DebugDraw = DebugDraw {
        ord: 8i32
    };
    #[doc(alias = "DEBUG_DRAW_SHADOW_ATLAS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SHADOW_ATLAS`"]
    pub const SHADOW_ATLAS: DebugDraw = DebugDraw {
        ord: 9i32
    };
    #[doc(alias = "DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS`"]
    pub const DIRECTIONAL_SHADOW_ATLAS: DebugDraw = DebugDraw {
        ord: 10i32
    };
    #[doc(alias = "DEBUG_DRAW_SCENE_LUMINANCE")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SCENE_LUMINANCE`"]
    pub const SCENE_LUMINANCE: DebugDraw = DebugDraw {
        ord: 11i32
    };
    #[doc(alias = "DEBUG_DRAW_SSAO")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SSAO`"]
    pub const SSAO: DebugDraw = DebugDraw {
        ord: 12i32
    };
    #[doc(alias = "DEBUG_DRAW_SSIL")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SSIL`"]
    pub const SSIL: DebugDraw = DebugDraw {
        ord: 13i32
    };
    #[doc(alias = "DEBUG_DRAW_PSSM_SPLITS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_PSSM_SPLITS`"]
    pub const PSSM_SPLITS: DebugDraw = DebugDraw {
        ord: 14i32
    };
    #[doc(alias = "DEBUG_DRAW_DECAL_ATLAS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_DECAL_ATLAS`"]
    pub const DECAL_ATLAS: DebugDraw = DebugDraw {
        ord: 15i32
    };
    #[doc(alias = "DEBUG_DRAW_SDFGI")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SDFGI`"]
    pub const SDFGI: DebugDraw = DebugDraw {
        ord: 16i32
    };
    #[doc(alias = "DEBUG_DRAW_SDFGI_PROBES")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_SDFGI_PROBES`"]
    pub const SDFGI_PROBES: DebugDraw = DebugDraw {
        ord: 17i32
    };
    #[doc(alias = "DEBUG_DRAW_GI_BUFFER")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_GI_BUFFER`"]
    pub const GI_BUFFER: DebugDraw = DebugDraw {
        ord: 18i32
    };
    #[doc(alias = "DEBUG_DRAW_DISABLE_LOD")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_DISABLE_LOD`"]
    pub const DISABLE_LOD: DebugDraw = DebugDraw {
        ord: 19i32
    };
    #[doc(alias = "DEBUG_DRAW_CLUSTER_OMNI_LIGHTS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_CLUSTER_OMNI_LIGHTS`"]
    pub const CLUSTER_OMNI_LIGHTS: DebugDraw = DebugDraw {
        ord: 20i32
    };
    #[doc(alias = "DEBUG_DRAW_CLUSTER_SPOT_LIGHTS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_CLUSTER_SPOT_LIGHTS`"]
    pub const CLUSTER_SPOT_LIGHTS: DebugDraw = DebugDraw {
        ord: 21i32
    };
    #[doc(alias = "DEBUG_DRAW_CLUSTER_DECALS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_CLUSTER_DECALS`"]
    pub const CLUSTER_DECALS: DebugDraw = DebugDraw {
        ord: 22i32
    };
    #[doc(alias = "DEBUG_DRAW_CLUSTER_REFLECTION_PROBES")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_CLUSTER_REFLECTION_PROBES`"]
    pub const CLUSTER_REFLECTION_PROBES: DebugDraw = DebugDraw {
        ord: 23i32
    };
    #[doc(alias = "DEBUG_DRAW_OCCLUDERS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_OCCLUDERS`"]
    pub const OCCLUDERS: DebugDraw = DebugDraw {
        ord: 24i32
    };
    #[doc(alias = "DEBUG_DRAW_MOTION_VECTORS")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_MOTION_VECTORS`"]
    pub const MOTION_VECTORS: DebugDraw = DebugDraw {
        ord: 25i32
    };
    #[doc(alias = "DEBUG_DRAW_INTERNAL_BUFFER")]
    #[doc = "Godot enumerator name: `DEBUG_DRAW_INTERNAL_BUFFER`"]
    pub const INTERNAL_BUFFER: DebugDraw = DebugDraw {
        ord: 26i32
    };
    
}
impl std::fmt::Debug for DebugDraw {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DebugDraw") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DebugDraw {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::UNSHADED => "UNSHADED", Self::LIGHTING => "LIGHTING", Self::OVERDRAW => "OVERDRAW", Self::WIREFRAME => "WIREFRAME", Self::NORMAL_BUFFER => "NORMAL_BUFFER", Self::VOXEL_GI_ALBEDO => "VOXEL_GI_ALBEDO", Self::VOXEL_GI_LIGHTING => "VOXEL_GI_LIGHTING", Self::VOXEL_GI_EMISSION => "VOXEL_GI_EMISSION", Self::SHADOW_ATLAS => "SHADOW_ATLAS", Self::DIRECTIONAL_SHADOW_ATLAS => "DIRECTIONAL_SHADOW_ATLAS", Self::SCENE_LUMINANCE => "SCENE_LUMINANCE", Self::SSAO => "SSAO", Self::SSIL => "SSIL", Self::PSSM_SPLITS => "PSSM_SPLITS", Self::DECAL_ATLAS => "DECAL_ATLAS", Self::SDFGI => "SDFGI", Self::SDFGI_PROBES => "SDFGI_PROBES", Self::GI_BUFFER => "GI_BUFFER", Self::DISABLE_LOD => "DISABLE_LOD", Self::CLUSTER_OMNI_LIGHTS => "CLUSTER_OMNI_LIGHTS", Self::CLUSTER_SPOT_LIGHTS => "CLUSTER_SPOT_LIGHTS", Self::CLUSTER_DECALS => "CLUSTER_DECALS", Self::CLUSTER_REFLECTION_PROBES => "CLUSTER_REFLECTION_PROBES", Self::OCCLUDERS => "OCCLUDERS", Self::MOTION_VECTORS => "MOTION_VECTORS", Self::INTERNAL_BUFFER => "INTERNAL_BUFFER", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DEBUG_DRAW_DISABLED", Self::UNSHADED => "DEBUG_DRAW_UNSHADED", Self::LIGHTING => "DEBUG_DRAW_LIGHTING", Self::OVERDRAW => "DEBUG_DRAW_OVERDRAW", Self::WIREFRAME => "DEBUG_DRAW_WIREFRAME", Self::NORMAL_BUFFER => "DEBUG_DRAW_NORMAL_BUFFER", Self::VOXEL_GI_ALBEDO => "DEBUG_DRAW_VOXEL_GI_ALBEDO", Self::VOXEL_GI_LIGHTING => "DEBUG_DRAW_VOXEL_GI_LIGHTING", Self::VOXEL_GI_EMISSION => "DEBUG_DRAW_VOXEL_GI_EMISSION", Self::SHADOW_ATLAS => "DEBUG_DRAW_SHADOW_ATLAS", Self::DIRECTIONAL_SHADOW_ATLAS => "DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS", Self::SCENE_LUMINANCE => "DEBUG_DRAW_SCENE_LUMINANCE", Self::SSAO => "DEBUG_DRAW_SSAO", Self::SSIL => "DEBUG_DRAW_SSIL", Self::PSSM_SPLITS => "DEBUG_DRAW_PSSM_SPLITS", Self::DECAL_ATLAS => "DEBUG_DRAW_DECAL_ATLAS", Self::SDFGI => "DEBUG_DRAW_SDFGI", Self::SDFGI_PROBES => "DEBUG_DRAW_SDFGI_PROBES", Self::GI_BUFFER => "DEBUG_DRAW_GI_BUFFER", Self::DISABLE_LOD => "DEBUG_DRAW_DISABLE_LOD", Self::CLUSTER_OMNI_LIGHTS => "DEBUG_DRAW_CLUSTER_OMNI_LIGHTS", Self::CLUSTER_SPOT_LIGHTS => "DEBUG_DRAW_CLUSTER_SPOT_LIGHTS", Self::CLUSTER_DECALS => "DEBUG_DRAW_CLUSTER_DECALS", Self::CLUSTER_REFLECTION_PROBES => "DEBUG_DRAW_CLUSTER_REFLECTION_PROBES", Self::OCCLUDERS => "DEBUG_DRAW_OCCLUDERS", Self::MOTION_VECTORS => "DEBUG_DRAW_MOTION_VECTORS", Self::INTERNAL_BUFFER => "DEBUG_DRAW_INTERNAL_BUFFER", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DebugDraw {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DebugDraw {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DebugDraw {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DefaultCanvasItemTextureFilter {
    ord: i32
}
impl DefaultCanvasItemTextureFilter {
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST`"]
    pub const NEAREST: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 0i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR`"]
    pub const LINEAR: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 1i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS`"]
    pub const LINEAR_WITH_MIPMAPS: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 2i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS`"]
    pub const NEAREST_WITH_MIPMAPS: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 3i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX`"]
    pub const MAX: DefaultCanvasItemTextureFilter = DefaultCanvasItemTextureFilter {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for DefaultCanvasItemTextureFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DefaultCanvasItemTextureFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DefaultCanvasItemTextureFilter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::LINEAR_WITH_MIPMAPS => "LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS => "NEAREST_WITH_MIPMAPS", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NEAREST => "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST", Self::LINEAR => "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR", Self::LINEAR_WITH_MIPMAPS => "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS => "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS", Self::MAX => "DEFAULT_CANVAS_ITEM_TEXTURE_FILTER_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for DefaultCanvasItemTextureFilter {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for DefaultCanvasItemTextureFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DefaultCanvasItemTextureFilter {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DefaultCanvasItemTextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DefaultCanvasItemTextureRepeat {
    ord: i32
}
impl DefaultCanvasItemTextureRepeat {
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED`"]
    pub const DISABLED: DefaultCanvasItemTextureRepeat = DefaultCanvasItemTextureRepeat {
        ord: 0i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED`"]
    pub const ENABLED: DefaultCanvasItemTextureRepeat = DefaultCanvasItemTextureRepeat {
        ord: 1i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR`"]
    pub const MIRROR: DefaultCanvasItemTextureRepeat = DefaultCanvasItemTextureRepeat {
        ord: 2i32
    };
    #[doc(alias = "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX")]
    #[doc = "Godot enumerator name: `DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX`"]
    pub const MAX: DefaultCanvasItemTextureRepeat = DefaultCanvasItemTextureRepeat {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for DefaultCanvasItemTextureRepeat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DefaultCanvasItemTextureRepeat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DefaultCanvasItemTextureRepeat {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::MIRROR => "MIRROR", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_DISABLED", Self::ENABLED => "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_ENABLED", Self::MIRROR => "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MIRROR", Self::MAX => "DEFAULT_CANVAS_ITEM_TEXTURE_REPEAT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for DefaultCanvasItemTextureRepeat {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for DefaultCanvasItemTextureRepeat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DefaultCanvasItemTextureRepeat {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DefaultCanvasItemTextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `SDFOversize`."]
pub struct SdfOversize {
    ord: i32
}
impl SdfOversize {
    #[doc(alias = "SDF_OVERSIZE_100_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_100_PERCENT`"]
    pub const OVERSIZE_100_PERCENT: SdfOversize = SdfOversize {
        ord: 0i32
    };
    #[doc(alias = "SDF_OVERSIZE_120_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_120_PERCENT`"]
    pub const OVERSIZE_120_PERCENT: SdfOversize = SdfOversize {
        ord: 1i32
    };
    #[doc(alias = "SDF_OVERSIZE_150_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_150_PERCENT`"]
    pub const OVERSIZE_150_PERCENT: SdfOversize = SdfOversize {
        ord: 2i32
    };
    #[doc(alias = "SDF_OVERSIZE_200_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_200_PERCENT`"]
    pub const OVERSIZE_200_PERCENT: SdfOversize = SdfOversize {
        ord: 3i32
    };
    #[doc(alias = "SDF_OVERSIZE_MAX")]
    #[doc = "Godot enumerator name: `SDF_OVERSIZE_MAX`"]
    pub const MAX: SdfOversize = SdfOversize {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SdfOversize {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SdfOversize") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SdfOversize {
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
            Self::OVERSIZE_100_PERCENT => "OVERSIZE_100_PERCENT", Self::OVERSIZE_120_PERCENT => "OVERSIZE_120_PERCENT", Self::OVERSIZE_150_PERCENT => "OVERSIZE_150_PERCENT", Self::OVERSIZE_200_PERCENT => "OVERSIZE_200_PERCENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OVERSIZE_100_PERCENT => "SDF_OVERSIZE_100_PERCENT", Self::OVERSIZE_120_PERCENT => "SDF_OVERSIZE_120_PERCENT", Self::OVERSIZE_150_PERCENT => "SDF_OVERSIZE_150_PERCENT", Self::OVERSIZE_200_PERCENT => "SDF_OVERSIZE_200_PERCENT", Self::MAX => "SDF_OVERSIZE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for SdfOversize {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for SdfOversize {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SdfOversize {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SdfOversize {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `SDFScale`."]
pub struct SdfScale {
    ord: i32
}
impl SdfScale {
    #[doc(alias = "SDF_SCALE_100_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_SCALE_100_PERCENT`"]
    pub const SCALE_100_PERCENT: SdfScale = SdfScale {
        ord: 0i32
    };
    #[doc(alias = "SDF_SCALE_50_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_SCALE_50_PERCENT`"]
    pub const SCALE_50_PERCENT: SdfScale = SdfScale {
        ord: 1i32
    };
    #[doc(alias = "SDF_SCALE_25_PERCENT")]
    #[doc = "Godot enumerator name: `SDF_SCALE_25_PERCENT`"]
    pub const SCALE_25_PERCENT: SdfScale = SdfScale {
        ord: 2i32
    };
    #[doc(alias = "SDF_SCALE_MAX")]
    #[doc = "Godot enumerator name: `SDF_SCALE_MAX`"]
    pub const MAX: SdfScale = SdfScale {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for SdfScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SdfScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SdfScale {
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
            Self::SCALE_100_PERCENT => "SCALE_100_PERCENT", Self::SCALE_50_PERCENT => "SCALE_50_PERCENT", Self::SCALE_25_PERCENT => "SCALE_25_PERCENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SCALE_100_PERCENT => "SDF_SCALE_100_PERCENT", Self::SCALE_50_PERCENT => "SDF_SCALE_50_PERCENT", Self::SCALE_25_PERCENT => "SDF_SCALE_25_PERCENT", Self::MAX => "SDF_SCALE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for SdfScale {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for SdfScale {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SdfScale {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SdfScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `VRSMode`."]
pub struct VrsMode {
    ord: i32
}
impl VrsMode {
    #[doc(alias = "VRS_DISABLED")]
    #[doc = "Godot enumerator name: `VRS_DISABLED`"]
    pub const DISABLED: VrsMode = VrsMode {
        ord: 0i32
    };
    #[doc(alias = "VRS_TEXTURE")]
    #[doc = "Godot enumerator name: `VRS_TEXTURE`"]
    pub const TEXTURE: VrsMode = VrsMode {
        ord: 1i32
    };
    #[doc(alias = "VRS_XR")]
    #[doc = "Godot enumerator name: `VRS_XR`"]
    pub const XR: VrsMode = VrsMode {
        ord: 2i32
    };
    #[doc(alias = "VRS_MAX")]
    #[doc = "Godot enumerator name: `VRS_MAX`"]
    pub const MAX: VrsMode = VrsMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for VrsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VrsMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VrsMode {
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
            Self::DISABLED => "DISABLED", Self::TEXTURE => "TEXTURE", Self::XR => "XR", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "VRS_DISABLED", Self::TEXTURE => "VRS_TEXTURE", Self::XR => "VRS_XR", Self::MAX => "VRS_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for VrsMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for VrsMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VrsMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VrsMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `VRSUpdateMode`."]
pub struct VrsUpdateMode {
    ord: i32
}
impl VrsUpdateMode {
    #[doc(alias = "VRS_UPDATE_DISABLED")]
    #[doc = "Godot enumerator name: `VRS_UPDATE_DISABLED`"]
    pub const DISABLED: VrsUpdateMode = VrsUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "VRS_UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `VRS_UPDATE_ONCE`"]
    pub const ONCE: VrsUpdateMode = VrsUpdateMode {
        ord: 1i32
    };
    #[doc(alias = "VRS_UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `VRS_UPDATE_ALWAYS`"]
    pub const ALWAYS: VrsUpdateMode = VrsUpdateMode {
        ord: 2i32
    };
    #[doc(alias = "VRS_UPDATE_MAX")]
    #[doc = "Godot enumerator name: `VRS_UPDATE_MAX`"]
    pub const MAX: VrsUpdateMode = VrsUpdateMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for VrsUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VrsUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VrsUpdateMode {
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
            Self::DISABLED => "DISABLED", Self::ONCE => "ONCE", Self::ALWAYS => "ALWAYS", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "VRS_UPDATE_DISABLED", Self::ONCE => "VRS_UPDATE_ONCE", Self::ALWAYS => "VRS_UPDATE_ALWAYS", Self::MAX => "VRS_UPDATE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for VrsUpdateMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for VrsUpdateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VrsUpdateMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VrsUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}