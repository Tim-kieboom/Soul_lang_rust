#include "subLib/soul_print.hpp"
#include "subLib/math/soul_math.hpp"
#include "subLib/soul_Fail_throw.hpp"
#include "subLib/Collections/soul_Collections.h"

/*
[<cpp args>] -> [<soul args>]

    __Soul_ARRAY__<__Soul_STR__> args(argc);
    for (int i = 0; i < argc; i++)
        args.__soul_UNSAFE_at__(i) = __Soul_STR__(argv[i]);
*/
