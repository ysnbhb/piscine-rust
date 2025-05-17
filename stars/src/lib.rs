pub fn stars(n: u32) -> String {
   String::from("*").repeat((2.0_f32).powf(n as f32) as usize)
}