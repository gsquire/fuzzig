use afl::fuzz;
use libc::size_t;

#[link(name = "json", kind = "static")]
extern "C" {
    fn validate_json(j: *const u8, j_len: size_t) -> size_t;
}

fn main() {
    fuzz!(|data: &[u8]| {
        unsafe {
            validate_json(data.as_ptr(), data.len());
        }
    });
}
