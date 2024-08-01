extern crate c_bind;

fn main() {
    let mut pm = c_bind::PluginManager::new ();

    let data = [0u8, 10];

    pm.load (String::from("led"), String::from("plugins/libled.so"));
    pm.load (String::from("button"), String::from("plugins/libbutton.so"));

    pm.read ("led".to_string(), &data);
    pm.write ("led".to_string(), &data);

    pm.read ("button".to_string(), &data);
    pm.write ("button".to_string(), &data);

    pm.close ();
}
