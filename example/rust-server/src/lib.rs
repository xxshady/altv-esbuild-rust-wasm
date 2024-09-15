use altv::{
    meta::{BaseObjectMetaEntry, StreamSyncedEntityMeta},
    BaseObjectPoolFuncs,
};

use serde::{Deserialize, Serialize};

#[altv::main]
fn main() {
    // let veh = altv::Vehicle::new("sultan2", (0, 3, 71), 0).unwrap();
    // let veh = altv::Vehicle::new("sultan2", (0, 3, 71), 0).unwrap();

    altv::events::on_player_connect(move |ctx| {
        new_player(ctx.player.clone())?;
        // let veh = veh.clone();
        // altv::set_timeout(
        //     move || {
        //         veh.destroy().unwrap();
        //         altv::Vehicle::new("sultan2", (0, 3, 71), 0).unwrap();
        //     },
        //     2000,
        // );

        Ok(())
    });

    altv::set_timeout(
        || {
            let all = altv::Player::all();
            dbg!(all.len());

            all.iter().for_each(|p| {
                new_player(p.clone()).unwrap();
            });
        },
        1000,
    );

    fn new_player(p: altv::PlayerContainer) -> altv::VoidResult {
        altv::log!("new player: {}", p.name()?);
        // p.emit("test", &[&bincode::serialize(&(123_i32,))?])?;

        altv::set_timeout(
            move || {
                let veh = altv::Vehicle::new("sultan2", 0, 0).unwrap();
                altv::log!("created vehicle: {}", veh.id().unwrap());

                p.emit(
                    "deserialize_base_object",
                    &[&bincode::serialize(&(AnyHandle {
                        id: veh.id().unwrap(),
                        generation: veh
                            .stream_synced_meta_entry::<u64>("&^#altv-rust")
                            .unwrap()
                            .get()
                            .unwrap()
                            .unwrap(),
                    },))?],
                )?;

                Ok(())
            },
            1000,
        );
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct AnyHandle {
    id: u32,
    generation: u64,
}
