use anyhow::Result;

const ARRAY: [i32; 6] = [0, 1, 2, 3, 4, 5];

// Not compliant because it may panic
pub fn example_non_compliant(index: usize) -> i32 {
    ARRAY[index]
}

pub fn example_using_explicit_bound_check(index: usize) -> Option<i32> {
    // This is not preferred because built-in mechanisms exists with the get() method
    // to achieve this more directly.
    if index < ARRAY.len() {
        Some(ARRAY[index])
    } else {
        None
    }
}

// A compliant (yet odd) example using slicing.
pub fn example_using_slice(index: usize) -> Option<i32> {
    let slice = ARRAY.get(..index + 1)?;
    slice.get(slice.len() - 1).copied()
}

// A compliant example using get() with a Result.
pub fn example_using_get_with_result(index: usize) -> Result<i32> {
    ARRAY
        .get(index)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("Error indexing into array"))
}

// A compliant example using get() with an Option.
pub fn example_using_get_with_option(index: usize) -> Option<i32> {
    ARRAY.get(index).copied()
}

fn main() -> Result<()> {
    println!(
        "Hello, world! {}",
        example_using_slice(30).ok_or_else(|| anyhow::anyhow!("Error getting value"))?
    );

    Ok(())
}
