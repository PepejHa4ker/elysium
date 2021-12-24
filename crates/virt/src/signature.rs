pub trait Signature {
    type Args;
    type Return;
}

impl<A, Return> Signature for extern "C" fn(A) -> Return {
    type Args = (A,);
    type Return = Return;
}

impl<A, B, Return> Signature for extern "C" fn(A, B) -> Return {
    type Args = (A, B);
    type Return = Return;
}

impl<A, B, C, Return> Signature for extern "C" fn(A, B, C) -> Return {
    type Args = (A, B, C);
    type Return = Return;
}

impl<A, B, C, D, Return> Signature for extern "C" fn(A, B, C, D) -> Return {
    type Args = (A, B, C, D);
    type Return = Return;
}

impl<A, B, C, D, E, Return> Signature for extern "C" fn(A, B, C, D, E) -> Return {
    type Args = (A, B, C, D, E);
    type Return = Return;
}

impl<A, B, C, D, E, F, Return> Signature for extern "C" fn(A, B, C, D, E, F) -> Return {
    type Args = (A, B, C, D, E, F);
    type Return = Return;
}

impl<A, Return> Signature for unsafe extern "C" fn(A) -> Return {
    type Args = (A,);
    type Return = Return;
}

impl<A, B, Return> Signature for unsafe extern "C" fn(A, B) -> Return {
    type Args = (A, B);
    type Return = Return;
}

impl<A, B, C, Return> Signature for unsafe extern "C" fn(A, B, C) -> Return {
    type Args = (A, B, C);
    type Return = Return;
}

impl<A, B, C, D, Return> Signature for unsafe extern "C" fn(A, B, C, D) -> Return {
    type Args = (A, B, C, D);
    type Return = Return;
}

impl<A, B, C, D, E, Return> Signature for unsafe extern "C" fn(A, B, C, D, E) -> Return {
    type Args = (A, B, C, D, E);
    type Return = Return;
}

impl<A, B, C, D, E, F, Return> Signature for unsafe extern "C" fn(A, B, C, D, E, F) -> Return {
    type Args = (A, B, C, D, E, F);
    type Return = Return;
}
