use digest::Digest;

// This compiles
fn repr<D: Digest>() -> usize {
    D::output_size()
}

fn main() {}
