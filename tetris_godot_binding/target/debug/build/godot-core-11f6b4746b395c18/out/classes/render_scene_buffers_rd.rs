#![doc = "Sidecar module for class [`RenderSceneBuffersRd`][crate::classes::RenderSceneBuffersRd].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderSceneBuffersRD` enums](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersrd.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderSceneBuffersRD.`\n\nInherits [`RenderSceneBuffers`][crate::classes::RenderSceneBuffers].\n\nRelated symbols:\n\n* [`render_scene_buffers_rd`][crate::classes::render_scene_buffers_rd]: sidecar module with related enum/flag types\n* [`IRenderSceneBuffersRd`][crate::classes::IRenderSceneBuffersRd]: virtual methods\n\n\nSee also [Godot docs for `RenderSceneBuffersRD`](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersrd.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RenderSceneBuffersRd::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderSceneBuffersRd {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RenderSceneBuffersRd`][crate::classes::RenderSceneBuffersRd].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RenderSceneBuffersRD` methods](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersrd.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRenderSceneBuffersRd: crate::obj::GodotClass < Base = RenderSceneBuffersRd > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RenderSceneBuffersRd {
        pub fn has_texture(&self, context: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> bool {
            type CallSig < 'a0, 'a1, > = (bool, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (context.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "has_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_texture(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, data_format: crate::classes::rendering_device::DataFormat, usage_bits: u32, texture_samples: crate::classes::rendering_device::TextureSamples, size: Vector2i, layers: u32, mipmaps: u32, unique: bool,) -> Rid {
            type CallSig < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, crate::classes::rendering_device::DataFormat, u32, crate::classes::rendering_device::TextureSamples, Vector2i, u32, u32, bool);
            let args = (context.into_arg(), name.into_arg(), data_format, usage_bits, texture_samples, size, layers, mipmaps, unique,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "create_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_texture_from_format(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, format: impl AsObjectArg < crate::classes::RdTextureFormat >, view: impl AsObjectArg < crate::classes::RdTextureView >, unique: bool,) -> Rid {
            type CallSig < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, ObjectArg < crate::classes::RdTextureFormat >, ObjectArg < crate::classes::RdTextureView >, bool);
            let args = (context.into_arg(), name.into_arg(), format.as_object_arg(), view.as_object_arg(), unique,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "create_texture_from_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_texture_view(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, view_name: impl AsArg < StringName >, view: impl AsObjectArg < crate::classes::RdTextureView >,) -> Rid {
            type CallSig < 'a0, 'a1, 'a2, > = (Rid, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >, ObjectArg < crate::classes::RdTextureView >);
            let args = (context.into_arg(), name.into_arg(), view_name.into_arg(), view.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "create_texture_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self, context: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> Rid {
            type CallSig < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (context.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_format(&self, context: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::RdTextureFormat > > {
            type CallSig < 'a0, 'a1, > = (Option < Gd < crate::classes::RdTextureFormat > >, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >);
            let args = (context.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_slice(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, layer: u32, mipmap: u32, layers: u32, mipmaps: u32,) -> Rid {
            type CallSig < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, u32, u32, u32, u32);
            let args = (context.into_arg(), name.into_arg(), layer, mipmap, layers, mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_slice", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_slice_view(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, layer: u32, mipmap: u32, layers: u32, mipmaps: u32, view: impl AsObjectArg < crate::classes::RdTextureView >,) -> Rid {
            type CallSig < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, u32, u32, u32, u32, ObjectArg < crate::classes::RdTextureView >);
            let args = (context.into_arg(), name.into_arg(), layer, mipmap, layers, mipmaps, view.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_slice_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_slice_size(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, mipmap: u32,) -> Vector2i {
            type CallSig < 'a0, 'a1, > = (Vector2i, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, u32);
            let args = (context.into_arg(), name.into_arg(), mipmap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_slice_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_context(&mut self, context: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (context.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "clear_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_color_texture_full(&mut self, msaa: bool,) -> Rid {
            type CallSig = (Rid, bool);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_color_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_color_texture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_color_texture(&mut self,) -> Rid {
            self.get_color_texture_ex() . done()
        }
        #[inline]
        pub fn get_color_texture_ex < 'a > (&'a mut self,) -> ExGetColorTexture < 'a > {
            ExGetColorTexture::new(self,)
        }
        pub(crate) fn get_color_layer_full(&mut self, layer: u32, msaa: bool,) -> Rid {
            type CallSig = (Rid, u32, bool);
            let args = (layer, msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_color_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_color_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_color_layer(&mut self, layer: u32,) -> Rid {
            self.get_color_layer_ex(layer,) . done()
        }
        #[inline]
        pub fn get_color_layer_ex < 'a > (&'a mut self, layer: u32,) -> ExGetColorLayer < 'a > {
            ExGetColorLayer::new(self, layer,)
        }
        pub(crate) fn get_depth_texture_full(&mut self, msaa: bool,) -> Rid {
            type CallSig = (Rid, bool);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_depth_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_depth_texture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_depth_texture(&mut self,) -> Rid {
            self.get_depth_texture_ex() . done()
        }
        #[inline]
        pub fn get_depth_texture_ex < 'a > (&'a mut self,) -> ExGetDepthTexture < 'a > {
            ExGetDepthTexture::new(self,)
        }
        pub(crate) fn get_depth_layer_full(&mut self, layer: u32, msaa: bool,) -> Rid {
            type CallSig = (Rid, u32, bool);
            let args = (layer, msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_depth_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_depth_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_depth_layer(&mut self, layer: u32,) -> Rid {
            self.get_depth_layer_ex(layer,) . done()
        }
        #[inline]
        pub fn get_depth_layer_ex < 'a > (&'a mut self, layer: u32,) -> ExGetDepthLayer < 'a > {
            ExGetDepthLayer::new(self, layer,)
        }
        pub(crate) fn get_velocity_texture_full(&mut self, msaa: bool,) -> Rid {
            type CallSig = (Rid, bool);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_velocity_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_velocity_texture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_velocity_texture(&mut self,) -> Rid {
            self.get_velocity_texture_ex() . done()
        }
        #[inline]
        pub fn get_velocity_texture_ex < 'a > (&'a mut self,) -> ExGetVelocityTexture < 'a > {
            ExGetVelocityTexture::new(self,)
        }
        pub(crate) fn get_velocity_layer_full(&mut self, layer: u32, msaa: bool,) -> Rid {
            type CallSig = (Rid, u32, bool);
            let args = (layer, msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_velocity_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_velocity_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_velocity_layer(&mut self, layer: u32,) -> Rid {
            self.get_velocity_layer_ex(layer,) . done()
        }
        #[inline]
        pub fn get_velocity_layer_ex < 'a > (&'a mut self, layer: u32,) -> ExGetVelocityLayer < 'a > {
            ExGetVelocityLayer::new(self, layer,)
        }
        pub fn get_render_target(&self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_render_target", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_view_count(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_view_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_internal_size(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_internal_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_target_size(&self,) -> Vector2i {
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_target_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scaling_3d_mode(&self,) -> crate::classes::rendering_server::ViewportScaling3DMode {
            type CallSig = (crate::classes::rendering_server::ViewportScaling3DMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_scaling_3d_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fsr_sharpness(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_fsr_sharpness", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_msaa_3d(&self,) -> crate::classes::rendering_server::ViewportMsaa {
            type CallSig = (crate::classes::rendering_server::ViewportMsaa,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_msaa_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_samples(&self,) -> crate::classes::rendering_device::TextureSamples {
            type CallSig = (crate::classes::rendering_device::TextureSamples,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_samples", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_screen_space_aa(&self,) -> crate::classes::rendering_server::ViewportScreenSpaceAa {
            type CallSig = (crate::classes::rendering_server::ViewportScreenSpaceAa,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_screen_space_aa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_taa(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_use_taa", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_debanding(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_use_debanding", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RenderSceneBuffersRd {
        type Base = crate::classes::RenderSceneBuffers;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"RenderSceneBuffersRD"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RenderSceneBuffersRd {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RenderSceneBuffers > for RenderSceneBuffersRd {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RenderSceneBuffersRd {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RenderSceneBuffersRd {
        
    }
    impl crate::obj::cap::GodotDefault for RenderSceneBuffersRd {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RenderSceneBuffersRd {
        type Target = crate::classes::RenderSceneBuffers;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderSceneBuffersRd {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RenderSceneBuffersRd`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_RenderSceneBuffersRd {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RenderSceneBuffersRd > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RenderSceneBuffers > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_color_texture_ex`][super::RenderSceneBuffersRd::get_color_texture_ex]."]
#[must_use]
pub struct ExGetColorTexture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColorTexture < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_color_texture_full(surround_object, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_color_layer_ex`][super::RenderSceneBuffersRd::get_color_layer_ex]."]
#[must_use]
pub struct ExGetColorLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColorLayer < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, layer, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_color_layer_full(surround_object, layer, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_depth_texture_ex`][super::RenderSceneBuffersRd::get_depth_texture_ex]."]
#[must_use]
pub struct ExGetDepthTexture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDepthTexture < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_depth_texture_full(surround_object, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_depth_layer_ex`][super::RenderSceneBuffersRd::get_depth_layer_ex]."]
#[must_use]
pub struct ExGetDepthLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDepthLayer < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, layer, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_depth_layer_full(surround_object, layer, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_velocity_texture_ex`][super::RenderSceneBuffersRd::get_velocity_texture_ex]."]
#[must_use]
pub struct ExGetVelocityTexture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVelocityTexture < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_velocity_texture_full(surround_object, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_velocity_layer_ex`][super::RenderSceneBuffersRd::get_velocity_layer_ex]."]
#[must_use]
pub struct ExGetVelocityLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVelocityLayer < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, layer, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_velocity_layer_full(surround_object, layer, msaa,)
    }
}