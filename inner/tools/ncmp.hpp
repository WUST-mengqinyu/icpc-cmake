#include "color.hpp"
#include "testlib.h"
#include <gtest/gtest.h>
#include <string>

bool ncmp(std::string _out, std::string _ans) {
  std::string firstElems;
  ouf.init(_out, TMode::_output);
  ans.init(_ans, TMode::_answer);

  int n = 0;
  while (!ans.seekEof() && !ouf.seekEof()) {
    n++;
    long long j = ans.readLong();
    long long p = ouf.readLong();
    if (j != p) {
      GTEST_LOG_(ERROR) << WA << n << englishEnding(n).c_str() << " numbers differ - expected: '" << vtos(j).c_str() << "', found: '" << vtos(p).c_str() << "'";
      return false;
    }
    // quitf(_wa, "%d%s numbers differ - expected: '%s', found: '%s'", n, englishEnding(n).c_str(),
    // vtos(j).c_str(), vtos(p).c_str());
    else if (n <= 5) {
      if (firstElems.length() > 0)
        firstElems += " ";
      firstElems += vtos(j);
    }
  }

  int extraInAnsCount = 0;

  while (!ans.seekEof()) {
    ans.readLong();
    extraInAnsCount++;
  }

  int extraInOufCount = 0;

  while (!ouf.seekEof()) {
    ouf.readLong();
    extraInOufCount++;
  }

  if (extraInAnsCount > 0) {
    GTEST_LOG_(ERROR) << WA << "Output contains longer sequence [length = " << n + extraInOufCount << "], but answer contains " << n << " elements";
    return false;
  }
  // quitf(_wa, "Answer contains longer sequence [length = %d], but output contains %d elements",
  // n + extraInAnsCount, n);

  if (extraInOufCount > 0) {
    GTEST_LOG_(ERROR) << WA << "Output contains longer sequence [length = " << n + extraInOufCount << "], but answer contains " << n << " elements";
    return false;
  }
  // quitf(_wa, "Output contains longer sequence [length = %d], but answer contains %d elements",
  // n + extraInOufCount, n);

  if (n <= 5) {
    GTEST_LOG_(INFO) << AC << n << " number(s)\"" << compress(firstElems).c_str() << "\"";
    return true;
    // quitf(_ok, "%d number(s): \"%s\"", n, compress(firstElems).c_str());
  } else {
    GTEST_LOG_(INFO) << AC << n << " numbers";
    return true;
    // quitf(_ok, "%d numbers", n);
  }
}