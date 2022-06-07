pub(in crate::scalar) mod gen_impl;
pub(in crate::scalar) mod gen_trait_impl;
pub(in crate::scalar) mod gen_from_primitive;
pub(in crate::scalar) mod gen_from_scalar;
pub(in crate::scalar) mod gen_binop;
pub(in crate::scalar) mod gen_unop;
pub(in crate::scalar) mod gen_binopassign;
pub(in crate::scalar) mod gen_sum_iter;

pub(in crate::scalar) use gen_impl::gen_impl;
pub(in crate::scalar) use gen_trait_impl::gen_trait_impl;
pub(in crate::scalar) use gen_from_primitive::gen_from_primitive;
pub(in crate::scalar) use gen_from_scalar::gen_from_scalar;
pub(in crate::scalar) use gen_binop::gen_binop;
pub(in crate::scalar) use gen_unop::gen_unop;
pub(in crate::scalar) use gen_binopassign::gen_binopassign;
pub(in crate::scalar) use gen_sum_iter::gen_sum_iter;
