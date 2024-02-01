/// Check if pointer is in linear memory
pub fn is_linear_ptr<P>(p: *const P) -> bool {
    let addr = p as usize;
    addr >= ctru_sys::OS_FCRAM_VADDR as usize
        && addr < (ctru_sys::OS_FCRAM_VADDR as usize + ctru_sys::OS_FCRAM_SIZE as usize)
}
