#[altv::main]
fn main() {
    let veh = altv::Vehicle::new("sultan2", (0, 3, 71), 0).unwrap();
    let veh = altv::Vehicle::new("sultan2", (0, 3, 71), 0).unwrap();

    altv::events::on_player_connect(move |_| {
        altv::log!("player connected!");
        let veh = veh.clone();
        altv::set_timeout(
            move || {
                veh.destroy().unwrap();
                altv::Vehicle::new("sultan2", (0, 3, 71), 0).unwrap();
            },
            2000,
        );
    });
}
