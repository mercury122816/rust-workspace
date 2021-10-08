Integer Types

Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
128-bit	i128	u128
arch	isize	usize

Integer Overflow

Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, integer overflow will occur. 

When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. <b>Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, 256 becomes 0, 257 becomes 1, and so on.</b>

<b>
To explicitly handle the possibility of overflow, you can use these families of methods that the standard library provides on primitive numeric types:

    - Wrap in all modes with the wrapping_* methods, such as wrapping_add
    - Return the None value if there is overflow with the checked_* methods
    - Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
    - Saturate at the value's minimum or maximum values with saturating_* methods
</b>


