/// Defines an aggregated attribute type and its decoder and encoder.
#[macro_export]
macro_rules! define_attribute_enums {
    ($attr:ident, $decoder:ident, $encoder:ident,[$($variant:ident),*]) => {
        /// Attribute set.
        #[allow(missing_docs)]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $attr {
            $($variant($variant)),*
        }
        $(impl From<$variant> for $attr {
            fn from(f: $variant) -> Self {
                $attr::$variant(f)
            }
        })*
        impl $crate::Attribute for $attr {
            type Decoder = $decoder;
            type Encoder = $encoder;

            fn get_type(&self) -> $crate::AttributeType {
                match self {
                    $($attr::$variant(a) => a.get_type()),*
                }
            }

            fn before_encode<A>(&mut self, message: &$crate::Message<A>) -> ::bytecodec::Result<()>
            where
                A: $crate::Attribute,
            {
                match self {
                    $($attr::$variant(a) => track!(a.before_encode(message), "attr={}", stringify!($variant))),*
                }
            }

            fn after_decode<A>(&mut self, message: &$crate::Message<A>) -> ::bytecodec::Result<()>
            where
                A: $crate::Attribute,
            {
                match self {
                    $($attr::$variant(a) => track!(a.after_decode(message), "attr={}", stringify!($variant))),*
                }
            }
        }

        /// Attribute set decoder.
        #[allow(missing_docs)]
        #[derive(Debug)]
        pub enum $decoder {
            $($variant(<$variant as $crate::Attribute>::Decoder)),*,
            None,
        }
        impl $decoder {
            /// Makes a new decoder instance.
            pub fn new() -> Self {
                Self::default()
            }
        }
        impl Default for $decoder {
            fn default() -> Self {
                $decoder::None
            }
        }
        $(impl From<<$variant as $crate::Attribute>::Decoder> for $decoder {
            fn from(f: <$variant as $crate::Attribute>::Decoder) -> Self {
                $decoder::$variant(f)
            }
        })*
        impl ::bytecodec::Decode for $decoder {
            type Item = $attr;

            fn decode(&mut self, buf: &[u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
                match self {
                    $($decoder::$variant(a) => track!(a.decode(buf, eos), "attr={}", stringify!($variant))),*,
                    $decoder::None => track_panic!(::bytecodec::ErrorKind::InconsistentState),
                }
            }

            fn finish_decoding(&mut self) -> ::bytecodec::Result<Self::Item> {
                let item = match self {
                    $($decoder::$variant(a) => track!(a.finish_decoding(), "attr={}", stringify!($variant))?.into()),*,
                    $decoder::None => track_panic!(::bytecodec::ErrorKind::IncompleteDecoding),
                };
                *self = $decoder::None;
                Ok(item)
            }

            fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
                match self {
                    $($decoder::$variant(a) => a.requiring_bytes()),*,
                    $decoder::None => ::bytecodec::ByteCount::Finite(0),
                }
            }

            fn is_idle(&self) -> bool {
                match self {
                    $($decoder::$variant(a) => a.is_idle()),*,
                    $decoder::None => true,
                }
            }
        }
        impl ::bytecodec::TryTaggedDecode for $decoder {
            type Tag = $crate::AttributeType;

            fn try_start_decoding(&mut self, tag: Self::Tag) -> ::bytecodec::Result<bool> {
                *self = match tag.as_u16() {
                    $($variant::CODEPOINT => <$variant as $crate::Attribute>::Decoder::default().into()),*,
                    _ => return Ok(false),
                };
                Ok(true)
            }
        }

        /// Attribute set encoder.
        #[allow(missing_docs)]
        #[derive(Debug)]
        pub enum $encoder {
            $($variant(<$variant as $crate::Attribute>::Encoder)),*,
            None,
        }
        impl $encoder {
            /// Makes a new encoder instance.
            pub fn new() -> Self {
                Self::default()
            }
        }
        impl Default for $encoder {
            fn default() -> Self {
                $encoder::None
            }
        }
        $(impl From<<$variant as $crate::Attribute>::Encoder> for $encoder {
            fn from(f: <$variant as $crate::Attribute>::Encoder) -> Self {
                $encoder::$variant(f)
            }
        })*
        impl ::bytecodec::Encode for $encoder {
            type Item = $attr;

            fn encode(&mut self, buf: &mut [u8], eos: ::bytecodec::Eos) -> ::bytecodec::Result<usize> {
                match self {
                    $($encoder::$variant(a) => track!(a.encode(buf, eos), "attr={}", stringify!($variant))),*,
                    $encoder::None => Ok(0),
                }
            }

            fn start_encoding(&mut self, item: Self::Item) -> ::bytecodec::Result<()> {
                track_assert!(self.is_idle(), ::bytecodec::ErrorKind::EncoderFull; item);
                *self = match item {
                    $($attr::$variant(a) => {
                        let mut encoder = <$variant as $crate::Attribute>::Encoder::default();
                        track!(encoder.start_encoding(a), "attr={}", stringify!($variant))?;
                        encoder.into()
                    }),*
                };
                Ok(())
            }

            fn requiring_bytes(&self) -> ::bytecodec::ByteCount {
                use ::bytecodec::SizedEncode;
                ::bytecodec::ByteCount::Finite(self.exact_requiring_bytes())
            }

            fn is_idle(&self) -> bool {
                match self {
                    $($encoder::$variant(a) => a.is_idle()),*,
                    $encoder::None => true,
                }
            }
        }
        impl ::bytecodec::SizedEncode for $encoder {
            fn exact_requiring_bytes(&self) -> u64 {
                match self {
                    $($encoder::$variant(a) => a.exact_requiring_bytes()),*,
                    $encoder::None => 0,
                }
            }
        }
    };
}
