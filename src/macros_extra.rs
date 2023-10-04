
fn macros() {
    // the same things: 
    
    let list = vec![1, 2, 3, 4];

    let list = {
        let mut list = Vec::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list
    };
}


