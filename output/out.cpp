#include "soul_hardCodedFunctions/soul_hardCodedFunctions.h"
extern const __Soul_ARRAY__<int> array;
extern const float __var_float;
extern const int number;
constexpr auto __Soul_c_str_2__ = __NEW_Soul_LITERAL_ARRAY_C_STR__("else");
constexpr auto __Soul_c_str_0__ = __NEW_Soul_LITERAL_ARRAY_C_STR__("num == 3");
constexpr auto __Soul_c_str_1__ = __NEW_Soul_LITERAL_ARRAY_C_STR__("true == true");
const int number = 1;
constexpr float gravity = 9.81;
const float __var_float = 1;
constexpr unsigned int uintNumber = 1;
const __Soul_ARRAY__<int> array = __NEW_Soul_LITERAL_ARRAY__<int,1,2,3,4>();
namespace __SOUL_ns_scp_1__{
int sum(const int a, const int b);
namespace __SOUL_ns_scp_2__{
int foo(const int a);
int foo(const int a) {
return 0;
}
}
int sum(const int a, const int b) {
return (a + b);
}
}
int main(int __SOUL_C_argsc, char** __SOUL_C_argsv) {
auto __var_args = __Soul_ARRAY__<__Soul_ARRAY__<char>>(__SOUL_C_argsc);
for(int i = 0; i < __SOUL_C_argsc; i++){
	__var_args[i] = toStr((const char*)__SOUL_C_argsv[i]);
}
auto& args = __var_args;const int num = (1 + 2);
int sec = __SOUL_ns_scp_1__::sum(num,2);
Println(num);
if((num == 3)) {
int a = 0;
if(true) {
int a = 1;
}
Println(__Soul_c_str_0__);
}
if(true) {
Println(__Soul_c_str_1__);
}
else {
Println(__Soul_c_str_2__);
}
constexpr int foo = (1 + __Soul_CompileConst_math__::pow(1,2));
Println(foo);
return 0;
}
