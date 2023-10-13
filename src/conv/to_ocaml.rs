// Copyright (c) Viable Systems and TezEdge Contributors
// SPDX-License-Identifier: MIT

use core::{borrow::Borrow, str};

use crate::{
    memory::{
        alloc_bigarray1, alloc_bytes, alloc_cons, alloc_double, alloc_error, alloc_int32,
        alloc_int64, alloc_ok, alloc_some, alloc_string, alloc_tuple, store_raw_field_at, OCamlRef,
    },
    mlvalues::{
        bigarray::{Array1, BigarrayElt},
        OCamlBytes, OCamlFloat, OCamlInt, OCamlInt32, OCamlInt64, OCamlList, RawOCaml, FALSE, NONE,
        TRUE,
    },
    runtime::OCamlRuntime,
    value::OCaml,
    BoxRoot,
};

/// Implements conversion from Rust values into OCaml values.
pub unsafe trait IntoOCaml<T>: Sized {
    /// Convert to OCaml value. Return an already rooted value as [`BoxRoot`]`<T>`.
    fn to_boxroot(self, cr: &mut OCamlRuntime) -> BoxRoot<T> {
        BoxRoot::new(self.to_ocaml(cr))
    }

    /// Convert to OCaml value.
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, T>;
}

unsafe impl<'root, T> IntoOCaml<T> for OCamlRef<'root, T> {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, T> {
        unsafe { OCaml::new(cr, self.get_raw()) }
    }
}

unsafe impl<T> IntoOCaml<T> for BoxRoot<T> {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, T> {
        self.get(cr)
    }
}

unsafe impl IntoOCaml<()> for () {
    fn to_ocaml(self, _cr: &mut OCamlRuntime) -> OCaml<'static, ()> {
        OCaml::unit()
    }
}

unsafe impl IntoOCaml<OCamlInt> for i64 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlInt> {
        unsafe { OCaml::new(cr, ((self << 1) | 1i64) as RawOCaml) }
    }
}

unsafe impl IntoOCaml<OCamlInt> for i32 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlInt> {
        (self as i64).to_ocaml(cr)
    }
}

unsafe impl IntoOCaml<OCamlInt32> for i32 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlInt32> {
        alloc_int32(cr, self)
    }
}

unsafe impl IntoOCaml<OCamlInt64> for i64 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlInt64> {
        alloc_int64(cr, self)
    }
}

unsafe impl IntoOCaml<OCamlFloat> for f64 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlFloat> {
        alloc_double(cr, self)
    }
}

unsafe impl IntoOCaml<bool> for bool {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, bool> {
        unsafe { OCaml::new(cr, if self { TRUE } else { FALSE }) }
    }
}

unsafe impl IntoOCaml<OCamlInt> for &i64 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlInt> {
        unsafe { OCaml::new(cr, ((self << 1) | 1i64) as RawOCaml) }
    }
}

unsafe impl IntoOCaml<OCamlInt> for &i32 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlInt> {
        (*self as i64).to_ocaml(cr)
    }
}

unsafe impl IntoOCaml<OCamlInt32> for &i32 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlInt32> {
        alloc_int32(cr, *self)
    }
}

unsafe impl IntoOCaml<OCamlInt64> for &i64 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlInt64> {
        alloc_int64(cr, *self)
    }
}

unsafe impl IntoOCaml<OCamlFloat> for &f64 {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlFloat> {
        alloc_double(cr, *self)
    }
}

unsafe impl IntoOCaml<bool> for &bool {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, bool> {
        unsafe { OCaml::new(cr, if *self { TRUE } else { FALSE }) }
    }
}

// TODO: figure out how to implement all this without so much duplication
// it is not as simple as implementing for Borrow<str/[u8]> because
// of the Box<T> implementation bellow, which causes a trait implementation
// conflict.

unsafe impl IntoOCaml<String> for &str {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, String> {
        alloc_string(cr, self)
    }
}

unsafe impl IntoOCaml<OCamlBytes> for &str {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlBytes> {
        alloc_bytes(cr, self.as_bytes())
    }
}

unsafe impl IntoOCaml<OCamlBytes> for &[u8] {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlBytes> {
        alloc_bytes(cr, self)
    }
}

unsafe impl IntoOCaml<String> for &[u8] {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, String> {
        alloc_string(cr, unsafe { str::from_utf8_unchecked(self) })
    }
}

unsafe impl IntoOCaml<String> for String {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, String> {
        self.as_str().to_ocaml(cr)
    }
}

unsafe impl IntoOCaml<OCamlBytes> for String {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlBytes> {
        self.as_str().to_ocaml(cr)
    }
}

unsafe impl IntoOCaml<String> for &String {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, String> {
        self.as_str().to_ocaml(cr)
    }
}

unsafe impl IntoOCaml<OCamlBytes> for &String {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlBytes> {
        self.as_str().to_ocaml(cr)
    }
}

unsafe impl IntoOCaml<String> for Vec<u8> {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, String> {
        self.as_slice().to_ocaml(cr)
    }
}

unsafe impl IntoOCaml<OCamlBytes> for Vec<u8> {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlBytes> {
        self.as_slice().to_ocaml(cr)
    }
}

unsafe impl<A, OCamlA> IntoOCaml<OCamlA> for &Box<A>
where
    for<'a> &'a A: IntoOCaml<OCamlA>,
{
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlA> {
        self.as_ref().to_ocaml(cr)
    }
}

unsafe impl<A, OCamlA: 'static> IntoOCaml<Option<OCamlA>> for Option<A>
where
    A: IntoOCaml<OCamlA>,
{
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, Option<OCamlA>> {
        if let Some(value) = self {
            let ocaml_value = value.to_boxroot(cr);
            alloc_some(cr, &ocaml_value)
        } else {
            unsafe { OCaml::new(cr, NONE) }
        }
    }
}

unsafe impl<A, OCamlA: 'static> IntoOCaml<Option<OCamlA>> for &Option<A>
where
    for<'a> &'a A: IntoOCaml<OCamlA>,
{
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, Option<OCamlA>> {
        if let Some(value) = self {
            let ocaml_value = value.to_boxroot(cr);
            alloc_some(cr, &ocaml_value)
        } else {
            unsafe { OCaml::new(cr, NONE) }
        }
    }
}

unsafe impl<A, OCamlA: 'static, Err, OCamlErr: 'static> IntoOCaml<Result<OCamlA, OCamlErr>>
    for Result<A, Err>
where
    A: IntoOCaml<OCamlA>,
    Err: IntoOCaml<OCamlErr>,
{
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, Result<OCamlA, OCamlErr>> {
        match self {
            Ok(value) => {
                let ocaml_value = value.to_boxroot(cr);
                alloc_ok(cr, &ocaml_value)
            }
            Err(error) => {
                let ocaml_error = error.to_boxroot(cr);
                alloc_error(cr, &ocaml_error)
            }
        }
    }
}

unsafe impl<A, OCamlA: 'static, Err, OCamlErr: 'static> IntoOCaml<Result<OCamlA, OCamlErr>>
    for &Result<A, Err>
where
    for<'a> &'a A: IntoOCaml<OCamlA>,
    for<'a> &'a Err: IntoOCaml<OCamlErr>,
{
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, Result<OCamlA, OCamlErr>> {
        match self {
            Ok(value) => {
                let ocaml_value = value.to_boxroot(cr);
                alloc_ok(cr, &ocaml_value)
            }
            Err(error) => {
                let ocaml_error = error.to_boxroot(cr);
                alloc_error(cr, &ocaml_error)
            }
        }
    }
}

unsafe impl<A, OCamlA: 'static> IntoOCaml<OCamlList<OCamlA>> for &[A]
where
    for<'a> &'a A: IntoOCaml<OCamlA>,
{
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlList<OCamlA>> {
        let mut result = BoxRoot::new(OCaml::nil());
        for elt in self.iter().rev() {
            let ov = elt.to_boxroot(cr);
            let cons = alloc_cons(cr, &ov, &result);
            result.keep(cons);
        }
        cr.get(&result)
    }
}

unsafe impl<A, OCamlA: 'static> IntoOCaml<OCamlList<OCamlA>> for &Vec<A>
where
    for<'a> &'a A: IntoOCaml<OCamlA>,
{
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlList<OCamlA>> {
        let mut result = BoxRoot::new(OCaml::nil());
        for elt in self.iter().rev() {
            let ov = elt.to_boxroot(cr);
            let cons = alloc_cons(cr, &ov, &result);
            result.keep(cons);
        }
        cr.get(&result)
    }
}

unsafe impl<A, OCamlA: 'static> IntoOCaml<OCamlList<OCamlA>> for Vec<A>
where
    A: IntoOCaml<OCamlA>,
{
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlList<OCamlA>> {
        let mut result = BoxRoot::new(OCaml::nil());
        for elt in self.into_iter().rev() {
            let ov = elt.to_boxroot(cr);
            let cons = alloc_cons(cr, &ov, &result);
            result.keep(cons);
        }
        cr.get(&result)
    }
}

unsafe impl<'b, 'c, T, OCamlT: 'static> IntoOCaml<OCamlT> for &'b &'c T
where
    for<'a> &'a T: IntoOCaml<OCamlT>,
    T: ?Sized,
{
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, OCamlT> {
        (*self).to_ocaml(cr)
    }
}

// Tuples

macro_rules! tuple_to_ocaml {
    ($($n:tt: $t:ident => $ot:ident),+) => {
        unsafe impl<$($t),+, $($ot: 'static),+> IntoOCaml<($($ot),+)> for ($($t),+)
        where
            $($t: IntoOCaml<$ot>),+
        {
            fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, ($($ot),+)> {
                let len = $crate::count_fields!($($t)*);

                    let ocaml_tuple: BoxRoot<($($ot),+)> = BoxRoot::new(unsafe { alloc_tuple(cr, len) });
                    $(
                        unsafe {
                            let field_val = self.$n.to_ocaml(cr).get_raw();
                            store_raw_field_at(cr, &ocaml_tuple, $n, field_val);
                        }
                    )+

                    cr.get(&ocaml_tuple)
            }
        }

        unsafe impl<$($t),+, $($ot: 'static),+> IntoOCaml<($($ot),+)> for &($($t),+)
        where
            $(for<'a> &'a $t: IntoOCaml<$ot>),+
        {
            fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, ($($ot),+)> {
                let len = $crate::count_fields!($($t)*);

                    let ocaml_tuple: BoxRoot<($($ot),+)> = BoxRoot::new(unsafe { alloc_tuple(cr, len) });
                    $(
                        unsafe {
                            let field_val = self.$n.to_ocaml(cr).get_raw();
                            store_raw_field_at(cr, &ocaml_tuple, $n, field_val);
                        }
                    )+

                    cr.get(&ocaml_tuple)
            }
        }
    };
}

tuple_to_ocaml!(
    0: A => OCamlA,
    1: B => OCamlB);
tuple_to_ocaml!(
    0: A => OCamlA,
    1: B => OCamlB,
    2: C => OCamlC);
tuple_to_ocaml!(
    0: A => OCamlA,
    1: B => OCamlB,
    2: C => OCamlC,
    3: D => OCamlD);
tuple_to_ocaml!(
    0: A => OCamlA,
    1: B => OCamlB,
    2: C => OCamlC,
    3: D => OCamlD,
    4: E => OCamlE);
tuple_to_ocaml!(
    0: A => OCamlA,
    1: B => OCamlB,
    2: C => OCamlC,
    3: D => OCamlD,
    4: E => OCamlE,
    5: F => OCamlF);
tuple_to_ocaml!(
    0: A => OCamlA,
    1: B => OCamlB,
    2: C => OCamlC,
    3: D => OCamlD,
    4: E => OCamlE,
    5: F => OCamlF,
    6: G => OCamlG);
tuple_to_ocaml!(
    0: A => OCamlA,
    1: B => OCamlB,
    2: C => OCamlC,
    3: D => OCamlD,
    4: E => OCamlE,
    5: F => OCamlF,
    6: G => OCamlG,
    7: H => OCamlH);
tuple_to_ocaml!(
    0: A => OCamlA,
    1: B => OCamlB,
    2: C => OCamlC,
    3: D => OCamlD,
    4: E => OCamlE,
    5: F => OCamlF,
    6: G => OCamlG,
    7: H => OCamlH,
    8: I => OCamlI);
tuple_to_ocaml!(
    0: A => OCamlA,
    1: B => OCamlB,
    2: C => OCamlC,
    3: D => OCamlD,
    4: E => OCamlE,
    5: F => OCamlF,
    6: G => OCamlG,
    7: H => OCamlH,
    8: I => OCamlI,
    9: J => OCamlJ);

// This copies
unsafe impl<A: BigarrayElt> IntoOCaml<Array1<A>> for &[A] {
    fn to_ocaml<'a>(self, cr: &'a mut OCamlRuntime) -> OCaml<'a, Array1<A>> {
        alloc_bigarray1(cr, self)
    }
}

// Note: we deliberately don't implement FromOCaml<Array1<A>>,
// because this trait doesn't have a lifetime parameter
// and implementing would force a copy.
impl<'a, A: BigarrayElt> Borrow<[A]> for OCaml<'a, Array1<A>> {
    fn borrow(&self) -> &[A] {
        unsafe {
            let ba = self.custom_ptr_val::<ocaml_sys::bigarray::Bigarray>();
            core::slice::from_raw_parts((*ba).data as *const A, self.len())
        }
    }
}
