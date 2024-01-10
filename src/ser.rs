use crate::error::Error;
use crate::token::Token;
use crate::TestResult;
use serde::ser::{self, Serialize};

/// A `Serializer` that ensures that a value serializes to a given list of
/// tokens.
#[derive(Debug)]
pub struct Serializer<'a> {
    tokens: &'a [Token],
}

impl<'a> Serializer<'a> {
    /// Creates the serializer.
    pub fn new(tokens: &'a [Token]) -> Self {
        Serializer { tokens }
    }

    /// Pulls the next token off of the serializer, ignoring it.
    fn next_token(&mut self) -> Option<Token> {
        if let Some((&first, rest)) = self.tokens.split_first() {
            self.tokens = rest;
            Some(first)
        } else {
            None
        }
    }

    pub fn remaining(&self) -> usize {
        self.tokens.len()
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.first().copied()
    }
}

macro_rules! assert_next_token {
    ($ser:expr, $actual:ident) => {{
        assert_next_token!($ser, stringify!($actual), Token::$actual, true);
    }};
    ($ser:expr, $actual:ident($v:expr)) => {{
        assert_next_token!(
            $ser,
            format_args!(concat!(stringify!($actual), "({:?})"), $v),
            Token::$actual(v),
            v == $v
        );
    }};
    ($ser:expr, $actual:ident { $($k:ident),* }) => {{
        let compare = ($($k,)*);
        let field_format = || {
            use std::fmt::Write;
            let mut buffer = String::new();
            $(
                write!(&mut buffer, concat!(stringify!($k), ": {:?}, "), $k).unwrap();
            )*
            buffer
        };
        assert_next_token!(
            $ser,
            format_args!(concat!(stringify!($actual), " {{ {}}}"), field_format()),
            Token::$actual { $($k),* },
            ($($k,)*) == compare
        );
    }};
    ($ser:expr, $actual:expr) => {
        assert_next_token!($ser, $actual, expected, expected == $actual);
    };
    ($ser:expr, $actual:expr, $pat:pat, $guard:expr) => {
        match $ser.next_token() {
            Some($pat) if $guard => {}
            Some(expected) => return Err(Error::assert_failed(
                format_args!("expected Token::{} but serialized as {}", expected, $actual)
            )),
            None => return Err(Error::assert_failed(
                format_args!("expected end of tokens, but {} was serialized", $actual)
            )),
        }
    };
}

impl<'s, 'a: 's> ser::Serializer for &'s mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ComplexSerializer<'s, 'a>;
    type SerializeTuple = ComplexSerializer<'s, 'a>;
    type SerializeTupleStruct = ComplexSerializer<'s, 'a>;
    type SerializeTupleVariant = ComplexSerializer<'s, 'a>;
    type SerializeMap = ComplexSerializer<'s, 'a>;
    type SerializeStruct = ComplexSerializer<'s, 'a>;
    type SerializeStructVariant = ComplexSerializer<'s, 'a>;

    fn serialize_bool(self, v: bool) -> Result<(), Error> {
        assert_next_token!(self, Bool(v));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), Error> {
        assert_next_token!(self, I8(v));
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        assert_next_token!(self, I16(v));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        assert_next_token!(self, I32(v));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        assert_next_token!(self, I64(v));
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> TestResult {
        assert_next_token!(self, I128(v));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        assert_next_token!(self, U8(v));
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        assert_next_token!(self, U16(v));
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        assert_next_token!(self, U32(v));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        assert_next_token!(self, U64(v));
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<(), Error> {
        assert_next_token!(self, U128(v));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), Error> {
        assert_next_token!(self, F32(v));
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<(), Error> {
        assert_next_token!(self, F64(v));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<(), Error> {
        assert_next_token!(self, Char(v));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        match self.tokens.first() {
            Some(Token::BorrowedStr(_)) => assert_next_token!(self, BorrowedStr(v)),
            Some(Token::String(_)) => assert_next_token!(self, String(v)),
            _ => assert_next_token!(self, Str(v)),
        }
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Self::Error> {
        match self.tokens.first() {
            Some(Token::BorrowedBytes(_)) => assert_next_token!(self, BorrowedBytes(v)),
            Some(Token::ByteBuf(_)) => assert_next_token!(self, ByteBuf(v)),
            _ => assert_next_token!(self, Bytes(v)),
        }
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Error> {
        assert_next_token!(self, None);
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        assert_next_token!(self, Some);
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Error> {
        assert_next_token!(self, Unit);
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<(), Error> {
        assert_next_token!(self, UnitStruct { name });
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        if self.tokens.first() == Some(&Token::Enum { name }) {
            self.next_token();
            assert_next_token!(self, Str(variant));
            assert_next_token!(self, Unit);
        } else {
            assert_next_token!(self, UnitVariant { name, variant });
        }
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        assert_next_token!(self, NewtypeStruct { name });
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Error>
    where
        T: Serialize,
    {
        if self.tokens.first() == Some(&Token::Enum { name }) {
            self.next_token();
            assert_next_token!(self, Str(variant));
        } else {
            assert_next_token!(self, NewtypeVariant { name, variant });
        }
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> TestResult<ComplexSerializer<'s, 'a>> {
        assert_next_token!(self, Seq { len });

        Ok(ComplexSerializer {
            ser: self,
            end: Token::SeqEnd,
        })
    }

    fn serialize_tuple(self, len: usize) -> TestResult<ComplexSerializer<'s, 'a>> {
        assert_next_token!(self, Tuple { len });

        Ok(ComplexSerializer {
            ser: self,
            end: Token::TupleEnd,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> TestResult<ComplexSerializer<'s, 'a>> {
        assert_next_token!(self, TupleStruct { name, len });

        Ok(ComplexSerializer {
            ser: self,
            end: Token::TupleStructEnd,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> TestResult<ComplexSerializer<'s, 'a>> {
        if self.tokens.first() == Some(&Token::Enum { name }) {
            self.next_token();
            assert_next_token!(self, Str(variant));
            let len = Some(len);
            assert_next_token!(self, Seq { len });

            Ok(ComplexSerializer {
                ser: self,
                end: Token::SeqEnd,
            })
        } else {
            assert_next_token!(self, TupleVariant { name, variant, len });

            Ok(ComplexSerializer {
                ser: self,
                end: Token::TupleVariantEnd,
            })
        }
    }

    fn serialize_map(self, len: Option<usize>) -> TestResult<ComplexSerializer<'s, 'a>> {
        assert_next_token!(self, Map { len });

        Ok(ComplexSerializer {
            ser: self,
            end: Token::MapEnd,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> TestResult<ComplexSerializer<'s, 'a>> {
        assert_next_token!(self, Struct { name, len });

        Ok(ComplexSerializer {
            ser: self,
            end: Token::StructEnd,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> TestResult<ComplexSerializer<'s, 'a>> {
        if self.tokens.first() == Some(&Token::Enum { name }) {
            self.next_token();
            assert_next_token!(self, Str(variant));
            let len = Some(len);
            assert_next_token!(self, Map { len });

            Ok(ComplexSerializer {
                ser: self,
                end: Token::MapEnd,
            })
        } else {
            assert_next_token!(self, StructVariant { name, variant, len });

            Ok(ComplexSerializer {
                ser: self,
                end: Token::StructVariantEnd,
            })
        }
    }

    fn is_human_readable(&self) -> bool {
        panic!(
            "Types which have different human-readable and compact representations \
             must explicitly mark their test cases with `serde_test::Configure`"
        );
    }
}

pub struct ComplexSerializer<'s, 'a: 's> {
    ser: &'s mut Serializer<'a>,
    end: Token,
}

impl<'s, 'a> ComplexSerializer<'s, 'a> {
    fn end(self) -> TestResult {
        assert_next_token!(self.ser, self.end);
        Ok(())
    }
}

macro_rules! impl_complex_serialize {
    ($tr:ident: $($method:ident),+) => {
        impl<'s, 'a> ser::$tr for ComplexSerializer<'s, 'a> {
            type Ok = ();
            type Error = Error;

            $(
            fn $method<T: Serialize + ?Sized>(&mut self, value: &T) -> TestResult {
                value.serialize(&mut *self.ser)
            }
            )+

            fn end(self) -> TestResult {
                self.end()
            }
        }
    };

    (struct $tr:ident: $method:ident) => {
        impl<'s, 'a> ser::$tr for ComplexSerializer<'s, 'a> {
            type Ok = ();
            type Error = Error;

            fn $method<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> TestResult {
                key.serialize(&mut *self.ser)?;
                value.serialize(&mut *self.ser)
            }

            fn end(self) -> TestResult {
                self.end()
            }
        }
    };
}

impl_complex_serialize!(SerializeSeq: serialize_element);
impl_complex_serialize!(SerializeTuple: serialize_element);
impl_complex_serialize!(SerializeTupleStruct: serialize_field);
impl_complex_serialize!(SerializeTupleVariant: serialize_field);
impl_complex_serialize!(SerializeMap: serialize_key, serialize_value);
impl_complex_serialize!(struct SerializeStruct: serialize_field);
impl_complex_serialize!(struct SerializeStructVariant: serialize_field);
