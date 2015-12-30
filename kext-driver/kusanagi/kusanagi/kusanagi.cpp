#include <IOKit/IOLib.h>
#include "kusanagi.hpp"

OSDefineMetaClassAndStructors(com_yupferris_driver_kusanagi, IOService)

#define super IOService

bool com_yupferris_driver_kusanagi::init(OSDictionary *dict)
{
    auto result = super::init(dict);
    
    IOLog("Initializing\n");
    
    return result;
}

void com_yupferris_driver_kusanagi::free()
{
    IOLog("Freeing\n");
    
    super::free();
}

IOService *com_yupferris_driver_kusanagi::probe(IOService *provider, SInt32 *score)
{
    auto result = super::probe(provider, score);
    
    IOLog("Probing\n");
    
    return result;
}

bool com_yupferris_driver_kusanagi::start(IOService *provider)
{
    auto result = super::start(provider);
    
    IOLog("Starting\n");
    
    return result;
}

void com_yupferris_driver_kusanagi::stop(IOService *provider)
{
    IOLog("Stopping\n");
    
    super::stop(provider);
}