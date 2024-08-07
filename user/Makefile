TASKS=hello_world-build

.PHONY: all clean $(TASKS:build=clean) $(TASKS:build=build)

all: $(TASKS:build=build)

clean: $(TASKS:build=clean)

$(TASKS:build=clean):
	@cd $(@:-clean=) && cargo clean

$(TASKS:build=build):
	@cd $(@:-build=) && cargo build
