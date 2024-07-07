# What are borrowed base objects

Base objects that are not owned by current resource,
for example remote players, vehicles

## Scopes

"Scopes" are needed so that borrowed base objects cannot be used
after destroy (invalid base objects)<br>
One scope is equal to one tick (both serverside and clientside)<br>
To use some borrowed base object (read something from it or call something on it)
it needs to be attached to some scope

```rust
// `context` is Scope
altv::set_timeout(|context| {
  let player: Option<altv::Player> = altv::Player::get_by_id(
    /* scope: */ context,
    /* id: */ 123
  );
  // let's assume player with such id is valid on this tick
  let player = player.unwrap();

  altv::set_timeout(|_| {
    // compile time error,
    // this player was attached to `context` and
    // may already be destroyed when this callback is called
    dbg!(player);
  }, Duration::from_secs(1));
}, Duration::from_secs(1));
```

Scopes are passed into event callbacks, timer callbacks, everywhere

```rust
// `context` is Scope
altv::events::on_game_entity_create(|context| {
// ...
});

// `context` is Scope
altv::events::on_net_owner_change(|context| {
// ...
});

// `context` is Scope
altv::every_tick(|context| {
// ...
});
```

But futures (async/await) is special case

```rust
let future = async {
  // from where do we have to get scope?
  let player: Option<altv::Player> = altv::Player::get_by_id(
    /* scope: */ ???,
    /* id: */ 123
  );
  altv::wait(Duration::from_secs(1)).await;
  // ???
  dbg!(player.name());
};
```

For this use case it's possible to create new scope anywhere using [`new_scope`](super::scope::new_scope)

```rust
let future = async {
  let player_name = altv::new_scope(|scope| {
    let player = altv::Player::get_by_id(scope, 123);
    // let's assume player with such id is valid on this tick
    let player = player.unwrap();

    // no need to hold reference to player instance if we only need data from it
    player.name()
  });

  altv::wait(Duration::from_secs(1)).await;

  dbg!(player_name);
};
```

If you want to hold reference to base object for longer than one scope
(one tick) there are detached base objects, for example [`DetachedPlayer`](super::detached_player::DetachedPlayer)

```rust
let future = async {
  let detached_player = altv::new_scope(|scope| {
    let player = altv::Player::get_by_id(scope, 123);
    // let's assume player with such id is valid on this tick
    let player = player.unwrap();

    player.detach_from_scope()
  });

  altv::wait(Duration::from_secs(1)).await;

  altv::new_scope(|scope| {
    let Some(player) = detached_player.attach_to_scope(scope) else {
      // player was destroyed
      return;
    };
    dbg!(player.name());
  });
};
```

See [`Scope`](super::scope::Scope#example) trait documentation for more examples of usage
