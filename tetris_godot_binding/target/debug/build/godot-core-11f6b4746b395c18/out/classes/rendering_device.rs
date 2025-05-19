#![doc = "Sidecar module for class [`RenderingDevice`][crate::classes::RenderingDevice].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderingDevice` enums](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderingDevice.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`rendering_device`][crate::classes::rendering_device]: sidecar module with related enum/flag types\n* [`IRenderingDevice`][crate::classes::IRenderingDevice]: virtual methods\n\n\nSee also [Godot docs for `RenderingDevice`](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<RenderingDevice>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderingDevice {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RenderingDevice`][crate::classes::RenderingDevice].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RenderingDevice` methods](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRenderingDevice: crate::obj::GodotClass < Base = RenderingDevice > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RenderingDevice {
        pub(crate) fn texture_create_full(&mut self, format: ObjectArg < crate::classes::RdTextureFormat >, view: ObjectArg < crate::classes::RdTextureView >, data: RefArg < Array < PackedByteArray > >,) -> Rid {
            type CallSig < 'a0, > = (Rid, ObjectArg < crate::classes::RdTextureFormat >, ObjectArg < crate::classes::RdTextureView >, RefArg < 'a0, Array < PackedByteArray > >);
            let args = (format, view, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_create(&mut self, format: impl AsObjectArg < crate::classes::RdTextureFormat >, view: impl AsObjectArg < crate::classes::RdTextureView >,) -> Rid {
            self.texture_create_ex(format, view,) . done()
        }
        #[inline]
        pub fn texture_create_ex < 'a > (&'a mut self, format: impl AsObjectArg < crate::classes::RdTextureFormat >, view: impl AsObjectArg < crate::classes::RdTextureView >,) -> ExTextureCreate < 'a > {
            ExTextureCreate::new(self, format, view,)
        }
        pub fn texture_create_shared(&mut self, view: impl AsObjectArg < crate::classes::RdTextureView >, with_texture: Rid,) -> Rid {
            type CallSig = (Rid, ObjectArg < crate::classes::RdTextureView >, Rid);
            let args = (view.as_object_arg(), with_texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_create_shared", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn texture_create_shared_from_slice_full(&mut self, view: ObjectArg < crate::classes::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32, mipmaps: u32, slice_type: crate::classes::rendering_device::TextureSliceType,) -> Rid {
            type CallSig = (Rid, ObjectArg < crate::classes::RdTextureView >, Rid, u32, u32, u32, crate::classes::rendering_device::TextureSliceType);
            let args = (view, with_texture, layer, mipmap, mipmaps, slice_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_create_shared_from_slice", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_create_shared_from_slice_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_create_shared_from_slice(&mut self, view: impl AsObjectArg < crate::classes::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32,) -> Rid {
            self.texture_create_shared_from_slice_ex(view, with_texture, layer, mipmap,) . done()
        }
        #[inline]
        pub fn texture_create_shared_from_slice_ex < 'a > (&'a mut self, view: impl AsObjectArg < crate::classes::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32,) -> ExTextureCreateSharedFromSlice < 'a > {
            ExTextureCreateSharedFromSlice::new(self, view, with_texture, layer, mipmap,)
        }
        pub fn texture_create_from_extension(&mut self, type_: crate::classes::rendering_device::TextureType, format: crate::classes::rendering_device::DataFormat, samples: crate::classes::rendering_device::TextureSamples, usage_flags: crate::classes::rendering_device::TextureUsageBits, image: u64, width: u64, height: u64, depth: u64, layers: u64,) -> Rid {
            type CallSig = (Rid, crate::classes::rendering_device::TextureType, crate::classes::rendering_device::DataFormat, crate::classes::rendering_device::TextureSamples, crate::classes::rendering_device::TextureUsageBits, u64, u64, u64, u64, u64);
            let args = (type_, format, samples, usage_flags, image, width, height, depth, layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_create_from_extension", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_update(&mut self, texture: Rid, layer: u32, data: &PackedByteArray,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, Rid, u32, RefArg < 'a0, PackedByteArray >);
            let args = (texture, layer, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_get_data(&mut self, texture: Rid, layer: u32,) -> PackedByteArray {
            type CallSig = (PackedByteArray, Rid, u32);
            let args = (texture, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_is_format_supported_for_usage(&self, format: crate::classes::rendering_device::DataFormat, usage_flags: crate::classes::rendering_device::TextureUsageBits,) -> bool {
            type CallSig = (bool, crate::classes::rendering_device::DataFormat, crate::classes::rendering_device::TextureUsageBits);
            let args = (format, usage_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_is_format_supported_for_usage", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_is_shared(&mut self, texture: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_is_shared", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_is_valid(&mut self, texture: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_copy(&mut self, from_texture: Rid, to_texture: Rid, from_pos: Vector3, to_pos: Vector3, size: Vector3, src_mipmap: u32, dst_mipmap: u32, src_layer: u32, dst_layer: u32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, Rid, Rid, Vector3, Vector3, Vector3, u32, u32, u32, u32);
            let args = (from_texture, to_texture, from_pos, to_pos, size, src_mipmap, dst_mipmap, src_layer, dst_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_copy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_clear(&mut self, texture: Rid, color: Color, base_mipmap: u32, mipmap_count: u32, base_layer: u32, layer_count: u32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, Rid, Color, u32, u32, u32, u32);
            let args = (texture, color, base_mipmap, mipmap_count, base_layer, layer_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_resolve_multisample(&mut self, from_texture: Rid, to_texture: Rid,) -> crate::global::Error {
            type CallSig = (crate::global::Error, Rid, Rid);
            let args = (from_texture, to_texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_resolve_multisample", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_get_format(&mut self, texture: Rid,) -> Option < Gd < crate::classes::RdTextureFormat > > {
            type CallSig = (Option < Gd < crate::classes::RdTextureFormat > >, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn texture_get_native_handle(&mut self, texture: Rid,) -> u64 {
            type CallSig = (u64, Rid);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_get_native_handle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn framebuffer_format_create_full(&mut self, attachments: RefArg < Array < Gd < crate::classes::RdAttachmentFormat > > >, view_count: u32,) -> i64 {
            type CallSig < 'a0, > = (i64, RefArg < 'a0, Array < Gd < crate::classes::RdAttachmentFormat > > >, u32);
            let args = (attachments, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_format_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_format_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_format_create(&mut self, attachments: &Array < Gd < crate::classes::RdAttachmentFormat > >,) -> i64 {
            self.framebuffer_format_create_ex(attachments,) . done()
        }
        #[inline]
        pub fn framebuffer_format_create_ex < 'a > (&'a mut self, attachments: &'a Array < Gd < crate::classes::RdAttachmentFormat > >,) -> ExFramebufferFormatCreate < 'a > {
            ExFramebufferFormatCreate::new(self, attachments,)
        }
        pub(crate) fn framebuffer_format_create_multipass_full(&mut self, attachments: RefArg < Array < Gd < crate::classes::RdAttachmentFormat > > >, passes: RefArg < Array < Gd < crate::classes::RdFramebufferPass > > >, view_count: u32,) -> i64 {
            type CallSig < 'a0, 'a1, > = (i64, RefArg < 'a0, Array < Gd < crate::classes::RdAttachmentFormat > > >, RefArg < 'a1, Array < Gd < crate::classes::RdFramebufferPass > > >, u32);
            let args = (attachments, passes, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_format_create_multipass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_format_create_multipass_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_format_create_multipass(&mut self, attachments: &Array < Gd < crate::classes::RdAttachmentFormat > >, passes: &Array < Gd < crate::classes::RdFramebufferPass > >,) -> i64 {
            self.framebuffer_format_create_multipass_ex(attachments, passes,) . done()
        }
        #[inline]
        pub fn framebuffer_format_create_multipass_ex < 'a > (&'a mut self, attachments: &'a Array < Gd < crate::classes::RdAttachmentFormat > >, passes: &'a Array < Gd < crate::classes::RdFramebufferPass > >,) -> ExFramebufferFormatCreateMultipass < 'a > {
            ExFramebufferFormatCreateMultipass::new(self, attachments, passes,)
        }
        pub(crate) fn framebuffer_format_create_empty_full(&mut self, samples: crate::classes::rendering_device::TextureSamples,) -> i64 {
            type CallSig = (i64, crate::classes::rendering_device::TextureSamples);
            let args = (samples,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_format_create_empty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_format_create_empty_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_format_create_empty(&mut self,) -> i64 {
            self.framebuffer_format_create_empty_ex() . done()
        }
        #[inline]
        pub fn framebuffer_format_create_empty_ex < 'a > (&'a mut self,) -> ExFramebufferFormatCreateEmpty < 'a > {
            ExFramebufferFormatCreateEmpty::new(self,)
        }
        pub(crate) fn framebuffer_format_get_texture_samples_full(&mut self, format: i64, render_pass: u32,) -> crate::classes::rendering_device::TextureSamples {
            type CallSig = (crate::classes::rendering_device::TextureSamples, i64, u32);
            let args = (format, render_pass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_format_get_texture_samples", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_format_get_texture_samples_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_format_get_texture_samples(&mut self, format: i64,) -> crate::classes::rendering_device::TextureSamples {
            self.framebuffer_format_get_texture_samples_ex(format,) . done()
        }
        #[inline]
        pub fn framebuffer_format_get_texture_samples_ex < 'a > (&'a mut self, format: i64,) -> ExFramebufferFormatGetTextureSamples < 'a > {
            ExFramebufferFormatGetTextureSamples::new(self, format,)
        }
        pub(crate) fn framebuffer_create_full(&mut self, textures: RefArg < Array < Rid > >, validate_with_format: i64, view_count: u32,) -> Rid {
            type CallSig < 'a0, > = (Rid, RefArg < 'a0, Array < Rid > >, i64, u32);
            let args = (textures, validate_with_format, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_create(&mut self, textures: &Array < Rid >,) -> Rid {
            self.framebuffer_create_ex(textures,) . done()
        }
        #[inline]
        pub fn framebuffer_create_ex < 'a > (&'a mut self, textures: &'a Array < Rid >,) -> ExFramebufferCreate < 'a > {
            ExFramebufferCreate::new(self, textures,)
        }
        pub(crate) fn framebuffer_create_multipass_full(&mut self, textures: RefArg < Array < Rid > >, passes: RefArg < Array < Gd < crate::classes::RdFramebufferPass > > >, validate_with_format: i64, view_count: u32,) -> Rid {
            type CallSig < 'a0, 'a1, > = (Rid, RefArg < 'a0, Array < Rid > >, RefArg < 'a1, Array < Gd < crate::classes::RdFramebufferPass > > >, i64, u32);
            let args = (textures, passes, validate_with_format, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_create_multipass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_create_multipass_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_create_multipass(&mut self, textures: &Array < Rid >, passes: &Array < Gd < crate::classes::RdFramebufferPass > >,) -> Rid {
            self.framebuffer_create_multipass_ex(textures, passes,) . done()
        }
        #[inline]
        pub fn framebuffer_create_multipass_ex < 'a > (&'a mut self, textures: &'a Array < Rid >, passes: &'a Array < Gd < crate::classes::RdFramebufferPass > >,) -> ExFramebufferCreateMultipass < 'a > {
            ExFramebufferCreateMultipass::new(self, textures, passes,)
        }
        pub(crate) fn framebuffer_create_empty_full(&mut self, size: Vector2i, samples: crate::classes::rendering_device::TextureSamples, validate_with_format: i64,) -> Rid {
            type CallSig = (Rid, Vector2i, crate::classes::rendering_device::TextureSamples, i64);
            let args = (size, samples, validate_with_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_create_empty", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_create_empty_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_create_empty(&mut self, size: Vector2i,) -> Rid {
            self.framebuffer_create_empty_ex(size,) . done()
        }
        #[inline]
        pub fn framebuffer_create_empty_ex < 'a > (&'a mut self, size: Vector2i,) -> ExFramebufferCreateEmpty < 'a > {
            ExFramebufferCreateEmpty::new(self, size,)
        }
        pub fn framebuffer_get_format(&mut self, framebuffer: Rid,) -> i64 {
            type CallSig = (i64, Rid);
            let args = (framebuffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn framebuffer_is_valid(&self, framebuffer: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (framebuffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sampler_create(&mut self, state: impl AsObjectArg < crate::classes::RdSamplerState >,) -> Rid {
            type CallSig = (Rid, ObjectArg < crate::classes::RdSamplerState >);
            let args = (state.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "sampler_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sampler_is_format_supported_for_filter(&self, format: crate::classes::rendering_device::DataFormat, sampler_filter: crate::classes::rendering_device::SamplerFilter,) -> bool {
            type CallSig = (bool, crate::classes::rendering_device::DataFormat, crate::classes::rendering_device::SamplerFilter);
            let args = (format, sampler_filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "sampler_is_format_supported_for_filter", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn vertex_buffer_create_full(&mut self, size_bytes: u32, data: RefArg < PackedByteArray >, use_as_storage: bool,) -> Rid {
            type CallSig < 'a0, > = (Rid, u32, RefArg < 'a0, PackedByteArray >, bool);
            let args = (size_bytes, data, use_as_storage,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "vertex_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::vertex_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn vertex_buffer_create(&mut self, size_bytes: u32,) -> Rid {
            self.vertex_buffer_create_ex(size_bytes,) . done()
        }
        #[inline]
        pub fn vertex_buffer_create_ex < 'a > (&'a mut self, size_bytes: u32,) -> ExVertexBufferCreate < 'a > {
            ExVertexBufferCreate::new(self, size_bytes,)
        }
        pub fn vertex_format_create(&mut self, vertex_descriptions: &Array < Gd < crate::classes::RdVertexAttribute > >,) -> i64 {
            type CallSig < 'a0, > = (i64, RefArg < 'a0, Array < Gd < crate::classes::RdVertexAttribute > > >);
            let args = (RefArg::new(vertex_descriptions),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7048usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "vertex_format_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn vertex_array_create_full(&mut self, vertex_count: u32, vertex_format: i64, src_buffers: RefArg < Array < Rid > >, offsets: RefArg < PackedInt64Array >,) -> Rid {
            type CallSig < 'a0, 'a1, > = (Rid, u32, i64, RefArg < 'a0, Array < Rid > >, RefArg < 'a1, PackedInt64Array >);
            let args = (vertex_count, vertex_format, src_buffers, offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7049usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "vertex_array_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::vertex_array_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn vertex_array_create(&mut self, vertex_count: u32, vertex_format: i64, src_buffers: &Array < Rid >,) -> Rid {
            self.vertex_array_create_ex(vertex_count, vertex_format, src_buffers,) . done()
        }
        #[inline]
        pub fn vertex_array_create_ex < 'a > (&'a mut self, vertex_count: u32, vertex_format: i64, src_buffers: &'a Array < Rid >,) -> ExVertexArrayCreate < 'a > {
            ExVertexArrayCreate::new(self, vertex_count, vertex_format, src_buffers,)
        }
        pub(crate) fn index_buffer_create_full(&mut self, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat, data: RefArg < PackedByteArray >, use_restart_indices: bool,) -> Rid {
            type CallSig < 'a0, > = (Rid, u32, crate::classes::rendering_device::IndexBufferFormat, RefArg < 'a0, PackedByteArray >, bool);
            let args = (size_indices, format, data, use_restart_indices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7050usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "index_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::index_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn index_buffer_create(&mut self, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat,) -> Rid {
            self.index_buffer_create_ex(size_indices, format,) . done()
        }
        #[inline]
        pub fn index_buffer_create_ex < 'a > (&'a mut self, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat,) -> ExIndexBufferCreate < 'a > {
            ExIndexBufferCreate::new(self, size_indices, format,)
        }
        pub fn index_array_create(&mut self, index_buffer: Rid, index_offset: u32, index_count: u32,) -> Rid {
            type CallSig = (Rid, Rid, u32, u32);
            let args = (index_buffer, index_offset, index_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7051usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "index_array_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn shader_compile_spirv_from_source_full(&mut self, shader_source: ObjectArg < crate::classes::RdShaderSource >, allow_cache: bool,) -> Option < Gd < crate::classes::RdShaderSpirv > > {
            type CallSig = (Option < Gd < crate::classes::RdShaderSpirv > >, ObjectArg < crate::classes::RdShaderSource >, bool);
            let args = (shader_source, allow_cache,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7052usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_compile_spirv_from_source", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_compile_spirv_from_source_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_compile_spirv_from_source(&mut self, shader_source: impl AsObjectArg < crate::classes::RdShaderSource >,) -> Option < Gd < crate::classes::RdShaderSpirv > > {
            self.shader_compile_spirv_from_source_ex(shader_source,) . done()
        }
        #[inline]
        pub fn shader_compile_spirv_from_source_ex < 'a > (&'a mut self, shader_source: impl AsObjectArg < crate::classes::RdShaderSource >,) -> ExShaderCompileSpirvFromSource < 'a > {
            ExShaderCompileSpirvFromSource::new(self, shader_source,)
        }
        pub(crate) fn shader_compile_binary_from_spirv_full(&mut self, spirv_data: ObjectArg < crate::classes::RdShaderSpirv >, name: CowArg < GString >,) -> PackedByteArray {
            type CallSig < 'a0, > = (PackedByteArray, ObjectArg < crate::classes::RdShaderSpirv >, CowArg < 'a0, GString >);
            let args = (spirv_data, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_compile_binary_from_spirv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_compile_binary_from_spirv_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_compile_binary_from_spirv(&mut self, spirv_data: impl AsObjectArg < crate::classes::RdShaderSpirv >,) -> PackedByteArray {
            self.shader_compile_binary_from_spirv_ex(spirv_data,) . done()
        }
        #[inline]
        pub fn shader_compile_binary_from_spirv_ex < 'a > (&'a mut self, spirv_data: impl AsObjectArg < crate::classes::RdShaderSpirv >,) -> ExShaderCompileBinaryFromSpirv < 'a > {
            ExShaderCompileBinaryFromSpirv::new(self, spirv_data,)
        }
        pub(crate) fn shader_create_from_spirv_full(&mut self, spirv_data: ObjectArg < crate::classes::RdShaderSpirv >, name: CowArg < GString >,) -> Rid {
            type CallSig < 'a0, > = (Rid, ObjectArg < crate::classes::RdShaderSpirv >, CowArg < 'a0, GString >);
            let args = (spirv_data, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_create_from_spirv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_create_from_spirv_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_create_from_spirv(&mut self, spirv_data: impl AsObjectArg < crate::classes::RdShaderSpirv >,) -> Rid {
            self.shader_create_from_spirv_ex(spirv_data,) . done()
        }
        #[inline]
        pub fn shader_create_from_spirv_ex < 'a > (&'a mut self, spirv_data: impl AsObjectArg < crate::classes::RdShaderSpirv >,) -> ExShaderCreateFromSpirv < 'a > {
            ExShaderCreateFromSpirv::new(self, spirv_data,)
        }
        pub(crate) fn shader_create_from_bytecode_full(&mut self, binary_data: RefArg < PackedByteArray >, placeholder_rid: Rid,) -> Rid {
            type CallSig < 'a0, > = (Rid, RefArg < 'a0, PackedByteArray >, Rid);
            let args = (binary_data, placeholder_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_create_from_bytecode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_create_from_bytecode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_create_from_bytecode(&mut self, binary_data: &PackedByteArray,) -> Rid {
            self.shader_create_from_bytecode_ex(binary_data,) . done()
        }
        #[inline]
        pub fn shader_create_from_bytecode_ex < 'a > (&'a mut self, binary_data: &'a PackedByteArray,) -> ExShaderCreateFromBytecode < 'a > {
            ExShaderCreateFromBytecode::new(self, binary_data,)
        }
        pub fn shader_create_placeholder(&mut self,) -> Rid {
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_create_placeholder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn shader_get_vertex_input_attribute_mask(&mut self, shader: Rid,) -> u64 {
            type CallSig = (u64, Rid);
            let args = (shader,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_get_vertex_input_attribute_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn uniform_buffer_create_full(&mut self, size_bytes: u32, data: RefArg < PackedByteArray >,) -> Rid {
            type CallSig < 'a0, > = (Rid, u32, RefArg < 'a0, PackedByteArray >);
            let args = (size_bytes, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7058usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "uniform_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::uniform_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn uniform_buffer_create(&mut self, size_bytes: u32,) -> Rid {
            self.uniform_buffer_create_ex(size_bytes,) . done()
        }
        #[inline]
        pub fn uniform_buffer_create_ex < 'a > (&'a mut self, size_bytes: u32,) -> ExUniformBufferCreate < 'a > {
            ExUniformBufferCreate::new(self, size_bytes,)
        }
        pub(crate) fn storage_buffer_create_full(&mut self, size_bytes: u32, data: RefArg < PackedByteArray >, usage: crate::classes::rendering_device::StorageBufferUsage,) -> Rid {
            type CallSig < 'a0, > = (Rid, u32, RefArg < 'a0, PackedByteArray >, crate::classes::rendering_device::StorageBufferUsage);
            let args = (size_bytes, data, usage,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7059usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "storage_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::storage_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn storage_buffer_create(&mut self, size_bytes: u32,) -> Rid {
            self.storage_buffer_create_ex(size_bytes,) . done()
        }
        #[inline]
        pub fn storage_buffer_create_ex < 'a > (&'a mut self, size_bytes: u32,) -> ExStorageBufferCreate < 'a > {
            ExStorageBufferCreate::new(self, size_bytes,)
        }
        pub(crate) fn texture_buffer_create_full(&mut self, size_bytes: u32, format: crate::classes::rendering_device::DataFormat, data: RefArg < PackedByteArray >,) -> Rid {
            type CallSig < 'a0, > = (Rid, u32, crate::classes::rendering_device::DataFormat, RefArg < 'a0, PackedByteArray >);
            let args = (size_bytes, format, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_buffer_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_buffer_create(&mut self, size_bytes: u32, format: crate::classes::rendering_device::DataFormat,) -> Rid {
            self.texture_buffer_create_ex(size_bytes, format,) . done()
        }
        #[inline]
        pub fn texture_buffer_create_ex < 'a > (&'a mut self, size_bytes: u32, format: crate::classes::rendering_device::DataFormat,) -> ExTextureBufferCreate < 'a > {
            ExTextureBufferCreate::new(self, size_bytes, format,)
        }
        pub fn uniform_set_create(&mut self, uniforms: &Array < Gd < crate::classes::RdUniform > >, shader: Rid, shader_set: u32,) -> Rid {
            type CallSig < 'a0, > = (Rid, RefArg < 'a0, Array < Gd < crate::classes::RdUniform > > >, Rid, u32);
            let args = (RefArg::new(uniforms), shader, shader_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "uniform_set_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn uniform_set_is_valid(&mut self, uniform_set: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (uniform_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "uniform_set_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn buffer_copy(&mut self, src_buffer: Rid, dst_buffer: Rid, src_offset: u32, dst_offset: u32, size: u32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, Rid, Rid, u32, u32, u32);
            let args = (src_buffer, dst_buffer, src_offset, dst_offset, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_copy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn buffer_update(&mut self, buffer: Rid, offset: u32, size_bytes: u32, data: &PackedByteArray,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, Rid, u32, u32, RefArg < 'a0, PackedByteArray >);
            let args = (buffer, offset, size_bytes, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn buffer_clear(&mut self, buffer: Rid, offset: u32, size_bytes: u32,) -> crate::global::Error {
            type CallSig = (crate::global::Error, Rid, u32, u32);
            let args = (buffer, offset, size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn buffer_get_data_full(&mut self, buffer: Rid, offset_bytes: u32, size_bytes: u32,) -> PackedByteArray {
            type CallSig = (PackedByteArray, Rid, u32, u32);
            let args = (buffer, offset_bytes, size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::buffer_get_data_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn buffer_get_data(&mut self, buffer: Rid,) -> PackedByteArray {
            self.buffer_get_data_ex(buffer,) . done()
        }
        #[inline]
        pub fn buffer_get_data_ex < 'a > (&'a mut self, buffer: Rid,) -> ExBufferGetData < 'a > {
            ExBufferGetData::new(self, buffer,)
        }
        pub(crate) fn render_pipeline_create_full(&mut self, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: ObjectArg < crate::classes::RdPipelineRasterizationState >, multisample_state: ObjectArg < crate::classes::RdPipelineMultisampleState >, stencil_state: ObjectArg < crate::classes::RdPipelineDepthStencilState >, color_blend_state: ObjectArg < crate::classes::RdPipelineColorBlendState >, dynamic_state_flags: crate::classes::rendering_device::PipelineDynamicStateFlags, for_render_pass: u32, specialization_constants: RefArg < Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,) -> Rid {
            type CallSig < 'a0, > = (Rid, Rid, i64, i64, crate::classes::rendering_device::RenderPrimitive, ObjectArg < crate::classes::RdPipelineRasterizationState >, ObjectArg < crate::classes::RdPipelineMultisampleState >, ObjectArg < crate::classes::RdPipelineDepthStencilState >, ObjectArg < crate::classes::RdPipelineColorBlendState >, crate::classes::rendering_device::PipelineDynamicStateFlags, u32, RefArg < 'a0, Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >);
            let args = (shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state, dynamic_state_flags, for_render_pass, specialization_constants,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "render_pipeline_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::render_pipeline_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn render_pipeline_create(&mut self, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: impl AsObjectArg < crate::classes::RdPipelineRasterizationState >, multisample_state: impl AsObjectArg < crate::classes::RdPipelineMultisampleState >, stencil_state: impl AsObjectArg < crate::classes::RdPipelineDepthStencilState >, color_blend_state: impl AsObjectArg < crate::classes::RdPipelineColorBlendState >,) -> Rid {
            self.render_pipeline_create_ex(shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state,) . done()
        }
        #[inline]
        pub fn render_pipeline_create_ex < 'a > (&'a mut self, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: impl AsObjectArg < crate::classes::RdPipelineRasterizationState >, multisample_state: impl AsObjectArg < crate::classes::RdPipelineMultisampleState >, stencil_state: impl AsObjectArg < crate::classes::RdPipelineDepthStencilState >, color_blend_state: impl AsObjectArg < crate::classes::RdPipelineColorBlendState >,) -> ExRenderPipelineCreate < 'a > {
            ExRenderPipelineCreate::new(self, shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state,)
        }
        pub fn render_pipeline_is_valid(&mut self, render_pipeline: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (render_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "render_pipeline_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn compute_pipeline_create_full(&mut self, shader: Rid, specialization_constants: RefArg < Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,) -> Rid {
            type CallSig < 'a0, > = (Rid, Rid, RefArg < 'a0, Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >);
            let args = (shader, specialization_constants,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_pipeline_create", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::compute_pipeline_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn compute_pipeline_create(&mut self, shader: Rid,) -> Rid {
            self.compute_pipeline_create_ex(shader,) . done()
        }
        #[inline]
        pub fn compute_pipeline_create_ex < 'a > (&'a mut self, shader: Rid,) -> ExComputePipelineCreate < 'a > {
            ExComputePipelineCreate::new(self, shader,)
        }
        pub fn compute_pipeline_is_valid(&mut self, compute_pipeline: Rid,) -> bool {
            type CallSig = (bool, Rid);
            let args = (compute_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_pipeline_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn screen_get_width_full(&self, screen: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "screen_get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_width_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_width(&self,) -> i32 {
            self.screen_get_width_ex() . done()
        }
        #[inline]
        pub fn screen_get_width_ex < 'a > (&'a self,) -> ExScreenGetWidth < 'a > {
            ExScreenGetWidth::new(self,)
        }
        pub(crate) fn screen_get_height_full(&self, screen: i32,) -> i32 {
            type CallSig = (i32, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7072usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "screen_get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_height_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_height(&self,) -> i32 {
            self.screen_get_height_ex() . done()
        }
        #[inline]
        pub fn screen_get_height_ex < 'a > (&'a self,) -> ExScreenGetHeight < 'a > {
            ExScreenGetHeight::new(self,)
        }
        pub(crate) fn screen_get_framebuffer_format_full(&self, screen: i32,) -> i64 {
            type CallSig = (i64, i32);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7073usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "screen_get_framebuffer_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_framebuffer_format_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_framebuffer_format(&self,) -> i64 {
            self.screen_get_framebuffer_format_ex() . done()
        }
        #[inline]
        pub fn screen_get_framebuffer_format_ex < 'a > (&'a self,) -> ExScreenGetFramebufferFormat < 'a > {
            ExScreenGetFramebufferFormat::new(self,)
        }
        pub(crate) fn draw_list_begin_for_screen_full(&mut self, screen: i32, clear_color: Color,) -> i64 {
            type CallSig = (i64, i32, Color);
            let args = (screen, clear_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7074usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_begin_for_screen", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_begin_for_screen_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_begin_for_screen(&mut self,) -> i64 {
            self.draw_list_begin_for_screen_ex() . done()
        }
        #[inline]
        pub fn draw_list_begin_for_screen_ex < 'a > (&'a mut self,) -> ExDrawListBeginForScreen < 'a > {
            ExDrawListBeginForScreen::new(self,)
        }
        pub(crate) fn draw_list_begin_full(&mut self, framebuffer: Rid, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction, clear_color_values: RefArg < PackedColorArray >, clear_depth: f32, clear_stencil: u32, region: Rect2,) -> i64 {
            type CallSig < 'a0, > = (i64, Rid, crate::classes::rendering_device::InitialAction, crate::classes::rendering_device::FinalAction, crate::classes::rendering_device::InitialAction, crate::classes::rendering_device::FinalAction, RefArg < 'a0, PackedColorArray >, f32, u32, Rect2);
            let args = (framebuffer, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values, clear_depth, clear_stencil, region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7075usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_begin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_begin(&mut self, framebuffer: Rid, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction,) -> i64 {
            self.draw_list_begin_ex(framebuffer, initial_color_action, final_color_action, initial_depth_action, final_depth_action,) . done()
        }
        #[inline]
        pub fn draw_list_begin_ex < 'a > (&'a mut self, framebuffer: Rid, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction,) -> ExDrawListBegin < 'a > {
            ExDrawListBegin::new(self, framebuffer, initial_color_action, final_color_action, initial_depth_action, final_depth_action,)
        }
        pub(crate) fn draw_list_begin_split_full(&mut self, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction, clear_color_values: RefArg < PackedColorArray >, clear_depth: f32, clear_stencil: u32, region: Rect2, storage_textures: RefArg < Array < Rid > >,) -> PackedInt64Array {
            type CallSig < 'a0, 'a1, > = (PackedInt64Array, Rid, u32, crate::classes::rendering_device::InitialAction, crate::classes::rendering_device::FinalAction, crate::classes::rendering_device::InitialAction, crate::classes::rendering_device::FinalAction, RefArg < 'a0, PackedColorArray >, f32, u32, Rect2, RefArg < 'a1, Array < Rid > >);
            let args = (framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values, clear_depth, clear_stencil, region, storage_textures,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7076usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_begin_split", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_begin_split_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_begin_split(&mut self, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction,) -> PackedInt64Array {
            self.draw_list_begin_split_ex(framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action,) . done()
        }
        #[inline]
        pub fn draw_list_begin_split_ex < 'a > (&'a mut self, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction,) -> ExDrawListBeginSplit < 'a > {
            ExDrawListBeginSplit::new(self, framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action,)
        }
        pub fn draw_list_set_blend_constants(&mut self, draw_list: i64, color: Color,) {
            type CallSig = ((), i64, Color);
            let args = (draw_list, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7077usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_set_blend_constants", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_bind_render_pipeline(&mut self, draw_list: i64, render_pipeline: Rid,) {
            type CallSig = ((), i64, Rid);
            let args = (draw_list, render_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7078usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_bind_render_pipeline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_bind_uniform_set(&mut self, draw_list: i64, uniform_set: Rid, set_index: u32,) {
            type CallSig = ((), i64, Rid, u32);
            let args = (draw_list, uniform_set, set_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7079usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_bind_uniform_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_bind_vertex_array(&mut self, draw_list: i64, vertex_array: Rid,) {
            type CallSig = ((), i64, Rid);
            let args = (draw_list, vertex_array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7080usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_bind_vertex_array", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_bind_index_array(&mut self, draw_list: i64, index_array: Rid,) {
            type CallSig = ((), i64, Rid);
            let args = (draw_list, index_array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7081usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_bind_index_array", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_set_push_constant(&mut self, draw_list: i64, buffer: &PackedByteArray, size_bytes: u32,) {
            type CallSig < 'a0, > = ((), i64, RefArg < 'a0, PackedByteArray >, u32);
            let args = (draw_list, RefArg::new(buffer), size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7082usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_set_push_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn draw_list_draw_full(&mut self, draw_list: i64, use_indices: bool, instances: u32, procedural_vertex_count: u32,) {
            type CallSig = ((), i64, bool, u32, u32);
            let args = (draw_list, use_indices, instances, procedural_vertex_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7083usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_draw", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_draw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_draw(&mut self, draw_list: i64, use_indices: bool, instances: u32,) {
            self.draw_list_draw_ex(draw_list, use_indices, instances,) . done()
        }
        #[inline]
        pub fn draw_list_draw_ex < 'a > (&'a mut self, draw_list: i64, use_indices: bool, instances: u32,) -> ExDrawListDraw < 'a > {
            ExDrawListDraw::new(self, draw_list, use_indices, instances,)
        }
        pub(crate) fn draw_list_enable_scissor_full(&mut self, draw_list: i64, rect: Rect2,) {
            type CallSig = ((), i64, Rect2);
            let args = (draw_list, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7084usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_enable_scissor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_enable_scissor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_enable_scissor(&mut self, draw_list: i64,) {
            self.draw_list_enable_scissor_ex(draw_list,) . done()
        }
        #[inline]
        pub fn draw_list_enable_scissor_ex < 'a > (&'a mut self, draw_list: i64,) -> ExDrawListEnableScissor < 'a > {
            ExDrawListEnableScissor::new(self, draw_list,)
        }
        pub fn draw_list_disable_scissor(&mut self, draw_list: i64,) {
            type CallSig = ((), i64);
            let args = (draw_list,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7085usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_disable_scissor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_switch_to_next_pass(&mut self,) -> i64 {
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7086usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_switch_to_next_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_switch_to_next_pass_split(&mut self, splits: u32,) -> PackedInt64Array {
            type CallSig = (PackedInt64Array, u32);
            let args = (splits,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7087usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_switch_to_next_pass_split", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_list_end(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7088usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_begin(&mut self,) -> i64 {
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7089usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_bind_compute_pipeline(&mut self, compute_list: i64, compute_pipeline: Rid,) {
            type CallSig = ((), i64, Rid);
            let args = (compute_list, compute_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7090usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_bind_compute_pipeline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_set_push_constant(&mut self, compute_list: i64, buffer: &PackedByteArray, size_bytes: u32,) {
            type CallSig < 'a0, > = ((), i64, RefArg < 'a0, PackedByteArray >, u32);
            let args = (compute_list, RefArg::new(buffer), size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7091usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_set_push_constant", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_bind_uniform_set(&mut self, compute_list: i64, uniform_set: Rid, set_index: u32,) {
            type CallSig = ((), i64, Rid, u32);
            let args = (compute_list, uniform_set, set_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7092usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_bind_uniform_set", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_dispatch(&mut self, compute_list: i64, x_groups: u32, y_groups: u32, z_groups: u32,) {
            type CallSig = ((), i64, u32, u32, u32);
            let args = (compute_list, x_groups, y_groups, z_groups,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7093usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_dispatch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_dispatch_indirect(&mut self, compute_list: i64, buffer: Rid, offset: u32,) {
            type CallSig = ((), i64, Rid, u32);
            let args = (compute_list, buffer, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7094usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_dispatch_indirect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_add_barrier(&mut self, compute_list: i64,) {
            type CallSig = ((), i64);
            let args = (compute_list,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7095usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_add_barrier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compute_list_end(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7096usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7097usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "free_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn capture_timestamp(&mut self, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7098usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "capture_timestamp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamps_count(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7099usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamps_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamps_frame(&self,) -> u64 {
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7100usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamps_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamp_gpu_time(&self, index: u32,) -> u64 {
            type CallSig = (u64, u32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7101usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamp_gpu_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamp_cpu_time(&self, index: u32,) -> u64 {
            type CallSig = (u64, u32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamp_cpu_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_captured_timestamp_name(&self, index: u32,) -> GString {
            type CallSig = (GString, u32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamp_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn limit_get(&self, limit: crate::classes::rendering_device::Limit,) -> u64 {
            type CallSig = (u64, crate::classes::rendering_device::Limit);
            let args = (limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "limit_get", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_frame_delay(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_frame_delay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn submit(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "submit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn sync(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "sync", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn barrier_full(&mut self, from: crate::classes::rendering_device::BarrierMask, to: crate::classes::rendering_device::BarrierMask,) {
            type CallSig = ((), crate::classes::rendering_device::BarrierMask, crate::classes::rendering_device::BarrierMask);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "barrier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::barrier_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn barrier(&mut self,) {
            self.barrier_ex() . done()
        }
        #[inline]
        pub fn barrier_ex < 'a > (&'a mut self,) -> ExBarrier < 'a > {
            ExBarrier::new(self,)
        }
        pub fn full_barrier(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "full_barrier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_local_device(&mut self,) -> Option < Gd < crate::classes::RenderingDevice > > {
            type CallSig = (Option < Gd < crate::classes::RenderingDevice > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "create_local_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_resource_name(&mut self, id: Rid, name: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), Rid, CowArg < 'a0, GString >);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "set_resource_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_command_begin_label(&mut self, name: impl AsArg < GString >, color: Color,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, Color);
            let args = (name.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7112usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_command_begin_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_command_insert_label(&mut self, name: impl AsArg < GString >, color: Color,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >, Color);
            let args = (name.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7113usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_command_insert_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn draw_command_end_label(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7114usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_command_end_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_device_vendor_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7115usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_vendor_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_device_name(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7116usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_device_pipeline_cache_uuid(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7117usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_pipeline_cache_uuid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_memory_usage(&self, type_: crate::classes::rendering_device::MemoryType,) -> u64 {
            type CallSig = (u64, crate::classes::rendering_device::MemoryType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7118usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_memory_usage", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_driver_resource(&mut self, resource: crate::classes::rendering_device::DriverResource, rid: Rid, index: u64,) -> u64 {
            type CallSig = (u64, crate::classes::rendering_device::DriverResource, Rid, u64);
            let args = (resource, rid, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7119usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_driver_resource", self.object_ptr, self.__checked_id(), args,)
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
        pub const INVALID_ID: i32 = - 1i32;
        pub const INVALID_FORMAT_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for RenderingDevice {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"RenderingDevice"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RenderingDevice {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RenderingDevice {
        
    }
    impl std::ops::Deref for RenderingDevice {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderingDevice {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RenderingDevice`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_RenderingDevice {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RenderingDevice > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_create_ex`][super::RenderingDevice::texture_create_ex]."]
#[must_use]
pub struct ExTextureCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, format: ObjectCow < crate::classes::RdTextureFormat >, view: ObjectCow < crate::classes::RdTextureView >, data: CowArg < 'a, Array < PackedByteArray > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, format: impl AsObjectArg < crate::classes::RdTextureFormat >, view: impl AsObjectArg < crate::classes::RdTextureView >,) -> Self {
        let data = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format: format.consume_arg(), view: view.consume_arg(), data: CowArg::Owned(data),
        }
    }
    #[inline]
    pub fn data(self, data: &'a Array < PackedByteArray >) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, format, view, data,
        }
        = self;
        re_export::RenderingDevice::texture_create_full(surround_object, format.cow_as_object_arg(), view.cow_as_object_arg(), data.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_create_shared_from_slice_ex`][super::RenderingDevice::texture_create_shared_from_slice_ex]."]
#[must_use]
pub struct ExTextureCreateSharedFromSlice < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, view: ObjectCow < crate::classes::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32, mipmaps: u32, slice_type: crate::classes::rendering_device::TextureSliceType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureCreateSharedFromSlice < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, view: impl AsObjectArg < crate::classes::RdTextureView >, with_texture: Rid, layer: u32, mipmap: u32,) -> Self {
        let mipmaps = 1u32;
        let slice_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, view: view.consume_arg(), with_texture: with_texture, layer: layer, mipmap: mipmap, mipmaps: mipmaps, slice_type: slice_type,
        }
    }
    #[inline]
    pub fn mipmaps(self, mipmaps: u32) -> Self {
        Self {
            mipmaps: mipmaps, .. self
        }
    }
    #[inline]
    pub fn slice_type(self, slice_type: crate::classes::rendering_device::TextureSliceType) -> Self {
        Self {
            slice_type: slice_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, view, with_texture, layer, mipmap, mipmaps, slice_type,
        }
        = self;
        re_export::RenderingDevice::texture_create_shared_from_slice_full(surround_object, view.cow_as_object_arg(), with_texture, layer, mipmap, mipmaps, slice_type,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_create_ex`][super::RenderingDevice::framebuffer_format_create_ex]."]
#[must_use]
pub struct ExFramebufferFormatCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, attachments: CowArg < 'a, Array < Gd < crate::classes::RdAttachmentFormat > > >, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, attachments: &'a Array < Gd < crate::classes::RdAttachmentFormat > >,) -> Self {
        let view_count = 1u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, attachments: CowArg::Borrowed(attachments), view_count: view_count,
        }
    }
    #[inline]
    pub fn view_count(self, view_count: u32) -> Self {
        Self {
            view_count: view_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, attachments, view_count,
        }
        = self;
        re_export::RenderingDevice::framebuffer_format_create_full(surround_object, attachments.cow_as_arg(), view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_create_multipass_ex`][super::RenderingDevice::framebuffer_format_create_multipass_ex]."]
#[must_use]
pub struct ExFramebufferFormatCreateMultipass < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, attachments: CowArg < 'a, Array < Gd < crate::classes::RdAttachmentFormat > > >, passes: CowArg < 'a, Array < Gd < crate::classes::RdFramebufferPass > > >, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatCreateMultipass < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, attachments: &'a Array < Gd < crate::classes::RdAttachmentFormat > >, passes: &'a Array < Gd < crate::classes::RdFramebufferPass > >,) -> Self {
        let view_count = 1u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, attachments: CowArg::Borrowed(attachments), passes: CowArg::Borrowed(passes), view_count: view_count,
        }
    }
    #[inline]
    pub fn view_count(self, view_count: u32) -> Self {
        Self {
            view_count: view_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, attachments, passes, view_count,
        }
        = self;
        re_export::RenderingDevice::framebuffer_format_create_multipass_full(surround_object, attachments.cow_as_arg(), passes.cow_as_arg(), view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_create_empty_ex`][super::RenderingDevice::framebuffer_format_create_empty_ex]."]
#[must_use]
pub struct ExFramebufferFormatCreateEmpty < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, samples: crate::classes::rendering_device::TextureSamples,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatCreateEmpty < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        let samples = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, samples: samples,
        }
    }
    #[inline]
    pub fn samples(self, samples: crate::classes::rendering_device::TextureSamples) -> Self {
        Self {
            samples: samples, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, samples,
        }
        = self;
        re_export::RenderingDevice::framebuffer_format_create_empty_full(surround_object, samples,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_get_texture_samples_ex`][super::RenderingDevice::framebuffer_format_get_texture_samples_ex]."]
#[must_use]
pub struct ExFramebufferFormatGetTextureSamples < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, format: i64, render_pass: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatGetTextureSamples < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, format: i64,) -> Self {
        let render_pass = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format: format, render_pass: render_pass,
        }
    }
    #[inline]
    pub fn render_pass(self, render_pass: u32) -> Self {
        Self {
            render_pass: render_pass, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::rendering_device::TextureSamples {
        let Self {
            _phantom, surround_object, format, render_pass,
        }
        = self;
        re_export::RenderingDevice::framebuffer_format_get_texture_samples_full(surround_object, format, render_pass,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_create_ex`][super::RenderingDevice::framebuffer_create_ex]."]
#[must_use]
pub struct ExFramebufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, textures: CowArg < 'a, Array < Rid > >, validate_with_format: i64, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, textures: &'a Array < Rid >,) -> Self {
        let validate_with_format = - 1i64;
        let view_count = 1u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, textures: CowArg::Borrowed(textures), validate_with_format: validate_with_format, view_count: view_count,
        }
    }
    #[inline]
    pub fn validate_with_format(self, validate_with_format: i64) -> Self {
        Self {
            validate_with_format: validate_with_format, .. self
        }
    }
    #[inline]
    pub fn view_count(self, view_count: u32) -> Self {
        Self {
            view_count: view_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, textures, validate_with_format, view_count,
        }
        = self;
        re_export::RenderingDevice::framebuffer_create_full(surround_object, textures.cow_as_arg(), validate_with_format, view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_create_multipass_ex`][super::RenderingDevice::framebuffer_create_multipass_ex]."]
#[must_use]
pub struct ExFramebufferCreateMultipass < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, textures: CowArg < 'a, Array < Rid > >, passes: CowArg < 'a, Array < Gd < crate::classes::RdFramebufferPass > > >, validate_with_format: i64, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferCreateMultipass < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, textures: &'a Array < Rid >, passes: &'a Array < Gd < crate::classes::RdFramebufferPass > >,) -> Self {
        let validate_with_format = - 1i64;
        let view_count = 1u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, textures: CowArg::Borrowed(textures), passes: CowArg::Borrowed(passes), validate_with_format: validate_with_format, view_count: view_count,
        }
    }
    #[inline]
    pub fn validate_with_format(self, validate_with_format: i64) -> Self {
        Self {
            validate_with_format: validate_with_format, .. self
        }
    }
    #[inline]
    pub fn view_count(self, view_count: u32) -> Self {
        Self {
            view_count: view_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, textures, passes, validate_with_format, view_count,
        }
        = self;
        re_export::RenderingDevice::framebuffer_create_multipass_full(surround_object, textures.cow_as_arg(), passes.cow_as_arg(), validate_with_format, view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_create_empty_ex`][super::RenderingDevice::framebuffer_create_empty_ex]."]
#[must_use]
pub struct ExFramebufferCreateEmpty < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size: Vector2i, samples: crate::classes::rendering_device::TextureSamples, validate_with_format: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferCreateEmpty < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size: Vector2i,) -> Self {
        let samples = crate::obj::EngineEnum::from_ord(0);
        let validate_with_format = - 1i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size: size, samples: samples, validate_with_format: validate_with_format,
        }
    }
    #[inline]
    pub fn samples(self, samples: crate::classes::rendering_device::TextureSamples) -> Self {
        Self {
            samples: samples, .. self
        }
    }
    #[inline]
    pub fn validate_with_format(self, validate_with_format: i64) -> Self {
        Self {
            validate_with_format: validate_with_format, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size, samples, validate_with_format,
        }
        = self;
        re_export::RenderingDevice::framebuffer_create_empty_full(surround_object, size, samples, validate_with_format,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::vertex_buffer_create_ex`][super::RenderingDevice::vertex_buffer_create_ex]."]
#[must_use]
pub struct ExVertexBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, data: CowArg < 'a, PackedByteArray >, use_as_storage: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVertexBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32,) -> Self {
        let data = PackedByteArray::new();
        let use_as_storage = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_bytes: size_bytes, data: CowArg::Owned(data), use_as_storage: use_as_storage,
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn use_as_storage(self, use_as_storage: bool) -> Self {
        Self {
            use_as_storage: use_as_storage, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_bytes, data, use_as_storage,
        }
        = self;
        re_export::RenderingDevice::vertex_buffer_create_full(surround_object, size_bytes, data.cow_as_arg(), use_as_storage,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::vertex_array_create_ex`][super::RenderingDevice::vertex_array_create_ex]."]
#[must_use]
pub struct ExVertexArrayCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, vertex_count: u32, vertex_format: i64, src_buffers: CowArg < 'a, Array < Rid > >, offsets: CowArg < 'a, PackedInt64Array >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVertexArrayCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, vertex_count: u32, vertex_format: i64, src_buffers: &'a Array < Rid >,) -> Self {
        let offsets = PackedInt64Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, vertex_count: vertex_count, vertex_format: vertex_format, src_buffers: CowArg::Borrowed(src_buffers), offsets: CowArg::Owned(offsets),
        }
    }
    #[inline]
    pub fn offsets(self, offsets: &'a PackedInt64Array) -> Self {
        Self {
            offsets: CowArg::Borrowed(offsets), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, vertex_count, vertex_format, src_buffers, offsets,
        }
        = self;
        re_export::RenderingDevice::vertex_array_create_full(surround_object, vertex_count, vertex_format, src_buffers.cow_as_arg(), offsets.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::index_buffer_create_ex`][super::RenderingDevice::index_buffer_create_ex]."]
#[must_use]
pub struct ExIndexBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat, data: CowArg < 'a, PackedByteArray >, use_restart_indices: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIndexBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat,) -> Self {
        let data = PackedByteArray::new();
        let use_restart_indices = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_indices: size_indices, format: format, data: CowArg::Owned(data), use_restart_indices: use_restart_indices,
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn use_restart_indices(self, use_restart_indices: bool) -> Self {
        Self {
            use_restart_indices: use_restart_indices, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_indices, format, data, use_restart_indices,
        }
        = self;
        re_export::RenderingDevice::index_buffer_create_full(surround_object, size_indices, format, data.cow_as_arg(), use_restart_indices,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_compile_spirv_from_source_ex`][super::RenderingDevice::shader_compile_spirv_from_source_ex]."]
#[must_use]
pub struct ExShaderCompileSpirvFromSource < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, shader_source: ObjectCow < crate::classes::RdShaderSource >, allow_cache: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCompileSpirvFromSource < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, shader_source: impl AsObjectArg < crate::classes::RdShaderSource >,) -> Self {
        let allow_cache = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader_source: shader_source.consume_arg(), allow_cache: allow_cache,
        }
    }
    #[inline]
    pub fn allow_cache(self, allow_cache: bool) -> Self {
        Self {
            allow_cache: allow_cache, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::RdShaderSpirv > > {
        let Self {
            _phantom, surround_object, shader_source, allow_cache,
        }
        = self;
        re_export::RenderingDevice::shader_compile_spirv_from_source_full(surround_object, shader_source.cow_as_object_arg(), allow_cache,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_compile_binary_from_spirv_ex`][super::RenderingDevice::shader_compile_binary_from_spirv_ex]."]
#[must_use]
pub struct ExShaderCompileBinaryFromSpirv < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, spirv_data: ObjectCow < crate::classes::RdShaderSpirv >, name: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCompileBinaryFromSpirv < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, spirv_data: impl AsObjectArg < crate::classes::RdShaderSpirv >,) -> Self {
        let name = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, spirv_data: spirv_data.consume_arg(), name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < GString > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, spirv_data, name,
        }
        = self;
        re_export::RenderingDevice::shader_compile_binary_from_spirv_full(surround_object, spirv_data.cow_as_object_arg(), name,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_create_from_spirv_ex`][super::RenderingDevice::shader_create_from_spirv_ex]."]
#[must_use]
pub struct ExShaderCreateFromSpirv < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, spirv_data: ObjectCow < crate::classes::RdShaderSpirv >, name: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCreateFromSpirv < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, spirv_data: impl AsObjectArg < crate::classes::RdShaderSpirv >,) -> Self {
        let name = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, spirv_data: spirv_data.consume_arg(), name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < GString > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, spirv_data, name,
        }
        = self;
        re_export::RenderingDevice::shader_create_from_spirv_full(surround_object, spirv_data.cow_as_object_arg(), name,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_create_from_bytecode_ex`][super::RenderingDevice::shader_create_from_bytecode_ex]."]
#[must_use]
pub struct ExShaderCreateFromBytecode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, binary_data: CowArg < 'a, PackedByteArray >, placeholder_rid: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCreateFromBytecode < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, binary_data: &'a PackedByteArray,) -> Self {
        let placeholder_rid = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, binary_data: CowArg::Borrowed(binary_data), placeholder_rid: placeholder_rid,
        }
    }
    #[inline]
    pub fn placeholder_rid(self, placeholder_rid: Rid) -> Self {
        Self {
            placeholder_rid: placeholder_rid, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, binary_data, placeholder_rid,
        }
        = self;
        re_export::RenderingDevice::shader_create_from_bytecode_full(surround_object, binary_data.cow_as_arg(), placeholder_rid,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::uniform_buffer_create_ex`][super::RenderingDevice::uniform_buffer_create_ex]."]
#[must_use]
pub struct ExUniformBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, data: CowArg < 'a, PackedByteArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExUniformBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32,) -> Self {
        let data = PackedByteArray::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_bytes: size_bytes, data: CowArg::Owned(data),
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_bytes, data,
        }
        = self;
        re_export::RenderingDevice::uniform_buffer_create_full(surround_object, size_bytes, data.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::storage_buffer_create_ex`][super::RenderingDevice::storage_buffer_create_ex]."]
#[must_use]
pub struct ExStorageBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, data: CowArg < 'a, PackedByteArray >, usage: crate::classes::rendering_device::StorageBufferUsage,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStorageBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32,) -> Self {
        let data = PackedByteArray::new();
        let usage = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_bytes: size_bytes, data: CowArg::Owned(data), usage: usage,
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn usage(self, usage: crate::classes::rendering_device::StorageBufferUsage) -> Self {
        Self {
            usage: usage, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_bytes, data, usage,
        }
        = self;
        re_export::RenderingDevice::storage_buffer_create_full(surround_object, size_bytes, data.cow_as_arg(), usage,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_buffer_create_ex`][super::RenderingDevice::texture_buffer_create_ex]."]
#[must_use]
pub struct ExTextureBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, format: crate::classes::rendering_device::DataFormat, data: CowArg < 'a, PackedByteArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, format: crate::classes::rendering_device::DataFormat,) -> Self {
        let data = PackedByteArray::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_bytes: size_bytes, format: format, data: CowArg::Owned(data),
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_bytes, format, data,
        }
        = self;
        re_export::RenderingDevice::texture_buffer_create_full(surround_object, size_bytes, format, data.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::buffer_get_data_ex`][super::RenderingDevice::buffer_get_data_ex]."]
#[must_use]
pub struct ExBufferGetData < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, buffer: Rid, offset_bytes: u32, size_bytes: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBufferGetData < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, buffer: Rid,) -> Self {
        let offset_bytes = 0u32;
        let size_bytes = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, buffer: buffer, offset_bytes: offset_bytes, size_bytes: size_bytes,
        }
    }
    #[inline]
    pub fn offset_bytes(self, offset_bytes: u32) -> Self {
        Self {
            offset_bytes: offset_bytes, .. self
        }
    }
    #[inline]
    pub fn size_bytes(self, size_bytes: u32) -> Self {
        Self {
            size_bytes: size_bytes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, buffer, offset_bytes, size_bytes,
        }
        = self;
        re_export::RenderingDevice::buffer_get_data_full(surround_object, buffer, offset_bytes, size_bytes,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::render_pipeline_create_ex`][super::RenderingDevice::render_pipeline_create_ex]."]
#[must_use]
pub struct ExRenderPipelineCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: ObjectCow < crate::classes::RdPipelineRasterizationState >, multisample_state: ObjectCow < crate::classes::RdPipelineMultisampleState >, stencil_state: ObjectCow < crate::classes::RdPipelineDepthStencilState >, color_blend_state: ObjectCow < crate::classes::RdPipelineColorBlendState >, dynamic_state_flags: crate::classes::rendering_device::PipelineDynamicStateFlags, for_render_pass: u32, specialization_constants: CowArg < 'a, Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRenderPipelineCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: impl AsObjectArg < crate::classes::RdPipelineRasterizationState >, multisample_state: impl AsObjectArg < crate::classes::RdPipelineMultisampleState >, stencil_state: impl AsObjectArg < crate::classes::RdPipelineDepthStencilState >, color_blend_state: impl AsObjectArg < crate::classes::RdPipelineColorBlendState >,) -> Self {
        let dynamic_state_flags = crate::obj::EngineBitfield::from_ord(0);
        let for_render_pass = 0u32;
        let specialization_constants = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader: shader, framebuffer_format: framebuffer_format, vertex_format: vertex_format, primitive: primitive, rasterization_state: rasterization_state.consume_arg(), multisample_state: multisample_state.consume_arg(), stencil_state: stencil_state.consume_arg(), color_blend_state: color_blend_state.consume_arg(), dynamic_state_flags: dynamic_state_flags, for_render_pass: for_render_pass, specialization_constants: CowArg::Owned(specialization_constants),
        }
    }
    #[inline]
    pub fn dynamic_state_flags(self, dynamic_state_flags: crate::classes::rendering_device::PipelineDynamicStateFlags) -> Self {
        Self {
            dynamic_state_flags: dynamic_state_flags, .. self
        }
    }
    #[inline]
    pub fn for_render_pass(self, for_render_pass: u32) -> Self {
        Self {
            for_render_pass: for_render_pass, .. self
        }
    }
    #[inline]
    pub fn specialization_constants(self, specialization_constants: &'a Array < Gd < crate::classes::RdPipelineSpecializationConstant > >) -> Self {
        Self {
            specialization_constants: CowArg::Borrowed(specialization_constants), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state, dynamic_state_flags, for_render_pass, specialization_constants,
        }
        = self;
        re_export::RenderingDevice::render_pipeline_create_full(surround_object, shader, framebuffer_format, vertex_format, primitive, rasterization_state.cow_as_object_arg(), multisample_state.cow_as_object_arg(), stencil_state.cow_as_object_arg(), color_blend_state.cow_as_object_arg(), dynamic_state_flags, for_render_pass, specialization_constants.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::compute_pipeline_create_ex`][super::RenderingDevice::compute_pipeline_create_ex]."]
#[must_use]
pub struct ExComputePipelineCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, shader: Rid, specialization_constants: CowArg < 'a, Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExComputePipelineCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, shader: Rid,) -> Self {
        let specialization_constants = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader: shader, specialization_constants: CowArg::Owned(specialization_constants),
        }
    }
    #[inline]
    pub fn specialization_constants(self, specialization_constants: &'a Array < Gd < crate::classes::RdPipelineSpecializationConstant > >) -> Self {
        Self {
            specialization_constants: CowArg::Borrowed(specialization_constants), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, shader, specialization_constants,
        }
        = self;
        re_export::RenderingDevice::compute_pipeline_create_full(surround_object, shader, specialization_constants.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::screen_get_width_ex`][super::RenderingDevice::screen_get_width_ex]."]
#[must_use]
pub struct ExScreenGetWidth < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingDevice, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetWidth < 'a > {
    fn new(surround_object: &'a re_export::RenderingDevice,) -> Self {
        let screen = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::RenderingDevice::screen_get_width_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::screen_get_height_ex`][super::RenderingDevice::screen_get_height_ex]."]
#[must_use]
pub struct ExScreenGetHeight < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingDevice, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetHeight < 'a > {
    fn new(surround_object: &'a re_export::RenderingDevice,) -> Self {
        let screen = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::RenderingDevice::screen_get_height_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::screen_get_framebuffer_format_ex`][super::RenderingDevice::screen_get_framebuffer_format_ex]."]
#[must_use]
pub struct ExScreenGetFramebufferFormat < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingDevice, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetFramebufferFormat < 'a > {
    fn new(surround_object: &'a re_export::RenderingDevice,) -> Self {
        let screen = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::RenderingDevice::screen_get_framebuffer_format_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_begin_for_screen_ex`][super::RenderingDevice::draw_list_begin_for_screen_ex]."]
#[must_use]
pub struct ExDrawListBeginForScreen < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, screen: i32, clear_color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListBeginForScreen < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        let screen = 0i32;
        let clear_color = Color::from_rgba(0 as _, 0 as _, 0 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen, clear_color: clear_color,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn clear_color(self, clear_color: Color) -> Self {
        Self {
            clear_color: clear_color, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, screen, clear_color,
        }
        = self;
        re_export::RenderingDevice::draw_list_begin_for_screen_full(surround_object, screen, clear_color,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_begin_ex`][super::RenderingDevice::draw_list_begin_ex]."]
#[must_use]
pub struct ExDrawListBegin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction, clear_color_values: CowArg < 'a, PackedColorArray >, clear_depth: f32, clear_stencil: u32, region: Rect2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListBegin < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction,) -> Self {
        let clear_color_values = PackedColorArray::new();
        let clear_depth = 1f32;
        let clear_stencil = 0u32;
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, framebuffer: framebuffer, initial_color_action: initial_color_action, final_color_action: final_color_action, initial_depth_action: initial_depth_action, final_depth_action: final_depth_action, clear_color_values: CowArg::Owned(clear_color_values), clear_depth: clear_depth, clear_stencil: clear_stencil, region: region,
        }
    }
    #[inline]
    pub fn clear_color_values(self, clear_color_values: &'a PackedColorArray) -> Self {
        Self {
            clear_color_values: CowArg::Borrowed(clear_color_values), .. self
        }
    }
    #[inline]
    pub fn clear_depth(self, clear_depth: f32) -> Self {
        Self {
            clear_depth: clear_depth, .. self
        }
    }
    #[inline]
    pub fn clear_stencil(self, clear_stencil: u32) -> Self {
        Self {
            clear_stencil: clear_stencil, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, framebuffer, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values, clear_depth, clear_stencil, region,
        }
        = self;
        re_export::RenderingDevice::draw_list_begin_full(surround_object, framebuffer, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values.cow_as_arg(), clear_depth, clear_stencil, region,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_begin_split_ex`][super::RenderingDevice::draw_list_begin_split_ex]."]
#[must_use]
pub struct ExDrawListBeginSplit < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction, clear_color_values: CowArg < 'a, PackedColorArray >, clear_depth: f32, clear_stencil: u32, region: Rect2, storage_textures: CowArg < 'a, Array < Rid > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListBeginSplit < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction,) -> Self {
        let clear_color_values = PackedColorArray::new();
        let clear_depth = 1f32;
        let clear_stencil = 0u32;
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let storage_textures = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, framebuffer: framebuffer, splits: splits, initial_color_action: initial_color_action, final_color_action: final_color_action, initial_depth_action: initial_depth_action, final_depth_action: final_depth_action, clear_color_values: CowArg::Owned(clear_color_values), clear_depth: clear_depth, clear_stencil: clear_stencil, region: region, storage_textures: CowArg::Owned(storage_textures),
        }
    }
    #[inline]
    pub fn clear_color_values(self, clear_color_values: &'a PackedColorArray) -> Self {
        Self {
            clear_color_values: CowArg::Borrowed(clear_color_values), .. self
        }
    }
    #[inline]
    pub fn clear_depth(self, clear_depth: f32) -> Self {
        Self {
            clear_depth: clear_depth, .. self
        }
    }
    #[inline]
    pub fn clear_stencil(self, clear_stencil: u32) -> Self {
        Self {
            clear_stencil: clear_stencil, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn storage_textures(self, storage_textures: &'a Array < Rid >) -> Self {
        Self {
            storage_textures: CowArg::Borrowed(storage_textures), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values, clear_depth, clear_stencil, region, storage_textures,
        }
        = self;
        re_export::RenderingDevice::draw_list_begin_split_full(surround_object, framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values.cow_as_arg(), clear_depth, clear_stencil, region, storage_textures.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_draw_ex`][super::RenderingDevice::draw_list_draw_ex]."]
#[must_use]
pub struct ExDrawListDraw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, use_indices: bool, instances: u32, procedural_vertex_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListDraw < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, use_indices: bool, instances: u32,) -> Self {
        let procedural_vertex_count = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, draw_list: draw_list, use_indices: use_indices, instances: instances, procedural_vertex_count: procedural_vertex_count,
        }
    }
    #[inline]
    pub fn procedural_vertex_count(self, procedural_vertex_count: u32) -> Self {
        Self {
            procedural_vertex_count: procedural_vertex_count, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, draw_list, use_indices, instances, procedural_vertex_count,
        }
        = self;
        re_export::RenderingDevice::draw_list_draw_full(surround_object, draw_list, use_indices, instances, procedural_vertex_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_enable_scissor_ex`][super::RenderingDevice::draw_list_enable_scissor_ex]."]
#[must_use]
pub struct ExDrawListEnableScissor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, rect: Rect2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListEnableScissor < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, draw_list: i64,) -> Self {
        let rect = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, draw_list: draw_list, rect: rect,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2) -> Self {
        Self {
            rect: rect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, draw_list, rect,
        }
        = self;
        re_export::RenderingDevice::draw_list_enable_scissor_full(surround_object, draw_list, rect,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::barrier_ex`][super::RenderingDevice::barrier_ex]."]
#[must_use]
pub struct ExBarrier < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, from: crate::classes::rendering_device::BarrierMask, to: crate::classes::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBarrier < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        let from = crate::obj::EngineBitfield::from_ord(32767);
        let to = crate::obj::EngineBitfield::from_ord(32767);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to,
        }
    }
    #[inline]
    pub fn from(self, from: crate::classes::rendering_device::BarrierMask) -> Self {
        Self {
            from: from, .. self
        }
    }
    #[inline]
    pub fn to(self, to: crate::classes::rendering_device::BarrierMask) -> Self {
        Self {
            to: to, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from, to,
        }
        = self;
        re_export::RenderingDevice::barrier_full(surround_object, from, to,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DeviceType {
    ord: i32
}
impl DeviceType {
    #[doc(alias = "DEVICE_TYPE_OTHER")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_OTHER`"]
    pub const OTHER: DeviceType = DeviceType {
        ord: 0i32
    };
    #[doc(alias = "DEVICE_TYPE_INTEGRATED_GPU")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_INTEGRATED_GPU`"]
    pub const INTEGRATED_GPU: DeviceType = DeviceType {
        ord: 1i32
    };
    #[doc(alias = "DEVICE_TYPE_DISCRETE_GPU")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_DISCRETE_GPU`"]
    pub const DISCRETE_GPU: DeviceType = DeviceType {
        ord: 2i32
    };
    #[doc(alias = "DEVICE_TYPE_VIRTUAL_GPU")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_VIRTUAL_GPU`"]
    pub const VIRTUAL_GPU: DeviceType = DeviceType {
        ord: 3i32
    };
    #[doc(alias = "DEVICE_TYPE_CPU")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_CPU`"]
    pub const CPU: DeviceType = DeviceType {
        ord: 4i32
    };
    #[doc(alias = "DEVICE_TYPE_MAX")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_MAX`"]
    pub const MAX: DeviceType = DeviceType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DeviceType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DeviceType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::OTHER => "OTHER", Self::INTEGRATED_GPU => "INTEGRATED_GPU", Self::DISCRETE_GPU => "DISCRETE_GPU", Self::VIRTUAL_GPU => "VIRTUAL_GPU", Self::CPU => "CPU", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::OTHER => "DEVICE_TYPE_OTHER", Self::INTEGRATED_GPU => "DEVICE_TYPE_INTEGRATED_GPU", Self::DISCRETE_GPU => "DEVICE_TYPE_DISCRETE_GPU", Self::VIRTUAL_GPU => "DEVICE_TYPE_VIRTUAL_GPU", Self::CPU => "DEVICE_TYPE_CPU", Self::MAX => "DEVICE_TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for DeviceType {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for DeviceType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DeviceType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DeviceType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DriverResource {
    ord: i32
}
impl DriverResource {
    #[doc(alias = "DRIVER_RESOURCE_LOGICAL_DEVICE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_LOGICAL_DEVICE`"]
    pub const LOGICAL_DEVICE: DriverResource = DriverResource {
        ord: 0i32
    };
    #[doc(alias = "DRIVER_RESOURCE_PHYSICAL_DEVICE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_PHYSICAL_DEVICE`"]
    pub const PHYSICAL_DEVICE: DriverResource = DriverResource {
        ord: 1i32
    };
    #[doc(alias = "DRIVER_RESOURCE_TOPMOST_OBJECT")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_TOPMOST_OBJECT`"]
    pub const TOPMOST_OBJECT: DriverResource = DriverResource {
        ord: 2i32
    };
    #[doc(alias = "DRIVER_RESOURCE_COMMAND_QUEUE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_COMMAND_QUEUE`"]
    pub const COMMAND_QUEUE: DriverResource = DriverResource {
        ord: 3i32
    };
    #[doc(alias = "DRIVER_RESOURCE_QUEUE_FAMILY")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_QUEUE_FAMILY`"]
    pub const QUEUE_FAMILY: DriverResource = DriverResource {
        ord: 4i32
    };
    #[doc(alias = "DRIVER_RESOURCE_TEXTURE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_TEXTURE`"]
    pub const TEXTURE: DriverResource = DriverResource {
        ord: 5i32
    };
    #[doc(alias = "DRIVER_RESOURCE_TEXTURE_VIEW")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_TEXTURE_VIEW`"]
    pub const TEXTURE_VIEW: DriverResource = DriverResource {
        ord: 6i32
    };
    #[doc(alias = "DRIVER_RESOURCE_TEXTURE_DATA_FORMAT")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_TEXTURE_DATA_FORMAT`"]
    pub const TEXTURE_DATA_FORMAT: DriverResource = DriverResource {
        ord: 7i32
    };
    #[doc(alias = "DRIVER_RESOURCE_SAMPLER")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_SAMPLER`"]
    pub const SAMPLER: DriverResource = DriverResource {
        ord: 8i32
    };
    #[doc(alias = "DRIVER_RESOURCE_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_UNIFORM_SET`"]
    pub const UNIFORM_SET: DriverResource = DriverResource {
        ord: 9i32
    };
    #[doc(alias = "DRIVER_RESOURCE_BUFFER")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_BUFFER`"]
    pub const BUFFER: DriverResource = DriverResource {
        ord: 10i32
    };
    #[doc(alias = "DRIVER_RESOURCE_COMPUTE_PIPELINE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_COMPUTE_PIPELINE`"]
    pub const COMPUTE_PIPELINE: DriverResource = DriverResource {
        ord: 11i32
    };
    #[doc(alias = "DRIVER_RESOURCE_RENDER_PIPELINE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_RENDER_PIPELINE`"]
    pub const RENDER_PIPELINE: DriverResource = DriverResource {
        ord: 12i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_DEVICE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_DEVICE`"]
    pub const VULKAN_DEVICE: DriverResource = DriverResource {
        ord: 0i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_PHYSICAL_DEVICE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_PHYSICAL_DEVICE`"]
    pub const VULKAN_PHYSICAL_DEVICE: DriverResource = DriverResource {
        ord: 1i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_INSTANCE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_INSTANCE`"]
    pub const VULKAN_INSTANCE: DriverResource = DriverResource {
        ord: 2i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_QUEUE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_QUEUE`"]
    pub const VULKAN_QUEUE: DriverResource = DriverResource {
        ord: 3i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_QUEUE_FAMILY_INDEX")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_QUEUE_FAMILY_INDEX`"]
    pub const VULKAN_QUEUE_FAMILY_INDEX: DriverResource = DriverResource {
        ord: 4i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_IMAGE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_IMAGE`"]
    pub const VULKAN_IMAGE: DriverResource = DriverResource {
        ord: 5i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_IMAGE_VIEW")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_IMAGE_VIEW`"]
    pub const VULKAN_IMAGE_VIEW: DriverResource = DriverResource {
        ord: 6i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT`"]
    pub const VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT: DriverResource = DriverResource {
        ord: 7i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_SAMPLER")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_SAMPLER`"]
    pub const VULKAN_SAMPLER: DriverResource = DriverResource {
        ord: 8i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_DESCRIPTOR_SET")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_DESCRIPTOR_SET`"]
    pub const VULKAN_DESCRIPTOR_SET: DriverResource = DriverResource {
        ord: 9i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_BUFFER")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_BUFFER`"]
    pub const VULKAN_BUFFER: DriverResource = DriverResource {
        ord: 10i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_COMPUTE_PIPELINE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_COMPUTE_PIPELINE`"]
    pub const VULKAN_COMPUTE_PIPELINE: DriverResource = DriverResource {
        ord: 11i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_RENDER_PIPELINE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_RENDER_PIPELINE`"]
    pub const VULKAN_RENDER_PIPELINE: DriverResource = DriverResource {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for DriverResource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DriverResource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DriverResource {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
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
            Self::LOGICAL_DEVICE => "LOGICAL_DEVICE", Self::PHYSICAL_DEVICE => "PHYSICAL_DEVICE", Self::TOPMOST_OBJECT => "TOPMOST_OBJECT", Self::COMMAND_QUEUE => "COMMAND_QUEUE", Self::QUEUE_FAMILY => "QUEUE_FAMILY", Self::TEXTURE => "TEXTURE", Self::TEXTURE_VIEW => "TEXTURE_VIEW", Self::TEXTURE_DATA_FORMAT => "TEXTURE_DATA_FORMAT", Self::SAMPLER => "SAMPLER", Self::UNIFORM_SET => "UNIFORM_SET", Self::BUFFER => "BUFFER", Self::COMPUTE_PIPELINE => "COMPUTE_PIPELINE", Self::RENDER_PIPELINE => "RENDER_PIPELINE", Self::VULKAN_DEVICE => "VULKAN_DEVICE", Self::VULKAN_PHYSICAL_DEVICE => "VULKAN_PHYSICAL_DEVICE", Self::VULKAN_INSTANCE => "VULKAN_INSTANCE", Self::VULKAN_QUEUE => "VULKAN_QUEUE", Self::VULKAN_QUEUE_FAMILY_INDEX => "VULKAN_QUEUE_FAMILY_INDEX", Self::VULKAN_IMAGE => "VULKAN_IMAGE", Self::VULKAN_IMAGE_VIEW => "VULKAN_IMAGE_VIEW", Self::VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT => "VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT", Self::VULKAN_SAMPLER => "VULKAN_SAMPLER", Self::VULKAN_DESCRIPTOR_SET => "VULKAN_DESCRIPTOR_SET", Self::VULKAN_BUFFER => "VULKAN_BUFFER", Self::VULKAN_COMPUTE_PIPELINE => "VULKAN_COMPUTE_PIPELINE", Self::VULKAN_RENDER_PIPELINE => "VULKAN_RENDER_PIPELINE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LOGICAL_DEVICE => "DRIVER_RESOURCE_LOGICAL_DEVICE", Self::PHYSICAL_DEVICE => "DRIVER_RESOURCE_PHYSICAL_DEVICE", Self::TOPMOST_OBJECT => "DRIVER_RESOURCE_TOPMOST_OBJECT", Self::COMMAND_QUEUE => "DRIVER_RESOURCE_COMMAND_QUEUE", Self::QUEUE_FAMILY => "DRIVER_RESOURCE_QUEUE_FAMILY", Self::TEXTURE => "DRIVER_RESOURCE_TEXTURE", Self::TEXTURE_VIEW => "DRIVER_RESOURCE_TEXTURE_VIEW", Self::TEXTURE_DATA_FORMAT => "DRIVER_RESOURCE_TEXTURE_DATA_FORMAT", Self::SAMPLER => "DRIVER_RESOURCE_SAMPLER", Self::UNIFORM_SET => "DRIVER_RESOURCE_UNIFORM_SET", Self::BUFFER => "DRIVER_RESOURCE_BUFFER", Self::COMPUTE_PIPELINE => "DRIVER_RESOURCE_COMPUTE_PIPELINE", Self::RENDER_PIPELINE => "DRIVER_RESOURCE_RENDER_PIPELINE", Self::VULKAN_DEVICE => "DRIVER_RESOURCE_VULKAN_DEVICE", Self::VULKAN_PHYSICAL_DEVICE => "DRIVER_RESOURCE_VULKAN_PHYSICAL_DEVICE", Self::VULKAN_INSTANCE => "DRIVER_RESOURCE_VULKAN_INSTANCE", Self::VULKAN_QUEUE => "DRIVER_RESOURCE_VULKAN_QUEUE", Self::VULKAN_QUEUE_FAMILY_INDEX => "DRIVER_RESOURCE_VULKAN_QUEUE_FAMILY_INDEX", Self::VULKAN_IMAGE => "DRIVER_RESOURCE_VULKAN_IMAGE", Self::VULKAN_IMAGE_VIEW => "DRIVER_RESOURCE_VULKAN_IMAGE_VIEW", Self::VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT => "DRIVER_RESOURCE_VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT", Self::VULKAN_SAMPLER => "DRIVER_RESOURCE_VULKAN_SAMPLER", Self::VULKAN_DESCRIPTOR_SET => "DRIVER_RESOURCE_VULKAN_DESCRIPTOR_SET", Self::VULKAN_BUFFER => "DRIVER_RESOURCE_VULKAN_BUFFER", Self::VULKAN_COMPUTE_PIPELINE => "DRIVER_RESOURCE_VULKAN_COMPUTE_PIPELINE", Self::VULKAN_RENDER_PIPELINE => "DRIVER_RESOURCE_VULKAN_RENDER_PIPELINE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for DriverResource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DriverResource {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DriverResource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DataFormat {
    ord: i32
}
impl DataFormat {
    #[doc(alias = "DATA_FORMAT_R4G4_UNORM_PACK8")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R4G4_UNORM_PACK8`"]
    pub const R4G4_UNORM_PACK8: DataFormat = DataFormat {
        ord: 0i32
    };
    #[doc(alias = "DATA_FORMAT_R4G4B4A4_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R4G4B4A4_UNORM_PACK16`"]
    pub const R4G4B4A4_UNORM_PACK16: DataFormat = DataFormat {
        ord: 1i32
    };
    #[doc(alias = "DATA_FORMAT_B4G4R4A4_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B4G4R4A4_UNORM_PACK16`"]
    pub const B4G4R4A4_UNORM_PACK16: DataFormat = DataFormat {
        ord: 2i32
    };
    #[doc(alias = "DATA_FORMAT_R5G6B5_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R5G6B5_UNORM_PACK16`"]
    pub const R5G6B5_UNORM_PACK16: DataFormat = DataFormat {
        ord: 3i32
    };
    #[doc(alias = "DATA_FORMAT_B5G6R5_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B5G6R5_UNORM_PACK16`"]
    pub const B5G6R5_UNORM_PACK16: DataFormat = DataFormat {
        ord: 4i32
    };
    #[doc(alias = "DATA_FORMAT_R5G5B5A1_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R5G5B5A1_UNORM_PACK16`"]
    pub const R5G5B5A1_UNORM_PACK16: DataFormat = DataFormat {
        ord: 5i32
    };
    #[doc(alias = "DATA_FORMAT_B5G5R5A1_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B5G5R5A1_UNORM_PACK16`"]
    pub const B5G5R5A1_UNORM_PACK16: DataFormat = DataFormat {
        ord: 6i32
    };
    #[doc(alias = "DATA_FORMAT_A1R5G5B5_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A1R5G5B5_UNORM_PACK16`"]
    pub const A1R5G5B5_UNORM_PACK16: DataFormat = DataFormat {
        ord: 7i32
    };
    #[doc(alias = "DATA_FORMAT_R8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_UNORM`"]
    pub const R8_UNORM: DataFormat = DataFormat {
        ord: 8i32
    };
    #[doc(alias = "DATA_FORMAT_R8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_SNORM`"]
    pub const R8_SNORM: DataFormat = DataFormat {
        ord: 9i32
    };
    #[doc(alias = "DATA_FORMAT_R8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_USCALED`"]
    pub const R8_USCALED: DataFormat = DataFormat {
        ord: 10i32
    };
    #[doc(alias = "DATA_FORMAT_R8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_SSCALED`"]
    pub const R8_SSCALED: DataFormat = DataFormat {
        ord: 11i32
    };
    #[doc(alias = "DATA_FORMAT_R8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_UINT`"]
    pub const R8_UINT: DataFormat = DataFormat {
        ord: 12i32
    };
    #[doc(alias = "DATA_FORMAT_R8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_SINT`"]
    pub const R8_SINT: DataFormat = DataFormat {
        ord: 13i32
    };
    #[doc(alias = "DATA_FORMAT_R8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_SRGB`"]
    pub const R8_SRGB: DataFormat = DataFormat {
        ord: 14i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_UNORM`"]
    pub const R8G8_UNORM: DataFormat = DataFormat {
        ord: 15i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_SNORM`"]
    pub const R8G8_SNORM: DataFormat = DataFormat {
        ord: 16i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_USCALED`"]
    pub const R8G8_USCALED: DataFormat = DataFormat {
        ord: 17i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_SSCALED`"]
    pub const R8G8_SSCALED: DataFormat = DataFormat {
        ord: 18i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_UINT`"]
    pub const R8G8_UINT: DataFormat = DataFormat {
        ord: 19i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_SINT`"]
    pub const R8G8_SINT: DataFormat = DataFormat {
        ord: 20i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_SRGB`"]
    pub const R8G8_SRGB: DataFormat = DataFormat {
        ord: 21i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_UNORM`"]
    pub const R8G8B8_UNORM: DataFormat = DataFormat {
        ord: 22i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_SNORM`"]
    pub const R8G8B8_SNORM: DataFormat = DataFormat {
        ord: 23i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_USCALED`"]
    pub const R8G8B8_USCALED: DataFormat = DataFormat {
        ord: 24i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_SSCALED`"]
    pub const R8G8B8_SSCALED: DataFormat = DataFormat {
        ord: 25i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_UINT`"]
    pub const R8G8B8_UINT: DataFormat = DataFormat {
        ord: 26i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_SINT`"]
    pub const R8G8B8_SINT: DataFormat = DataFormat {
        ord: 27i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_SRGB`"]
    pub const R8G8B8_SRGB: DataFormat = DataFormat {
        ord: 28i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_UNORM`"]
    pub const B8G8R8_UNORM: DataFormat = DataFormat {
        ord: 29i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_SNORM`"]
    pub const B8G8R8_SNORM: DataFormat = DataFormat {
        ord: 30i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_USCALED`"]
    pub const B8G8R8_USCALED: DataFormat = DataFormat {
        ord: 31i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_SSCALED`"]
    pub const B8G8R8_SSCALED: DataFormat = DataFormat {
        ord: 32i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_UINT`"]
    pub const B8G8R8_UINT: DataFormat = DataFormat {
        ord: 33i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_SINT`"]
    pub const B8G8R8_SINT: DataFormat = DataFormat {
        ord: 34i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_SRGB`"]
    pub const B8G8R8_SRGB: DataFormat = DataFormat {
        ord: 35i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_UNORM`"]
    pub const R8G8B8A8_UNORM: DataFormat = DataFormat {
        ord: 36i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_SNORM`"]
    pub const R8G8B8A8_SNORM: DataFormat = DataFormat {
        ord: 37i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_USCALED`"]
    pub const R8G8B8A8_USCALED: DataFormat = DataFormat {
        ord: 38i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_SSCALED`"]
    pub const R8G8B8A8_SSCALED: DataFormat = DataFormat {
        ord: 39i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_UINT`"]
    pub const R8G8B8A8_UINT: DataFormat = DataFormat {
        ord: 40i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_SINT`"]
    pub const R8G8B8A8_SINT: DataFormat = DataFormat {
        ord: 41i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_SRGB`"]
    pub const R8G8B8A8_SRGB: DataFormat = DataFormat {
        ord: 42i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_UNORM`"]
    pub const B8G8R8A8_UNORM: DataFormat = DataFormat {
        ord: 43i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_SNORM`"]
    pub const B8G8R8A8_SNORM: DataFormat = DataFormat {
        ord: 44i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_USCALED`"]
    pub const B8G8R8A8_USCALED: DataFormat = DataFormat {
        ord: 45i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_SSCALED`"]
    pub const B8G8R8A8_SSCALED: DataFormat = DataFormat {
        ord: 46i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_UINT`"]
    pub const B8G8R8A8_UINT: DataFormat = DataFormat {
        ord: 47i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_SINT`"]
    pub const B8G8R8A8_SINT: DataFormat = DataFormat {
        ord: 48i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_SRGB`"]
    pub const B8G8R8A8_SRGB: DataFormat = DataFormat {
        ord: 49i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_UNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_UNORM_PACK32`"]
    pub const A8B8G8R8_UNORM_PACK32: DataFormat = DataFormat {
        ord: 50i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_SNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_SNORM_PACK32`"]
    pub const A8B8G8R8_SNORM_PACK32: DataFormat = DataFormat {
        ord: 51i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_USCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_USCALED_PACK32`"]
    pub const A8B8G8R8_USCALED_PACK32: DataFormat = DataFormat {
        ord: 52i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_SSCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_SSCALED_PACK32`"]
    pub const A8B8G8R8_SSCALED_PACK32: DataFormat = DataFormat {
        ord: 53i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_UINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_UINT_PACK32`"]
    pub const A8B8G8R8_UINT_PACK32: DataFormat = DataFormat {
        ord: 54i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_SINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_SINT_PACK32`"]
    pub const A8B8G8R8_SINT_PACK32: DataFormat = DataFormat {
        ord: 55i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_SRGB_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_SRGB_PACK32`"]
    pub const A8B8G8R8_SRGB_PACK32: DataFormat = DataFormat {
        ord: 56i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_UNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_UNORM_PACK32`"]
    pub const A2R10G10B10_UNORM_PACK32: DataFormat = DataFormat {
        ord: 57i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_SNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_SNORM_PACK32`"]
    pub const A2R10G10B10_SNORM_PACK32: DataFormat = DataFormat {
        ord: 58i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_USCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_USCALED_PACK32`"]
    pub const A2R10G10B10_USCALED_PACK32: DataFormat = DataFormat {
        ord: 59i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_SSCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_SSCALED_PACK32`"]
    pub const A2R10G10B10_SSCALED_PACK32: DataFormat = DataFormat {
        ord: 60i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_UINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_UINT_PACK32`"]
    pub const A2R10G10B10_UINT_PACK32: DataFormat = DataFormat {
        ord: 61i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_SINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_SINT_PACK32`"]
    pub const A2R10G10B10_SINT_PACK32: DataFormat = DataFormat {
        ord: 62i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_UNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_UNORM_PACK32`"]
    pub const A2B10G10R10_UNORM_PACK32: DataFormat = DataFormat {
        ord: 63i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_SNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_SNORM_PACK32`"]
    pub const A2B10G10R10_SNORM_PACK32: DataFormat = DataFormat {
        ord: 64i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_USCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_USCALED_PACK32`"]
    pub const A2B10G10R10_USCALED_PACK32: DataFormat = DataFormat {
        ord: 65i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_SSCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_SSCALED_PACK32`"]
    pub const A2B10G10R10_SSCALED_PACK32: DataFormat = DataFormat {
        ord: 66i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_UINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_UINT_PACK32`"]
    pub const A2B10G10R10_UINT_PACK32: DataFormat = DataFormat {
        ord: 67i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_SINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_SINT_PACK32`"]
    pub const A2B10G10R10_SINT_PACK32: DataFormat = DataFormat {
        ord: 68i32
    };
    #[doc(alias = "DATA_FORMAT_R16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_UNORM`"]
    pub const R16_UNORM: DataFormat = DataFormat {
        ord: 69i32
    };
    #[doc(alias = "DATA_FORMAT_R16_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_SNORM`"]
    pub const R16_SNORM: DataFormat = DataFormat {
        ord: 70i32
    };
    #[doc(alias = "DATA_FORMAT_R16_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_USCALED`"]
    pub const R16_USCALED: DataFormat = DataFormat {
        ord: 71i32
    };
    #[doc(alias = "DATA_FORMAT_R16_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_SSCALED`"]
    pub const R16_SSCALED: DataFormat = DataFormat {
        ord: 72i32
    };
    #[doc(alias = "DATA_FORMAT_R16_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_UINT`"]
    pub const R16_UINT: DataFormat = DataFormat {
        ord: 73i32
    };
    #[doc(alias = "DATA_FORMAT_R16_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_SINT`"]
    pub const R16_SINT: DataFormat = DataFormat {
        ord: 74i32
    };
    #[doc(alias = "DATA_FORMAT_R16_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_SFLOAT`"]
    pub const R16_SFLOAT: DataFormat = DataFormat {
        ord: 75i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_UNORM`"]
    pub const R16G16_UNORM: DataFormat = DataFormat {
        ord: 76i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_SNORM`"]
    pub const R16G16_SNORM: DataFormat = DataFormat {
        ord: 77i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_USCALED`"]
    pub const R16G16_USCALED: DataFormat = DataFormat {
        ord: 78i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_SSCALED`"]
    pub const R16G16_SSCALED: DataFormat = DataFormat {
        ord: 79i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_UINT`"]
    pub const R16G16_UINT: DataFormat = DataFormat {
        ord: 80i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_SINT`"]
    pub const R16G16_SINT: DataFormat = DataFormat {
        ord: 81i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_SFLOAT`"]
    pub const R16G16_SFLOAT: DataFormat = DataFormat {
        ord: 82i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_UNORM`"]
    pub const R16G16B16_UNORM: DataFormat = DataFormat {
        ord: 83i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_SNORM`"]
    pub const R16G16B16_SNORM: DataFormat = DataFormat {
        ord: 84i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_USCALED`"]
    pub const R16G16B16_USCALED: DataFormat = DataFormat {
        ord: 85i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_SSCALED`"]
    pub const R16G16B16_SSCALED: DataFormat = DataFormat {
        ord: 86i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_UINT`"]
    pub const R16G16B16_UINT: DataFormat = DataFormat {
        ord: 87i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_SINT`"]
    pub const R16G16B16_SINT: DataFormat = DataFormat {
        ord: 88i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_SFLOAT`"]
    pub const R16G16B16_SFLOAT: DataFormat = DataFormat {
        ord: 89i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_UNORM`"]
    pub const R16G16B16A16_UNORM: DataFormat = DataFormat {
        ord: 90i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_SNORM`"]
    pub const R16G16B16A16_SNORM: DataFormat = DataFormat {
        ord: 91i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_USCALED`"]
    pub const R16G16B16A16_USCALED: DataFormat = DataFormat {
        ord: 92i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_SSCALED`"]
    pub const R16G16B16A16_SSCALED: DataFormat = DataFormat {
        ord: 93i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_UINT`"]
    pub const R16G16B16A16_UINT: DataFormat = DataFormat {
        ord: 94i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_SINT`"]
    pub const R16G16B16A16_SINT: DataFormat = DataFormat {
        ord: 95i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_SFLOAT`"]
    pub const R16G16B16A16_SFLOAT: DataFormat = DataFormat {
        ord: 96i32
    };
    #[doc(alias = "DATA_FORMAT_R32_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32_UINT`"]
    pub const R32_UINT: DataFormat = DataFormat {
        ord: 97i32
    };
    #[doc(alias = "DATA_FORMAT_R32_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32_SINT`"]
    pub const R32_SINT: DataFormat = DataFormat {
        ord: 98i32
    };
    #[doc(alias = "DATA_FORMAT_R32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32_SFLOAT`"]
    pub const R32_SFLOAT: DataFormat = DataFormat {
        ord: 99i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32_UINT`"]
    pub const R32G32_UINT: DataFormat = DataFormat {
        ord: 100i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32_SINT`"]
    pub const R32G32_SINT: DataFormat = DataFormat {
        ord: 101i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32_SFLOAT`"]
    pub const R32G32_SFLOAT: DataFormat = DataFormat {
        ord: 102i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32_UINT`"]
    pub const R32G32B32_UINT: DataFormat = DataFormat {
        ord: 103i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32_SINT`"]
    pub const R32G32B32_SINT: DataFormat = DataFormat {
        ord: 104i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32_SFLOAT`"]
    pub const R32G32B32_SFLOAT: DataFormat = DataFormat {
        ord: 105i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32A32_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32A32_UINT`"]
    pub const R32G32B32A32_UINT: DataFormat = DataFormat {
        ord: 106i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32A32_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32A32_SINT`"]
    pub const R32G32B32A32_SINT: DataFormat = DataFormat {
        ord: 107i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32A32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32A32_SFLOAT`"]
    pub const R32G32B32A32_SFLOAT: DataFormat = DataFormat {
        ord: 108i32
    };
    #[doc(alias = "DATA_FORMAT_R64_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64_UINT`"]
    pub const R64_UINT: DataFormat = DataFormat {
        ord: 109i32
    };
    #[doc(alias = "DATA_FORMAT_R64_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64_SINT`"]
    pub const R64_SINT: DataFormat = DataFormat {
        ord: 110i32
    };
    #[doc(alias = "DATA_FORMAT_R64_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64_SFLOAT`"]
    pub const R64_SFLOAT: DataFormat = DataFormat {
        ord: 111i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64_UINT`"]
    pub const R64G64_UINT: DataFormat = DataFormat {
        ord: 112i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64_SINT`"]
    pub const R64G64_SINT: DataFormat = DataFormat {
        ord: 113i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64_SFLOAT`"]
    pub const R64G64_SFLOAT: DataFormat = DataFormat {
        ord: 114i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64_UINT`"]
    pub const R64G64B64_UINT: DataFormat = DataFormat {
        ord: 115i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64_SINT`"]
    pub const R64G64B64_SINT: DataFormat = DataFormat {
        ord: 116i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64_SFLOAT`"]
    pub const R64G64B64_SFLOAT: DataFormat = DataFormat {
        ord: 117i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64A64_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64A64_UINT`"]
    pub const R64G64B64A64_UINT: DataFormat = DataFormat {
        ord: 118i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64A64_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64A64_SINT`"]
    pub const R64G64B64A64_SINT: DataFormat = DataFormat {
        ord: 119i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64A64_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64A64_SFLOAT`"]
    pub const R64G64B64A64_SFLOAT: DataFormat = DataFormat {
        ord: 120i32
    };
    #[doc(alias = "DATA_FORMAT_B10G11R11_UFLOAT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B10G11R11_UFLOAT_PACK32`"]
    pub const B10G11R11_UFLOAT_PACK32: DataFormat = DataFormat {
        ord: 121i32
    };
    #[doc(alias = "DATA_FORMAT_E5B9G9R9_UFLOAT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_E5B9G9R9_UFLOAT_PACK32`"]
    pub const E5B9G9R9_UFLOAT_PACK32: DataFormat = DataFormat {
        ord: 122i32
    };
    #[doc(alias = "DATA_FORMAT_D16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D16_UNORM`"]
    pub const D16_UNORM: DataFormat = DataFormat {
        ord: 123i32
    };
    #[doc(alias = "DATA_FORMAT_X8_D24_UNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_X8_D24_UNORM_PACK32`"]
    pub const X8_D24_UNORM_PACK32: DataFormat = DataFormat {
        ord: 124i32
    };
    #[doc(alias = "DATA_FORMAT_D32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D32_SFLOAT`"]
    pub const D32_SFLOAT: DataFormat = DataFormat {
        ord: 125i32
    };
    #[doc(alias = "DATA_FORMAT_S8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_S8_UINT`"]
    pub const S8_UINT: DataFormat = DataFormat {
        ord: 126i32
    };
    #[doc(alias = "DATA_FORMAT_D16_UNORM_S8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D16_UNORM_S8_UINT`"]
    pub const D16_UNORM_S8_UINT: DataFormat = DataFormat {
        ord: 127i32
    };
    #[doc(alias = "DATA_FORMAT_D24_UNORM_S8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D24_UNORM_S8_UINT`"]
    pub const D24_UNORM_S8_UINT: DataFormat = DataFormat {
        ord: 128i32
    };
    #[doc(alias = "DATA_FORMAT_D32_SFLOAT_S8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D32_SFLOAT_S8_UINT`"]
    pub const D32_SFLOAT_S8_UINT: DataFormat = DataFormat {
        ord: 129i32
    };
    #[doc(alias = "DATA_FORMAT_BC1_RGB_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC1_RGB_UNORM_BLOCK`"]
    pub const BC1_RGB_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 130i32
    };
    #[doc(alias = "DATA_FORMAT_BC1_RGB_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC1_RGB_SRGB_BLOCK`"]
    pub const BC1_RGB_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 131i32
    };
    #[doc(alias = "DATA_FORMAT_BC1_RGBA_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC1_RGBA_UNORM_BLOCK`"]
    pub const BC1_RGBA_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 132i32
    };
    #[doc(alias = "DATA_FORMAT_BC1_RGBA_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC1_RGBA_SRGB_BLOCK`"]
    pub const BC1_RGBA_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 133i32
    };
    #[doc(alias = "DATA_FORMAT_BC2_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC2_UNORM_BLOCK`"]
    pub const BC2_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 134i32
    };
    #[doc(alias = "DATA_FORMAT_BC2_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC2_SRGB_BLOCK`"]
    pub const BC2_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 135i32
    };
    #[doc(alias = "DATA_FORMAT_BC3_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC3_UNORM_BLOCK`"]
    pub const BC3_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 136i32
    };
    #[doc(alias = "DATA_FORMAT_BC3_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC3_SRGB_BLOCK`"]
    pub const BC3_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 137i32
    };
    #[doc(alias = "DATA_FORMAT_BC4_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC4_UNORM_BLOCK`"]
    pub const BC4_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 138i32
    };
    #[doc(alias = "DATA_FORMAT_BC4_SNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC4_SNORM_BLOCK`"]
    pub const BC4_SNORM_BLOCK: DataFormat = DataFormat {
        ord: 139i32
    };
    #[doc(alias = "DATA_FORMAT_BC5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC5_UNORM_BLOCK`"]
    pub const BC5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 140i32
    };
    #[doc(alias = "DATA_FORMAT_BC5_SNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC5_SNORM_BLOCK`"]
    pub const BC5_SNORM_BLOCK: DataFormat = DataFormat {
        ord: 141i32
    };
    #[doc(alias = "DATA_FORMAT_BC6H_UFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC6H_UFLOAT_BLOCK`"]
    pub const BC6H_UFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 142i32
    };
    #[doc(alias = "DATA_FORMAT_BC6H_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC6H_SFLOAT_BLOCK`"]
    pub const BC6H_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 143i32
    };
    #[doc(alias = "DATA_FORMAT_BC7_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC7_UNORM_BLOCK`"]
    pub const BC7_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 144i32
    };
    #[doc(alias = "DATA_FORMAT_BC7_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC7_SRGB_BLOCK`"]
    pub const BC7_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 145i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8_UNORM_BLOCK`"]
    pub const ETC2_R8G8B8_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 146i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8_SRGB_BLOCK`"]
    pub const ETC2_R8G8B8_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 147i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK`"]
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 148i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK`"]
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 149i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK`"]
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 150i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK`"]
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 151i32
    };
    #[doc(alias = "DATA_FORMAT_EAC_R11_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_EAC_R11_UNORM_BLOCK`"]
    pub const EAC_R11_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 152i32
    };
    #[doc(alias = "DATA_FORMAT_EAC_R11_SNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_EAC_R11_SNORM_BLOCK`"]
    pub const EAC_R11_SNORM_BLOCK: DataFormat = DataFormat {
        ord: 153i32
    };
    #[doc(alias = "DATA_FORMAT_EAC_R11G11_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_EAC_R11G11_UNORM_BLOCK`"]
    pub const EAC_R11G11_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 154i32
    };
    #[doc(alias = "DATA_FORMAT_EAC_R11G11_SNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_EAC_R11G11_SNORM_BLOCK`"]
    pub const EAC_R11G11_SNORM_BLOCK: DataFormat = DataFormat {
        ord: 155i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_4x4_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_4x4_UNORM_BLOCK`"]
    pub const ASTC_4x4_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 156i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_4x4_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_4x4_SRGB_BLOCK`"]
    pub const ASTC_4x4_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 157i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x4_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x4_UNORM_BLOCK`"]
    pub const ASTC_5x4_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 158i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x4_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x4_SRGB_BLOCK`"]
    pub const ASTC_5x4_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 159i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x5_UNORM_BLOCK`"]
    pub const ASTC_5x5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 160i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x5_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x5_SRGB_BLOCK`"]
    pub const ASTC_5x5_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 161i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x5_UNORM_BLOCK`"]
    pub const ASTC_6x5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 162i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x5_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x5_SRGB_BLOCK`"]
    pub const ASTC_6x5_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 163i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x6_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x6_UNORM_BLOCK`"]
    pub const ASTC_6x6_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 164i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x6_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x6_SRGB_BLOCK`"]
    pub const ASTC_6x6_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 165i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x5_UNORM_BLOCK`"]
    pub const ASTC_8x5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 166i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x5_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x5_SRGB_BLOCK`"]
    pub const ASTC_8x5_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 167i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x6_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x6_UNORM_BLOCK`"]
    pub const ASTC_8x6_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 168i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x6_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x6_SRGB_BLOCK`"]
    pub const ASTC_8x6_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 169i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x8_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x8_UNORM_BLOCK`"]
    pub const ASTC_8x8_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 170i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x8_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x8_SRGB_BLOCK`"]
    pub const ASTC_8x8_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 171i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x5_UNORM_BLOCK`"]
    pub const ASTC_10x5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 172i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x5_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x5_SRGB_BLOCK`"]
    pub const ASTC_10x5_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 173i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x6_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x6_UNORM_BLOCK`"]
    pub const ASTC_10x6_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 174i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x6_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x6_SRGB_BLOCK`"]
    pub const ASTC_10x6_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 175i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x8_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x8_UNORM_BLOCK`"]
    pub const ASTC_10x8_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 176i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x8_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x8_SRGB_BLOCK`"]
    pub const ASTC_10x8_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 177i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x10_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x10_UNORM_BLOCK`"]
    pub const ASTC_10x10_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 178i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x10_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x10_SRGB_BLOCK`"]
    pub const ASTC_10x10_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 179i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x10_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x10_UNORM_BLOCK`"]
    pub const ASTC_12x10_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 180i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x10_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x10_SRGB_BLOCK`"]
    pub const ASTC_12x10_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 181i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x12_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x12_UNORM_BLOCK`"]
    pub const ASTC_12x12_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 182i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x12_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x12_SRGB_BLOCK`"]
    pub const ASTC_12x12_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 183i32
    };
    #[doc(alias = "DATA_FORMAT_G8B8G8R8_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8B8G8R8_422_UNORM`"]
    pub const G8B8G8R8_422_UNORM: DataFormat = DataFormat {
        ord: 184i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8G8_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8G8_422_UNORM`"]
    pub const B8G8R8G8_422_UNORM: DataFormat = DataFormat {
        ord: 185i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8_R8_3PLANE_420_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8_R8_3PLANE_420_UNORM`"]
    pub const G8_B8_R8_3PLANE_420_UNORM: DataFormat = DataFormat {
        ord: 186i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8R8_2PLANE_420_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8R8_2PLANE_420_UNORM`"]
    pub const G8_B8R8_2PLANE_420_UNORM: DataFormat = DataFormat {
        ord: 187i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8_R8_3PLANE_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8_R8_3PLANE_422_UNORM`"]
    pub const G8_B8_R8_3PLANE_422_UNORM: DataFormat = DataFormat {
        ord: 188i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8R8_2PLANE_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8R8_2PLANE_422_UNORM`"]
    pub const G8_B8R8_2PLANE_422_UNORM: DataFormat = DataFormat {
        ord: 189i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8_R8_3PLANE_444_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8_R8_3PLANE_444_UNORM`"]
    pub const G8_B8_R8_3PLANE_444_UNORM: DataFormat = DataFormat {
        ord: 190i32
    };
    #[doc(alias = "DATA_FORMAT_R10X6_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R10X6_UNORM_PACK16`"]
    pub const R10X6_UNORM_PACK16: DataFormat = DataFormat {
        ord: 191i32
    };
    #[doc(alias = "DATA_FORMAT_R10X6G10X6_UNORM_2PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R10X6G10X6_UNORM_2PACK16`"]
    pub const R10X6G10X6_UNORM_2PACK16: DataFormat = DataFormat {
        ord: 192i32
    };
    #[doc(alias = "DATA_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16`"]
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 193i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16`"]
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 194i32
    };
    #[doc(alias = "DATA_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16`"]
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 195i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16`"]
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 196i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16`"]
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 197i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16`"]
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 198i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16`"]
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 199i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16`"]
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 200i32
    };
    #[doc(alias = "DATA_FORMAT_R12X4_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R12X4_UNORM_PACK16`"]
    pub const R12X4_UNORM_PACK16: DataFormat = DataFormat {
        ord: 201i32
    };
    #[doc(alias = "DATA_FORMAT_R12X4G12X4_UNORM_2PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R12X4G12X4_UNORM_2PACK16`"]
    pub const R12X4G12X4_UNORM_2PACK16: DataFormat = DataFormat {
        ord: 202i32
    };
    #[doc(alias = "DATA_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16`"]
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 203i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16`"]
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 204i32
    };
    #[doc(alias = "DATA_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16`"]
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 205i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16`"]
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 206i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16`"]
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 207i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16`"]
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 208i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16`"]
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 209i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16`"]
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 210i32
    };
    #[doc(alias = "DATA_FORMAT_G16B16G16R16_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16B16G16R16_422_UNORM`"]
    pub const G16B16G16R16_422_UNORM: DataFormat = DataFormat {
        ord: 211i32
    };
    #[doc(alias = "DATA_FORMAT_B16G16R16G16_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B16G16R16G16_422_UNORM`"]
    pub const B16G16R16G16_422_UNORM: DataFormat = DataFormat {
        ord: 212i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16_R16_3PLANE_420_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16_R16_3PLANE_420_UNORM`"]
    pub const G16_B16_R16_3PLANE_420_UNORM: DataFormat = DataFormat {
        ord: 213i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16R16_2PLANE_420_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16R16_2PLANE_420_UNORM`"]
    pub const G16_B16R16_2PLANE_420_UNORM: DataFormat = DataFormat {
        ord: 214i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16_R16_3PLANE_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16_R16_3PLANE_422_UNORM`"]
    pub const G16_B16_R16_3PLANE_422_UNORM: DataFormat = DataFormat {
        ord: 215i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16R16_2PLANE_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16R16_2PLANE_422_UNORM`"]
    pub const G16_B16R16_2PLANE_422_UNORM: DataFormat = DataFormat {
        ord: 216i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16_R16_3PLANE_444_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16_R16_3PLANE_444_UNORM`"]
    pub const G16_B16_R16_3PLANE_444_UNORM: DataFormat = DataFormat {
        ord: 217i32
    };
    #[doc(alias = "DATA_FORMAT_MAX")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_MAX`"]
    pub const MAX: DataFormat = DataFormat {
        ord: 218i32
    };
    
}
impl std::fmt::Debug for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DataFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DataFormat {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 | ord @ 46i32 | ord @ 47i32 | ord @ 48i32 | ord @ 49i32 | ord @ 50i32 | ord @ 51i32 | ord @ 52i32 | ord @ 53i32 | ord @ 54i32 | ord @ 55i32 | ord @ 56i32 | ord @ 57i32 | ord @ 58i32 | ord @ 59i32 | ord @ 60i32 | ord @ 61i32 | ord @ 62i32 | ord @ 63i32 | ord @ 64i32 | ord @ 65i32 | ord @ 66i32 | ord @ 67i32 | ord @ 68i32 | ord @ 69i32 | ord @ 70i32 | ord @ 71i32 | ord @ 72i32 | ord @ 73i32 | ord @ 74i32 | ord @ 75i32 | ord @ 76i32 | ord @ 77i32 | ord @ 78i32 | ord @ 79i32 | ord @ 80i32 | ord @ 81i32 | ord @ 82i32 | ord @ 83i32 | ord @ 84i32 | ord @ 85i32 | ord @ 86i32 | ord @ 87i32 | ord @ 88i32 | ord @ 89i32 | ord @ 90i32 | ord @ 91i32 | ord @ 92i32 | ord @ 93i32 | ord @ 94i32 | ord @ 95i32 | ord @ 96i32 | ord @ 97i32 | ord @ 98i32 | ord @ 99i32 | ord @ 100i32 | ord @ 101i32 | ord @ 102i32 | ord @ 103i32 | ord @ 104i32 | ord @ 105i32 | ord @ 106i32 | ord @ 107i32 | ord @ 108i32 | ord @ 109i32 | ord @ 110i32 | ord @ 111i32 | ord @ 112i32 | ord @ 113i32 | ord @ 114i32 | ord @ 115i32 | ord @ 116i32 | ord @ 117i32 | ord @ 118i32 | ord @ 119i32 | ord @ 120i32 | ord @ 121i32 | ord @ 122i32 | ord @ 123i32 | ord @ 124i32 | ord @ 125i32 | ord @ 126i32 | ord @ 127i32 | ord @ 128i32 | ord @ 129i32 | ord @ 130i32 | ord @ 131i32 | ord @ 132i32 | ord @ 133i32 | ord @ 134i32 | ord @ 135i32 | ord @ 136i32 | ord @ 137i32 | ord @ 138i32 | ord @ 139i32 | ord @ 140i32 | ord @ 141i32 | ord @ 142i32 | ord @ 143i32 | ord @ 144i32 | ord @ 145i32 | ord @ 146i32 | ord @ 147i32 | ord @ 148i32 | ord @ 149i32 | ord @ 150i32 | ord @ 151i32 | ord @ 152i32 | ord @ 153i32 | ord @ 154i32 | ord @ 155i32 | ord @ 156i32 | ord @ 157i32 | ord @ 158i32 | ord @ 159i32 | ord @ 160i32 | ord @ 161i32 | ord @ 162i32 | ord @ 163i32 | ord @ 164i32 | ord @ 165i32 | ord @ 166i32 | ord @ 167i32 | ord @ 168i32 | ord @ 169i32 | ord @ 170i32 | ord @ 171i32 | ord @ 172i32 | ord @ 173i32 | ord @ 174i32 | ord @ 175i32 | ord @ 176i32 | ord @ 177i32 | ord @ 178i32 | ord @ 179i32 | ord @ 180i32 | ord @ 181i32 | ord @ 182i32 | ord @ 183i32 | ord @ 184i32 | ord @ 185i32 | ord @ 186i32 | ord @ 187i32 | ord @ 188i32 | ord @ 189i32 | ord @ 190i32 | ord @ 191i32 | ord @ 192i32 | ord @ 193i32 | ord @ 194i32 | ord @ 195i32 | ord @ 196i32 | ord @ 197i32 | ord @ 198i32 | ord @ 199i32 | ord @ 200i32 | ord @ 201i32 | ord @ 202i32 | ord @ 203i32 | ord @ 204i32 | ord @ 205i32 | ord @ 206i32 | ord @ 207i32 | ord @ 208i32 | ord @ 209i32 | ord @ 210i32 | ord @ 211i32 | ord @ 212i32 | ord @ 213i32 | ord @ 214i32 | ord @ 215i32 | ord @ 216i32 | ord @ 217i32 | ord @ 218i32 => Some(Self {
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
            Self::R4G4_UNORM_PACK8 => "R4G4_UNORM_PACK8", Self::R4G4B4A4_UNORM_PACK16 => "R4G4B4A4_UNORM_PACK16", Self::B4G4R4A4_UNORM_PACK16 => "B4G4R4A4_UNORM_PACK16", Self::R5G6B5_UNORM_PACK16 => "R5G6B5_UNORM_PACK16", Self::B5G6R5_UNORM_PACK16 => "B5G6R5_UNORM_PACK16", Self::R5G5B5A1_UNORM_PACK16 => "R5G5B5A1_UNORM_PACK16", Self::B5G5R5A1_UNORM_PACK16 => "B5G5R5A1_UNORM_PACK16", Self::A1R5G5B5_UNORM_PACK16 => "A1R5G5B5_UNORM_PACK16", Self::R8_UNORM => "R8_UNORM", Self::R8_SNORM => "R8_SNORM", Self::R8_USCALED => "R8_USCALED", Self::R8_SSCALED => "R8_SSCALED", Self::R8_UINT => "R8_UINT", Self::R8_SINT => "R8_SINT", Self::R8_SRGB => "R8_SRGB", Self::R8G8_UNORM => "R8G8_UNORM", Self::R8G8_SNORM => "R8G8_SNORM", Self::R8G8_USCALED => "R8G8_USCALED", Self::R8G8_SSCALED => "R8G8_SSCALED", Self::R8G8_UINT => "R8G8_UINT", Self::R8G8_SINT => "R8G8_SINT", Self::R8G8_SRGB => "R8G8_SRGB", Self::R8G8B8_UNORM => "R8G8B8_UNORM", Self::R8G8B8_SNORM => "R8G8B8_SNORM", Self::R8G8B8_USCALED => "R8G8B8_USCALED", Self::R8G8B8_SSCALED => "R8G8B8_SSCALED", Self::R8G8B8_UINT => "R8G8B8_UINT", Self::R8G8B8_SINT => "R8G8B8_SINT", Self::R8G8B8_SRGB => "R8G8B8_SRGB", Self::B8G8R8_UNORM => "B8G8R8_UNORM", Self::B8G8R8_SNORM => "B8G8R8_SNORM", Self::B8G8R8_USCALED => "B8G8R8_USCALED", Self::B8G8R8_SSCALED => "B8G8R8_SSCALED", Self::B8G8R8_UINT => "B8G8R8_UINT", Self::B8G8R8_SINT => "B8G8R8_SINT", Self::B8G8R8_SRGB => "B8G8R8_SRGB", Self::R8G8B8A8_UNORM => "R8G8B8A8_UNORM", Self::R8G8B8A8_SNORM => "R8G8B8A8_SNORM", Self::R8G8B8A8_USCALED => "R8G8B8A8_USCALED", Self::R8G8B8A8_SSCALED => "R8G8B8A8_SSCALED", Self::R8G8B8A8_UINT => "R8G8B8A8_UINT", Self::R8G8B8A8_SINT => "R8G8B8A8_SINT", Self::R8G8B8A8_SRGB => "R8G8B8A8_SRGB", Self::B8G8R8A8_UNORM => "B8G8R8A8_UNORM", Self::B8G8R8A8_SNORM => "B8G8R8A8_SNORM", Self::B8G8R8A8_USCALED => "B8G8R8A8_USCALED", Self::B8G8R8A8_SSCALED => "B8G8R8A8_SSCALED", Self::B8G8R8A8_UINT => "B8G8R8A8_UINT", Self::B8G8R8A8_SINT => "B8G8R8A8_SINT", Self::B8G8R8A8_SRGB => "B8G8R8A8_SRGB", Self::A8B8G8R8_UNORM_PACK32 => "A8B8G8R8_UNORM_PACK32", Self::A8B8G8R8_SNORM_PACK32 => "A8B8G8R8_SNORM_PACK32", Self::A8B8G8R8_USCALED_PACK32 => "A8B8G8R8_USCALED_PACK32", Self::A8B8G8R8_SSCALED_PACK32 => "A8B8G8R8_SSCALED_PACK32", Self::A8B8G8R8_UINT_PACK32 => "A8B8G8R8_UINT_PACK32", Self::A8B8G8R8_SINT_PACK32 => "A8B8G8R8_SINT_PACK32", Self::A8B8G8R8_SRGB_PACK32 => "A8B8G8R8_SRGB_PACK32", Self::A2R10G10B10_UNORM_PACK32 => "A2R10G10B10_UNORM_PACK32", Self::A2R10G10B10_SNORM_PACK32 => "A2R10G10B10_SNORM_PACK32", Self::A2R10G10B10_USCALED_PACK32 => "A2R10G10B10_USCALED_PACK32", Self::A2R10G10B10_SSCALED_PACK32 => "A2R10G10B10_SSCALED_PACK32", Self::A2R10G10B10_UINT_PACK32 => "A2R10G10B10_UINT_PACK32", Self::A2R10G10B10_SINT_PACK32 => "A2R10G10B10_SINT_PACK32", Self::A2B10G10R10_UNORM_PACK32 => "A2B10G10R10_UNORM_PACK32", Self::A2B10G10R10_SNORM_PACK32 => "A2B10G10R10_SNORM_PACK32", Self::A2B10G10R10_USCALED_PACK32 => "A2B10G10R10_USCALED_PACK32", Self::A2B10G10R10_SSCALED_PACK32 => "A2B10G10R10_SSCALED_PACK32", Self::A2B10G10R10_UINT_PACK32 => "A2B10G10R10_UINT_PACK32", Self::A2B10G10R10_SINT_PACK32 => "A2B10G10R10_SINT_PACK32", Self::R16_UNORM => "R16_UNORM", Self::R16_SNORM => "R16_SNORM", Self::R16_USCALED => "R16_USCALED", Self::R16_SSCALED => "R16_SSCALED", Self::R16_UINT => "R16_UINT", Self::R16_SINT => "R16_SINT", Self::R16_SFLOAT => "R16_SFLOAT", Self::R16G16_UNORM => "R16G16_UNORM", Self::R16G16_SNORM => "R16G16_SNORM", Self::R16G16_USCALED => "R16G16_USCALED", Self::R16G16_SSCALED => "R16G16_SSCALED", Self::R16G16_UINT => "R16G16_UINT", Self::R16G16_SINT => "R16G16_SINT", Self::R16G16_SFLOAT => "R16G16_SFLOAT", Self::R16G16B16_UNORM => "R16G16B16_UNORM", Self::R16G16B16_SNORM => "R16G16B16_SNORM", Self::R16G16B16_USCALED => "R16G16B16_USCALED", Self::R16G16B16_SSCALED => "R16G16B16_SSCALED", Self::R16G16B16_UINT => "R16G16B16_UINT", Self::R16G16B16_SINT => "R16G16B16_SINT", Self::R16G16B16_SFLOAT => "R16G16B16_SFLOAT", Self::R16G16B16A16_UNORM => "R16G16B16A16_UNORM", Self::R16G16B16A16_SNORM => "R16G16B16A16_SNORM", Self::R16G16B16A16_USCALED => "R16G16B16A16_USCALED", Self::R16G16B16A16_SSCALED => "R16G16B16A16_SSCALED", Self::R16G16B16A16_UINT => "R16G16B16A16_UINT", Self::R16G16B16A16_SINT => "R16G16B16A16_SINT", Self::R16G16B16A16_SFLOAT => "R16G16B16A16_SFLOAT", Self::R32_UINT => "R32_UINT", Self::R32_SINT => "R32_SINT", Self::R32_SFLOAT => "R32_SFLOAT", Self::R32G32_UINT => "R32G32_UINT", Self::R32G32_SINT => "R32G32_SINT", Self::R32G32_SFLOAT => "R32G32_SFLOAT", Self::R32G32B32_UINT => "R32G32B32_UINT", Self::R32G32B32_SINT => "R32G32B32_SINT", Self::R32G32B32_SFLOAT => "R32G32B32_SFLOAT", Self::R32G32B32A32_UINT => "R32G32B32A32_UINT", Self::R32G32B32A32_SINT => "R32G32B32A32_SINT", Self::R32G32B32A32_SFLOAT => "R32G32B32A32_SFLOAT", Self::R64_UINT => "R64_UINT", Self::R64_SINT => "R64_SINT", Self::R64_SFLOAT => "R64_SFLOAT", Self::R64G64_UINT => "R64G64_UINT", Self::R64G64_SINT => "R64G64_SINT", Self::R64G64_SFLOAT => "R64G64_SFLOAT", Self::R64G64B64_UINT => "R64G64B64_UINT", Self::R64G64B64_SINT => "R64G64B64_SINT", Self::R64G64B64_SFLOAT => "R64G64B64_SFLOAT", Self::R64G64B64A64_UINT => "R64G64B64A64_UINT", Self::R64G64B64A64_SINT => "R64G64B64A64_SINT", Self::R64G64B64A64_SFLOAT => "R64G64B64A64_SFLOAT", Self::B10G11R11_UFLOAT_PACK32 => "B10G11R11_UFLOAT_PACK32", Self::E5B9G9R9_UFLOAT_PACK32 => "E5B9G9R9_UFLOAT_PACK32", Self::D16_UNORM => "D16_UNORM", Self::X8_D24_UNORM_PACK32 => "X8_D24_UNORM_PACK32", Self::D32_SFLOAT => "D32_SFLOAT", Self::S8_UINT => "S8_UINT", Self::D16_UNORM_S8_UINT => "D16_UNORM_S8_UINT", Self::D24_UNORM_S8_UINT => "D24_UNORM_S8_UINT", Self::D32_SFLOAT_S8_UINT => "D32_SFLOAT_S8_UINT", Self::BC1_RGB_UNORM_BLOCK => "BC1_RGB_UNORM_BLOCK", Self::BC1_RGB_SRGB_BLOCK => "BC1_RGB_SRGB_BLOCK", Self::BC1_RGBA_UNORM_BLOCK => "BC1_RGBA_UNORM_BLOCK", Self::BC1_RGBA_SRGB_BLOCK => "BC1_RGBA_SRGB_BLOCK", Self::BC2_UNORM_BLOCK => "BC2_UNORM_BLOCK", Self::BC2_SRGB_BLOCK => "BC2_SRGB_BLOCK", Self::BC3_UNORM_BLOCK => "BC3_UNORM_BLOCK", Self::BC3_SRGB_BLOCK => "BC3_SRGB_BLOCK", Self::BC4_UNORM_BLOCK => "BC4_UNORM_BLOCK", Self::BC4_SNORM_BLOCK => "BC4_SNORM_BLOCK", Self::BC5_UNORM_BLOCK => "BC5_UNORM_BLOCK", Self::BC5_SNORM_BLOCK => "BC5_SNORM_BLOCK", Self::BC6H_UFLOAT_BLOCK => "BC6H_UFLOAT_BLOCK", Self::BC6H_SFLOAT_BLOCK => "BC6H_SFLOAT_BLOCK", Self::BC7_UNORM_BLOCK => "BC7_UNORM_BLOCK", Self::BC7_SRGB_BLOCK => "BC7_SRGB_BLOCK", Self::ETC2_R8G8B8_UNORM_BLOCK => "ETC2_R8G8B8_UNORM_BLOCK", Self::ETC2_R8G8B8_SRGB_BLOCK => "ETC2_R8G8B8_SRGB_BLOCK", Self::ETC2_R8G8B8A1_UNORM_BLOCK => "ETC2_R8G8B8A1_UNORM_BLOCK", Self::ETC2_R8G8B8A1_SRGB_BLOCK => "ETC2_R8G8B8A1_SRGB_BLOCK", Self::ETC2_R8G8B8A8_UNORM_BLOCK => "ETC2_R8G8B8A8_UNORM_BLOCK", Self::ETC2_R8G8B8A8_SRGB_BLOCK => "ETC2_R8G8B8A8_SRGB_BLOCK", Self::EAC_R11_UNORM_BLOCK => "EAC_R11_UNORM_BLOCK", Self::EAC_R11_SNORM_BLOCK => "EAC_R11_SNORM_BLOCK", Self::EAC_R11G11_UNORM_BLOCK => "EAC_R11G11_UNORM_BLOCK", Self::EAC_R11G11_SNORM_BLOCK => "EAC_R11G11_SNORM_BLOCK", Self::ASTC_4x4_UNORM_BLOCK => "ASTC_4x4_UNORM_BLOCK", Self::ASTC_4x4_SRGB_BLOCK => "ASTC_4x4_SRGB_BLOCK", Self::ASTC_5x4_UNORM_BLOCK => "ASTC_5x4_UNORM_BLOCK", Self::ASTC_5x4_SRGB_BLOCK => "ASTC_5x4_SRGB_BLOCK", Self::ASTC_5x5_UNORM_BLOCK => "ASTC_5x5_UNORM_BLOCK", Self::ASTC_5x5_SRGB_BLOCK => "ASTC_5x5_SRGB_BLOCK", Self::ASTC_6x5_UNORM_BLOCK => "ASTC_6x5_UNORM_BLOCK", Self::ASTC_6x5_SRGB_BLOCK => "ASTC_6x5_SRGB_BLOCK", Self::ASTC_6x6_UNORM_BLOCK => "ASTC_6x6_UNORM_BLOCK", Self::ASTC_6x6_SRGB_BLOCK => "ASTC_6x6_SRGB_BLOCK", Self::ASTC_8x5_UNORM_BLOCK => "ASTC_8x5_UNORM_BLOCK", Self::ASTC_8x5_SRGB_BLOCK => "ASTC_8x5_SRGB_BLOCK", Self::ASTC_8x6_UNORM_BLOCK => "ASTC_8x6_UNORM_BLOCK", Self::ASTC_8x6_SRGB_BLOCK => "ASTC_8x6_SRGB_BLOCK", Self::ASTC_8x8_UNORM_BLOCK => "ASTC_8x8_UNORM_BLOCK", Self::ASTC_8x8_SRGB_BLOCK => "ASTC_8x8_SRGB_BLOCK", Self::ASTC_10x5_UNORM_BLOCK => "ASTC_10x5_UNORM_BLOCK", Self::ASTC_10x5_SRGB_BLOCK => "ASTC_10x5_SRGB_BLOCK", Self::ASTC_10x6_UNORM_BLOCK => "ASTC_10x6_UNORM_BLOCK", Self::ASTC_10x6_SRGB_BLOCK => "ASTC_10x6_SRGB_BLOCK", Self::ASTC_10x8_UNORM_BLOCK => "ASTC_10x8_UNORM_BLOCK", Self::ASTC_10x8_SRGB_BLOCK => "ASTC_10x8_SRGB_BLOCK", Self::ASTC_10x10_UNORM_BLOCK => "ASTC_10x10_UNORM_BLOCK", Self::ASTC_10x10_SRGB_BLOCK => "ASTC_10x10_SRGB_BLOCK", Self::ASTC_12x10_UNORM_BLOCK => "ASTC_12x10_UNORM_BLOCK", Self::ASTC_12x10_SRGB_BLOCK => "ASTC_12x10_SRGB_BLOCK", Self::ASTC_12x12_UNORM_BLOCK => "ASTC_12x12_UNORM_BLOCK", Self::ASTC_12x12_SRGB_BLOCK => "ASTC_12x12_SRGB_BLOCK", Self::G8B8G8R8_422_UNORM => "G8B8G8R8_422_UNORM", Self::B8G8R8G8_422_UNORM => "B8G8R8G8_422_UNORM", Self::G8_B8_R8_3PLANE_420_UNORM => "G8_B8_R8_3PLANE_420_UNORM", Self::G8_B8R8_2PLANE_420_UNORM => "G8_B8R8_2PLANE_420_UNORM", Self::G8_B8_R8_3PLANE_422_UNORM => "G8_B8_R8_3PLANE_422_UNORM", Self::G8_B8R8_2PLANE_422_UNORM => "G8_B8R8_2PLANE_422_UNORM", Self::G8_B8_R8_3PLANE_444_UNORM => "G8_B8_R8_3PLANE_444_UNORM", Self::R10X6_UNORM_PACK16 => "R10X6_UNORM_PACK16", Self::R10X6G10X6_UNORM_2PACK16 => "R10X6G10X6_UNORM_2PACK16", Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => "R10X6G10X6B10X6A10X6_UNORM_4PACK16", Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => "G10X6B10X6G10X6R10X6_422_UNORM_4PACK16", Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => "B10X6G10X6R10X6G10X6_422_UNORM_4PACK16", Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => "G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16", Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => "G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16", Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => "G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16", Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => "G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16", Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => "G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16", Self::R12X4_UNORM_PACK16 => "R12X4_UNORM_PACK16", Self::R12X4G12X4_UNORM_2PACK16 => "R12X4G12X4_UNORM_2PACK16", Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => "R12X4G12X4B12X4A12X4_UNORM_4PACK16", Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => "G12X4B12X4G12X4R12X4_422_UNORM_4PACK16", Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => "B12X4G12X4R12X4G12X4_422_UNORM_4PACK16", Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => "G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16", Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => "G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16", Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => "G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16", Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => "G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16", Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => "G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16", Self::G16B16G16R16_422_UNORM => "G16B16G16R16_422_UNORM", Self::B16G16R16G16_422_UNORM => "B16G16R16G16_422_UNORM", Self::G16_B16_R16_3PLANE_420_UNORM => "G16_B16_R16_3PLANE_420_UNORM", Self::G16_B16R16_2PLANE_420_UNORM => "G16_B16R16_2PLANE_420_UNORM", Self::G16_B16_R16_3PLANE_422_UNORM => "G16_B16_R16_3PLANE_422_UNORM", Self::G16_B16R16_2PLANE_422_UNORM => "G16_B16R16_2PLANE_422_UNORM", Self::G16_B16_R16_3PLANE_444_UNORM => "G16_B16_R16_3PLANE_444_UNORM", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::R4G4_UNORM_PACK8 => "DATA_FORMAT_R4G4_UNORM_PACK8", Self::R4G4B4A4_UNORM_PACK16 => "DATA_FORMAT_R4G4B4A4_UNORM_PACK16", Self::B4G4R4A4_UNORM_PACK16 => "DATA_FORMAT_B4G4R4A4_UNORM_PACK16", Self::R5G6B5_UNORM_PACK16 => "DATA_FORMAT_R5G6B5_UNORM_PACK16", Self::B5G6R5_UNORM_PACK16 => "DATA_FORMAT_B5G6R5_UNORM_PACK16", Self::R5G5B5A1_UNORM_PACK16 => "DATA_FORMAT_R5G5B5A1_UNORM_PACK16", Self::B5G5R5A1_UNORM_PACK16 => "DATA_FORMAT_B5G5R5A1_UNORM_PACK16", Self::A1R5G5B5_UNORM_PACK16 => "DATA_FORMAT_A1R5G5B5_UNORM_PACK16", Self::R8_UNORM => "DATA_FORMAT_R8_UNORM", Self::R8_SNORM => "DATA_FORMAT_R8_SNORM", Self::R8_USCALED => "DATA_FORMAT_R8_USCALED", Self::R8_SSCALED => "DATA_FORMAT_R8_SSCALED", Self::R8_UINT => "DATA_FORMAT_R8_UINT", Self::R8_SINT => "DATA_FORMAT_R8_SINT", Self::R8_SRGB => "DATA_FORMAT_R8_SRGB", Self::R8G8_UNORM => "DATA_FORMAT_R8G8_UNORM", Self::R8G8_SNORM => "DATA_FORMAT_R8G8_SNORM", Self::R8G8_USCALED => "DATA_FORMAT_R8G8_USCALED", Self::R8G8_SSCALED => "DATA_FORMAT_R8G8_SSCALED", Self::R8G8_UINT => "DATA_FORMAT_R8G8_UINT", Self::R8G8_SINT => "DATA_FORMAT_R8G8_SINT", Self::R8G8_SRGB => "DATA_FORMAT_R8G8_SRGB", Self::R8G8B8_UNORM => "DATA_FORMAT_R8G8B8_UNORM", Self::R8G8B8_SNORM => "DATA_FORMAT_R8G8B8_SNORM", Self::R8G8B8_USCALED => "DATA_FORMAT_R8G8B8_USCALED", Self::R8G8B8_SSCALED => "DATA_FORMAT_R8G8B8_SSCALED", Self::R8G8B8_UINT => "DATA_FORMAT_R8G8B8_UINT", Self::R8G8B8_SINT => "DATA_FORMAT_R8G8B8_SINT", Self::R8G8B8_SRGB => "DATA_FORMAT_R8G8B8_SRGB", Self::B8G8R8_UNORM => "DATA_FORMAT_B8G8R8_UNORM", Self::B8G8R8_SNORM => "DATA_FORMAT_B8G8R8_SNORM", Self::B8G8R8_USCALED => "DATA_FORMAT_B8G8R8_USCALED", Self::B8G8R8_SSCALED => "DATA_FORMAT_B8G8R8_SSCALED", Self::B8G8R8_UINT => "DATA_FORMAT_B8G8R8_UINT", Self::B8G8R8_SINT => "DATA_FORMAT_B8G8R8_SINT", Self::B8G8R8_SRGB => "DATA_FORMAT_B8G8R8_SRGB", Self::R8G8B8A8_UNORM => "DATA_FORMAT_R8G8B8A8_UNORM", Self::R8G8B8A8_SNORM => "DATA_FORMAT_R8G8B8A8_SNORM", Self::R8G8B8A8_USCALED => "DATA_FORMAT_R8G8B8A8_USCALED", Self::R8G8B8A8_SSCALED => "DATA_FORMAT_R8G8B8A8_SSCALED", Self::R8G8B8A8_UINT => "DATA_FORMAT_R8G8B8A8_UINT", Self::R8G8B8A8_SINT => "DATA_FORMAT_R8G8B8A8_SINT", Self::R8G8B8A8_SRGB => "DATA_FORMAT_R8G8B8A8_SRGB", Self::B8G8R8A8_UNORM => "DATA_FORMAT_B8G8R8A8_UNORM", Self::B8G8R8A8_SNORM => "DATA_FORMAT_B8G8R8A8_SNORM", Self::B8G8R8A8_USCALED => "DATA_FORMAT_B8G8R8A8_USCALED", Self::B8G8R8A8_SSCALED => "DATA_FORMAT_B8G8R8A8_SSCALED", Self::B8G8R8A8_UINT => "DATA_FORMAT_B8G8R8A8_UINT", Self::B8G8R8A8_SINT => "DATA_FORMAT_B8G8R8A8_SINT", Self::B8G8R8A8_SRGB => "DATA_FORMAT_B8G8R8A8_SRGB", Self::A8B8G8R8_UNORM_PACK32 => "DATA_FORMAT_A8B8G8R8_UNORM_PACK32", Self::A8B8G8R8_SNORM_PACK32 => "DATA_FORMAT_A8B8G8R8_SNORM_PACK32", Self::A8B8G8R8_USCALED_PACK32 => "DATA_FORMAT_A8B8G8R8_USCALED_PACK32", Self::A8B8G8R8_SSCALED_PACK32 => "DATA_FORMAT_A8B8G8R8_SSCALED_PACK32", Self::A8B8G8R8_UINT_PACK32 => "DATA_FORMAT_A8B8G8R8_UINT_PACK32", Self::A8B8G8R8_SINT_PACK32 => "DATA_FORMAT_A8B8G8R8_SINT_PACK32", Self::A8B8G8R8_SRGB_PACK32 => "DATA_FORMAT_A8B8G8R8_SRGB_PACK32", Self::A2R10G10B10_UNORM_PACK32 => "DATA_FORMAT_A2R10G10B10_UNORM_PACK32", Self::A2R10G10B10_SNORM_PACK32 => "DATA_FORMAT_A2R10G10B10_SNORM_PACK32", Self::A2R10G10B10_USCALED_PACK32 => "DATA_FORMAT_A2R10G10B10_USCALED_PACK32", Self::A2R10G10B10_SSCALED_PACK32 => "DATA_FORMAT_A2R10G10B10_SSCALED_PACK32", Self::A2R10G10B10_UINT_PACK32 => "DATA_FORMAT_A2R10G10B10_UINT_PACK32", Self::A2R10G10B10_SINT_PACK32 => "DATA_FORMAT_A2R10G10B10_SINT_PACK32", Self::A2B10G10R10_UNORM_PACK32 => "DATA_FORMAT_A2B10G10R10_UNORM_PACK32", Self::A2B10G10R10_SNORM_PACK32 => "DATA_FORMAT_A2B10G10R10_SNORM_PACK32", Self::A2B10G10R10_USCALED_PACK32 => "DATA_FORMAT_A2B10G10R10_USCALED_PACK32", Self::A2B10G10R10_SSCALED_PACK32 => "DATA_FORMAT_A2B10G10R10_SSCALED_PACK32", Self::A2B10G10R10_UINT_PACK32 => "DATA_FORMAT_A2B10G10R10_UINT_PACK32", Self::A2B10G10R10_SINT_PACK32 => "DATA_FORMAT_A2B10G10R10_SINT_PACK32", Self::R16_UNORM => "DATA_FORMAT_R16_UNORM", Self::R16_SNORM => "DATA_FORMAT_R16_SNORM", Self::R16_USCALED => "DATA_FORMAT_R16_USCALED", Self::R16_SSCALED => "DATA_FORMAT_R16_SSCALED", Self::R16_UINT => "DATA_FORMAT_R16_UINT", Self::R16_SINT => "DATA_FORMAT_R16_SINT", Self::R16_SFLOAT => "DATA_FORMAT_R16_SFLOAT", Self::R16G16_UNORM => "DATA_FORMAT_R16G16_UNORM", Self::R16G16_SNORM => "DATA_FORMAT_R16G16_SNORM", Self::R16G16_USCALED => "DATA_FORMAT_R16G16_USCALED", Self::R16G16_SSCALED => "DATA_FORMAT_R16G16_SSCALED", Self::R16G16_UINT => "DATA_FORMAT_R16G16_UINT", Self::R16G16_SINT => "DATA_FORMAT_R16G16_SINT", Self::R16G16_SFLOAT => "DATA_FORMAT_R16G16_SFLOAT", Self::R16G16B16_UNORM => "DATA_FORMAT_R16G16B16_UNORM", Self::R16G16B16_SNORM => "DATA_FORMAT_R16G16B16_SNORM", Self::R16G16B16_USCALED => "DATA_FORMAT_R16G16B16_USCALED", Self::R16G16B16_SSCALED => "DATA_FORMAT_R16G16B16_SSCALED", Self::R16G16B16_UINT => "DATA_FORMAT_R16G16B16_UINT", Self::R16G16B16_SINT => "DATA_FORMAT_R16G16B16_SINT", Self::R16G16B16_SFLOAT => "DATA_FORMAT_R16G16B16_SFLOAT", Self::R16G16B16A16_UNORM => "DATA_FORMAT_R16G16B16A16_UNORM", Self::R16G16B16A16_SNORM => "DATA_FORMAT_R16G16B16A16_SNORM", Self::R16G16B16A16_USCALED => "DATA_FORMAT_R16G16B16A16_USCALED", Self::R16G16B16A16_SSCALED => "DATA_FORMAT_R16G16B16A16_SSCALED", Self::R16G16B16A16_UINT => "DATA_FORMAT_R16G16B16A16_UINT", Self::R16G16B16A16_SINT => "DATA_FORMAT_R16G16B16A16_SINT", Self::R16G16B16A16_SFLOAT => "DATA_FORMAT_R16G16B16A16_SFLOAT", Self::R32_UINT => "DATA_FORMAT_R32_UINT", Self::R32_SINT => "DATA_FORMAT_R32_SINT", Self::R32_SFLOAT => "DATA_FORMAT_R32_SFLOAT", Self::R32G32_UINT => "DATA_FORMAT_R32G32_UINT", Self::R32G32_SINT => "DATA_FORMAT_R32G32_SINT", Self::R32G32_SFLOAT => "DATA_FORMAT_R32G32_SFLOAT", Self::R32G32B32_UINT => "DATA_FORMAT_R32G32B32_UINT", Self::R32G32B32_SINT => "DATA_FORMAT_R32G32B32_SINT", Self::R32G32B32_SFLOAT => "DATA_FORMAT_R32G32B32_SFLOAT", Self::R32G32B32A32_UINT => "DATA_FORMAT_R32G32B32A32_UINT", Self::R32G32B32A32_SINT => "DATA_FORMAT_R32G32B32A32_SINT", Self::R32G32B32A32_SFLOAT => "DATA_FORMAT_R32G32B32A32_SFLOAT", Self::R64_UINT => "DATA_FORMAT_R64_UINT", Self::R64_SINT => "DATA_FORMAT_R64_SINT", Self::R64_SFLOAT => "DATA_FORMAT_R64_SFLOAT", Self::R64G64_UINT => "DATA_FORMAT_R64G64_UINT", Self::R64G64_SINT => "DATA_FORMAT_R64G64_SINT", Self::R64G64_SFLOAT => "DATA_FORMAT_R64G64_SFLOAT", Self::R64G64B64_UINT => "DATA_FORMAT_R64G64B64_UINT", Self::R64G64B64_SINT => "DATA_FORMAT_R64G64B64_SINT", Self::R64G64B64_SFLOAT => "DATA_FORMAT_R64G64B64_SFLOAT", Self::R64G64B64A64_UINT => "DATA_FORMAT_R64G64B64A64_UINT", Self::R64G64B64A64_SINT => "DATA_FORMAT_R64G64B64A64_SINT", Self::R64G64B64A64_SFLOAT => "DATA_FORMAT_R64G64B64A64_SFLOAT", Self::B10G11R11_UFLOAT_PACK32 => "DATA_FORMAT_B10G11R11_UFLOAT_PACK32", Self::E5B9G9R9_UFLOAT_PACK32 => "DATA_FORMAT_E5B9G9R9_UFLOAT_PACK32", Self::D16_UNORM => "DATA_FORMAT_D16_UNORM", Self::X8_D24_UNORM_PACK32 => "DATA_FORMAT_X8_D24_UNORM_PACK32", Self::D32_SFLOAT => "DATA_FORMAT_D32_SFLOAT", Self::S8_UINT => "DATA_FORMAT_S8_UINT", Self::D16_UNORM_S8_UINT => "DATA_FORMAT_D16_UNORM_S8_UINT", Self::D24_UNORM_S8_UINT => "DATA_FORMAT_D24_UNORM_S8_UINT", Self::D32_SFLOAT_S8_UINT => "DATA_FORMAT_D32_SFLOAT_S8_UINT", Self::BC1_RGB_UNORM_BLOCK => "DATA_FORMAT_BC1_RGB_UNORM_BLOCK", Self::BC1_RGB_SRGB_BLOCK => "DATA_FORMAT_BC1_RGB_SRGB_BLOCK", Self::BC1_RGBA_UNORM_BLOCK => "DATA_FORMAT_BC1_RGBA_UNORM_BLOCK", Self::BC1_RGBA_SRGB_BLOCK => "DATA_FORMAT_BC1_RGBA_SRGB_BLOCK", Self::BC2_UNORM_BLOCK => "DATA_FORMAT_BC2_UNORM_BLOCK", Self::BC2_SRGB_BLOCK => "DATA_FORMAT_BC2_SRGB_BLOCK", Self::BC3_UNORM_BLOCK => "DATA_FORMAT_BC3_UNORM_BLOCK", Self::BC3_SRGB_BLOCK => "DATA_FORMAT_BC3_SRGB_BLOCK", Self::BC4_UNORM_BLOCK => "DATA_FORMAT_BC4_UNORM_BLOCK", Self::BC4_SNORM_BLOCK => "DATA_FORMAT_BC4_SNORM_BLOCK", Self::BC5_UNORM_BLOCK => "DATA_FORMAT_BC5_UNORM_BLOCK", Self::BC5_SNORM_BLOCK => "DATA_FORMAT_BC5_SNORM_BLOCK", Self::BC6H_UFLOAT_BLOCK => "DATA_FORMAT_BC6H_UFLOAT_BLOCK", Self::BC6H_SFLOAT_BLOCK => "DATA_FORMAT_BC6H_SFLOAT_BLOCK", Self::BC7_UNORM_BLOCK => "DATA_FORMAT_BC7_UNORM_BLOCK", Self::BC7_SRGB_BLOCK => "DATA_FORMAT_BC7_SRGB_BLOCK", Self::ETC2_R8G8B8_UNORM_BLOCK => "DATA_FORMAT_ETC2_R8G8B8_UNORM_BLOCK", Self::ETC2_R8G8B8_SRGB_BLOCK => "DATA_FORMAT_ETC2_R8G8B8_SRGB_BLOCK", Self::ETC2_R8G8B8A1_UNORM_BLOCK => "DATA_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK", Self::ETC2_R8G8B8A1_SRGB_BLOCK => "DATA_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK", Self::ETC2_R8G8B8A8_UNORM_BLOCK => "DATA_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK", Self::ETC2_R8G8B8A8_SRGB_BLOCK => "DATA_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK", Self::EAC_R11_UNORM_BLOCK => "DATA_FORMAT_EAC_R11_UNORM_BLOCK", Self::EAC_R11_SNORM_BLOCK => "DATA_FORMAT_EAC_R11_SNORM_BLOCK", Self::EAC_R11G11_UNORM_BLOCK => "DATA_FORMAT_EAC_R11G11_UNORM_BLOCK", Self::EAC_R11G11_SNORM_BLOCK => "DATA_FORMAT_EAC_R11G11_SNORM_BLOCK", Self::ASTC_4x4_UNORM_BLOCK => "DATA_FORMAT_ASTC_4x4_UNORM_BLOCK", Self::ASTC_4x4_SRGB_BLOCK => "DATA_FORMAT_ASTC_4x4_SRGB_BLOCK", Self::ASTC_5x4_UNORM_BLOCK => "DATA_FORMAT_ASTC_5x4_UNORM_BLOCK", Self::ASTC_5x4_SRGB_BLOCK => "DATA_FORMAT_ASTC_5x4_SRGB_BLOCK", Self::ASTC_5x5_UNORM_BLOCK => "DATA_FORMAT_ASTC_5x5_UNORM_BLOCK", Self::ASTC_5x5_SRGB_BLOCK => "DATA_FORMAT_ASTC_5x5_SRGB_BLOCK", Self::ASTC_6x5_UNORM_BLOCK => "DATA_FORMAT_ASTC_6x5_UNORM_BLOCK", Self::ASTC_6x5_SRGB_BLOCK => "DATA_FORMAT_ASTC_6x5_SRGB_BLOCK", Self::ASTC_6x6_UNORM_BLOCK => "DATA_FORMAT_ASTC_6x6_UNORM_BLOCK", Self::ASTC_6x6_SRGB_BLOCK => "DATA_FORMAT_ASTC_6x6_SRGB_BLOCK", Self::ASTC_8x5_UNORM_BLOCK => "DATA_FORMAT_ASTC_8x5_UNORM_BLOCK", Self::ASTC_8x5_SRGB_BLOCK => "DATA_FORMAT_ASTC_8x5_SRGB_BLOCK", Self::ASTC_8x6_UNORM_BLOCK => "DATA_FORMAT_ASTC_8x6_UNORM_BLOCK", Self::ASTC_8x6_SRGB_BLOCK => "DATA_FORMAT_ASTC_8x6_SRGB_BLOCK", Self::ASTC_8x8_UNORM_BLOCK => "DATA_FORMAT_ASTC_8x8_UNORM_BLOCK", Self::ASTC_8x8_SRGB_BLOCK => "DATA_FORMAT_ASTC_8x8_SRGB_BLOCK", Self::ASTC_10x5_UNORM_BLOCK => "DATA_FORMAT_ASTC_10x5_UNORM_BLOCK", Self::ASTC_10x5_SRGB_BLOCK => "DATA_FORMAT_ASTC_10x5_SRGB_BLOCK", Self::ASTC_10x6_UNORM_BLOCK => "DATA_FORMAT_ASTC_10x6_UNORM_BLOCK", Self::ASTC_10x6_SRGB_BLOCK => "DATA_FORMAT_ASTC_10x6_SRGB_BLOCK", Self::ASTC_10x8_UNORM_BLOCK => "DATA_FORMAT_ASTC_10x8_UNORM_BLOCK", Self::ASTC_10x8_SRGB_BLOCK => "DATA_FORMAT_ASTC_10x8_SRGB_BLOCK", Self::ASTC_10x10_UNORM_BLOCK => "DATA_FORMAT_ASTC_10x10_UNORM_BLOCK", Self::ASTC_10x10_SRGB_BLOCK => "DATA_FORMAT_ASTC_10x10_SRGB_BLOCK", Self::ASTC_12x10_UNORM_BLOCK => "DATA_FORMAT_ASTC_12x10_UNORM_BLOCK", Self::ASTC_12x10_SRGB_BLOCK => "DATA_FORMAT_ASTC_12x10_SRGB_BLOCK", Self::ASTC_12x12_UNORM_BLOCK => "DATA_FORMAT_ASTC_12x12_UNORM_BLOCK", Self::ASTC_12x12_SRGB_BLOCK => "DATA_FORMAT_ASTC_12x12_SRGB_BLOCK", Self::G8B8G8R8_422_UNORM => "DATA_FORMAT_G8B8G8R8_422_UNORM", Self::B8G8R8G8_422_UNORM => "DATA_FORMAT_B8G8R8G8_422_UNORM", Self::G8_B8_R8_3PLANE_420_UNORM => "DATA_FORMAT_G8_B8_R8_3PLANE_420_UNORM", Self::G8_B8R8_2PLANE_420_UNORM => "DATA_FORMAT_G8_B8R8_2PLANE_420_UNORM", Self::G8_B8_R8_3PLANE_422_UNORM => "DATA_FORMAT_G8_B8_R8_3PLANE_422_UNORM", Self::G8_B8R8_2PLANE_422_UNORM => "DATA_FORMAT_G8_B8R8_2PLANE_422_UNORM", Self::G8_B8_R8_3PLANE_444_UNORM => "DATA_FORMAT_G8_B8_R8_3PLANE_444_UNORM", Self::R10X6_UNORM_PACK16 => "DATA_FORMAT_R10X6_UNORM_PACK16", Self::R10X6G10X6_UNORM_2PACK16 => "DATA_FORMAT_R10X6G10X6_UNORM_2PACK16", Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => "DATA_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16", Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => "DATA_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16", Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => "DATA_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16", Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16", Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => "DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16", Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16", Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => "DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16", Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16", Self::R12X4_UNORM_PACK16 => "DATA_FORMAT_R12X4_UNORM_PACK16", Self::R12X4G12X4_UNORM_2PACK16 => "DATA_FORMAT_R12X4G12X4_UNORM_2PACK16", Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => "DATA_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16", Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => "DATA_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16", Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => "DATA_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16", Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16", Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => "DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16", Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16", Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => "DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16", Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16", Self::G16B16G16R16_422_UNORM => "DATA_FORMAT_G16B16G16R16_422_UNORM", Self::B16G16R16G16_422_UNORM => "DATA_FORMAT_B16G16R16G16_422_UNORM", Self::G16_B16_R16_3PLANE_420_UNORM => "DATA_FORMAT_G16_B16_R16_3PLANE_420_UNORM", Self::G16_B16R16_2PLANE_420_UNORM => "DATA_FORMAT_G16_B16R16_2PLANE_420_UNORM", Self::G16_B16_R16_3PLANE_422_UNORM => "DATA_FORMAT_G16_B16_R16_3PLANE_422_UNORM", Self::G16_B16R16_2PLANE_422_UNORM => "DATA_FORMAT_G16_B16R16_2PLANE_422_UNORM", Self::G16_B16_R16_3PLANE_444_UNORM => "DATA_FORMAT_G16_B16_R16_3PLANE_444_UNORM", Self::MAX => "DATA_FORMAT_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for DataFormat {
    const ENUMERATOR_COUNT: usize = 218usize;
    
}
impl crate::meta::GodotConvert for DataFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DataFormat {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DataFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct BarrierMask {
    ord: u64
}
impl BarrierMask {
    #[doc(alias = "BARRIER_MASK_VERTEX")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_VERTEX`"]
    pub const VERTEX: BarrierMask = BarrierMask {
        ord: 1u64
    };
    #[doc(alias = "BARRIER_MASK_FRAGMENT")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_FRAGMENT`"]
    pub const FRAGMENT: BarrierMask = BarrierMask {
        ord: 8u64
    };
    #[doc(alias = "BARRIER_MASK_COMPUTE")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_COMPUTE`"]
    pub const COMPUTE: BarrierMask = BarrierMask {
        ord: 2u64
    };
    #[doc(alias = "BARRIER_MASK_TRANSFER")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_TRANSFER`"]
    pub const TRANSFER: BarrierMask = BarrierMask {
        ord: 4u64
    };
    #[doc(alias = "BARRIER_MASK_RASTER")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_RASTER`"]
    pub const RASTER: BarrierMask = BarrierMask {
        ord: 9u64
    };
    #[doc(alias = "BARRIER_MASK_ALL_BARRIERS")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_ALL_BARRIERS`"]
    pub const ALL_BARRIERS: BarrierMask = BarrierMask {
        ord: 32767u64
    };
    #[doc(alias = "BARRIER_MASK_NO_BARRIER")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_NO_BARRIER`"]
    pub const NO_BARRIER: BarrierMask = BarrierMask {
        ord: 32768u64
    };
    
}
impl std::fmt::Debug for BarrierMask {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::VERTEX => "VERTEX", Self::FRAGMENT => "FRAGMENT", Self::COMPUTE => "COMPUTE", Self::TRANSFER => "TRANSFER", Self::RASTER => "RASTER", Self::ALL_BARRIERS => "ALL_BARRIERS", Self::NO_BARRIER => "NO_BARRIER", _ => {
                f.debug_struct("BarrierMask") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for BarrierMask {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for BarrierMask {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for BarrierMask {
    type Via = u64;
    
}
impl crate::meta::ToGodot for BarrierMask {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BarrierMask {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureType {
    ord: i32
}
impl TextureType {
    #[doc(alias = "TEXTURE_TYPE_1D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_1D`"]
    pub const TYPE_1D: TextureType = TextureType {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_TYPE_2D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_2D`"]
    pub const TYPE_2D: TextureType = TextureType {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_TYPE_3D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_3D`"]
    pub const TYPE_3D: TextureType = TextureType {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_TYPE_CUBE")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_CUBE`"]
    pub const CUBE: TextureType = TextureType {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_TYPE_1D_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_1D_ARRAY`"]
    pub const TYPE_1D_ARRAY: TextureType = TextureType {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_TYPE_2D_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_2D_ARRAY`"]
    pub const TYPE_2D_ARRAY: TextureType = TextureType {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_TYPE_CUBE_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_CUBE_ARRAY`"]
    pub const CUBE_ARRAY: TextureType = TextureType {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_TYPE_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_MAX`"]
    pub const MAX: TextureType = TextureType {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for TextureType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureType {
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
            Self::TYPE_1D => "TYPE_1D", Self::TYPE_2D => "TYPE_2D", Self::TYPE_3D => "TYPE_3D", Self::CUBE => "CUBE", Self::TYPE_1D_ARRAY => "TYPE_1D_ARRAY", Self::TYPE_2D_ARRAY => "TYPE_2D_ARRAY", Self::CUBE_ARRAY => "CUBE_ARRAY", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TYPE_1D => "TEXTURE_TYPE_1D", Self::TYPE_2D => "TEXTURE_TYPE_2D", Self::TYPE_3D => "TEXTURE_TYPE_3D", Self::CUBE => "TEXTURE_TYPE_CUBE", Self::TYPE_1D_ARRAY => "TEXTURE_TYPE_1D_ARRAY", Self::TYPE_2D_ARRAY => "TEXTURE_TYPE_2D_ARRAY", Self::CUBE_ARRAY => "TEXTURE_TYPE_CUBE_ARRAY", Self::MAX => "TEXTURE_TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TextureType {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for TextureType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureSamples {
    ord: i32
}
impl TextureSamples {
    #[doc(alias = "TEXTURE_SAMPLES_1")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_1`"]
    pub const SAMPLES_1: TextureSamples = TextureSamples {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_2")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_2`"]
    pub const SAMPLES_2: TextureSamples = TextureSamples {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_4")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_4`"]
    pub const SAMPLES_4: TextureSamples = TextureSamples {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_8")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_8`"]
    pub const SAMPLES_8: TextureSamples = TextureSamples {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_16")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_16`"]
    pub const SAMPLES_16: TextureSamples = TextureSamples {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_32")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_32`"]
    pub const SAMPLES_32: TextureSamples = TextureSamples {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_64")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_64`"]
    pub const SAMPLES_64: TextureSamples = TextureSamples {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_MAX`"]
    pub const MAX: TextureSamples = TextureSamples {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for TextureSamples {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureSamples") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureSamples {
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
            Self::SAMPLES_1 => "SAMPLES_1", Self::SAMPLES_2 => "SAMPLES_2", Self::SAMPLES_4 => "SAMPLES_4", Self::SAMPLES_8 => "SAMPLES_8", Self::SAMPLES_16 => "SAMPLES_16", Self::SAMPLES_32 => "SAMPLES_32", Self::SAMPLES_64 => "SAMPLES_64", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SAMPLES_1 => "TEXTURE_SAMPLES_1", Self::SAMPLES_2 => "TEXTURE_SAMPLES_2", Self::SAMPLES_4 => "TEXTURE_SAMPLES_4", Self::SAMPLES_8 => "TEXTURE_SAMPLES_8", Self::SAMPLES_16 => "TEXTURE_SAMPLES_16", Self::SAMPLES_32 => "TEXTURE_SAMPLES_32", Self::SAMPLES_64 => "TEXTURE_SAMPLES_64", Self::MAX => "TEXTURE_SAMPLES_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TextureSamples {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for TextureSamples {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureSamples {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureSamples {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct TextureUsageBits {
    ord: u64
}
impl TextureUsageBits {
    #[doc(alias = "TEXTURE_USAGE_SAMPLING_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_SAMPLING_BIT`"]
    pub const SAMPLING_BIT: TextureUsageBits = TextureUsageBits {
        ord: 1u64
    };
    #[doc(alias = "TEXTURE_USAGE_COLOR_ATTACHMENT_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_COLOR_ATTACHMENT_BIT`"]
    pub const COLOR_ATTACHMENT_BIT: TextureUsageBits = TextureUsageBits {
        ord: 2u64
    };
    #[doc(alias = "TEXTURE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`"]
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: TextureUsageBits = TextureUsageBits {
        ord: 4u64
    };
    #[doc(alias = "TEXTURE_USAGE_STORAGE_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_STORAGE_BIT`"]
    pub const STORAGE_BIT: TextureUsageBits = TextureUsageBits {
        ord: 8u64
    };
    #[doc(alias = "TEXTURE_USAGE_STORAGE_ATOMIC_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_STORAGE_ATOMIC_BIT`"]
    pub const STORAGE_ATOMIC_BIT: TextureUsageBits = TextureUsageBits {
        ord: 16u64
    };
    #[doc(alias = "TEXTURE_USAGE_CPU_READ_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_CPU_READ_BIT`"]
    pub const CPU_READ_BIT: TextureUsageBits = TextureUsageBits {
        ord: 32u64
    };
    #[doc(alias = "TEXTURE_USAGE_CAN_UPDATE_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_CAN_UPDATE_BIT`"]
    pub const CAN_UPDATE_BIT: TextureUsageBits = TextureUsageBits {
        ord: 64u64
    };
    #[doc(alias = "TEXTURE_USAGE_CAN_COPY_FROM_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_CAN_COPY_FROM_BIT`"]
    pub const CAN_COPY_FROM_BIT: TextureUsageBits = TextureUsageBits {
        ord: 128u64
    };
    #[doc(alias = "TEXTURE_USAGE_CAN_COPY_TO_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_CAN_COPY_TO_BIT`"]
    pub const CAN_COPY_TO_BIT: TextureUsageBits = TextureUsageBits {
        ord: 256u64
    };
    #[doc(alias = "TEXTURE_USAGE_INPUT_ATTACHMENT_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_INPUT_ATTACHMENT_BIT`"]
    pub const INPUT_ATTACHMENT_BIT: TextureUsageBits = TextureUsageBits {
        ord: 512u64
    };
    
}
impl std::fmt::Debug for TextureUsageBits {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::SAMPLING_BIT => "SAMPLING_BIT", Self::COLOR_ATTACHMENT_BIT => "COLOR_ATTACHMENT_BIT", Self::DEPTH_STENCIL_ATTACHMENT_BIT => "DEPTH_STENCIL_ATTACHMENT_BIT", Self::STORAGE_BIT => "STORAGE_BIT", Self::STORAGE_ATOMIC_BIT => "STORAGE_ATOMIC_BIT", Self::CPU_READ_BIT => "CPU_READ_BIT", Self::CAN_UPDATE_BIT => "CAN_UPDATE_BIT", Self::CAN_COPY_FROM_BIT => "CAN_COPY_FROM_BIT", Self::CAN_COPY_TO_BIT => "CAN_COPY_TO_BIT", Self::INPUT_ATTACHMENT_BIT => "INPUT_ATTACHMENT_BIT", _ => {
                f.debug_struct("TextureUsageBits") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for TextureUsageBits {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for TextureUsageBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for TextureUsageBits {
    type Via = u64;
    
}
impl crate::meta::ToGodot for TextureUsageBits {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureUsageBits {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureSwizzle {
    ord: i32
}
impl TextureSwizzle {
    #[doc(alias = "TEXTURE_SWIZZLE_IDENTITY")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_IDENTITY`"]
    pub const IDENTITY: TextureSwizzle = TextureSwizzle {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_ZERO")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_ZERO`"]
    pub const ZERO: TextureSwizzle = TextureSwizzle {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_ONE")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_ONE`"]
    pub const ONE: TextureSwizzle = TextureSwizzle {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_R")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_R`"]
    pub const R: TextureSwizzle = TextureSwizzle {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_G")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_G`"]
    pub const G: TextureSwizzle = TextureSwizzle {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_B")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_B`"]
    pub const B: TextureSwizzle = TextureSwizzle {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_A")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_A`"]
    pub const A: TextureSwizzle = TextureSwizzle {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_MAX`"]
    pub const MAX: TextureSwizzle = TextureSwizzle {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for TextureSwizzle {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureSwizzle") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureSwizzle {
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
            Self::IDENTITY => "IDENTITY", Self::ZERO => "ZERO", Self::ONE => "ONE", Self::R => "R", Self::G => "G", Self::B => "B", Self::A => "A", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::IDENTITY => "TEXTURE_SWIZZLE_IDENTITY", Self::ZERO => "TEXTURE_SWIZZLE_ZERO", Self::ONE => "TEXTURE_SWIZZLE_ONE", Self::R => "TEXTURE_SWIZZLE_R", Self::G => "TEXTURE_SWIZZLE_G", Self::B => "TEXTURE_SWIZZLE_B", Self::A => "TEXTURE_SWIZZLE_A", Self::MAX => "TEXTURE_SWIZZLE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for TextureSwizzle {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for TextureSwizzle {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureSwizzle {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureSwizzle {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureSliceType {
    ord: i32
}
impl TextureSliceType {
    #[doc(alias = "TEXTURE_SLICE_2D")]
    #[doc = "Godot enumerator name: `TEXTURE_SLICE_2D`"]
    pub const SLICE_2D: TextureSliceType = TextureSliceType {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_SLICE_CUBEMAP")]
    #[doc = "Godot enumerator name: `TEXTURE_SLICE_CUBEMAP`"]
    pub const CUBEMAP: TextureSliceType = TextureSliceType {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_SLICE_3D")]
    #[doc = "Godot enumerator name: `TEXTURE_SLICE_3D`"]
    pub const SLICE_3D: TextureSliceType = TextureSliceType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TextureSliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureSliceType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureSliceType {
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
            Self::SLICE_2D => "SLICE_2D", Self::CUBEMAP => "CUBEMAP", Self::SLICE_3D => "SLICE_3D", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SLICE_2D => "TEXTURE_SLICE_2D", Self::CUBEMAP => "TEXTURE_SLICE_CUBEMAP", Self::SLICE_3D => "TEXTURE_SLICE_3D", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TextureSliceType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureSliceType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureSliceType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SamplerFilter {
    ord: i32
}
impl SamplerFilter {
    #[doc(alias = "SAMPLER_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `SAMPLER_FILTER_NEAREST`"]
    pub const NEAREST: SamplerFilter = SamplerFilter {
        ord: 0i32
    };
    #[doc(alias = "SAMPLER_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `SAMPLER_FILTER_LINEAR`"]
    pub const LINEAR: SamplerFilter = SamplerFilter {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for SamplerFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SamplerFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SamplerFilter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NEAREST => "SAMPLER_FILTER_NEAREST", Self::LINEAR => "SAMPLER_FILTER_LINEAR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for SamplerFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SamplerFilter {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SamplerFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SamplerRepeatMode {
    ord: i32
}
impl SamplerRepeatMode {
    #[doc(alias = "SAMPLER_REPEAT_MODE_REPEAT")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_REPEAT`"]
    pub const REPEAT: SamplerRepeatMode = SamplerRepeatMode {
        ord: 0i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_MIRRORED_REPEAT")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_MIRRORED_REPEAT`"]
    pub const MIRRORED_REPEAT: SamplerRepeatMode = SamplerRepeatMode {
        ord: 1i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_CLAMP_TO_EDGE")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_CLAMP_TO_EDGE`"]
    pub const CLAMP_TO_EDGE: SamplerRepeatMode = SamplerRepeatMode {
        ord: 2i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_CLAMP_TO_BORDER")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_CLAMP_TO_BORDER`"]
    pub const CLAMP_TO_BORDER: SamplerRepeatMode = SamplerRepeatMode {
        ord: 3i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_MIRROR_CLAMP_TO_EDGE")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_MIRROR_CLAMP_TO_EDGE`"]
    pub const MIRROR_CLAMP_TO_EDGE: SamplerRepeatMode = SamplerRepeatMode {
        ord: 4i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_MAX")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_MAX`"]
    pub const MAX: SamplerRepeatMode = SamplerRepeatMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for SamplerRepeatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SamplerRepeatMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SamplerRepeatMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::REPEAT => "REPEAT", Self::MIRRORED_REPEAT => "MIRRORED_REPEAT", Self::CLAMP_TO_EDGE => "CLAMP_TO_EDGE", Self::CLAMP_TO_BORDER => "CLAMP_TO_BORDER", Self::MIRROR_CLAMP_TO_EDGE => "MIRROR_CLAMP_TO_EDGE", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::REPEAT => "SAMPLER_REPEAT_MODE_REPEAT", Self::MIRRORED_REPEAT => "SAMPLER_REPEAT_MODE_MIRRORED_REPEAT", Self::CLAMP_TO_EDGE => "SAMPLER_REPEAT_MODE_CLAMP_TO_EDGE", Self::CLAMP_TO_BORDER => "SAMPLER_REPEAT_MODE_CLAMP_TO_BORDER", Self::MIRROR_CLAMP_TO_EDGE => "SAMPLER_REPEAT_MODE_MIRROR_CLAMP_TO_EDGE", Self::MAX => "SAMPLER_REPEAT_MODE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for SamplerRepeatMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for SamplerRepeatMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SamplerRepeatMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SamplerRepeatMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SamplerBorderColor {
    ord: i32
}
impl SamplerBorderColor {
    #[doc(alias = "SAMPLER_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK`"]
    pub const FLOAT_TRANSPARENT_BLACK: SamplerBorderColor = SamplerBorderColor {
        ord: 0i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_INT_TRANSPARENT_BLACK")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_INT_TRANSPARENT_BLACK`"]
    pub const INT_TRANSPARENT_BLACK: SamplerBorderColor = SamplerBorderColor {
        ord: 1i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_BLACK")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_BLACK`"]
    pub const FLOAT_OPAQUE_BLACK: SamplerBorderColor = SamplerBorderColor {
        ord: 2i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_INT_OPAQUE_BLACK")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_INT_OPAQUE_BLACK`"]
    pub const INT_OPAQUE_BLACK: SamplerBorderColor = SamplerBorderColor {
        ord: 3i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_WHITE")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_WHITE`"]
    pub const FLOAT_OPAQUE_WHITE: SamplerBorderColor = SamplerBorderColor {
        ord: 4i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_INT_OPAQUE_WHITE")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_INT_OPAQUE_WHITE`"]
    pub const INT_OPAQUE_WHITE: SamplerBorderColor = SamplerBorderColor {
        ord: 5i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_MAX")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_MAX`"]
    pub const MAX: SamplerBorderColor = SamplerBorderColor {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for SamplerBorderColor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SamplerBorderColor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SamplerBorderColor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::FLOAT_TRANSPARENT_BLACK => "FLOAT_TRANSPARENT_BLACK", Self::INT_TRANSPARENT_BLACK => "INT_TRANSPARENT_BLACK", Self::FLOAT_OPAQUE_BLACK => "FLOAT_OPAQUE_BLACK", Self::INT_OPAQUE_BLACK => "INT_OPAQUE_BLACK", Self::FLOAT_OPAQUE_WHITE => "FLOAT_OPAQUE_WHITE", Self::INT_OPAQUE_WHITE => "INT_OPAQUE_WHITE", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::FLOAT_TRANSPARENT_BLACK => "SAMPLER_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK", Self::INT_TRANSPARENT_BLACK => "SAMPLER_BORDER_COLOR_INT_TRANSPARENT_BLACK", Self::FLOAT_OPAQUE_BLACK => "SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_BLACK", Self::INT_OPAQUE_BLACK => "SAMPLER_BORDER_COLOR_INT_OPAQUE_BLACK", Self::FLOAT_OPAQUE_WHITE => "SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_WHITE", Self::INT_OPAQUE_WHITE => "SAMPLER_BORDER_COLOR_INT_OPAQUE_WHITE", Self::MAX => "SAMPLER_BORDER_COLOR_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for SamplerBorderColor {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for SamplerBorderColor {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SamplerBorderColor {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SamplerBorderColor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VertexFrequency {
    ord: i32
}
impl VertexFrequency {
    #[doc(alias = "VERTEX_FREQUENCY_VERTEX")]
    #[doc = "Godot enumerator name: `VERTEX_FREQUENCY_VERTEX`"]
    pub const VERTEX: VertexFrequency = VertexFrequency {
        ord: 0i32
    };
    #[doc(alias = "VERTEX_FREQUENCY_INSTANCE")]
    #[doc = "Godot enumerator name: `VERTEX_FREQUENCY_INSTANCE`"]
    pub const INSTANCE: VertexFrequency = VertexFrequency {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for VertexFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VertexFrequency") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VertexFrequency {
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
            Self::VERTEX => "VERTEX", Self::INSTANCE => "INSTANCE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::VERTEX => "VERTEX_FREQUENCY_VERTEX", Self::INSTANCE => "VERTEX_FREQUENCY_INSTANCE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for VertexFrequency {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VertexFrequency {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VertexFrequency {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct IndexBufferFormat {
    ord: i32
}
impl IndexBufferFormat {
    #[doc(alias = "INDEX_BUFFER_FORMAT_UINT16")]
    #[doc = "Godot enumerator name: `INDEX_BUFFER_FORMAT_UINT16`"]
    pub const UINT16: IndexBufferFormat = IndexBufferFormat {
        ord: 0i32
    };
    #[doc(alias = "INDEX_BUFFER_FORMAT_UINT32")]
    #[doc = "Godot enumerator name: `INDEX_BUFFER_FORMAT_UINT32`"]
    pub const UINT32: IndexBufferFormat = IndexBufferFormat {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for IndexBufferFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("IndexBufferFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for IndexBufferFormat {
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
            Self::UINT16 => "UINT16", Self::UINT32 => "UINT32", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UINT16 => "INDEX_BUFFER_FORMAT_UINT16", Self::UINT32 => "INDEX_BUFFER_FORMAT_UINT32", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for IndexBufferFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for IndexBufferFormat {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for IndexBufferFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct StorageBufferUsage {
    ord: u64
}
impl StorageBufferUsage {
    #[doc(alias = "STORAGE_BUFFER_USAGE_DISPATCH_INDIRECT")]
    #[doc = "Godot enumerator name: `STORAGE_BUFFER_USAGE_DISPATCH_INDIRECT`"]
    pub const DISPATCH_INDIRECT: StorageBufferUsage = StorageBufferUsage {
        ord: 1u64
    };
    
}
impl std::fmt::Debug for StorageBufferUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::DISPATCH_INDIRECT => "DISPATCH_INDIRECT", _ => {
                f.debug_struct("StorageBufferUsage") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for StorageBufferUsage {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for StorageBufferUsage {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for StorageBufferUsage {
    type Via = u64;
    
}
impl crate::meta::ToGodot for StorageBufferUsage {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StorageBufferUsage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct UniformType {
    ord: i32
}
impl UniformType {
    #[doc(alias = "UNIFORM_TYPE_SAMPLER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_SAMPLER`"]
    pub const SAMPLER: UniformType = UniformType {
        ord: 0i32
    };
    #[doc(alias = "UNIFORM_TYPE_SAMPLER_WITH_TEXTURE")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_SAMPLER_WITH_TEXTURE`"]
    pub const SAMPLER_WITH_TEXTURE: UniformType = UniformType {
        ord: 1i32
    };
    #[doc(alias = "UNIFORM_TYPE_TEXTURE")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_TEXTURE`"]
    pub const TEXTURE: UniformType = UniformType {
        ord: 2i32
    };
    #[doc(alias = "UNIFORM_TYPE_IMAGE")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_IMAGE`"]
    pub const IMAGE: UniformType = UniformType {
        ord: 3i32
    };
    #[doc(alias = "UNIFORM_TYPE_TEXTURE_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_TEXTURE_BUFFER`"]
    pub const TEXTURE_BUFFER: UniformType = UniformType {
        ord: 4i32
    };
    #[doc(alias = "UNIFORM_TYPE_SAMPLER_WITH_TEXTURE_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_SAMPLER_WITH_TEXTURE_BUFFER`"]
    pub const SAMPLER_WITH_TEXTURE_BUFFER: UniformType = UniformType {
        ord: 5i32
    };
    #[doc(alias = "UNIFORM_TYPE_IMAGE_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_IMAGE_BUFFER`"]
    pub const IMAGE_BUFFER: UniformType = UniformType {
        ord: 6i32
    };
    #[doc(alias = "UNIFORM_TYPE_UNIFORM_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_UNIFORM_BUFFER`"]
    pub const UNIFORM_BUFFER: UniformType = UniformType {
        ord: 7i32
    };
    #[doc(alias = "UNIFORM_TYPE_STORAGE_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_STORAGE_BUFFER`"]
    pub const STORAGE_BUFFER: UniformType = UniformType {
        ord: 8i32
    };
    #[doc(alias = "UNIFORM_TYPE_INPUT_ATTACHMENT")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_INPUT_ATTACHMENT`"]
    pub const INPUT_ATTACHMENT: UniformType = UniformType {
        ord: 9i32
    };
    #[doc(alias = "UNIFORM_TYPE_MAX")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_MAX`"]
    pub const MAX: UniformType = UniformType {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for UniformType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UniformType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UniformType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
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
            Self::SAMPLER => "SAMPLER", Self::SAMPLER_WITH_TEXTURE => "SAMPLER_WITH_TEXTURE", Self::TEXTURE => "TEXTURE", Self::IMAGE => "IMAGE", Self::TEXTURE_BUFFER => "TEXTURE_BUFFER", Self::SAMPLER_WITH_TEXTURE_BUFFER => "SAMPLER_WITH_TEXTURE_BUFFER", Self::IMAGE_BUFFER => "IMAGE_BUFFER", Self::UNIFORM_BUFFER => "UNIFORM_BUFFER", Self::STORAGE_BUFFER => "STORAGE_BUFFER", Self::INPUT_ATTACHMENT => "INPUT_ATTACHMENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::SAMPLER => "UNIFORM_TYPE_SAMPLER", Self::SAMPLER_WITH_TEXTURE => "UNIFORM_TYPE_SAMPLER_WITH_TEXTURE", Self::TEXTURE => "UNIFORM_TYPE_TEXTURE", Self::IMAGE => "UNIFORM_TYPE_IMAGE", Self::TEXTURE_BUFFER => "UNIFORM_TYPE_TEXTURE_BUFFER", Self::SAMPLER_WITH_TEXTURE_BUFFER => "UNIFORM_TYPE_SAMPLER_WITH_TEXTURE_BUFFER", Self::IMAGE_BUFFER => "UNIFORM_TYPE_IMAGE_BUFFER", Self::UNIFORM_BUFFER => "UNIFORM_TYPE_UNIFORM_BUFFER", Self::STORAGE_BUFFER => "UNIFORM_TYPE_STORAGE_BUFFER", Self::INPUT_ATTACHMENT => "UNIFORM_TYPE_INPUT_ATTACHMENT", Self::MAX => "UNIFORM_TYPE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for UniformType {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::meta::GodotConvert for UniformType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for UniformType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UniformType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderPrimitive {
    ord: i32
}
impl RenderPrimitive {
    #[doc(alias = "RENDER_PRIMITIVE_POINTS")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_POINTS`"]
    pub const POINTS: RenderPrimitive = RenderPrimitive {
        ord: 0i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_LINES")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_LINES`"]
    pub const LINES: RenderPrimitive = RenderPrimitive {
        ord: 1i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_LINES_WITH_ADJACENCY")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_LINES_WITH_ADJACENCY`"]
    pub const LINES_WITH_ADJACENCY: RenderPrimitive = RenderPrimitive {
        ord: 2i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_LINESTRIPS")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_LINESTRIPS`"]
    pub const LINESTRIPS: RenderPrimitive = RenderPrimitive {
        ord: 3i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_LINESTRIPS_WITH_ADJACENCY")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_LINESTRIPS_WITH_ADJACENCY`"]
    pub const LINESTRIPS_WITH_ADJACENCY: RenderPrimitive = RenderPrimitive {
        ord: 4i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLES")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLES`"]
    pub const TRIANGLES: RenderPrimitive = RenderPrimitive {
        ord: 5i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLES_WITH_ADJACENCY")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLES_WITH_ADJACENCY`"]
    pub const TRIANGLES_WITH_ADJACENCY: RenderPrimitive = RenderPrimitive {
        ord: 6i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLE_STRIPS")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLE_STRIPS`"]
    pub const TRIANGLE_STRIPS: RenderPrimitive = RenderPrimitive {
        ord: 7i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_AJACENCY")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_AJACENCY`"]
    pub const TRIANGLE_STRIPS_WITH_AJACENCY: RenderPrimitive = RenderPrimitive {
        ord: 8i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_RESTART_INDEX")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_RESTART_INDEX`"]
    pub const TRIANGLE_STRIPS_WITH_RESTART_INDEX: RenderPrimitive = RenderPrimitive {
        ord: 9i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TESSELATION_PATCH")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TESSELATION_PATCH`"]
    pub const TESSELATION_PATCH: RenderPrimitive = RenderPrimitive {
        ord: 10i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_MAX")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_MAX`"]
    pub const MAX: RenderPrimitive = RenderPrimitive {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for RenderPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderPrimitive") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderPrimitive {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
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
            Self::POINTS => "POINTS", Self::LINES => "LINES", Self::LINES_WITH_ADJACENCY => "LINES_WITH_ADJACENCY", Self::LINESTRIPS => "LINESTRIPS", Self::LINESTRIPS_WITH_ADJACENCY => "LINESTRIPS_WITH_ADJACENCY", Self::TRIANGLES => "TRIANGLES", Self::TRIANGLES_WITH_ADJACENCY => "TRIANGLES_WITH_ADJACENCY", Self::TRIANGLE_STRIPS => "TRIANGLE_STRIPS", Self::TRIANGLE_STRIPS_WITH_AJACENCY => "TRIANGLE_STRIPS_WITH_AJACENCY", Self::TRIANGLE_STRIPS_WITH_RESTART_INDEX => "TRIANGLE_STRIPS_WITH_RESTART_INDEX", Self::TESSELATION_PATCH => "TESSELATION_PATCH", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::POINTS => "RENDER_PRIMITIVE_POINTS", Self::LINES => "RENDER_PRIMITIVE_LINES", Self::LINES_WITH_ADJACENCY => "RENDER_PRIMITIVE_LINES_WITH_ADJACENCY", Self::LINESTRIPS => "RENDER_PRIMITIVE_LINESTRIPS", Self::LINESTRIPS_WITH_ADJACENCY => "RENDER_PRIMITIVE_LINESTRIPS_WITH_ADJACENCY", Self::TRIANGLES => "RENDER_PRIMITIVE_TRIANGLES", Self::TRIANGLES_WITH_ADJACENCY => "RENDER_PRIMITIVE_TRIANGLES_WITH_ADJACENCY", Self::TRIANGLE_STRIPS => "RENDER_PRIMITIVE_TRIANGLE_STRIPS", Self::TRIANGLE_STRIPS_WITH_AJACENCY => "RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_AJACENCY", Self::TRIANGLE_STRIPS_WITH_RESTART_INDEX => "RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_RESTART_INDEX", Self::TESSELATION_PATCH => "RENDER_PRIMITIVE_TESSELATION_PATCH", Self::MAX => "RENDER_PRIMITIVE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for RenderPrimitive {
    const ENUMERATOR_COUNT: usize = 11usize;
    
}
impl crate::meta::GodotConvert for RenderPrimitive {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RenderPrimitive {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderPrimitive {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolygonCullMode {
    ord: i32
}
impl PolygonCullMode {
    #[doc(alias = "POLYGON_CULL_DISABLED")]
    #[doc = "Godot enumerator name: `POLYGON_CULL_DISABLED`"]
    pub const DISABLED: PolygonCullMode = PolygonCullMode {
        ord: 0i32
    };
    #[doc(alias = "POLYGON_CULL_FRONT")]
    #[doc = "Godot enumerator name: `POLYGON_CULL_FRONT`"]
    pub const FRONT: PolygonCullMode = PolygonCullMode {
        ord: 1i32
    };
    #[doc(alias = "POLYGON_CULL_BACK")]
    #[doc = "Godot enumerator name: `POLYGON_CULL_BACK`"]
    pub const BACK: PolygonCullMode = PolygonCullMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PolygonCullMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolygonCullMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolygonCullMode {
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
            Self::DISABLED => "DISABLED", Self::FRONT => "FRONT", Self::BACK => "BACK", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "POLYGON_CULL_DISABLED", Self::FRONT => "POLYGON_CULL_FRONT", Self::BACK => "POLYGON_CULL_BACK", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PolygonCullMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PolygonCullMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolygonCullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolygonFrontFace {
    ord: i32
}
impl PolygonFrontFace {
    #[doc(alias = "POLYGON_FRONT_FACE_CLOCKWISE")]
    #[doc = "Godot enumerator name: `POLYGON_FRONT_FACE_CLOCKWISE`"]
    pub const CLOCKWISE: PolygonFrontFace = PolygonFrontFace {
        ord: 0i32
    };
    #[doc(alias = "POLYGON_FRONT_FACE_COUNTER_CLOCKWISE")]
    #[doc = "Godot enumerator name: `POLYGON_FRONT_FACE_COUNTER_CLOCKWISE`"]
    pub const COUNTER_CLOCKWISE: PolygonFrontFace = PolygonFrontFace {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for PolygonFrontFace {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolygonFrontFace") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolygonFrontFace {
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
            Self::CLOCKWISE => "CLOCKWISE", Self::COUNTER_CLOCKWISE => "COUNTER_CLOCKWISE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CLOCKWISE => "POLYGON_FRONT_FACE_CLOCKWISE", Self::COUNTER_CLOCKWISE => "POLYGON_FRONT_FACE_COUNTER_CLOCKWISE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PolygonFrontFace {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PolygonFrontFace {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolygonFrontFace {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StencilOperation {
    ord: i32
}
impl StencilOperation {
    #[doc(alias = "STENCIL_OP_KEEP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_KEEP`"]
    pub const KEEP: StencilOperation = StencilOperation {
        ord: 0i32
    };
    #[doc(alias = "STENCIL_OP_ZERO")]
    #[doc = "Godot enumerator name: `STENCIL_OP_ZERO`"]
    pub const ZERO: StencilOperation = StencilOperation {
        ord: 1i32
    };
    #[doc(alias = "STENCIL_OP_REPLACE")]
    #[doc = "Godot enumerator name: `STENCIL_OP_REPLACE`"]
    pub const REPLACE: StencilOperation = StencilOperation {
        ord: 2i32
    };
    #[doc(alias = "STENCIL_OP_INCREMENT_AND_CLAMP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_INCREMENT_AND_CLAMP`"]
    pub const INCREMENT_AND_CLAMP: StencilOperation = StencilOperation {
        ord: 3i32
    };
    #[doc(alias = "STENCIL_OP_DECREMENT_AND_CLAMP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_DECREMENT_AND_CLAMP`"]
    pub const DECREMENT_AND_CLAMP: StencilOperation = StencilOperation {
        ord: 4i32
    };
    #[doc(alias = "STENCIL_OP_INVERT")]
    #[doc = "Godot enumerator name: `STENCIL_OP_INVERT`"]
    pub const INVERT: StencilOperation = StencilOperation {
        ord: 5i32
    };
    #[doc(alias = "STENCIL_OP_INCREMENT_AND_WRAP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_INCREMENT_AND_WRAP`"]
    pub const INCREMENT_AND_WRAP: StencilOperation = StencilOperation {
        ord: 6i32
    };
    #[doc(alias = "STENCIL_OP_DECREMENT_AND_WRAP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_DECREMENT_AND_WRAP`"]
    pub const DECREMENT_AND_WRAP: StencilOperation = StencilOperation {
        ord: 7i32
    };
    #[doc(alias = "STENCIL_OP_MAX")]
    #[doc = "Godot enumerator name: `STENCIL_OP_MAX`"]
    pub const MAX: StencilOperation = StencilOperation {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for StencilOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StencilOperation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StencilOperation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::KEEP => "KEEP", Self::ZERO => "ZERO", Self::REPLACE => "REPLACE", Self::INCREMENT_AND_CLAMP => "INCREMENT_AND_CLAMP", Self::DECREMENT_AND_CLAMP => "DECREMENT_AND_CLAMP", Self::INVERT => "INVERT", Self::INCREMENT_AND_WRAP => "INCREMENT_AND_WRAP", Self::DECREMENT_AND_WRAP => "DECREMENT_AND_WRAP", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::KEEP => "STENCIL_OP_KEEP", Self::ZERO => "STENCIL_OP_ZERO", Self::REPLACE => "STENCIL_OP_REPLACE", Self::INCREMENT_AND_CLAMP => "STENCIL_OP_INCREMENT_AND_CLAMP", Self::DECREMENT_AND_CLAMP => "STENCIL_OP_DECREMENT_AND_CLAMP", Self::INVERT => "STENCIL_OP_INVERT", Self::INCREMENT_AND_WRAP => "STENCIL_OP_INCREMENT_AND_WRAP", Self::DECREMENT_AND_WRAP => "STENCIL_OP_DECREMENT_AND_WRAP", Self::MAX => "STENCIL_OP_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for StencilOperation {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for StencilOperation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StencilOperation {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StencilOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompareOperator {
    ord: i32
}
impl CompareOperator {
    #[doc(alias = "COMPARE_OP_NEVER")]
    #[doc = "Godot enumerator name: `COMPARE_OP_NEVER`"]
    pub const NEVER: CompareOperator = CompareOperator {
        ord: 0i32
    };
    #[doc(alias = "COMPARE_OP_LESS")]
    #[doc = "Godot enumerator name: `COMPARE_OP_LESS`"]
    pub const LESS: CompareOperator = CompareOperator {
        ord: 1i32
    };
    #[doc(alias = "COMPARE_OP_EQUAL")]
    #[doc = "Godot enumerator name: `COMPARE_OP_EQUAL`"]
    pub const EQUAL: CompareOperator = CompareOperator {
        ord: 2i32
    };
    #[doc(alias = "COMPARE_OP_LESS_OR_EQUAL")]
    #[doc = "Godot enumerator name: `COMPARE_OP_LESS_OR_EQUAL`"]
    pub const LESS_OR_EQUAL: CompareOperator = CompareOperator {
        ord: 3i32
    };
    #[doc(alias = "COMPARE_OP_GREATER")]
    #[doc = "Godot enumerator name: `COMPARE_OP_GREATER`"]
    pub const GREATER: CompareOperator = CompareOperator {
        ord: 4i32
    };
    #[doc(alias = "COMPARE_OP_NOT_EQUAL")]
    #[doc = "Godot enumerator name: `COMPARE_OP_NOT_EQUAL`"]
    pub const NOT_EQUAL: CompareOperator = CompareOperator {
        ord: 5i32
    };
    #[doc(alias = "COMPARE_OP_GREATER_OR_EQUAL")]
    #[doc = "Godot enumerator name: `COMPARE_OP_GREATER_OR_EQUAL`"]
    pub const GREATER_OR_EQUAL: CompareOperator = CompareOperator {
        ord: 6i32
    };
    #[doc(alias = "COMPARE_OP_ALWAYS")]
    #[doc = "Godot enumerator name: `COMPARE_OP_ALWAYS`"]
    pub const ALWAYS: CompareOperator = CompareOperator {
        ord: 7i32
    };
    #[doc(alias = "COMPARE_OP_MAX")]
    #[doc = "Godot enumerator name: `COMPARE_OP_MAX`"]
    pub const MAX: CompareOperator = CompareOperator {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for CompareOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompareOperator") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompareOperator {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::NEVER => "NEVER", Self::LESS => "LESS", Self::EQUAL => "EQUAL", Self::LESS_OR_EQUAL => "LESS_OR_EQUAL", Self::GREATER => "GREATER", Self::NOT_EQUAL => "NOT_EQUAL", Self::GREATER_OR_EQUAL => "GREATER_OR_EQUAL", Self::ALWAYS => "ALWAYS", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NEVER => "COMPARE_OP_NEVER", Self::LESS => "COMPARE_OP_LESS", Self::EQUAL => "COMPARE_OP_EQUAL", Self::LESS_OR_EQUAL => "COMPARE_OP_LESS_OR_EQUAL", Self::GREATER => "COMPARE_OP_GREATER", Self::NOT_EQUAL => "COMPARE_OP_NOT_EQUAL", Self::GREATER_OR_EQUAL => "COMPARE_OP_GREATER_OR_EQUAL", Self::ALWAYS => "COMPARE_OP_ALWAYS", Self::MAX => "COMPARE_OP_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for CompareOperator {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for CompareOperator {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompareOperator {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompareOperator {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LogicOperation {
    ord: i32
}
impl LogicOperation {
    #[doc(alias = "LOGIC_OP_CLEAR")]
    #[doc = "Godot enumerator name: `LOGIC_OP_CLEAR`"]
    pub const CLEAR: LogicOperation = LogicOperation {
        ord: 0i32
    };
    #[doc(alias = "LOGIC_OP_AND")]
    #[doc = "Godot enumerator name: `LOGIC_OP_AND`"]
    pub const AND: LogicOperation = LogicOperation {
        ord: 1i32
    };
    #[doc(alias = "LOGIC_OP_AND_REVERSE")]
    #[doc = "Godot enumerator name: `LOGIC_OP_AND_REVERSE`"]
    pub const AND_REVERSE: LogicOperation = LogicOperation {
        ord: 2i32
    };
    #[doc(alias = "LOGIC_OP_COPY")]
    #[doc = "Godot enumerator name: `LOGIC_OP_COPY`"]
    pub const COPY: LogicOperation = LogicOperation {
        ord: 3i32
    };
    #[doc(alias = "LOGIC_OP_AND_INVERTED")]
    #[doc = "Godot enumerator name: `LOGIC_OP_AND_INVERTED`"]
    pub const AND_INVERTED: LogicOperation = LogicOperation {
        ord: 4i32
    };
    #[doc(alias = "LOGIC_OP_NO_OP")]
    #[doc = "Godot enumerator name: `LOGIC_OP_NO_OP`"]
    pub const NO_OP: LogicOperation = LogicOperation {
        ord: 5i32
    };
    #[doc(alias = "LOGIC_OP_XOR")]
    #[doc = "Godot enumerator name: `LOGIC_OP_XOR`"]
    pub const XOR: LogicOperation = LogicOperation {
        ord: 6i32
    };
    #[doc(alias = "LOGIC_OP_OR")]
    #[doc = "Godot enumerator name: `LOGIC_OP_OR`"]
    pub const OR: LogicOperation = LogicOperation {
        ord: 7i32
    };
    #[doc(alias = "LOGIC_OP_NOR")]
    #[doc = "Godot enumerator name: `LOGIC_OP_NOR`"]
    pub const NOR: LogicOperation = LogicOperation {
        ord: 8i32
    };
    #[doc(alias = "LOGIC_OP_EQUIVALENT")]
    #[doc = "Godot enumerator name: `LOGIC_OP_EQUIVALENT`"]
    pub const EQUIVALENT: LogicOperation = LogicOperation {
        ord: 9i32
    };
    #[doc(alias = "LOGIC_OP_INVERT")]
    #[doc = "Godot enumerator name: `LOGIC_OP_INVERT`"]
    pub const INVERT: LogicOperation = LogicOperation {
        ord: 10i32
    };
    #[doc(alias = "LOGIC_OP_OR_REVERSE")]
    #[doc = "Godot enumerator name: `LOGIC_OP_OR_REVERSE`"]
    pub const OR_REVERSE: LogicOperation = LogicOperation {
        ord: 11i32
    };
    #[doc(alias = "LOGIC_OP_COPY_INVERTED")]
    #[doc = "Godot enumerator name: `LOGIC_OP_COPY_INVERTED`"]
    pub const COPY_INVERTED: LogicOperation = LogicOperation {
        ord: 12i32
    };
    #[doc(alias = "LOGIC_OP_OR_INVERTED")]
    #[doc = "Godot enumerator name: `LOGIC_OP_OR_INVERTED`"]
    pub const OR_INVERTED: LogicOperation = LogicOperation {
        ord: 13i32
    };
    #[doc(alias = "LOGIC_OP_NAND")]
    #[doc = "Godot enumerator name: `LOGIC_OP_NAND`"]
    pub const NAND: LogicOperation = LogicOperation {
        ord: 14i32
    };
    #[doc(alias = "LOGIC_OP_SET")]
    #[doc = "Godot enumerator name: `LOGIC_OP_SET`"]
    pub const SET: LogicOperation = LogicOperation {
        ord: 15i32
    };
    #[doc(alias = "LOGIC_OP_MAX")]
    #[doc = "Godot enumerator name: `LOGIC_OP_MAX`"]
    pub const MAX: LogicOperation = LogicOperation {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for LogicOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LogicOperation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LogicOperation {
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
            Self::CLEAR => "CLEAR", Self::AND => "AND", Self::AND_REVERSE => "AND_REVERSE", Self::COPY => "COPY", Self::AND_INVERTED => "AND_INVERTED", Self::NO_OP => "NO_OP", Self::XOR => "XOR", Self::OR => "OR", Self::NOR => "NOR", Self::EQUIVALENT => "EQUIVALENT", Self::INVERT => "INVERT", Self::OR_REVERSE => "OR_REVERSE", Self::COPY_INVERTED => "COPY_INVERTED", Self::OR_INVERTED => "OR_INVERTED", Self::NAND => "NAND", Self::SET => "SET", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CLEAR => "LOGIC_OP_CLEAR", Self::AND => "LOGIC_OP_AND", Self::AND_REVERSE => "LOGIC_OP_AND_REVERSE", Self::COPY => "LOGIC_OP_COPY", Self::AND_INVERTED => "LOGIC_OP_AND_INVERTED", Self::NO_OP => "LOGIC_OP_NO_OP", Self::XOR => "LOGIC_OP_XOR", Self::OR => "LOGIC_OP_OR", Self::NOR => "LOGIC_OP_NOR", Self::EQUIVALENT => "LOGIC_OP_EQUIVALENT", Self::INVERT => "LOGIC_OP_INVERT", Self::OR_REVERSE => "LOGIC_OP_OR_REVERSE", Self::COPY_INVERTED => "LOGIC_OP_COPY_INVERTED", Self::OR_INVERTED => "LOGIC_OP_OR_INVERTED", Self::NAND => "LOGIC_OP_NAND", Self::SET => "LOGIC_OP_SET", Self::MAX => "LOGIC_OP_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for LogicOperation {
    const ENUMERATOR_COUNT: usize = 16usize;
    
}
impl crate::meta::GodotConvert for LogicOperation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LogicOperation {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LogicOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendFactor {
    ord: i32
}
impl BlendFactor {
    #[doc(alias = "BLEND_FACTOR_ZERO")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ZERO`"]
    pub const ZERO: BlendFactor = BlendFactor {
        ord: 0i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE`"]
    pub const ONE: BlendFactor = BlendFactor {
        ord: 1i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC_COLOR`"]
    pub const SRC_COLOR: BlendFactor = BlendFactor {
        ord: 2i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_SRC_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_SRC_COLOR`"]
    pub const ONE_MINUS_SRC_COLOR: BlendFactor = BlendFactor {
        ord: 3i32
    };
    #[doc(alias = "BLEND_FACTOR_DST_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_DST_COLOR`"]
    pub const DST_COLOR: BlendFactor = BlendFactor {
        ord: 4i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_DST_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_DST_COLOR`"]
    pub const ONE_MINUS_DST_COLOR: BlendFactor = BlendFactor {
        ord: 5i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC_ALPHA`"]
    pub const SRC_ALPHA: BlendFactor = BlendFactor {
        ord: 6i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_SRC_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_SRC_ALPHA`"]
    pub const ONE_MINUS_SRC_ALPHA: BlendFactor = BlendFactor {
        ord: 7i32
    };
    #[doc(alias = "BLEND_FACTOR_DST_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_DST_ALPHA`"]
    pub const DST_ALPHA: BlendFactor = BlendFactor {
        ord: 8i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_DST_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_DST_ALPHA`"]
    pub const ONE_MINUS_DST_ALPHA: BlendFactor = BlendFactor {
        ord: 9i32
    };
    #[doc(alias = "BLEND_FACTOR_CONSTANT_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_CONSTANT_COLOR`"]
    pub const CONSTANT_COLOR: BlendFactor = BlendFactor {
        ord: 10i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR`"]
    pub const ONE_MINUS_CONSTANT_COLOR: BlendFactor = BlendFactor {
        ord: 11i32
    };
    #[doc(alias = "BLEND_FACTOR_CONSTANT_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_CONSTANT_ALPHA`"]
    pub const CONSTANT_ALPHA: BlendFactor = BlendFactor {
        ord: 12i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA`"]
    pub const ONE_MINUS_CONSTANT_ALPHA: BlendFactor = BlendFactor {
        ord: 13i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC_ALPHA_SATURATE")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC_ALPHA_SATURATE`"]
    pub const SRC_ALPHA_SATURATE: BlendFactor = BlendFactor {
        ord: 14i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC1_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC1_COLOR`"]
    pub const SRC1_COLOR: BlendFactor = BlendFactor {
        ord: 15i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_SRC1_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`"]
    pub const ONE_MINUS_SRC1_COLOR: BlendFactor = BlendFactor {
        ord: 16i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC1_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC1_ALPHA`"]
    pub const SRC1_ALPHA: BlendFactor = BlendFactor {
        ord: 17i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA`"]
    pub const ONE_MINUS_SRC1_ALPHA: BlendFactor = BlendFactor {
        ord: 18i32
    };
    #[doc(alias = "BLEND_FACTOR_MAX")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_MAX`"]
    pub const MAX: BlendFactor = BlendFactor {
        ord: 19i32
    };
    
}
impl std::fmt::Debug for BlendFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendFactor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendFactor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 => Some(Self {
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
            Self::ZERO => "ZERO", Self::ONE => "ONE", Self::SRC_COLOR => "SRC_COLOR", Self::ONE_MINUS_SRC_COLOR => "ONE_MINUS_SRC_COLOR", Self::DST_COLOR => "DST_COLOR", Self::ONE_MINUS_DST_COLOR => "ONE_MINUS_DST_COLOR", Self::SRC_ALPHA => "SRC_ALPHA", Self::ONE_MINUS_SRC_ALPHA => "ONE_MINUS_SRC_ALPHA", Self::DST_ALPHA => "DST_ALPHA", Self::ONE_MINUS_DST_ALPHA => "ONE_MINUS_DST_ALPHA", Self::CONSTANT_COLOR => "CONSTANT_COLOR", Self::ONE_MINUS_CONSTANT_COLOR => "ONE_MINUS_CONSTANT_COLOR", Self::CONSTANT_ALPHA => "CONSTANT_ALPHA", Self::ONE_MINUS_CONSTANT_ALPHA => "ONE_MINUS_CONSTANT_ALPHA", Self::SRC_ALPHA_SATURATE => "SRC_ALPHA_SATURATE", Self::SRC1_COLOR => "SRC1_COLOR", Self::ONE_MINUS_SRC1_COLOR => "ONE_MINUS_SRC1_COLOR", Self::SRC1_ALPHA => "SRC1_ALPHA", Self::ONE_MINUS_SRC1_ALPHA => "ONE_MINUS_SRC1_ALPHA", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ZERO => "BLEND_FACTOR_ZERO", Self::ONE => "BLEND_FACTOR_ONE", Self::SRC_COLOR => "BLEND_FACTOR_SRC_COLOR", Self::ONE_MINUS_SRC_COLOR => "BLEND_FACTOR_ONE_MINUS_SRC_COLOR", Self::DST_COLOR => "BLEND_FACTOR_DST_COLOR", Self::ONE_MINUS_DST_COLOR => "BLEND_FACTOR_ONE_MINUS_DST_COLOR", Self::SRC_ALPHA => "BLEND_FACTOR_SRC_ALPHA", Self::ONE_MINUS_SRC_ALPHA => "BLEND_FACTOR_ONE_MINUS_SRC_ALPHA", Self::DST_ALPHA => "BLEND_FACTOR_DST_ALPHA", Self::ONE_MINUS_DST_ALPHA => "BLEND_FACTOR_ONE_MINUS_DST_ALPHA", Self::CONSTANT_COLOR => "BLEND_FACTOR_CONSTANT_COLOR", Self::ONE_MINUS_CONSTANT_COLOR => "BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR", Self::CONSTANT_ALPHA => "BLEND_FACTOR_CONSTANT_ALPHA", Self::ONE_MINUS_CONSTANT_ALPHA => "BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA", Self::SRC_ALPHA_SATURATE => "BLEND_FACTOR_SRC_ALPHA_SATURATE", Self::SRC1_COLOR => "BLEND_FACTOR_SRC1_COLOR", Self::ONE_MINUS_SRC1_COLOR => "BLEND_FACTOR_ONE_MINUS_SRC1_COLOR", Self::SRC1_ALPHA => "BLEND_FACTOR_SRC1_ALPHA", Self::ONE_MINUS_SRC1_ALPHA => "BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA", Self::MAX => "BLEND_FACTOR_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for BlendFactor {
    const ENUMERATOR_COUNT: usize = 19usize;
    
}
impl crate::meta::GodotConvert for BlendFactor {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendFactor {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendFactor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendOperation {
    ord: i32
}
impl BlendOperation {
    #[doc(alias = "BLEND_OP_ADD")]
    #[doc = "Godot enumerator name: `BLEND_OP_ADD`"]
    pub const ADD: BlendOperation = BlendOperation {
        ord: 0i32
    };
    #[doc(alias = "BLEND_OP_SUBTRACT")]
    #[doc = "Godot enumerator name: `BLEND_OP_SUBTRACT`"]
    pub const SUBTRACT: BlendOperation = BlendOperation {
        ord: 1i32
    };
    #[doc(alias = "BLEND_OP_REVERSE_SUBTRACT")]
    #[doc = "Godot enumerator name: `BLEND_OP_REVERSE_SUBTRACT`"]
    pub const REVERSE_SUBTRACT: BlendOperation = BlendOperation {
        ord: 2i32
    };
    #[doc(alias = "BLEND_OP_MINIMUM")]
    #[doc = "Godot enumerator name: `BLEND_OP_MINIMUM`"]
    pub const MINIMUM: BlendOperation = BlendOperation {
        ord: 3i32
    };
    #[doc(alias = "BLEND_OP_MAXIMUM")]
    #[doc = "Godot enumerator name: `BLEND_OP_MAXIMUM`"]
    pub const MAXIMUM: BlendOperation = BlendOperation {
        ord: 4i32
    };
    #[doc(alias = "BLEND_OP_MAX")]
    #[doc = "Godot enumerator name: `BLEND_OP_MAX`"]
    pub const MAX: BlendOperation = BlendOperation {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for BlendOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendOperation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendOperation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::ADD => "ADD", Self::SUBTRACT => "SUBTRACT", Self::REVERSE_SUBTRACT => "REVERSE_SUBTRACT", Self::MINIMUM => "MINIMUM", Self::MAXIMUM => "MAXIMUM", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::ADD => "BLEND_OP_ADD", Self::SUBTRACT => "BLEND_OP_SUBTRACT", Self::REVERSE_SUBTRACT => "BLEND_OP_REVERSE_SUBTRACT", Self::MINIMUM => "BLEND_OP_MINIMUM", Self::MAXIMUM => "BLEND_OP_MAXIMUM", Self::MAX => "BLEND_OP_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for BlendOperation {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for BlendOperation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendOperation {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct PipelineDynamicStateFlags {
    ord: u64
}
impl PipelineDynamicStateFlags {
    #[doc(alias = "DYNAMIC_STATE_LINE_WIDTH")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_LINE_WIDTH`"]
    pub const LINE_WIDTH: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 1u64
    };
    #[doc(alias = "DYNAMIC_STATE_DEPTH_BIAS")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_DEPTH_BIAS`"]
    pub const DEPTH_BIAS: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 2u64
    };
    #[doc(alias = "DYNAMIC_STATE_BLEND_CONSTANTS")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_BLEND_CONSTANTS`"]
    pub const BLEND_CONSTANTS: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 4u64
    };
    #[doc(alias = "DYNAMIC_STATE_DEPTH_BOUNDS")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_DEPTH_BOUNDS`"]
    pub const DEPTH_BOUNDS: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 8u64
    };
    #[doc(alias = "DYNAMIC_STATE_STENCIL_COMPARE_MASK")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_STENCIL_COMPARE_MASK`"]
    pub const STENCIL_COMPARE_MASK: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 16u64
    };
    #[doc(alias = "DYNAMIC_STATE_STENCIL_WRITE_MASK")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_STENCIL_WRITE_MASK`"]
    pub const STENCIL_WRITE_MASK: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 32u64
    };
    #[doc(alias = "DYNAMIC_STATE_STENCIL_REFERENCE")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_STENCIL_REFERENCE`"]
    pub const STENCIL_REFERENCE: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 64u64
    };
    
}
impl std::fmt::Debug for PipelineDynamicStateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::LINE_WIDTH => "LINE_WIDTH", Self::DEPTH_BIAS => "DEPTH_BIAS", Self::BLEND_CONSTANTS => "BLEND_CONSTANTS", Self::DEPTH_BOUNDS => "DEPTH_BOUNDS", Self::STENCIL_COMPARE_MASK => "STENCIL_COMPARE_MASK", Self::STENCIL_WRITE_MASK => "STENCIL_WRITE_MASK", Self::STENCIL_REFERENCE => "STENCIL_REFERENCE", _ => {
                f.debug_struct("PipelineDynamicStateFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for PipelineDynamicStateFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for PipelineDynamicStateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::meta::GodotConvert for PipelineDynamicStateFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for PipelineDynamicStateFlags {
    type ToVia < 'v > = u64;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PipelineDynamicStateFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InitialAction {
    ord: i32
}
impl InitialAction {
    #[doc(alias = "INITIAL_ACTION_LOAD")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_LOAD`"]
    pub const LOAD: InitialAction = InitialAction {
        ord: 0i32
    };
    #[doc(alias = "INITIAL_ACTION_CLEAR")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_CLEAR`"]
    pub const CLEAR: InitialAction = InitialAction {
        ord: 1i32
    };
    #[doc(alias = "INITIAL_ACTION_DISCARD")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_DISCARD`"]
    pub const DISCARD: InitialAction = InitialAction {
        ord: 2i32
    };
    #[doc(alias = "INITIAL_ACTION_MAX")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_MAX`"]
    pub const MAX: InitialAction = InitialAction {
        ord: 3i32
    };
    #[doc(alias = "INITIAL_ACTION_CLEAR_REGION")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_CLEAR_REGION`"]
    pub const CLEAR_REGION: InitialAction = InitialAction {
        ord: 1i32
    };
    #[doc(alias = "INITIAL_ACTION_CLEAR_REGION_CONTINUE")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_CLEAR_REGION_CONTINUE`"]
    pub const CLEAR_REGION_CONTINUE: InitialAction = InitialAction {
        ord: 1i32
    };
    #[doc(alias = "INITIAL_ACTION_KEEP")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_KEEP`"]
    pub const KEEP: InitialAction = InitialAction {
        ord: 0i32
    };
    #[doc(alias = "INITIAL_ACTION_DROP")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_DROP`"]
    pub const DROP: InitialAction = InitialAction {
        ord: 2i32
    };
    #[doc(alias = "INITIAL_ACTION_CONTINUE")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_CONTINUE`"]
    pub const CONTINUE: InitialAction = InitialAction {
        ord: 0i32
    };
    
}
impl std::fmt::Debug for InitialAction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InitialAction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InitialAction {
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
            Self::LOAD => "LOAD", Self::CLEAR => "CLEAR", Self::DISCARD => "DISCARD", Self::MAX => "MAX", Self::CLEAR_REGION => "CLEAR_REGION", Self::CLEAR_REGION_CONTINUE => "CLEAR_REGION_CONTINUE", Self::KEEP => "KEEP", Self::DROP => "DROP", Self::CONTINUE => "CONTINUE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::LOAD => "INITIAL_ACTION_LOAD", Self::CLEAR => "INITIAL_ACTION_CLEAR", Self::DISCARD => "INITIAL_ACTION_DISCARD", Self::MAX => "INITIAL_ACTION_MAX", Self::CLEAR_REGION => "INITIAL_ACTION_CLEAR_REGION", Self::CLEAR_REGION_CONTINUE => "INITIAL_ACTION_CLEAR_REGION_CONTINUE", Self::KEEP => "INITIAL_ACTION_KEEP", Self::DROP => "INITIAL_ACTION_DROP", Self::CONTINUE => "INITIAL_ACTION_CONTINUE", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for InitialAction {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for InitialAction {
    type Via = i32;
    
}
impl crate::meta::ToGodot for InitialAction {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InitialAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FinalAction {
    ord: i32
}
impl FinalAction {
    #[doc(alias = "FINAL_ACTION_STORE")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_STORE`"]
    pub const STORE: FinalAction = FinalAction {
        ord: 0i32
    };
    #[doc(alias = "FINAL_ACTION_DISCARD")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_DISCARD`"]
    pub const DISCARD: FinalAction = FinalAction {
        ord: 1i32
    };
    #[doc(alias = "FINAL_ACTION_MAX")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_MAX`"]
    pub const MAX: FinalAction = FinalAction {
        ord: 2i32
    };
    #[doc(alias = "FINAL_ACTION_READ")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_READ`"]
    pub const READ: FinalAction = FinalAction {
        ord: 0i32
    };
    #[doc(alias = "FINAL_ACTION_CONTINUE")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_CONTINUE`"]
    pub const CONTINUE: FinalAction = FinalAction {
        ord: 0i32
    };
    
}
impl std::fmt::Debug for FinalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FinalAction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FinalAction {
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
            Self::STORE => "STORE", Self::DISCARD => "DISCARD", Self::MAX => "MAX", Self::READ => "READ", Self::CONTINUE => "CONTINUE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::STORE => "FINAL_ACTION_STORE", Self::DISCARD => "FINAL_ACTION_DISCARD", Self::MAX => "FINAL_ACTION_MAX", Self::READ => "FINAL_ACTION_READ", Self::CONTINUE => "FINAL_ACTION_CONTINUE", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for FinalAction {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for FinalAction {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FinalAction {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FinalAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShaderStage {
    ord: i32
}
impl ShaderStage {
    #[doc(alias = "SHADER_STAGE_VERTEX")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_VERTEX`"]
    pub const VERTEX: ShaderStage = ShaderStage {
        ord: 0i32
    };
    #[doc(alias = "SHADER_STAGE_FRAGMENT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_FRAGMENT`"]
    pub const FRAGMENT: ShaderStage = ShaderStage {
        ord: 1i32
    };
    #[doc(alias = "SHADER_STAGE_TESSELATION_CONTROL")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_TESSELATION_CONTROL`"]
    pub const TESSELATION_CONTROL: ShaderStage = ShaderStage {
        ord: 2i32
    };
    #[doc(alias = "SHADER_STAGE_TESSELATION_EVALUATION")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_TESSELATION_EVALUATION`"]
    pub const TESSELATION_EVALUATION: ShaderStage = ShaderStage {
        ord: 3i32
    };
    #[doc(alias = "SHADER_STAGE_COMPUTE")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_COMPUTE`"]
    pub const COMPUTE: ShaderStage = ShaderStage {
        ord: 4i32
    };
    #[doc(alias = "SHADER_STAGE_MAX")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_MAX`"]
    pub const MAX: ShaderStage = ShaderStage {
        ord: 5i32
    };
    #[doc(alias = "SHADER_STAGE_VERTEX_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_VERTEX_BIT`"]
    pub const VERTEX_BIT: ShaderStage = ShaderStage {
        ord: 1i32
    };
    #[doc(alias = "SHADER_STAGE_FRAGMENT_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_FRAGMENT_BIT`"]
    pub const FRAGMENT_BIT: ShaderStage = ShaderStage {
        ord: 2i32
    };
    #[doc(alias = "SHADER_STAGE_TESSELATION_CONTROL_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_TESSELATION_CONTROL_BIT`"]
    pub const TESSELATION_CONTROL_BIT: ShaderStage = ShaderStage {
        ord: 4i32
    };
    #[doc(alias = "SHADER_STAGE_TESSELATION_EVALUATION_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_TESSELATION_EVALUATION_BIT`"]
    pub const TESSELATION_EVALUATION_BIT: ShaderStage = ShaderStage {
        ord: 8i32
    };
    #[doc(alias = "SHADER_STAGE_COMPUTE_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_COMPUTE_BIT`"]
    pub const COMPUTE_BIT: ShaderStage = ShaderStage {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for ShaderStage {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShaderStage") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShaderStage {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 8i32 | ord @ 16i32 => Some(Self {
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
            Self::VERTEX => "VERTEX", Self::FRAGMENT => "FRAGMENT", Self::TESSELATION_CONTROL => "TESSELATION_CONTROL", Self::TESSELATION_EVALUATION => "TESSELATION_EVALUATION", Self::COMPUTE => "COMPUTE", Self::MAX => "MAX", Self::VERTEX_BIT => "VERTEX_BIT", Self::FRAGMENT_BIT => "FRAGMENT_BIT", Self::TESSELATION_CONTROL_BIT => "TESSELATION_CONTROL_BIT", Self::TESSELATION_EVALUATION_BIT => "TESSELATION_EVALUATION_BIT", Self::COMPUTE_BIT => "COMPUTE_BIT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::VERTEX => "SHADER_STAGE_VERTEX", Self::FRAGMENT => "SHADER_STAGE_FRAGMENT", Self::TESSELATION_CONTROL => "SHADER_STAGE_TESSELATION_CONTROL", Self::TESSELATION_EVALUATION => "SHADER_STAGE_TESSELATION_EVALUATION", Self::COMPUTE => "SHADER_STAGE_COMPUTE", Self::MAX => "SHADER_STAGE_MAX", Self::VERTEX_BIT => "SHADER_STAGE_VERTEX_BIT", Self::FRAGMENT_BIT => "SHADER_STAGE_FRAGMENT_BIT", Self::TESSELATION_CONTROL_BIT => "SHADER_STAGE_TESSELATION_CONTROL_BIT", Self::TESSELATION_EVALUATION_BIT => "SHADER_STAGE_TESSELATION_EVALUATION_BIT", Self::COMPUTE_BIT => "SHADER_STAGE_COMPUTE_BIT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ShaderStage {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShaderStage {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShaderStage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShaderLanguage {
    ord: i32
}
impl ShaderLanguage {
    #[doc(alias = "SHADER_LANGUAGE_GLSL")]
    #[doc = "Godot enumerator name: `SHADER_LANGUAGE_GLSL`"]
    pub const GLSL: ShaderLanguage = ShaderLanguage {
        ord: 0i32
    };
    #[doc(alias = "SHADER_LANGUAGE_HLSL")]
    #[doc = "Godot enumerator name: `SHADER_LANGUAGE_HLSL`"]
    pub const HLSL: ShaderLanguage = ShaderLanguage {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ShaderLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShaderLanguage") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShaderLanguage {
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
            Self::GLSL => "GLSL", Self::HLSL => "HLSL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::GLSL => "SHADER_LANGUAGE_GLSL", Self::HLSL => "SHADER_LANGUAGE_HLSL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ShaderLanguage {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShaderLanguage {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShaderLanguage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PipelineSpecializationConstantType {
    ord: i32
}
impl PipelineSpecializationConstantType {
    #[doc(alias = "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_BOOL")]
    #[doc = "Godot enumerator name: `PIPELINE_SPECIALIZATION_CONSTANT_TYPE_BOOL`"]
    pub const BOOL: PipelineSpecializationConstantType = PipelineSpecializationConstantType {
        ord: 0i32
    };
    #[doc(alias = "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_INT")]
    #[doc = "Godot enumerator name: `PIPELINE_SPECIALIZATION_CONSTANT_TYPE_INT`"]
    pub const INT: PipelineSpecializationConstantType = PipelineSpecializationConstantType {
        ord: 1i32
    };
    #[doc(alias = "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_FLOAT")]
    #[doc = "Godot enumerator name: `PIPELINE_SPECIALIZATION_CONSTANT_TYPE_FLOAT`"]
    pub const FLOAT: PipelineSpecializationConstantType = PipelineSpecializationConstantType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PipelineSpecializationConstantType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PipelineSpecializationConstantType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PipelineSpecializationConstantType {
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
            Self::BOOL => "BOOL", Self::INT => "INT", Self::FLOAT => "FLOAT", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::BOOL => "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_BOOL", Self::INT => "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_INT", Self::FLOAT => "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_FLOAT", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PipelineSpecializationConstantType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PipelineSpecializationConstantType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PipelineSpecializationConstantType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Limit {
    ord: i32
}
impl Limit {
    #[doc(alias = "LIMIT_MAX_BOUND_UNIFORM_SETS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_BOUND_UNIFORM_SETS`"]
    pub const MAX_BOUND_UNIFORM_SETS: Limit = Limit {
        ord: 0i32
    };
    #[doc(alias = "LIMIT_MAX_FRAMEBUFFER_COLOR_ATTACHMENTS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_FRAMEBUFFER_COLOR_ATTACHMENTS`"]
    pub const MAX_FRAMEBUFFER_COLOR_ATTACHMENTS: Limit = Limit {
        ord: 1i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURES_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURES_PER_UNIFORM_SET`"]
    pub const MAX_TEXTURES_PER_UNIFORM_SET: Limit = Limit {
        ord: 2i32
    };
    #[doc(alias = "LIMIT_MAX_SAMPLERS_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_SAMPLERS_PER_UNIFORM_SET`"]
    pub const MAX_SAMPLERS_PER_UNIFORM_SET: Limit = Limit {
        ord: 3i32
    };
    #[doc(alias = "LIMIT_MAX_STORAGE_BUFFERS_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_STORAGE_BUFFERS_PER_UNIFORM_SET`"]
    pub const MAX_STORAGE_BUFFERS_PER_UNIFORM_SET: Limit = Limit {
        ord: 4i32
    };
    #[doc(alias = "LIMIT_MAX_STORAGE_IMAGES_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_STORAGE_IMAGES_PER_UNIFORM_SET`"]
    pub const MAX_STORAGE_IMAGES_PER_UNIFORM_SET: Limit = Limit {
        ord: 5i32
    };
    #[doc(alias = "LIMIT_MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET`"]
    pub const MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET: Limit = Limit {
        ord: 6i32
    };
    #[doc(alias = "LIMIT_MAX_DRAW_INDEXED_INDEX")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_DRAW_INDEXED_INDEX`"]
    pub const MAX_DRAW_INDEXED_INDEX: Limit = Limit {
        ord: 7i32
    };
    #[doc(alias = "LIMIT_MAX_FRAMEBUFFER_HEIGHT")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_FRAMEBUFFER_HEIGHT`"]
    pub const MAX_FRAMEBUFFER_HEIGHT: Limit = Limit {
        ord: 8i32
    };
    #[doc(alias = "LIMIT_MAX_FRAMEBUFFER_WIDTH")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_FRAMEBUFFER_WIDTH`"]
    pub const MAX_FRAMEBUFFER_WIDTH: Limit = Limit {
        ord: 9i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_ARRAY_LAYERS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_ARRAY_LAYERS`"]
    pub const MAX_TEXTURE_ARRAY_LAYERS: Limit = Limit {
        ord: 10i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_SIZE_1D")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_SIZE_1D`"]
    pub const MAX_TEXTURE_SIZE_1D: Limit = Limit {
        ord: 11i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_SIZE_2D")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_SIZE_2D`"]
    pub const MAX_TEXTURE_SIZE_2D: Limit = Limit {
        ord: 12i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_SIZE_3D")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_SIZE_3D`"]
    pub const MAX_TEXTURE_SIZE_3D: Limit = Limit {
        ord: 13i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_SIZE_CUBE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_SIZE_CUBE`"]
    pub const MAX_TEXTURE_SIZE_CUBE: Limit = Limit {
        ord: 14i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURES_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURES_PER_SHADER_STAGE`"]
    pub const MAX_TEXTURES_PER_SHADER_STAGE: Limit = Limit {
        ord: 15i32
    };
    #[doc(alias = "LIMIT_MAX_SAMPLERS_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_SAMPLERS_PER_SHADER_STAGE`"]
    pub const MAX_SAMPLERS_PER_SHADER_STAGE: Limit = Limit {
        ord: 16i32
    };
    #[doc(alias = "LIMIT_MAX_STORAGE_BUFFERS_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_STORAGE_BUFFERS_PER_SHADER_STAGE`"]
    pub const MAX_STORAGE_BUFFERS_PER_SHADER_STAGE: Limit = Limit {
        ord: 17i32
    };
    #[doc(alias = "LIMIT_MAX_STORAGE_IMAGES_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_STORAGE_IMAGES_PER_SHADER_STAGE`"]
    pub const MAX_STORAGE_IMAGES_PER_SHADER_STAGE: Limit = Limit {
        ord: 18i32
    };
    #[doc(alias = "LIMIT_MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE`"]
    pub const MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE: Limit = Limit {
        ord: 19i32
    };
    #[doc(alias = "LIMIT_MAX_PUSH_CONSTANT_SIZE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_PUSH_CONSTANT_SIZE`"]
    pub const MAX_PUSH_CONSTANT_SIZE: Limit = Limit {
        ord: 20i32
    };
    #[doc(alias = "LIMIT_MAX_UNIFORM_BUFFER_SIZE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_UNIFORM_BUFFER_SIZE`"]
    pub const MAX_UNIFORM_BUFFER_SIZE: Limit = Limit {
        ord: 21i32
    };
    #[doc(alias = "LIMIT_MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET`"]
    pub const MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET: Limit = Limit {
        ord: 22i32
    };
    #[doc(alias = "LIMIT_MAX_VERTEX_INPUT_ATTRIBUTES")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VERTEX_INPUT_ATTRIBUTES`"]
    pub const MAX_VERTEX_INPUT_ATTRIBUTES: Limit = Limit {
        ord: 23i32
    };
    #[doc(alias = "LIMIT_MAX_VERTEX_INPUT_BINDINGS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VERTEX_INPUT_BINDINGS`"]
    pub const MAX_VERTEX_INPUT_BINDINGS: Limit = Limit {
        ord: 24i32
    };
    #[doc(alias = "LIMIT_MAX_VERTEX_INPUT_BINDING_STRIDE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VERTEX_INPUT_BINDING_STRIDE`"]
    pub const MAX_VERTEX_INPUT_BINDING_STRIDE: Limit = Limit {
        ord: 25i32
    };
    #[doc(alias = "LIMIT_MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT")]
    #[doc = "Godot enumerator name: `LIMIT_MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT`"]
    pub const MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT: Limit = Limit {
        ord: 26i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_SHARED_MEMORY_SIZE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_SHARED_MEMORY_SIZE`"]
    pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: Limit = Limit {
        ord: 27i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_X")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_X`"]
    pub const MAX_COMPUTE_WORKGROUP_COUNT_X: Limit = Limit {
        ord: 28i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Y")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Y`"]
    pub const MAX_COMPUTE_WORKGROUP_COUNT_Y: Limit = Limit {
        ord: 29i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Z")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Z`"]
    pub const MAX_COMPUTE_WORKGROUP_COUNT_Z: Limit = Limit {
        ord: 30i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_INVOCATIONS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_INVOCATIONS`"]
    pub const MAX_COMPUTE_WORKGROUP_INVOCATIONS: Limit = Limit {
        ord: 31i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_X")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_X`"]
    pub const MAX_COMPUTE_WORKGROUP_SIZE_X: Limit = Limit {
        ord: 32i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Y")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Y`"]
    pub const MAX_COMPUTE_WORKGROUP_SIZE_Y: Limit = Limit {
        ord: 33i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Z")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Z`"]
    pub const MAX_COMPUTE_WORKGROUP_SIZE_Z: Limit = Limit {
        ord: 34i32
    };
    #[doc(alias = "LIMIT_MAX_VIEWPORT_DIMENSIONS_X")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VIEWPORT_DIMENSIONS_X`"]
    pub const MAX_VIEWPORT_DIMENSIONS_X: Limit = Limit {
        ord: 35i32
    };
    #[doc(alias = "LIMIT_MAX_VIEWPORT_DIMENSIONS_Y")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VIEWPORT_DIMENSIONS_Y`"]
    pub const MAX_VIEWPORT_DIMENSIONS_Y: Limit = Limit {
        ord: 36i32
    };
    
}
impl std::fmt::Debug for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Limit") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Limit {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 => Some(Self {
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
            Self::MAX_BOUND_UNIFORM_SETS => "MAX_BOUND_UNIFORM_SETS", Self::MAX_FRAMEBUFFER_COLOR_ATTACHMENTS => "MAX_FRAMEBUFFER_COLOR_ATTACHMENTS", Self::MAX_TEXTURES_PER_UNIFORM_SET => "MAX_TEXTURES_PER_UNIFORM_SET", Self::MAX_SAMPLERS_PER_UNIFORM_SET => "MAX_SAMPLERS_PER_UNIFORM_SET", Self::MAX_STORAGE_BUFFERS_PER_UNIFORM_SET => "MAX_STORAGE_BUFFERS_PER_UNIFORM_SET", Self::MAX_STORAGE_IMAGES_PER_UNIFORM_SET => "MAX_STORAGE_IMAGES_PER_UNIFORM_SET", Self::MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET => "MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET", Self::MAX_DRAW_INDEXED_INDEX => "MAX_DRAW_INDEXED_INDEX", Self::MAX_FRAMEBUFFER_HEIGHT => "MAX_FRAMEBUFFER_HEIGHT", Self::MAX_FRAMEBUFFER_WIDTH => "MAX_FRAMEBUFFER_WIDTH", Self::MAX_TEXTURE_ARRAY_LAYERS => "MAX_TEXTURE_ARRAY_LAYERS", Self::MAX_TEXTURE_SIZE_1D => "MAX_TEXTURE_SIZE_1D", Self::MAX_TEXTURE_SIZE_2D => "MAX_TEXTURE_SIZE_2D", Self::MAX_TEXTURE_SIZE_3D => "MAX_TEXTURE_SIZE_3D", Self::MAX_TEXTURE_SIZE_CUBE => "MAX_TEXTURE_SIZE_CUBE", Self::MAX_TEXTURES_PER_SHADER_STAGE => "MAX_TEXTURES_PER_SHADER_STAGE", Self::MAX_SAMPLERS_PER_SHADER_STAGE => "MAX_SAMPLERS_PER_SHADER_STAGE", Self::MAX_STORAGE_BUFFERS_PER_SHADER_STAGE => "MAX_STORAGE_BUFFERS_PER_SHADER_STAGE", Self::MAX_STORAGE_IMAGES_PER_SHADER_STAGE => "MAX_STORAGE_IMAGES_PER_SHADER_STAGE", Self::MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE => "MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE", Self::MAX_PUSH_CONSTANT_SIZE => "MAX_PUSH_CONSTANT_SIZE", Self::MAX_UNIFORM_BUFFER_SIZE => "MAX_UNIFORM_BUFFER_SIZE", Self::MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET => "MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET", Self::MAX_VERTEX_INPUT_ATTRIBUTES => "MAX_VERTEX_INPUT_ATTRIBUTES", Self::MAX_VERTEX_INPUT_BINDINGS => "MAX_VERTEX_INPUT_BINDINGS", Self::MAX_VERTEX_INPUT_BINDING_STRIDE => "MAX_VERTEX_INPUT_BINDING_STRIDE", Self::MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT => "MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT", Self::MAX_COMPUTE_SHARED_MEMORY_SIZE => "MAX_COMPUTE_SHARED_MEMORY_SIZE", Self::MAX_COMPUTE_WORKGROUP_COUNT_X => "MAX_COMPUTE_WORKGROUP_COUNT_X", Self::MAX_COMPUTE_WORKGROUP_COUNT_Y => "MAX_COMPUTE_WORKGROUP_COUNT_Y", Self::MAX_COMPUTE_WORKGROUP_COUNT_Z => "MAX_COMPUTE_WORKGROUP_COUNT_Z", Self::MAX_COMPUTE_WORKGROUP_INVOCATIONS => "MAX_COMPUTE_WORKGROUP_INVOCATIONS", Self::MAX_COMPUTE_WORKGROUP_SIZE_X => "MAX_COMPUTE_WORKGROUP_SIZE_X", Self::MAX_COMPUTE_WORKGROUP_SIZE_Y => "MAX_COMPUTE_WORKGROUP_SIZE_Y", Self::MAX_COMPUTE_WORKGROUP_SIZE_Z => "MAX_COMPUTE_WORKGROUP_SIZE_Z", Self::MAX_VIEWPORT_DIMENSIONS_X => "MAX_VIEWPORT_DIMENSIONS_X", Self::MAX_VIEWPORT_DIMENSIONS_Y => "MAX_VIEWPORT_DIMENSIONS_Y", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::MAX_BOUND_UNIFORM_SETS => "LIMIT_MAX_BOUND_UNIFORM_SETS", Self::MAX_FRAMEBUFFER_COLOR_ATTACHMENTS => "LIMIT_MAX_FRAMEBUFFER_COLOR_ATTACHMENTS", Self::MAX_TEXTURES_PER_UNIFORM_SET => "LIMIT_MAX_TEXTURES_PER_UNIFORM_SET", Self::MAX_SAMPLERS_PER_UNIFORM_SET => "LIMIT_MAX_SAMPLERS_PER_UNIFORM_SET", Self::MAX_STORAGE_BUFFERS_PER_UNIFORM_SET => "LIMIT_MAX_STORAGE_BUFFERS_PER_UNIFORM_SET", Self::MAX_STORAGE_IMAGES_PER_UNIFORM_SET => "LIMIT_MAX_STORAGE_IMAGES_PER_UNIFORM_SET", Self::MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET => "LIMIT_MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET", Self::MAX_DRAW_INDEXED_INDEX => "LIMIT_MAX_DRAW_INDEXED_INDEX", Self::MAX_FRAMEBUFFER_HEIGHT => "LIMIT_MAX_FRAMEBUFFER_HEIGHT", Self::MAX_FRAMEBUFFER_WIDTH => "LIMIT_MAX_FRAMEBUFFER_WIDTH", Self::MAX_TEXTURE_ARRAY_LAYERS => "LIMIT_MAX_TEXTURE_ARRAY_LAYERS", Self::MAX_TEXTURE_SIZE_1D => "LIMIT_MAX_TEXTURE_SIZE_1D", Self::MAX_TEXTURE_SIZE_2D => "LIMIT_MAX_TEXTURE_SIZE_2D", Self::MAX_TEXTURE_SIZE_3D => "LIMIT_MAX_TEXTURE_SIZE_3D", Self::MAX_TEXTURE_SIZE_CUBE => "LIMIT_MAX_TEXTURE_SIZE_CUBE", Self::MAX_TEXTURES_PER_SHADER_STAGE => "LIMIT_MAX_TEXTURES_PER_SHADER_STAGE", Self::MAX_SAMPLERS_PER_SHADER_STAGE => "LIMIT_MAX_SAMPLERS_PER_SHADER_STAGE", Self::MAX_STORAGE_BUFFERS_PER_SHADER_STAGE => "LIMIT_MAX_STORAGE_BUFFERS_PER_SHADER_STAGE", Self::MAX_STORAGE_IMAGES_PER_SHADER_STAGE => "LIMIT_MAX_STORAGE_IMAGES_PER_SHADER_STAGE", Self::MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE => "LIMIT_MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE", Self::MAX_PUSH_CONSTANT_SIZE => "LIMIT_MAX_PUSH_CONSTANT_SIZE", Self::MAX_UNIFORM_BUFFER_SIZE => "LIMIT_MAX_UNIFORM_BUFFER_SIZE", Self::MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET => "LIMIT_MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET", Self::MAX_VERTEX_INPUT_ATTRIBUTES => "LIMIT_MAX_VERTEX_INPUT_ATTRIBUTES", Self::MAX_VERTEX_INPUT_BINDINGS => "LIMIT_MAX_VERTEX_INPUT_BINDINGS", Self::MAX_VERTEX_INPUT_BINDING_STRIDE => "LIMIT_MAX_VERTEX_INPUT_BINDING_STRIDE", Self::MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT => "LIMIT_MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT", Self::MAX_COMPUTE_SHARED_MEMORY_SIZE => "LIMIT_MAX_COMPUTE_SHARED_MEMORY_SIZE", Self::MAX_COMPUTE_WORKGROUP_COUNT_X => "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_X", Self::MAX_COMPUTE_WORKGROUP_COUNT_Y => "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Y", Self::MAX_COMPUTE_WORKGROUP_COUNT_Z => "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Z", Self::MAX_COMPUTE_WORKGROUP_INVOCATIONS => "LIMIT_MAX_COMPUTE_WORKGROUP_INVOCATIONS", Self::MAX_COMPUTE_WORKGROUP_SIZE_X => "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_X", Self::MAX_COMPUTE_WORKGROUP_SIZE_Y => "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Y", Self::MAX_COMPUTE_WORKGROUP_SIZE_Z => "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Z", Self::MAX_VIEWPORT_DIMENSIONS_X => "LIMIT_MAX_VIEWPORT_DIMENSIONS_X", Self::MAX_VIEWPORT_DIMENSIONS_Y => "LIMIT_MAX_VIEWPORT_DIMENSIONS_Y", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Limit {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Limit {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Limit {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MemoryType {
    ord: i32
}
impl MemoryType {
    #[doc(alias = "MEMORY_TEXTURES")]
    #[doc = "Godot enumerator name: `MEMORY_TEXTURES`"]
    pub const TEXTURES: MemoryType = MemoryType {
        ord: 0i32
    };
    #[doc(alias = "MEMORY_BUFFERS")]
    #[doc = "Godot enumerator name: `MEMORY_BUFFERS`"]
    pub const BUFFERS: MemoryType = MemoryType {
        ord: 1i32
    };
    #[doc(alias = "MEMORY_TOTAL")]
    #[doc = "Godot enumerator name: `MEMORY_TOTAL`"]
    pub const TOTAL: MemoryType = MemoryType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MemoryType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MemoryType {
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
            Self::TEXTURES => "TEXTURES", Self::BUFFERS => "BUFFERS", Self::TOTAL => "TOTAL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TEXTURES => "MEMORY_TEXTURES", Self::BUFFERS => "MEMORY_BUFFERS", Self::TOTAL => "MEMORY_TOTAL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for MemoryType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MemoryType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MemoryType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}