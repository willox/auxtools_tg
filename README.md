# Compiling
Follow the build instructions from https://github.com/willox/auxtools/blob/master/README.md

# Using maptick
This proc has to exist:
```dm
/proc/auxtools_stack_trace(msg)
	CRASH(msg)
```

Init the lib like this:
```dm
/world/New()
	var/auxtools_tg = (world.system_type == MS_WINDOWS ? "./auxtools_tg.dll" : "./libauxtools_tg.so")
	if (auxtools_tg)
	// Optional: Log the result somewhere if the returned value is not "SUCCESS"
		call(auxtools_tg, "auxtools_init")()
	. = ..()
```

Shutdown the lib like this (this is 100% necessary):
```dm
	var/auxtools_tg = (world.system_type == MS_WINDOWS ? "./auxtools_tg.dll" : "./libauxtools_tg.so")
	if (auxtools_tg)
		call(auxtools_tg, "auxtools_shutdown")()
	. = ..()
```
