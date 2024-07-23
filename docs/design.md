# design

## pathfinding

picked up from [starcraft talk](https://www.gdcvault.com/play/1014514/AI-Navigation-It-s-Not)

### planning

* path mesh
    * constrained delaunay triangulation
    * point location via cached jump-and-walk
        * location yourself
        * overlayed sparse regular grid
    * statically allocated
    * 16 bytes per face, 4 bytes per vertex
    * no mobile units
* planning
    * A* search on the path mesh (hierarchical??)
    * support with different radii (using funnel algorithm??)

### steering

* Craig Reynold's boids
    * Following
    * Flocking
    * Grouping
    * Seperation
    * Avoidance
    * Arrival
* Need to do checks to ensure steering doesn't steer units into walls
* Prevent hallway dance with mind reading of local units
* Avoidance steering allows navigation around concave depressions in clumps of other stationary units

### collision

* Units can push other (non-stationary) units
* Physics simulation using circles and a limited number of rounds
* Pathing mesh is used for collision logic to ensure units aren't pushed into unwalkable terrain

## networking

client-server architecture with "client-prediction" and/or "rollback" optionally enabled/disabled for specific player actions (possibly other game events)

watch the [overwatch talk](https://www.gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and) again for more info

UDP with optional in-order reliability layer

## wide (content)
* status effects (buffs/CC)
* damage types
* damage collision types
* movement effects
* offensive stats
* defensive stats
* misc stats (movespeed,...)
* character kits
* items
* masteries
* jungle mobs
* misc environmental interactives
