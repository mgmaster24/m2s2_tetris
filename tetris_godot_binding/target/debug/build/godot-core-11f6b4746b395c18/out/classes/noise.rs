#![doc = "Sidecar module for class [`Noise`][crate::classes::Noise].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Noise` enums](https://docs.godotengine.org/en/stable/classes/class_noise.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Noise.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`noise`][crate::classes::noise]: sidecar module with related enum/flag types\n* [`INoise`][crate::classes::INoise]: virtual methods\n\n\nSee also [Godot docs for `Noise`](https://docs.godotengine.org/en/stable/classes/class_noise.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Noise>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Noise {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Noise`][crate::classes::Noise].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Noise` methods](https://docs.godotengine.org/en/stable/classes/class_noise.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INoise: crate::obj::GodotClass < Base = Noise > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Noise {
        pub fn get_noise_1d(&self, x: f32,) -> f32 {
            type CallSig = (f32, f32);
            let args = (x,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Noise", "get_noise_1d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_2d(&self, x: f32, y: f32,) -> f32 {
            type CallSig = (f32, f32, f32);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Noise", "get_noise_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_2dv(&self, v: Vector2,) -> f32 {
            type CallSig = (f32, Vector2);
            let args = (v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Noise", "get_noise_2dv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_3d(&self, x: f32, y: f32, z: f32,) -> f32 {
            type CallSig = (f32, f32, f32, f32);
            let args = (x, y, z,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Noise", "get_noise_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_noise_3dv(&self, v: Vector3,) -> f32 {
            type CallSig = (f32, Vector3);
            let args = (v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Noise", "get_noise_3dv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_image_full(&self, width: i32, height: i32, invert: bool, in_3d_space: bool, normalize: bool,) -> Option < Gd < crate::classes::Image > > {
            type CallSig = (Option < Gd < crate::classes::Image > >, i32, i32, bool, bool, bool);
            let args = (width, height, invert, in_3d_space, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Noise", "get_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_image(&self, width: i32, height: i32,) -> Option < Gd < crate::classes::Image > > {
            self.get_image_ex(width, height,) . done()
        }
        #[inline]
        pub fn get_image_ex < 'a > (&'a self, width: i32, height: i32,) -> ExGetImage < 'a > {
            ExGetImage::new(self, width, height,)
        }
        pub(crate) fn get_seamless_image_full(&self, width: i32, height: i32, invert: bool, in_3d_space: bool, skirt: f32, normalize: bool,) -> Option < Gd < crate::classes::Image > > {
            type CallSig = (Option < Gd < crate::classes::Image > >, i32, i32, bool, bool, f32, bool);
            let args = (width, height, invert, in_3d_space, skirt, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Noise", "get_seamless_image", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_seamless_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_seamless_image(&self, width: i32, height: i32,) -> Option < Gd < crate::classes::Image > > {
            self.get_seamless_image_ex(width, height,) . done()
        }
        #[inline]
        pub fn get_seamless_image_ex < 'a > (&'a self, width: i32, height: i32,) -> ExGetSeamlessImage < 'a > {
            ExGetSeamlessImage::new(self, width, height,)
        }
        pub(crate) fn get_image_3d_full(&self, width: i32, height: i32, depth: i32, invert: bool, normalize: bool,) -> Array < Gd < crate::classes::Image > > {
            type CallSig = (Array < Gd < crate::classes::Image > >, i32, i32, i32, bool, bool);
            let args = (width, height, depth, invert, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Noise", "get_image_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_image_3d_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_image_3d(&self, width: i32, height: i32, depth: i32,) -> Array < Gd < crate::classes::Image > > {
            self.get_image_3d_ex(width, height, depth,) . done()
        }
        #[inline]
        pub fn get_image_3d_ex < 'a > (&'a self, width: i32, height: i32, depth: i32,) -> ExGetImage3d < 'a > {
            ExGetImage3d::new(self, width, height, depth,)
        }
        pub(crate) fn get_seamless_image_3d_full(&self, width: i32, height: i32, depth: i32, invert: bool, skirt: f32, normalize: bool,) -> Array < Gd < crate::classes::Image > > {
            type CallSig = (Array < Gd < crate::classes::Image > >, i32, i32, i32, bool, f32, bool);
            let args = (width, height, depth, invert, skirt, normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Noise", "get_seamless_image_3d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_seamless_image_3d_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_seamless_image_3d(&self, width: i32, height: i32, depth: i32,) -> Array < Gd < crate::classes::Image > > {
            self.get_seamless_image_3d_ex(width, height, depth,) . done()
        }
        #[inline]
        pub fn get_seamless_image_3d_ex < 'a > (&'a self, width: i32, height: i32, depth: i32,) -> ExGetSeamlessImage3d < 'a > {
            ExGetSeamlessImage3d::new(self, width, height, depth,)
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
    impl crate::obj::GodotClass for Noise {
        type Base = crate::classes::Resource;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"Noise"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Noise {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Noise {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Noise {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Noise {
        
    }
    impl std::ops::Deref for Noise {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Noise {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Noise`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Noise {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Noise > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Noise::get_image_ex`][super::Noise::get_image_ex]."]
#[must_use]
pub struct ExGetImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Noise, width: i32, height: i32, invert: bool, in_3d_space: bool, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetImage < 'a > {
    fn new(surround_object: &'a re_export::Noise, width: i32, height: i32,) -> Self {
        let invert = false;
        let in_3d_space = false;
        let normalize = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, invert: invert, in_3d_space: in_3d_space, normalize: normalize,
        }
    }
    #[inline]
    pub fn invert(self, invert: bool) -> Self {
        Self {
            invert: invert, .. self
        }
    }
    #[inline]
    pub fn in_3d_space(self, in_3d_space: bool) -> Self {
        Self {
            in_3d_space: in_3d_space, .. self
        }
    }
    #[inline]
    pub fn normalize(self, normalize: bool) -> Self {
        Self {
            normalize: normalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, width, height, invert, in_3d_space, normalize,
        }
        = self;
        re_export::Noise::get_image_full(surround_object, width, height, invert, in_3d_space, normalize,)
    }
}
#[doc = "Default-param extender for [`Noise::get_seamless_image_ex`][super::Noise::get_seamless_image_ex]."]
#[must_use]
pub struct ExGetSeamlessImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Noise, width: i32, height: i32, invert: bool, in_3d_space: bool, skirt: f32, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSeamlessImage < 'a > {
    fn new(surround_object: &'a re_export::Noise, width: i32, height: i32,) -> Self {
        let invert = false;
        let in_3d_space = false;
        let skirt = 0.1f32;
        let normalize = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, invert: invert, in_3d_space: in_3d_space, skirt: skirt, normalize: normalize,
        }
    }
    #[inline]
    pub fn invert(self, invert: bool) -> Self {
        Self {
            invert: invert, .. self
        }
    }
    #[inline]
    pub fn in_3d_space(self, in_3d_space: bool) -> Self {
        Self {
            in_3d_space: in_3d_space, .. self
        }
    }
    #[inline]
    pub fn skirt(self, skirt: f32) -> Self {
        Self {
            skirt: skirt, .. self
        }
    }
    #[inline]
    pub fn normalize(self, normalize: bool) -> Self {
        Self {
            normalize: normalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, width, height, invert, in_3d_space, skirt, normalize,
        }
        = self;
        re_export::Noise::get_seamless_image_full(surround_object, width, height, invert, in_3d_space, skirt, normalize,)
    }
}
#[doc = "Default-param extender for [`Noise::get_image_3d_ex`][super::Noise::get_image_3d_ex]."]
#[must_use]
pub struct ExGetImage3d < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Noise, width: i32, height: i32, depth: i32, invert: bool, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetImage3d < 'a > {
    fn new(surround_object: &'a re_export::Noise, width: i32, height: i32, depth: i32,) -> Self {
        let invert = false;
        let normalize = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, depth: depth, invert: invert, normalize: normalize,
        }
    }
    #[inline]
    pub fn invert(self, invert: bool) -> Self {
        Self {
            invert: invert, .. self
        }
    }
    #[inline]
    pub fn normalize(self, normalize: bool) -> Self {
        Self {
            normalize: normalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, width, height, depth, invert, normalize,
        }
        = self;
        re_export::Noise::get_image_3d_full(surround_object, width, height, depth, invert, normalize,)
    }
}
#[doc = "Default-param extender for [`Noise::get_seamless_image_3d_ex`][super::Noise::get_seamless_image_3d_ex]."]
#[must_use]
pub struct ExGetSeamlessImage3d < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Noise, width: i32, height: i32, depth: i32, invert: bool, skirt: f32, normalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSeamlessImage3d < 'a > {
    fn new(surround_object: &'a re_export::Noise, width: i32, height: i32, depth: i32,) -> Self {
        let invert = false;
        let skirt = 0.1f32;
        let normalize = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, depth: depth, invert: invert, skirt: skirt, normalize: normalize,
        }
    }
    #[inline]
    pub fn invert(self, invert: bool) -> Self {
        Self {
            invert: invert, .. self
        }
    }
    #[inline]
    pub fn skirt(self, skirt: f32) -> Self {
        Self {
            skirt: skirt, .. self
        }
    }
    #[inline]
    pub fn normalize(self, normalize: bool) -> Self {
        Self {
            normalize: normalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Image > > {
        let Self {
            _phantom, surround_object, width, height, depth, invert, skirt, normalize,
        }
        = self;
        re_export::Noise::get_seamless_image_3d_full(surround_object, width, height, depth, invert, skirt, normalize,)
    }
}