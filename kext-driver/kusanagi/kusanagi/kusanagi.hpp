#include <IOKit/IOService.h>
class com_yupferris_driver_kusanagi : public IOService
{
OSDeclareDefaultStructors(com_yupferris_driver_kusanagi)
public:
    virtual bool init(OSDictionary *dictionary = nullptr);
    virtual void free();
    
    virtual IOService *probe(IOService *provider, SInt32 *score);
    
    virtual bool start(IOService *provider);
    virtual void stop(IOService *provider);
};