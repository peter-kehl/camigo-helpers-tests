use cami_helpers::{cami_partial_eq, Locality};

#[test]
fn main() {}

struct Empty {}

cami_partial_eq! {
    {Empty}
    (Locality::Both)
    []
    []
    []
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SimpleGeneric<T> {
    t: T,
}
/* */
cami_partial_eq! {
    <[T]>
    {SimpleGeneric <T>}
    where {T: PartialEq + Sized}
    (Locality::PureLocal)
    [t]
    []
    []
}
/* */

/*
impl < T> cami :: CamiPartialEq for SimpleGeneric < T>
where T: PartialEq
{
    const LOCALITY : Locality = Locality::PureLocal; fn
    eq_local(& self, other : & Self) -> bool
    {
        Self :: LOCALITY.debug_reachable_for_local(); let this = self; true &&
        {
            let getter = ::cami_helpers::always_equal_ref; getter(this) ==
            getter(other)
        } && this.t == other.t &&
        {
            let getter = ::cami_helpers::always_equal_ref;
            getter(this).eq_local(getter(other))
        }
    } fn eq_non_local(& self, other : & Self) -> bool
    {
        Self :: LOCALITY.debug_reachable_for_non_local(); let this = self;
        true &&
        {
            let getter = ::cami_helpers::always_equal_ref; getter(this) ==
            getter(other)
        } &&
        {
            let getter = ::cami_helpers::always_equal_ref;
            getter(this).eq_non_local(getter(other))
        }
    }
}
*/
