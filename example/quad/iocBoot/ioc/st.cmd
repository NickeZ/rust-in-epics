#!../../bin/linux-x86_64/Quad

## You may have to change ioc to something else
## everywhere it appears in this file

#< envPaths

## Register all support components
dbLoadDatabase("../../dbd/Quad.dbd",0,0)
Quad_registerRecordDeviceDriver(pdbbase) 

## Load record instances
dbLoadRecords("../../db/quad.db","user=niklas")

iocInit()

## Start any sequence programs
#seq sncioc,"user=niklas"
