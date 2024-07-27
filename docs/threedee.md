- ambient light resource added, giving color and brightness
- animations resource stores a vector of "AnimationNodeIndex" and a Handle to an AnimationGraph

- have ResMut for meshes, (standard)materials, animationgraphs
- also have a resource for the assetserver

- clips are added to the (root node of the) animation graph, as labels, which are then used by the asset server to produce handles to these animations

- this animation graph is then added to the resource that stores animation graphs and the handle is stored for later

- the "Animations" resource is being used to store the handle to the animation graph and all the animation node indexes for each of the animations

- a 3d camera is spawned
- a plane is spawned
- a directional light is spawned

- the gltf model is spawned using a "SceneBundle", also using a GLTFAssetLabel, the path to the file, the asset server, handles, ...

the "setup_scene_once_loaded" function queries for recently added "AnimationPlayer" componentss, creates a new "AnimationTransitions" struct and uses the Animations resource to grab the first AnimationNodeIndex, gives it to the AnimationTransitions guy, tells him to play the animation player on repeat, also inserts a clone of the animation graph into the entity containing the animationplayer component, and inserts the animationtransitions component

so we have:

```
Entity:
-> SceneBundle??
-> AnimationPlayer
-> AnimationTransitions
    -> AnimationNodeIndex
-> Handle<AnimationGraph>
```
