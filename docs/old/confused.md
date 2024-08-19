# Confused :(

Too many things floating in my head

- Concepts
- Arrangements
- Abstractions

---

I don't know how to make steady progress any more, the complexity has become unmanagable already.

I can't commit to anything because I'm very scared that it won't scale.

I need to find out how large AAA game projects tend to be structured.

I can think of some things that probably split reasonably nicely into chunks:

- GameInput: Keyboard / Mouse / Gamepad
- GameProcess: Logic / AI / Physics
- GameOutput: Graphics / Animation / Sound
- GameInterface: OS / Assets / Networking / Scripting

Thinking in terms of:

- Systems
- Components
- Resources
- Bundles
- Events
- States

I think having seperate plugins has been a huge waste of time for me, modules are necessary to facillitate the file structure I'm looking for, they're fine.

I think having no plugins or plugingroups is best and having exactly this structure:

- input
  - mouse (incl some existing camera logic)
  - keyboard (incl some existing camera logic)
- process
  - logic (already pretty hefty)
  - physics (layer over avian)
- output
  - graphics (hefty + incl cameras)
  - sound (not touched yet)
- interface
  - os (incl window and quit)
  - assets (not touched yet)

actually maybe we just collapse this hierachy again, and remove untouched

- mouse
  - collects polling/button info into a simpler interface
- keyboard
  - collects polling/button info into a simpler interface (specifically with WASDCtrlSpace as a normalized Vec3)
- inputmap
  - uses keyboard and mouse interfaces, hooks them up to other stuff through interfaces/events etc..
- logic
  - defines all logical game types which it utilises, also may be frequently used by graphics as many are renderable
  - importantly, logical units should be specifed as _components_ not as bundles, bundles should only be used when it is necessary to include other components/bundles from the engine, or when there is behaviour to be legitimately recycled between different types of units (e.g. auto-attack behaviour between minions, player characters, monsters, ...), start with components and factorise _where necessary_
- physics
  - convenience layer around avian
- graphics
  - so far we have a system where you can add components that request meshes and materials, by name, and the correct handles will be assigned on the associated components, this seems to work great! even better than events, I think
  - I think we can do even better:
    basically if we publically export every logical component from the logical module, we can literally just query for every one of these components that doesn't also have a "rendered" tag with it, and simply then add the correct mesh and material, this allows those components to be defined in logic without having to specify any information about rendering
  - I cannot imagine a use case for actual events when this system is possible tbh, just adding and removing tags of information that can be collected whenever you like
- animation (empty)
- camera
  - I guess camera is different enough that it warrants its own module?
- os
  - quit event, window settings
- sound (empty)
- assets (empty)
- network (empty)
- scripting (empty)

all may define resources for settings relevent to themselves

all may define components/systems/bundles for the meat of their functionality

all may define init systems for their own initialization

all may define events for input/output from/to other systems
