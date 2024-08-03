target extended-remote :3333
layout split
monitor reset init
load
monitor reset init
break Reset
i r pc sp
continue

