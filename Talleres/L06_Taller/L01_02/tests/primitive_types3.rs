#[cfg(test)]
mod tests {
    #[test]
    fn test_primitive_types3() {
        let a = [1,213,321,123,3];
        //let a = "fsdfdsf"; // tambien funciona

        if a.len() >= 100 {
            println!("Wow, that's a big array!");
        } else {
            println!("Meh, I eat arrays like that for breakfast.");
        }        
    }
}