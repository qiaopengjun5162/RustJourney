use mlua::{Lua, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua = Lua::new();

    // -- Load
    let chunk = lua.load(
        r#"
        local num = 123
        print("Hello, from Lua! " .. num)
        return num +  1
    "#,
    );

    // -- Execute
    // let result = chunk.exec::<i64>()?;
    // println!("Result: {}", result);

    // -- Eval
    let result = chunk.eval::<Value>()?;
    println!("Result: --> {:?}", result);

    Ok(())
}

