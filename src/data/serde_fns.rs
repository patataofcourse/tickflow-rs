use serde::{de::Visitor, Deserializer, Serializer};
use tickflow_binaries::data::{ArrayType, ValueType};

pub fn serialize_type_yaml<S: Serializer>(ty: &ValueType, ser: S) -> Result<S::Ok, S::Error> {
    let mut tmp_str;
    ser.collect_str(match ty {
        ValueType::Bool => "bool",
        ValueType::Signed => "int",
        ValueType::Unsigned => "uint",
        ValueType::String => "str",
        ValueType::Utf16 => "wstr",
        ValueType::Array{ depth, inner } =>  {
            tmp_str= match inner {
                ArrayType::Word => "u32",
                ArrayType::SignedWord => "i32",
                ArrayType::Byte => "u8",
                ArrayType::SignedByte => "i8",
                ArrayType::Half => "u16",
                ArrayType::SignedHalf => "i16",
                ArrayType::String => "str",
                ArrayType::Utf16 => "wstr",
                ArrayType::TfPointer => "sub",
                ArrayType::TfPointerSync => "sub_sync",
            }.to_string();
            tmp_str += &"[]".repeat(*depth as usize);
            &tmp_str
        },
        ValueType::TfPointer => "sub",
        ValueType::TfPointerSync => "sub_sync",
    })
}

pub fn deserialize_type_yaml<'de, D: Deserializer<'de>>(de: D) -> Result<ValueType, D::Error> {
    de.deserialize_str(V)
}

struct V;

impl V {
    pub fn get_val_type<E: serde::de::Error>(&self, ty: &str) -> Result<ValueType, E> {

        Ok(match ty {
            c if c.ends_with("[]") => {
                let mut depth = 0;
                let mut s = c;
                while let Some(new_s) = s.strip_suffix("[]") {
                    s = new_s;
                    depth += 1;
                }
                
                ValueType::Array {
                    depth,
                    inner: self.get_arr_inner_type(s)?,
                }
                
            }
            "bool" => ValueType::Bool,
            "int" => ValueType::Signed,
            "uint" => ValueType::Unsigned,
            "str" => ValueType::String,
            "wstr" | "ustr" => ValueType::Utf16,
            "sub" => ValueType::TfPointer,
            "sub_sync" => ValueType::TfPointerSync,
            c => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(c),
                &"a tickscript type",
            ))?,
        })
    }

    pub fn get_arr_inner_type<E: serde::de::Error>(
        &self,
        ty: &str
    ) -> Result<ArrayType, E> {
        Ok(match ty {
            "u32" => ArrayType::Word,
            "s32" | "i32" => ArrayType::SignedWord,
            "u8" => ArrayType::Byte,
            "i8" | "s8" => ArrayType::SignedByte,
            "u16" => ArrayType::Half,
            "i16" | "s16" => ArrayType::SignedHalf,
            "str" => ArrayType::String,
            "wstr" | "ustr" => ArrayType::Utf16,
            "sub" => ArrayType::TfPointer,
            "sub_sync" => ArrayType::TfPointerSync,
            c => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(c),
                &"a tickscript type",
            ))?,
        })
    }
}

impl<'de> Visitor<'de> for V {
    type Value = ValueType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.get_val_type(&v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.get_val_type(v)
    }
}
