#include "inner/prelude"

namespace inner {
  namespace num_bit {
    int ceil_pow2(int n) {
#if __cplusplus > 201703L && __GNUC__
      return std::bit_width(std::uint32_t(n));
#else
      int x = 0;
      while ((1U << x) < (unsigned int) (n)) x++;
      return x;
#endif
    }
  }// namespace num_bit
}// namespace inner