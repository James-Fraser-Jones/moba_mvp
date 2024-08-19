# mvp

## broad

- map

  - square
  - _no_ jungle
  - 2 _square_ lanes: top, bot
  - diagonal mid lane
  - river
  - all lanes and river have some arbitrary width
  - 4 identical triangles of impassable terrain
  - 3 turrets per team just off center in each lane (to allow minion pathing)
  - bases in corners:
    - heart
    - two turrets
  - 180 degree rotational symmetry around midpoint of map for
    - turret placement
    - heart placement
    - minion paths
  - linear symmetry also

- minions

  - spawning
  - navigating
  - following path
  - collision avoidance
  - attack moving
  - 4 melees, 4 ranged
  - different stats:
    - health
    - attack speed
    - attack damage
  - attack speed
    - windup
    - register
    - winddown
  - projectiles
  - health bars
  - dying

- characters

  - need 10 (naturally)
  - effectively powered up minions
  - have abilities
  - can "stutter step" i.e. cancel attack winddown with new move command without penalty
  - no turning speed

## steps

1. **(DONE)** single unit, displaying

   - position
   - radius (colored circle)
   - orientation (colored triangle within)

2. **(DONE)** multiple of these units

3. **(DONE)** units moving along their given orientations in a straight line

4. **(DONE)** randomized spawns

5. **(DONE)** naive unit collision implemented

6. **(DONE)** basic map layout drawn

7. **(DONE)** spawners implemented

8. **(DONE)** project separated into plugins

9. **(DONE)** create wavemanager resource and have correctly spawning minions waves

10. **(DONE)** make minions walk to midpoints of lanes and then turn and walk the rest

11. **(DONE)** make minions switch into attack mode when they see nearby enemies

---

3D Detour

12. Loading models

13. Instancing many of the same model (for minions??)

14. Do unlit materials work well?

15. Render graph, blending animations (specifically walk/run)

16. Fog of war effect with alpha blending or some other technique

17. 3D camera positioned correctly, pannable, zoomable

18. 3D unit space still -1000, 1000 in all axes

19. Loading/unloading extra parts of the mesh (weapons etc..)

---

12. split minions into melee and ranged with differing attack ranges

13. give minions an attack animation

14. give minions health bars

15. make attacks hurt minions

16. make minions die when their health falls to 0

17. make ranged minions have projectiles
