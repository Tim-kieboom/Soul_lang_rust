#include "soul_hardCodedFunctions/soul_hardCodedFunctions.h"
#if defined(__clang__) && defined(__cplusplus)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wparentheses-equality"
#elif defined(__GNUC__) && !defined(__clang__) && defined(__cplusplus)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wparentheses"
#endif
extern const char ch;
extern const float __var_float;
extern const int number;
extern const __Soul_ARRAY__<__Soul_ARRAY__<char>::AsConst>::AsConst strarray;

constexpr char __temp__Soul_c_str_6__[] = "1 + 2 = ";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_6__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_6__);
constexpr char __temp__Soul_c_str_8__[] = "!!error!! test error";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_8__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_8__);
constexpr char __temp__Soul_c_str_2__[] = "ello";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_2__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_2__);
constexpr char __temp__Soul_c_str_7__[] = "input: ";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_7__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_7__);
constexpr char __temp__Soul_c_str_3__[] = "";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_3__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_3__);
constexpr char __temp__Soul_c_str_4__[] = ".";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_4__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_4__);
constexpr char __temp__Soul_c_str_0__[] = "werg";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_0__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_0__);
constexpr char __temp__Soul_c_str_1__[] = "hoiu";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_1__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_1__);
constexpr char __temp__Soul_c_str_5__[] = "hello world";
constexpr __Soul_ARRAY__<char>::AsConst __Soul_c_str_5__ = __Soul_ARRAY_LiteralCtor__(char, __temp__Soul_c_str_5__);

void ifs();
void whileArgs(__Soul_ARRAY__<__Soul_ARRAY__<char>>::AsConst const* args);
void array();
void Input(__Soul_ARRAY__<char>* buffer, const __Soul_ARRAY__<char>::AsConst prefix);
void ref();

constexpr int __programMemory_5[] = {1};
constexpr __Soul_ARRAY__<char>::AsConst __programMemory_3[] = {__Soul_c_str_2__,__Soul_c_str_1__,__Soul_c_str_0__,__Soul_c_str_0__};
constexpr int __programMemory_4[] = {1,2,3,4};

const char ch = 'a';
const int number = 1;
const float __var_float = 1;
constexpr float gravity = 9.81;
constexpr unsigned int uintNumber = 1;
const __Soul_ARRAY__<__Soul_ARRAY__<char>::AsConst>::AsConst strarray = __Soul_ARRAY_LiteralCtor__(__Soul_ARRAY__<char>,__programMemory_3);


void ifs() {
	int condition = 1;
	if((condition == 1)) {
		int f = 1;
		condition = 1;
		Println(condition);
	}
	if((condition == 2)) {
		int f = 1;
		Println(condition);
	}
	else {
		int f = 1;
		Println(condition);
	}

}


void array() {
	__Soul_ARRAY__<int> array = __Soul_ARRAY_LiteralCtor__(int,__programMemory_4);
	unsigned int index = 3;
	int32_t name = 1;
	int const* second = (int const*)&array[1];
	second = (int const*)&array[index];
	array[0] = 200;
	Println(array[0]);
}


void whileArgs(__Soul_ARRAY__<__Soul_ARRAY__<char>>::AsConst const* args) {
	unsigned int argsLen = __soul_internal_length__(args);
	int i = -1;
	int lastIndex = int(argsLen);
	lastIndex = (lastIndex - 1);
	while((i++ < lastIndex)) {
		Println(__soul_format_string__(__Soul_c_str_3__,i,__Soul_c_str_4__,(*args)[uint(i)],__Soul_c_str_3__));
	}
}


void ref() {
	int const* f = (int const*)&__programMemory_5;
	int32_t num = i32(1);
	int32_t const* ref = (int32_t const*)&num;
	__Soul_ARRAY__<char> buffer = str();
	__Soul_ARRAY__<char>::AsConst const* refstr = (__Soul_ARRAY__<char>::AsConst const*)&buffer;
}


void Input(__Soul_ARRAY__<char>* buffer, const __Soul_ARRAY__<char>::AsConst prefix) {
	Print(prefix);
	Input(buffer);
}


namespace __SOUL_ns_scp_10__{
	int sum(const int a, const int b);
	int sum(const int a, const int b) {
		
		__soul_free(b);
		__soul_free(a);
		return (a + b);
	}
}
int main(int __SOUL_C_argsc, char** __SOUL_C_argsv) {

	__GET_SOUL_ARGS(args)
	Println((*args)[0]);
	Println(__Soul_c_str_5__);
	Println(__soul_format_string__(__Soul_c_str_6__,__SOUL_ns_scp_10__::sum(1,2),__Soul_c_str_3__));
	array();
	whileArgs(args);
	ref();
	__Soul_ARRAY__<char> buffer = str();
	Input(&buffer,__Soul_c_str_7__);
	Println(buffer);
	ifs();
	EPrintln(__Soul_c_str_8__);
	
	__soul_free(buffer);
	__soul_free(args);
	return 0;
}

#if defined(__clang__) && defined(__cplusplus)
#pragma clang diagnostic pop
#elif defined(__GNUC__) && !defined(__clang__) && defined(__cplusplus)
#pragma GCC diagnostic pop
#endif