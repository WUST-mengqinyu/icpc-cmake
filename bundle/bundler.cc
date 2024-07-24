#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <iterator>
#include <map>
#include <print>
#include <set>
#include <stack>
#include <string>
#include <vector>

#include "consts.h"

#include "ctre.hpp"

const auto REPLACE_HEADERS = std::vector{
        std::tuple{"inner/", PROJECT_DIR}};
const auto EXCEPT_DEF = "badcw";

std::set<std::string> get_whitelist_libs() {
  // TODO: use env var to control whitelist
  // auto bundler_whitelist_libs_str = std::getenv("BUNDLER_WHITELIST_LIBS");
  std::set<std::string> res;
  // for (auto whitelist: DEFAULT_WHITELIST) {
  //   res.insert(whitelist);
  // }
  return res;
}


template<typename T>
void print_vector(const T &it) {
  std::print("[");
  for (auto i = it.begin(); i != it.end(); i = std::next(i)) {
    std::print("{0}", *i);
    if (std::next(i) != it.end()) {
      std::print(", ");
    }
  }
  std::println("]");
}

auto get_file_except_some_def(const std::string &absolute_path) -> std::string {
  std::ifstream file(absolute_path);

  if (file.is_open()) {

    std::string line;
    std::string processedText;
    std::stack<bool> blockStack;
    bool skipBlock = false;
    while (std::getline(file, line)) {
      if (ctre::match<R"(^\s*#ifdef\s+XYZ\s*$)">(line) || ctre::match<R"(^\s*#ifndef\s+XYZ\s*$)">(line)) {
        blockStack.push(true);
        skipBlock = true;
      } else if (ctre::match<R"(^\s*#ifdef\s+(?:(?!XYZ).)+\s*$)">(line) || ctre::match<R"(^\s*#ifndef\s+(?:(?!XYZ).)+\s*$)">(line)) {
        blockStack.push(false);
      } else if (ctre::match<R"(^\s*#else\s*$)">(line) || ctre::match<R"(^\s*#elif\s+.+$)">(line)) {
        if (!blockStack.empty()) {
          if (blockStack.top()) {
            skipBlock = !skipBlock;
          }
          if (ctre::match<R"(^\s*#elif\s+.+$)">(line)) {
            blockStack.push(skipBlock);
          }
        }
      } else if (ctre::match<R"(^\s*#endif\s*$)">(line)) {
        if (!blockStack.empty()) {
          bool wasSkipping = blockStack.top();
          blockStack.pop();
          skipBlock = (!blockStack.empty()) ? blockStack.top() : false;
          if (wasSkipping) {
            continue;
          }
        }
      }
      if (!skipBlock) {
        processedText += line + "\n";
      }
    }

    file.close();
    return processedText;
  }

  return "";
}

auto get_file_quote_dep_headers(const std::string &absolute_path) -> std::vector<std::string> {
  static auto file_content_cache = std::map<std::string, std::vector<std::string>>{};
  const auto file_content_cache_max = 128;

  if (file_content_cache.contains(absolute_path)) {
    return file_content_cache[absolute_path];
  }

  std::vector<std::string> includes;
  std::set<std::string> has_included;// used to remove dup

  auto content = get_file_except_some_def(absolute_path);// remove except parts

  constexpr auto pattern = ctll::fixed_string{R"(#include \"([^\"]*)\")"};
  for (auto &&match: ctre::search_all<pattern>(content)) {
    auto include = std::string(match.get<1>().to_view());
    if (has_included.contains(include)) {
      continue;
    }
    has_included.insert(include);
    includes.push_back(include);
  }

  // write back to cache
  if (file_content_cache.size() < file_content_cache_max) {
    file_content_cache.emplace(absolute_path, includes);
  }

  return includes;
}


struct GetDepsRecusiveContext {
  std::set<std::string> has_found;
  std::vector<std::string> all_deps;
  // std::stack<std::string> stk;
  GetDepsRecusiveContext() {
    has_found.clear();
    all_deps.clear();
  }
};

auto get_file_quote_deps_recusive(GetDepsRecusiveContext &ctx, const std::string &absolute_path) {
  if (ctx.has_found.contains(absolute_path)) {
    return;
  }
  ctx.has_found.insert(absolute_path);
  for (auto x: get_file_quote_dep_headers(absolute_path)) {
    for (auto &[match_prefix, replace]: REPLACE_HEADERS) {
      int i = 0;
      bool is_prefix = true;
      while (true) {
        if (match_prefix[i] == 0) break;
        else if (match_prefix[i] == x[i])
          i++;
        else {
          is_prefix = false;
          break;
        }
      }
      if (!is_prefix) {
        continue;
      }
      get_file_quote_deps_recusive(ctx, std::format("{}/{}", replace, x));
    }
  }
  ctx.all_deps.push_back(absolute_path);
}

std::vector<std::string> split_string_by_semicolon(const std::string &input) {
  std::vector<std::string> result;
  size_t start = 0;
  size_t end = input.find(';');// 查找第一个分号

  // 遍历字符串直到找不到分号
  while (end != std::string::npos) {
    result.push_back(input.substr(start, end - start));// 添加当前子串到结果中
    start = end + 1;                                   // 移动到下一个子串的起始位置
    end = input.find(';', start);                      // 查找下一个分号
  }

  // 添加最后一个子串到结果中（如果有的话）
  result.push_back(input.substr(start));

  return result;
}

void clear_output_path(std::string path) {
  namespace fs = std::filesystem;
  fs::path out_path(path);
  try {
    if (fs::exists(out_path) && fs::is_directory(out_path)) {
      for (const auto &entry: fs::directory_iterator(out_path)) {
        if (fs::is_regular_file(entry)) {
          if (entry.path().filename() == ".gitkeep") {
            continue;
          }
          fs::remove(entry.path());
          std::println("delete {0}", entry.path().c_str());
        }
      }
    } else {
      std::println("path does not exist or is not a directory: {0}", path);
    }
  } catch (const fs::filesystem_error &e) {
    std::println(stderr, "clear output path failed: {0}", e.what());
  }
}


auto replace_header_with_file_content(std::ofstream &of, const std::string &absolute_path, const std::vector<std::string> &deps_path) {
  std::ifstream source(absolute_path);
  std::string line;
  while (std::getline(source, line)) {
    constexpr auto pattern = ctll::fixed_string{R"(#include \"([^\"]*)\")"};
    bool remove = false;
    for (auto &&match: ctre::search_all<pattern>(line)) {
      std::string_view matchStr = match.get<1>();
      for (auto &dep: deps_path) {
        if (dep.ends_with(matchStr)) {
          remove = true;
          break;
        }
      }
    }
    if (!remove) of << line << std::endl;
  }
}

auto main() -> int {
  clear_output_path(BUNDLE_OUT_DIR);

  auto icpc_main_sources = split_string_by_semicolon(ICPC_MAIN_SOURCE);
  auto icpc_executable_lists = split_string_by_semicolon(ICPC_EXECUTABLE_LIST);
  if (icpc_main_sources.size() != icpc_executable_lists.size()) {
    std::println("unexpected ICPC_MAIN_SOURCE num detected, source {0} has num:{1}, executable {2} has num:{3}", ICPC_MAIN_SOURCE, icpc_main_sources.size(), ICPC_EXECUTABLE_LIST, icpc_executable_lists.size());
    return -1;
  }
  int all_targets_size = icpc_main_sources.size();
  for (int i = 0; i < all_targets_size; ++i) {
    auto &target = icpc_executable_lists[i];
    auto &source = icpc_main_sources[i];
    auto ctx = GetDepsRecusiveContext{};
    get_file_quote_deps_recusive(ctx, source);
    std::ofstream outfile(std::format("{}/{}.cc", BUNDLE_OUT_DIR, target));
    for (auto &dep: ctx.all_deps) {
      replace_header_with_file_content(outfile, dep, ctx.all_deps);
    }
    outfile.close();
  }
  return 0;
}