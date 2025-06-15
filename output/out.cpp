#include "soul_hardCodedFunctions/soul_hardCodedFunctions.h"
extern const __Soul_ARRAY__<int> array;
extern const char ch;
extern const float __var_float;
extern const int number;
constexpr auto __Soul_c_str_0__ = __NEW_Soul_LITERAL_ARRAY_C_STR__("input: ");
const char ch = 'a';
const int number = 1;
constexpr float gravity = 9.81;
const float __var_float = 1;
constexpr unsigned int uintNumber = 1;

const __Soul_ARRAY__<int> array = __NEW_Soul_LITERAL_ARRAY__<int,1,2,3,4>();
int main(int __SOUL_C_argsc, char** __SOUL_C_argsv) {
auto __var_args = __Soul_ARRAY__<__Soul_ARRAY__<char>>(__SOUL_C_argsc);
for(int i = 0; i < __SOUL_C_argsc; i++){
	__var_args[i] = str((const char*)__SOUL_C_argsv[i]);
}
auto const* args = &__var_args;
int32_t num = 1;
int32_t const* ref = &num;
__Soul_ARRAY__<char> buffer = str();
Print(__Soul_c_str_0__);
Input(&buffer);
Println(buffer);
return 0;
}
