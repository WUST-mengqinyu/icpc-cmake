namespace inner {
  namespace type_traits {
#include <type_traits>
    using UI = unsigned int;
#define SC std::conditional
#define SS std::is_same
#define I128 __int128
    template<class T>
    using IS_SI_128 = typename SC<SS<T, I128>::value || SS<T, I128>::value, std::true_type, std::false_type>::type;
    template<class T>
    using IS_UI_128 = typename SC<SS<T, __uint128_t>::value || SS<T, unsigned I128>::value, std::true_type, std::false_type>::type;
    template<class T>
    using TO_UI_128 = typename SC<SS<T, I128>::value, __uint128_t, unsigned I128>;
    template<class T>
    using is_integral = typename SC<std::is_integral<T>::value || IS_SI_128<T>::value || IS_UI_128<T>::value, std::true_type, std::false_type>::type;
    template<class T>
    using IS_SI = typename SC<(is_integral<T>::value && std::is_signed<T>::value) || IS_SI_128<T>::value, std::true_type, std::false_type>::type;
    template<class T>
    using IS_UI = typename SC<(is_integral<T>::value && std::is_unsigned<T>::value) || IS_UI_128<T>::value, std::true_type, std::false_type>::type;
    template<class T>
    using TO_U = typename SC<IS_SI_128<T>::value, TO_UI_128<T>, typename SC<std::is_signed<T>::value, std::make_unsigned<T>, std::common_type<T>>::type>::type;
    template<class T>
    using IS_UI_T = std::enable_if_t<IS_UI<T>::value>;
    template<class T>
    using IS_SI_T = std::enable_if_t<IS_SI<T>::value>;
#undef SC
  }// namespace type_traits
}// namespace inner
