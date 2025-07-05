#include "subLib/soul_stdio.hpp"
#include "subLib/soul_stdmem.hpp"
#include "subLib/math/soul_math.hpp"
#include "subLib/Collections/soul_Collections.hpp"

#define __GET_SOUL_ARGS(name) auto __var_args = __Soul_ARRAY__<__Soul_ARRAY__<char>>(__SOUL_C_argsc); for(int i = 0; i < __SOUL_C_argsc; i++){ __var_args[i] = str((const char*)__SOUL_C_argsv[i]); } __Soul_ARRAY__<__Soul_ARRAY__<char>>::AsConst* const name = (__Soul_ARRAY__<__Soul_ARRAY__<char>>::AsConst* const)&__var_args;









