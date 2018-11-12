TOP=../..

include $(TOP)/configure/CONFIG
#----------------------------------------
#  ADD MACRO DEFINITIONS AFTER THIS LINE
#=============================

#=============================
# Build the IOC application

PROD_IOC = Quad
# Quad.dbd will be created and installed
DBD += Quad.dbd

# Quad.dbd will be made up from these files:
Quad_DBD += base.dbd

# Include dbd files from all support applications:
Quad_DBD += rust.dbd

# Add all the support libraries needed by this IOC
#Quad_LIBS += xxx

# Quad_registerRecordDeviceDriver.cpp derives from Quad.dbd
Quad_SRCS += Quad_registerRecordDeviceDriver.cpp

# Build the main IOC entry point on workstation OSs.
Quad_SRCS_DEFAULT += QuadMain.cpp
Quad_SRCS_vxWorks += -nil-

# Add support from base/src/vxWorks if needed
#Quad_OBJS_vxWorks += $(EPICS_BASE_BIN)/vxComLibrary

Quad_LDFLAGS += -pthread
Quad_SYS_LIBS += dl
Quad_LIBS += quad

# Finally link to the EPICS Base libraries
Quad_LIBS += $(EPICS_BASE_IOC_LIBS)

quad_DIR = ${TOP}/QuadApp/src/quad/target/release

#===========================

include $(TOP)/configure/RULES
#----------------------------------------
#  ADD RULES AFTER THIS LINE

$(quad_DIR)/libquad.so $(quad_DIR)/libquad.a: ${TOP}/QuadApp/src/quad/src/lib.rs
	cargo build --release --manifest-path ${TOP}/QuadApp/src/quad/Cargo.toml
