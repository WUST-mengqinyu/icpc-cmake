#ifndef INCLUDE_INNER_TOOLS_COLOR_HPP
#define INCLUDE_INNER_TOOLS_COLOR_HPP

#include <ostream>
#include <string>
#define color_black 0
#define color_dark_blue 1
#define color_dark_green 2
#define color_light_blue 3
#define color_dark_red 4
#define color_magenta 5
#define color_orange 6
#define color_light_gray 7
#define color_gray 8
#define color_blue 9
#define color_green 10
#define color_cyan 11
#define color_red 12
#define color_pink 13
#define color_yellow 14
#define color_white 15

std::string get_textcolor_code(const int textcolor) {// Linux only
  switch (textcolor) {
    case 0:
      return "30";// color_black      0
    case 1:
      return "34";// color_dark_blue  1
    case 2:
      return "32";// color_dark_green 2
    case 3:
      return "36";// color_light_blue 3
    case 4:
      return "31";// color_dark_red   4
    case 5:
      return "35";// color_magenta    5
    case 6:
      return "33";// color_orange     6
    case 7:
      return "37";// color_light_gray 7
    case 8:
      return "90";// color_gray       8
    case 9:
      return "94";// color_blue       9
    case 10:
      return "92";// color_green     10
    case 11:
      return "96";// color_cyan      11
    case 12:
      return "91";// color_red       12
    case 13:
      return "95";// color_pink      13
    case 14:
      return "93";// color_yellow    14
    case 15:
      return "97";// color_white     15
    default:
      return "37";
  }
}
std::string get_backgroundcolor_code(const int backgroundcolor) {// Linux only
  switch (backgroundcolor) {
    case 0:
      return "40";// color_black      0
    case 1:
      return "44";// color_dark_blue  1
    case 2:
      return "42";// color_dark_green 2
    case 3:
      return "46";// color_light_blue 3
    case 4:
      return "41";// color_dark_red   4
    case 5:
      return "45";// color_magenta    5
    case 6:
      return "43";// color_orange     6
    case 7:
      return "47";// color_light_gray 7
    case 8:
      return "100";// color_gray       8
    case 9:
      return "104";// color_blue       9
    case 10:
      return "102";// color_green     10
    case 11:
      return "106";// color_cyan      11
    case 12:
      return "101";// color_red       12
    case 13:
      return "105";// color_pink      13
    case 14:
      return "103";// color_yellow    14
    case 15:
      return "107";// color_white     15
    default:
      return "40";
  }
}
std::string get_print_color(const int textcolor) {// Linux only
  return "\033[" + get_textcolor_code(textcolor) + "m";
}

std::string get_print_color(const int textcolor, const int backgroundcolor) {// Linux only
  return "\033[" + get_textcolor_code(textcolor) + ";" + get_backgroundcolor_code(backgroundcolor) + "m";
}

template<class T>
struct ColorS {
  T t;
  int c1, c2;
  ColorS(int textcolor, T t) : c1(textcolor), c2(0), t(t) {}
  ColorS(int textcolor, int backcolor, T t) : c1(textcolor), c2(backcolor), t(t) {}
  friend std::ostream &operator<<(std::ostream &os, const ColorS &c) {
    if (c.c2 == 0) {
      return os << get_print_color(c.c1) << c.t << "\033[0m";
    }
    return os << get_print_color(c.c1, c.c2) << c.t << "\033[0m";
  }
};

auto static WA = ColorS(color_red, color_dark_red, "wrong answer");
auto static AC = ColorS(color_green, color_dark_green, "ok");
auto static RE = ColorS(color_red, color_dark_red, "runtime error");

#endif