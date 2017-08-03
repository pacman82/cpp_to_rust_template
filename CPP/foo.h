#include "../C/foo.h"

class foo{

    foo_rust * impl = nullptr;

public:

    void bar(){
        foo_bar(impl);
    }

    foo(){
        foo_create(&impl);
    }

    ~foo(){
        foo_free(impl);
    }
}