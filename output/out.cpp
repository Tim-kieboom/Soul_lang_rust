#include "soul_hardCodedFunctions/soul_hardCodedFunctions.h"

extern const char ch;
extern const float __var_float;
extern const int number;
extern const __Soul_ARRAY__<__Soul_ARRAY__<char>::AsConst>::AsConst strarray;

constexpr char __temp__Soul_c_str_4__[] = "";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_4__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_4__);
constexpr char __temp__Soul_c_str_1__[] = "hoiu";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_1__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_1__);
constexpr char __temp__Soul_c_str_5__[] = "1 + 2 = ";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_5__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_5__);
constexpr char __temp__Soul_c_str_3__[] = "hello world";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_3__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_3__);
constexpr char __temp__Soul_c_str_0__[] = "werg";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_0__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_0__);
constexpr char __temp__Soul_c_str_2__[] = "ello";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_2__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_2__);

void array();

constexpr __Soul_ARRAY__<char>::AsConst __programMemory_2[] = {__Soul_c_str_2__,__Soul_c_str_1__,__Soul_c_str_0__,__Soul_c_str_0__};
constexpr int __programMemory_3[] = {1,2,3,4};

const char ch = 'a';
const int number = 1;
constexpr float gravity = 9.81;
const float __var_float = 1;
constexpr unsigned int uintNumber = 1;
const __Soul_ARRAY__<__Soul_ARRAY__<char>::AsConst>::AsConst strarray = __Soul_ARRAY_LiteralCtor__(__Soul_ARRAY__<char>,__programMemory_2);

void array() {
	__Soul_ARRAY__<int> array = __Soul_ARRAY_LiteralCtor__(int,__programMemory_3);
	int second = array[1];
	Println(array[0]);
}


namespace __SOUL_ns_scp_2__{
	int sum(const int a, const int b);
	int sum(const int a, const int b) {
		
		__soul_free(b);
		__soul_free(a);
		return (a + b);
	}
}
int main(int __SOUL_C_argsc, char** __SOUL_C_argsv) {

	auto __var_args = __Soul_ARRAY__<__Soul_ARRAY__<char>>(__SOUL_C_argsc); for(int i = 0; i < __SOUL_C_argsc; i++){ __var_args[i] = str((const char*)__SOUL_C_argsv[i]); } __Soul_ARRAY__<__Soul_ARRAY__<char>>* const args = (__Soul_ARRAY__<__Soul_ARRAY__<char>>* const)&__var_args; 
	Println(args[0]);
	Println(__soul_format_string__(__Soul_c_str_3__));
	Println(__soul_format_string__(__Soul_c_str_5__,__SOUL_ns_scp_2__::sum(1,2),__Soul_c_str_4__));
	array();
	
	__soul_free(args);
	return 0;
}

