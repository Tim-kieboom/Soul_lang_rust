#include "soul_hardCodedFunctions/soul_hardCodedFunctions.h"

extern const __Soul_ARRAY__<int>::AsConst array;
extern __Soul_ARRAY__<char> strarray;

constexpr char __temp__Soul_c_str_2__[] = "ello";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_2__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_2__);
constexpr char __temp__Soul_c_str_0__[] = "werg";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_0__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_0__);
constexpr char __temp__Soul_c_str_3__[] = "foo ";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_3__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_3__);
constexpr char __temp__Soul_c_str_1__[] = "hoiu";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_1__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_1__);

void wrong();

__Soul_ARRAY__<char> __programMemory_5[] = {__Soul_c_str_2__,__Soul_c_str_1__,__Soul_c_str_0__,__Soul_c_str_0__};
constexpr int __programMemory_4[] = {1,2,3,4};
constexpr int __programMemory_7[] = {1};
constexpr int* __programMemory_6 = nullptr;

const __Soul_ARRAY__<int>::AsConst array = __Soul_ARRAY_LiteralCtor__(int,__programMemory_4);
// __Soul_ARRAY__<char> strarray = __Soul_ARRAY_LiteralCtor__(__Soul_ARRAY__<char>,__programMemory_5);

void wrong() {
	__Soul_ARRAY__<int> array2 = __Soul_ARRAY_LiteralCtor__(int,__programMemory_6);
}


int main(int __SOUL_C_argsc, char** __SOUL_C_argsv) {

	auto __var_args = __Soul_ARRAY__<__Soul_ARRAY__<char>>(__SOUL_C_argsc); for(int i = 0; i < __SOUL_C_argsc; i++){ *__var_args.__get_mutRef(i) = str((const char*)__SOUL_C_argsv[i]); } auto const* args = &__var_args; 
	Println(args);
	int const* f = (const int*)&__programMemory_7;
	int32_t num = i32(1);
	int32_t const* ref = (const int32_t*)&num;
	__Soul_ARRAY__<char> buffer = str();
	__Soul_ARRAY__<char>::AsConst const* refstr = (__Soul_ARRAY__<char>::AsConst const*)&buffer;
	Print(__Soul_c_str_3__);
	Input(&buffer);
	Println((__Soul_ARRAY__<char>::AsConst const*)&buffer);
	return 0;
}

