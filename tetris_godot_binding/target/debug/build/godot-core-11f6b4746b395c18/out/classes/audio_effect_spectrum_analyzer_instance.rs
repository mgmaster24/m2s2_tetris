#![doc = "Sidecar module for class [`AudioEffectSpectrumAnalyzerInstance`][crate::classes::AudioEffectSpectrumAnalyzerInstance].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzerInstance` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzerinstance.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectSpectrumAnalyzerInstance.`\n\nInherits [`AudioEffectInstance`][crate::classes::AudioEffectInstance].\n\nRelated symbols:\n\n* [`audio_effect_spectrum_analyzer_instance`][crate::classes::audio_effect_spectrum_analyzer_instance]: sidecar module with related enum/flag types\n* [`IAudioEffectSpectrumAnalyzerInstance`][crate::classes::IAudioEffectSpectrumAnalyzerInstance]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzerInstance`](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzerinstance.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<AudioEffectSpectrumAnalyzerInstance>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectSpectrumAnalyzerInstance {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioEffectSpectrumAnalyzerInstance`][crate::classes::AudioEffectSpectrumAnalyzerInstance].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzerInstance` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzerinstance.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectSpectrumAnalyzerInstance: crate::obj::GodotClass < Base = AudioEffectSpectrumAnalyzerInstance > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn process(&mut self, src_buffer: * const c_void, dst_buffer: * mut AudioFrame, frame_count: i32,);
        fn process_silence(&self,) -> bool {
            unimplemented !()
        }
    }
    impl AudioEffectSpectrumAnalyzerInstance {
        pub(crate) fn get_magnitude_for_frequency_range_full(&self, from_hz: f32, to_hz: f32, mode: crate::classes::audio_effect_spectrum_analyzer_instance::MagnitudeMode,) -> Vector2 {
            type CallSig = (Vector2, f32, f32, crate::classes::audio_effect_spectrum_analyzer_instance::MagnitudeMode);
            let args = (from_hz, to_hz, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(782usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "AudioEffectSpectrumAnalyzerInstance", "get_magnitude_for_frequency_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_magnitude_for_frequency_range_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_magnitude_for_frequency_range(&self, from_hz: f32, to_hz: f32,) -> Vector2 {
            self.get_magnitude_for_frequency_range_ex(from_hz, to_hz,) . done()
        }
        #[inline]
        pub fn get_magnitude_for_frequency_range_ex < 'a > (&'a self, from_hz: f32, to_hz: f32,) -> ExGetMagnitudeForFrequencyRange < 'a > {
            ExGetMagnitudeForFrequencyRange::new(self, from_hz, to_hz,)
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
    impl crate::obj::GodotClass for AudioEffectSpectrumAnalyzerInstance {
        type Base = crate::classes::AudioEffectInstance;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"AudioEffectSpectrumAnalyzerInstance"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectSpectrumAnalyzerInstance {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioEffectInstance > for AudioEffectSpectrumAnalyzerInstance {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioEffectSpectrumAnalyzerInstance {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioEffectSpectrumAnalyzerInstance {
        
    }
    impl std::ops::Deref for AudioEffectSpectrumAnalyzerInstance {
        type Target = crate::classes::AudioEffectInstance;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectSpectrumAnalyzerInstance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioEffectSpectrumAnalyzerInstance`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_AudioEffectSpectrumAnalyzerInstance {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffectSpectrumAnalyzerInstance > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffectInstance > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AudioEffectSpectrumAnalyzerInstance::get_magnitude_for_frequency_range_ex`][super::AudioEffectSpectrumAnalyzerInstance::get_magnitude_for_frequency_range_ex]."]
#[must_use]
pub struct ExGetMagnitudeForFrequencyRange < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::AudioEffectSpectrumAnalyzerInstance, from_hz: f32, to_hz: f32, mode: crate::classes::audio_effect_spectrum_analyzer_instance::MagnitudeMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMagnitudeForFrequencyRange < 'a > {
    fn new(surround_object: &'a re_export::AudioEffectSpectrumAnalyzerInstance, from_hz: f32, to_hz: f32,) -> Self {
        let mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_hz: from_hz, to_hz: to_hz, mode: mode,
        }
    }
    #[inline]
    pub fn mode(self, mode: crate::classes::audio_effect_spectrum_analyzer_instance::MagnitudeMode) -> Self {
        Self {
            mode: mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, from_hz, to_hz, mode,
        }
        = self;
        re_export::AudioEffectSpectrumAnalyzerInstance::get_magnitude_for_frequency_range_full(surround_object, from_hz, to_hz, mode,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MagnitudeMode {
    ord: i32
}
impl MagnitudeMode {
    #[doc(alias = "MAGNITUDE_AVERAGE")]
    #[doc = "Godot enumerator name: `MAGNITUDE_AVERAGE`"]
    pub const AVERAGE: MagnitudeMode = MagnitudeMode {
        ord: 0i32
    };
    #[doc(alias = "MAGNITUDE_MAX")]
    #[doc = "Godot enumerator name: `MAGNITUDE_MAX`"]
    pub const MAX: MagnitudeMode = MagnitudeMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MagnitudeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MagnitudeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MagnitudeMode {
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
            Self::AVERAGE => "AVERAGE", Self::MAX => "MAX", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::AVERAGE => "MAGNITUDE_AVERAGE", Self::MAX => "MAGNITUDE_MAX", _ => self.as_str(),
        }
    }
}
impl crate::obj::IndexEnum for MagnitudeMode {
    const ENUMERATOR_COUNT: usize = 1usize;
    
}
impl crate::meta::GodotConvert for MagnitudeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MagnitudeMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MagnitudeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}