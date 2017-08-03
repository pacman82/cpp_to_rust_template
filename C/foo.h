#ifdef __cpluspuls
extern "C"{
#endif

//Declaration only. Never implemented. Just for additional type safety in C++.
struct foo_rs;

int32_t foo_create(out **foo_rs);
void foo_free(self *foo_rs);
void foo_bar(self * const foo_rs);

#ifdef __cpluspuls
extern }
#endif