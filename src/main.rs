use digest::Digest;

// This doesn't compile
fn repr<D: Digest>() -> usize {
    D::output_size()
}

fn main() {}
