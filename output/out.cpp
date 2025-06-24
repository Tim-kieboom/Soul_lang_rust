#include "soul_hardCodedFunctions/soul_hardCodedFunctions.h"


constexpr char __temp__Soul_c_str_2__[] = "args length: ";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_2__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_2__);
constexpr char __temp__Soul_c_str_5__[] = "foo ";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_5__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_5__);
constexpr char __temp__Soul_c_str_3__[] = "hello world";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_3__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_3__);
constexpr char __temp__Soul_c_str_0__[] = "";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_0__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_0__);
constexpr char __temp__Soul_c_str_4__[] = "1 + 2 = ";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_4__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_4__);
constexpr char __temp__Soul_c_str_1__[] = " values: ";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_1__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_1__);


constexpr int __programMemory_1[] = {1};


namespace __SOUL_ns_scp_1__{
	int sum(const int a, const int b);
	int sum(const int a, const int b) {
		
		__soul_free(b);
		__soul_free(a);
		return (a + b);
	}
}
int main(int __SOUL_C_argsc, char** __SOUL_C_argsv) {

	auto __var_args = __Soul_ARRAY__<__Soul_ARRAY__<char>>(__SOUL_C_argsc); for(int i = 0; i < __SOUL_C_argsc; i++){ *__var_args.__get_mutRef(i) = str((const char*)__SOUL_C_argsv[i]); } __Soul_ARRAY__<__Soul_ARRAY__<char>>* const args = (__Soul_ARRAY__<__Soul_ARRAY__<char>>* const)&__var_args; 
	unsigned int argsLen = __soul_internal_length__(args);
	Println(__soul_format_string__(__Soul_c_str_2__,argsLen,__Soul_c_str_1__,args,__Soul_c_str_0__));
	Println(__soul_format_string__(__Soul_c_str_3__));
	Println(__soul_format_string__(__Soul_c_str_4__,__SOUL_ns_scp_1__::sum(1,2),__Soul_c_str_0__));
	int* const f = (int* const)&__programMemory_1;
	int32_t num = i32(1);
	int32_t* const ref = (int32_t* const)&num;
	__Soul_ARRAY__<char> buffer = str();
	__Soul_ARRAY__<char>::AsConst* const refstr = (__Soul_ARRAY__<char>::AsConst* const)&buffer;
	Print(__Soul_c_str_5__);
	Input(&buffer);
	Println((__Soul_ARRAY__<char>::AsConst* const)&buffer);
	
	__soul_free(refstr);
	__soul_free(ref);
	__soul_free(num);
	__soul_free(f);
	__soul_free(buffer);
	__soul_free(argsLen);
	__soul_free(args);
	return 0;
}

