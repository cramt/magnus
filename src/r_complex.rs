use std::{fmt, ops::Deref};

use rb_sys::{
    rb_complex_abs, rb_complex_arg, rb_complex_conjugate, rb_complex_imag, rb_complex_new,
    rb_complex_new_polar, rb_complex_real, ruby_value_type, VALUE,
};

use crate::{
    error::{protect, Error},
    exception,
    float::Float,
    into_value::IntoValue,
    numeric::Numeric,
    ruby_handle::RubyHandle,
    try_convert::TryConvert,
    value::{private, NonZeroValue, ReprValue, Value},
};

/// A Value pointer to a RComplex struct, Ruby's internal representation of
/// complex numbers.
///
/// All [`Value`] methods should be available on this type through [`Deref`],
/// but some may be missed by this documentation.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RComplex(NonZeroValue);

impl RComplex {
    /// Return `Some(RComplex)` if `val` is a `RComplex`, `None` otherwise.
    #[inline]
    pub fn from_value(val: Value) -> Option<Self> {
        unsafe {
            (val.rb_type() == ruby_value_type::RUBY_T_COMPLEX)
                .then(|| Self(NonZeroValue::new_unchecked(val)))
        }
    }

    #[inline]
    pub(crate) unsafe fn from_rb_value_unchecked(val: VALUE) -> Self {
        Self(NonZeroValue::new_unchecked(Value::new(val)))
    }

    /// Create a new `RComplex`.
    ///
    /// # Examples
    ///
    /// ```
    /// use magnus::{Integer, RComplex};
    /// # let _cleanup = unsafe { magnus::embed::init() };
    ///
    /// let complex = RComplex::new(Integer::from_i64(2), Integer::from_i64(1));
    /// assert_eq!(complex.to_string(), "2+1i");
    /// ```
    pub fn new<T, U>(real: T, imag: U) -> RComplex
    where
        T: Numeric,
        U: Numeric,
    {
        unsafe {
            RComplex::from_rb_value_unchecked(rb_complex_new(
                real.as_rb_value(),
                imag.as_rb_value(),
            ))
        }
    }

    /// Create a new `RComplex` using polar representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use magnus::{Integer, RComplex};
    /// # let _cleanup = unsafe { magnus::embed::init() };
    ///
    /// let complex = RComplex::polar(Integer::from_i64(2), Integer::from_i64(3)).unwrap();
    /// assert_eq!(complex.to_string(), "-1.9799849932008908+0.2822400161197344i");
    /// ```
    pub fn polar<T, U>(real: T, imag: U) -> Result<RComplex, Error>
    where
        T: Numeric,
        U: Numeric,
    {
        protect(|| unsafe {
            RComplex::from_rb_value_unchecked(rb_complex_new_polar(
                real.as_rb_value(),
                imag.as_rb_value(),
            ))
        })
    }

    /// Returns the real part of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use magnus::{Integer, RComplex};
    /// # let _cleanup = unsafe { magnus::embed::init() };
    ///
    /// let complex = RComplex::new(Integer::from_i64(9), Integer::from_i64(-4));
    /// assert_eq!(complex.real::<i64>().unwrap(), 9);
    /// ```
    pub fn real<T>(self) -> Result<T, Error>
    where
        T: TryConvert,
    {
        let val = unsafe { Value::new(rb_complex_real(self.as_rb_value())) };
        val.try_convert()
    }

    /// Returns the imaginary part of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use magnus::{Integer, RComplex};
    /// # let _cleanup = unsafe { magnus::embed::init() };
    ///
    /// let complex = RComplex::new(Integer::from_i64(9), Integer::from_i64(-4));
    /// assert_eq!(complex.imag::<i64>().unwrap(), -4);
    /// ```
    pub fn imag<T>(self) -> Result<T, Error>
    where
        T: TryConvert,
    {
        let val = unsafe { Value::new(rb_complex_imag(self.as_rb_value())) };
        val.try_convert()
    }

    /// Returns the complex conjugate.
    ///
    /// # Examples
    ///
    /// ```
    /// use magnus::{Integer, RComplex};
    /// # let _cleanup = unsafe { magnus::embed::init() };
    ///
    /// let complex = RComplex::new(Integer::from_i64(1), Integer::from_i64(2));
    /// assert_eq!(complex.conjugate().to_string(), "1-2i");
    /// ```
    pub fn conjugate(self) -> Self {
        unsafe { Self::from_rb_value_unchecked(rb_complex_conjugate(self.as_rb_value())) }
    }

    /// Returns the absolute (or the magnitude) of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use magnus::{Integer, RComplex};
    /// # let _cleanup = unsafe { magnus::embed::init() };
    ///
    /// let complex = RComplex::new(Integer::from_i64(3), Integer::from_i64(-4));
    /// assert_eq!(complex.abs(), 5.0);
    /// ```
    pub fn abs(self) -> f64 {
        unsafe { Float::from_rb_value_unchecked(rb_complex_abs(self.as_rb_value())).to_f64() }
    }

    /// Returns the argument (or the angle) of the polar form of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::PI;
    /// use magnus::{Integer, Float, RComplex};
    /// # let _cleanup = unsafe { magnus::embed::init() };
    ///
    /// let complex = RComplex::polar(Integer::from_i64(3), Float::from_f64(PI / 2.0)).unwrap();
    /// assert_eq!(complex.arg(), 1.5707963267948966);
    /// ```
    pub fn arg(self) -> f64 {
        unsafe { Float::from_rb_value_unchecked(rb_complex_arg(self.as_rb_value())).to_f64() }
    }
}

impl Deref for RComplex {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        self.0.get_ref()
    }
}

impl fmt::Display for RComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", unsafe { self.to_s_infallible() })
    }
}

impl fmt::Debug for RComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inspect())
    }
}

impl IntoValue for RComplex {
    fn into_value_with(self, _: &RubyHandle) -> Value {
        *self
    }
}

impl From<RComplex> for Value {
    fn from(val: RComplex) -> Self {
        *val
    }
}

unsafe impl private::ReprValue for RComplex {
    fn to_value(self) -> Value {
        *self
    }

    unsafe fn from_value_unchecked(val: Value) -> Self {
        Self(NonZeroValue::new_unchecked(val))
    }
}

impl Numeric for RComplex {}

impl ReprValue for RComplex {}

impl TryConvert for RComplex {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Self::from_value(val).ok_or_else(|| {
            Error::new(
                exception::type_error(),
                format!("no implicit conversion of {} into Complex", unsafe {
                    val.classname()
                },),
            )
        })
    }
}
