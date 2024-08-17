# ordering

so we have some complex behaviour we should figure out:

3 things moving:

- the cursor
- the 3d cursor
- the camera
- the player

and they can influence each other in several ways:

- `cursor -> camera` (through edge-panning)
- `(cursor, camera) -> 3d cursor` (projecting through ground plane)
- `3d cursor -> player` (move command)
- `player -> camera` (follow player with spacebar)

we want to guarantee a couple of things, to prevent stuttering:

- 3d cursor is set _after_ camera transform **and** global transform have been updated
- camera transform is set _after_ player has been moved

unfortunately, this is a cycle  
I suggest we simply have a seperate system for updating the camera to the player position after player move, if the spacebar is held

to be clear, since 3d cursor is in input, this involves:

- get last cursor position from input
- update camera, based on last cursor position panning, also update camera global transform
- update input 3d cursor, based on camera global transform
- update player move logic based on 3d cursor
- update camera transform again, based on player transform

I don't like this  
`Input -> Camera -> Input -> Player -> Camera`

I think we should have the 3d cursor as part of the orbit camera, actually  
or have it as a seperate module, `Cursor`

similar to the camera, cursor is essentially a 3d object within the game that affects things in a spooky weird way

it's also _wholly_ dependent on the transform of the camera, its updated position each frame (which is the whole point of it, as an indicator and tool for interaction) solely depends on the 2d cursor and the orbit camera's transform

I should really just add the projection functions as helper methods to the orbit camera component to indicate usage, ofc queries will need to be made seperately and that's okay

if we have the cursor as part of the camera, it will simplify it to:

`Input -> Camera -> Player -> Camera`

I'm still not super happy with this

I think we might be able to avoid stuttering a different way:
updating the camera position at the _start_ of the update method
no because then the player will be one ahead of the camera

the camera really should just be set _after_ the game objects, because that's the only way to guarantee smooth following, so basically:

`Input -> Player -> Camera`

and of course the player will be using camera also, but the camera from the previous frame, I think this is okay!

and camera will use Input also, and that's also fine

this way perhaps we can also get away without updating the global transform?
each frame, we really _should_ draw the information which is "correct" i.e. used to determine the player movement

actually, no, we really shouldn't because one frame of glad between input is better??

we'll try this, and we'll draw the 3d cursor and healthbars _after_ the camera has been updated

okay, it's done and works perfectly: `Input -> Player -> Camera`

in the larger structure we have:

- Cameras
- Graphics
- Logic
- Dev
- Input
- OS
- Player
- Types

splitting it up:

inert:

- Types
- OS

draws to screen, hence needs to be set _after_ camera transform updated:

- Graphics
- Dev

remaining:

- Cameras
- Logic
- Input
- Player

of these 4, the natural ordering would be:

`Input -> Player -> Logic -> Camera`

so I guess we have the following ordering:

`Input -> Player -> Logic -> Cameras -> (Graphics, Dev)`

something I don't like is that, currently, the cursor used by the player is from the previous frame

okay here's an idea, the 3d cursor actually belongs on Player because it's an essential part of player action (i.e. being able to select things etc..)

also, it's a shame but we only really need to update the camera based on input before logic runs so that we can get the projection correctly, I propose that we are explcit about using camera for the purposes of 3d projection whilst also wanting to provide the most up-to-date info for logic but without causing stuttering

in this case it seems in fact useful to have the following set up:

`Input -> Cameras -> Player -> Logic -> Cameras -> (Graphics, Dev)`

so essentially: 2 update cycles on the main camera, the first to respond to inputs to give Player the most up-to-date information, the second to sync the camera up with the resulting game state after logic has run

After having looked at the first section of the Bevy book on "Transform Interpolation/Extrapolation" I've been reminded that actually my game logic should be running in Fixedupdate

this has a few implications:

Bevy's update goes like this:

`PreUpdate -> (PreFixedUpdate -> FixedUpdate -> PostFixedUpdate -> Loop...) -> Update -> PostUpdate`

where the fixed update loop runs `0..n` times

this implies that my previous ordering is null because we basically want to put everything other than `Player` and `Logic` into `Update` to guarantee smooth operation

also, I think there's a general principle here that what the player is seeing on their screen is what their inputs are interfacing with, and that isn't true when you're taking their inputs to move the camera and _then_ in the same update cycle using that new camera position to determine player cursor selection before you've even rendered it to their screen

hence, we'll actually have the following:

```
PreUpdate: Input
FixedUpdate: Player -> Logic -> Physics  ...Networking???
Update: Camera, Graphics, Gizmos
PostUpdate: Healthbars, 3D Cursor, ..anything else which needs to be drawn *after* the camera was updated this frame
```

Input needs to go in `PreUpdate` specifically because it needs to be available before the fixedupdate loop (which is before normal `Update`)

Also, we're going to have a 32Hz tick rate as opposed to the default of 64Hz.

Also I think we need to get networking set up sooner rather than later because it might demand significant project structure changes

Also I found out that `Pub Use` is a thing that exists

Also I think module files should have their own naming conventions and should largely be used to facilitate the desired file structure rather than being bevy plugins in their own right

I'm going to look at the bevy project structure for inspiration

Also does Avian have a deterministic mode like Rapier?
It doesn't, so we're going to use Rapier I think.

Regardling modules. I basically only started using modules because I wanted to be able to use seperate files in a folder hierarchy for the sake of code organization.

With this in mind, I find it quite frustrating that both of Rust's options for facillitating this are not ideal.

Actually what am I talking about this is fine. I'll just use mod.rs from now on as it makes things considerably simpler.
