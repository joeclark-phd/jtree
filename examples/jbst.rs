

use jtree::jbst::Jbst;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_tree = Jbst::new();
    my_tree.add(5)?;
    my_tree.add(1)?;
    my_tree.add(3)?;
    my_tree.add(2)?;
    my_tree.add(4)?;
    println!("L to R: {:?}", my_tree.as_vec());
    println!("L to R: {:?}", my_tree.as_vec_r_to_l());

    println!("debug output: {:?}", my_tree);
    Ok(())
}