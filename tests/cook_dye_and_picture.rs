use cooking::{Cook, CookError};

#[test]
fn test_dye() -> Result<(), CookError>{
    let cook = Cook::new()?;
    cook.cook(&["Navy", "Orange", "Brown", "Gray"])?;
    Ok(())
}

#[test]
fn test_picture() -> Result<(), CookError> {
    let cook = Cook::new()?;
    cook.cook(&[
        "Fauna Picture", 
        "Enemy Picture",
        "Material Picture",
        "Other Picture"
    ])?;
    cook.cook(&[
        "Weapon Picture", 
        "Elite Enemy Picture",
    ])?;
    Ok(())
}
