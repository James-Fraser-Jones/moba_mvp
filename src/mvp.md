# mvp

## broad
* map
    * square
    * *no* jungle
    * 2 *square* lanes: top, bot
    * diagonal mid lane
    * river
    * all lanes and river have some arbitrary width
    * 4 identical triangles of impassable terrain
    * 3 turrets per team just off center in each lane (to allow minion pathing)
    * bases in corners:
        * heart
        * two turrets
    * 180 degree rotational symmetry around midpoint of map for 
        * turret placement
        * heart placement
        * minion paths
    * linear symmetry also
* minions
    * spawning
    * navigating
    * following path
    * collision avoidance
    * attack moving
    * 4 melees, 4 ranged
    * different stats: 
        * health
        * attack speed
        * attack damage
    * attack speed
        * windup
        * register
        * winddown
    * projectiles
    * health bars
    * dying
* characters
    * need 10 (naturally)
    * effectively powered up minions
    * have abilities
    * can "stutter step" i.e. cancel attack winddown with new move command without penalty
    * no turning speed

## steps

1. **(DONE)** single unit, displaying
    * position
    * radius (colored circle)
    * orientation (colored triangle within)

2. multiple of these units
3. units having randomised positions, orientations, radiuses
