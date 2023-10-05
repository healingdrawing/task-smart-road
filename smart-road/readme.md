The folder for the project implementation.
Later can be copied to the gitea repository, which is restricted by size/type of files etc.

Let is say the global flow can be as follows:
1. terminal `cargo run` to run the project, the game screen appears, it can not be resized. The planned cross road size is 1152px x 1152px.

2. user press any of x4 arrow keys to generate the vehicle on the road, if previous vehicle generated on this place already arrived to next point before start move cross road, appropriate to the key pressed.
The road includes x12 lanes, from each side of the cross road, x3 lanes for each direction.
Not more than x3 vehicles can be randomly generated on the one lane. If the lane is full nothing happens.
The arrow set the vehicle start direction.
The lane will be selected randomly.
The vehicle will be generated on the lane, which is selected randomly.
Depends on the lane, the vehicle can go forward, left or right, after crossing the cross road.

3. When th vehicle is moved to the position closest to the start of the cross road, it starts to move cross the cross road only when the move on this path is allowed.
The order of allowed moves is as follows:
- to north and to south without turning
- to east and to west without turning
- turn left to north
- turn left to south
- turn left to east
- turn left to west
The turn right is always allowed, because this direction is always free.
Every switch of allowed direction for move the vehicle, allows to move only one vehicle from the lane, which is closest to the cross road. After that the next direction is allowed.

