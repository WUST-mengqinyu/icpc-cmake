#include "inner/prelude"
using namespace inner;
const int N = 1e5 + 8;

MAIN() {
  std::string s;
  R(s);
  VI prefix(s.length(), 0);
  for (int i = 1; i < s.length(); i++) {
    int a = prefix[i - 1];
    while (a > 0 && s[i] != s[a]) {
      a = prefix[a - 1];
    }
    if (s[i] == s[a]) {
    }
    if (s[i] == s[a]) {
      a++;
    }
    prefix[i] = a;
  }

  std::string m = s.substr(0, prefix[s.length() - 1]);
  if (m.length() > s.length() / 2) {
    W("YES");
    W(m);
  } else {
    W("NO");
  }
}