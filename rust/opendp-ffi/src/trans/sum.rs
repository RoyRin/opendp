use std::convert::TryFrom;
use std::iter::Sum;
use std::ops::Sub;
use std::os::raw::{c_char, c_uint};

use opendp::err;
use opendp::traits::{Abs, DistanceConstant, InfCast, SaturatingAdd, ExactIntCast, CheckedMul, CheckNull};
use opendp::trans::{make_bounded_sum, make_sized_bounded_sum};

use crate::any::{AnyTransformation, AnyObject, Downcast};
use crate::core::{FfiResult, IntoAnyTransformationFfiResultExt};
use crate::util::Type;
use num::Zero;
use opendp::dist::IntDistance;

#[no_mangle]
pub extern "C" fn opendp_trans__make_bounded_sum(
    bounds: *const AnyObject,
    T: *const c_char,
) -> FfiResult<*mut AnyTransformation> {
    fn monomorphize<T>(
        bounds: *const AnyObject,
    ) -> FfiResult<*mut AnyTransformation>
        where T: DistanceConstant<IntDistance> + Sub<Output=T> + Abs + SaturatingAdd + Zero + CheckNull,
              IntDistance: InfCast<T> {
        let bounds = try_!(try_as_ref!(bounds).downcast_ref::<(T, T)>()).clone();
        make_bounded_sum::<T>(bounds).into_any()
    }
    let T = try_!(Type::try_from(T));
    dispatch!(monomorphize, [
        (T, @numbers)
    ], (bounds))
}

#[no_mangle]
pub extern "C" fn opendp_trans__make_sized_bounded_sum(
    size: c_uint, bounds: *const AnyObject,
    T: *const c_char,
) -> FfiResult<*mut AnyTransformation> {
    fn monomorphize<T>(size: usize, bounds: *const AnyObject) -> FfiResult<*mut AnyTransformation>
        where T: DistanceConstant<IntDistance> + Sub<Output=T> + ExactIntCast<usize> + CheckedMul + CheckNull,
              for<'a> T: Sum<&'a T>,
              IntDistance: InfCast<T> {
        let bounds = try_!(try_as_ref!(bounds).downcast_ref::<(T, T)>()).clone();
        make_sized_bounded_sum::<T>(size, bounds).into_any()
    }
    let size = size as usize;
    let T = try_!(Type::try_from(T));
    dispatch!(monomorphize, [(T, @numbers)], (size, bounds))
}


#[cfg(test)]
mod tests {
    use opendp::error::Fallible;

    use crate::any::{AnyObject, Downcast};
    use crate::core;
    use crate::util;
    use crate::util::ToCharP;

    use super::*;

    #[test]
    fn test_make_bounded_sum() -> Fallible<()> {
        let transformation = Result::from(opendp_trans__make_bounded_sum(
            util::into_raw(AnyObject::new((0., 10.))),
            "f64".to_char_p(),
        ))?;
        let arg = AnyObject::new_raw(vec![1.0, 2.0, 3.0]);
        let res = core::opendp_core__transformation_invoke(&transformation, arg);
        let res: f64 = Fallible::from(res)?.downcast()?;
        assert_eq!(res, 6.0);
        Ok(())
    }

    #[test]
    fn test_make_bounded_sum_n() -> Fallible<()> {
        let transformation = Result::from(opendp_trans__make_sized_bounded_sum(
            3 as c_uint,
            util::into_raw(AnyObject::new((0., 10.))),
            "f64".to_char_p(),
        ))?;
        let arg = AnyObject::new_raw(vec![1.0, 2.0, 3.0]);
        let res = core::opendp_core__transformation_invoke(&transformation, arg);
        let res: f64 = Fallible::from(res)?.downcast()?;
        assert_eq!(res, 6.0);
        Ok(())
    }
}
