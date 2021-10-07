struct A; // Concreate type A
struct S(A); // Concreate type S
struct SGen<T>(T); // Generic type SGen

fn reg_fn(_s: S) {}
fn gen_spect_t(_s: SGen<A>) {}
fn gen_spect_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spect_t(SGen(A));
    gen_spect_i32(SGen(3));

    generic::<char>(SGen('a')); // Explicitly specified type
    generic(SGen('b')); // Implicitly specified type
}
