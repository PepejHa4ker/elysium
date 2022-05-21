use elysium_sdk_material::MaterialKind;

fn main() {
    println!("{:?}", MaterialKind::Gold);
    println!("{:?}", MaterialKind::Gold.name());
    println!("{:?}", MaterialKind::Gold.base());
    println!("{:?}", MaterialKind::Gold.vdf());
}
