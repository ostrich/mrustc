V ?= @

OBJDIR := .obj/

BIN := ../../bin/testrunner
OBJS := main.o path.o

LINKFLAGS := -g
CXXFLAGS := -Wall -std=c++14 -g -O2

CXXFLAGS += $(CXXFLAGS_EXTRA)
LINKFLAGS += $(LINKFLAGS_EXTRA)

OBJS := $(OBJS:%=$(OBJDIR)%)

.PHONY: all clean

all: $(BIN)

clean:
	rm $(BIN) $(OBJS) $(OBJS:%.o=%.o.dep)

$(BIN): $(OBJS)
	@+mkdir -p $(dir $@)
	@echo [CXX] -o $@
	$V$(CXX) -o $@ $(OBJS) $(LINKFLAGS)

$(OBJDIR)%.o: %.cpp
	@+mkdir -p $(dir $@)
	@echo [CXX] $<
	$V$(CXX) -o $@ -c $< $(CXXFLAGS) -MMD -MP -MF $@.dep
# - Include files from common (TODO: Be less hacky)
$(OBJDIR)%.o: ../common/%.cpp
	@+mkdir -p $(dir $@)
	@echo [CXX] $<
	$V$(CXX) -o $@ -c $< $(CXXFLAGS) -MMD -MP -MF $@.dep

-include $(OBJS:%.o=%.o.dep)

# Optional ABI/constant tests without target Rust libraries. Cross compiler and
# runner options can be passed with CODEGEN_TEST_ARGS (see README.md).
.PHONY: check-codegen
check-codegen:
	python3 codegen-tests.py --output ../../output/codegen-tests $(CODEGEN_TEST_ARGS)

.PHONY: check-floats
check-floats:
	mkdir -p ../../output
	$(CXX) -std=c++14 -O2 $(CXXFLAGS_EXTRA) -I../../src/include floats.cpp -o ../../output/test-floats $(LINKFLAGS_EXTRA)
	../../output/test-floats
