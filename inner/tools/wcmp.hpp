#include "color.hpp"
#include "testlib.h"
#include <gtest/gtest.h>

bool wcmp(std::string _out, std::string _ans) {
  std::string firstElems;
  ouf.init(_out, TMode::_output);
  ans.init(_ans, TMode::_answer);

  int n = 0;
  std::string j, p;

  while (!ans.seekEof() && !ouf.seekEof()) {
    n++;

    ans.readWordTo(j);
    ouf.readWordTo(p);

    if (j != p) {
      std::cerr << WA << " " << n << englishEnding(n).c_str() << " words differ - expected: '" << compress(j).c_str() << "', found: '" << compress(p).c_str() << "'" << std::endl;
      // GTEST_LOG_(ERROR) << WA << n << englishEnding(n).c_str() << " words differ - expected: '" << compress(j).c_str() << "', found: '" << compress(p).c_str() << "'";
      return false;
    }
    // quitf(_wa, "%d%s words differ - expected: '%s', found: '%s'", n, englishEnding(n).c_str(),
    // compress(j).c_str(), compress(p).c_str());
  }

  if (ans.seekEof() && ouf.seekEof()) {
    if (n == 1) {
      std::cout << AC << " " << compress(j).c_str() << std::endl;
      return true;
    }
    // quitf(_ok, "\"%s\"", compress(j).c_str());
    else {
      std::cout << AC << " " << n << " tokens" << std::endl;
      return true;
    }
    // quitf(_ok, "%d tokens", n);
  } else {
    if (ans.seekEof()) {
      std::cerr << WA << " " << "Participant output contains extra tokens" << std::endl;
      // quitf(_wa, "Participant output contains extra tokens");
    } else {
      std::cerr << WA << " " << "Unexpected EOF in the participants output" << std::endl;
      // quitf(_wa, "Unexpected EOF in the participants output");
    }
    return false;
  }
}