#include "inner/prelude"

namespace inner {
  enum NodeType { Add = 0,
                  Mul,
                  Div,
                  Mod,
                  Set,
                  Max,
                  Min,
                  Null };

  template<class Num, NodeType Tp>
  struct PolyTag {
    NodeType tp;
    PolyTag(Num x) : x(x), tp(Tp) {}
    PolyTag() : x(0), tp(NodeType::Null) {}
    template<NodeType Tp2>
    void apply(const PolyTag<Num, Tp2> &oth) {
      if (oth.tp == NodeType::Null) return;
      tp = Tp;
      switch (Tp2) {
        case NodeType::Add:
          this->x += oth.val();
          break;
        case NodeType::Mul:
          this->x *= oth.val();
          break;
        case NodeType::Div:
          this->x /= oth.val();
          break;
        // case NodeType::Mod:
        //   this->x %= oth.val();
        //   break;
        case NodeType::Set:
          this->x = oth.val();
          break;
        // case NodeType::Max:
        // if (this->x < oth.val()) this->x = oth.val();
        // break;
        // case NodeType::Min:
        // if (this->x > oth.val()) this->x = oth.val();
        // break;
        default:
          break;
      }
    }
    Num val() const {
      return x;
    }
    virtual const bool is_zero() const { return false; }
    template<NodeType Tp2>
    void value_add(const PolyTag<Num, Tp2> &b) { this->apply(b); }

    template<NodeType Tp2>
    PolyTag<Num, Tp> &operator+=(const PolyTag<Num, Tp2> &oth) {
      apply(oth);
      return *this;
    }

private:
    Num x;
  };

  template<class Num, NodeType Tp>
  struct PolyNode {
    int len;
    PolyNode(Num x) : len(1), x(x) {}
    PolyNode() : len(0), x(0) {}
    Num val() const {
      return x;
    }
    virtual const bool is_zero() const { return false; }
    template<NodeType Tp2>
    PolyNode<Num, Tp> &operator+=(const PolyTag<Num, Tp2> &oth) {
      switch (oth.tp) {
        case NodeType::Add:
          this->x += Num(this->len) * oth.val();
          break;
        // case NodeType::Min:
        // if (this->x > oth.val()) this->x = oth.val();
        // break;
        // case NodeType::Max:
        // if (this->x < oth.val()) this->x = oth.val();
        // break;
        default:
          break;
      }
      return *this;
    }

    template<NodeType Tp2>
    PolyNode<Num, Tp> &operator+=(const PolyNode<Num, Tp2> &oth) {
      switch (Tp2) {
        case NodeType::Add:
          this->x += oth.val();
          break;
        // case NodeType::Min:
        // if (this->x > oth.val()) this->x = oth.val();
        // break;
        // case NodeType::Max:
        // if (this->x < oth.val()) this->x = oth.val();
        // break;
        default:
          break;
      }
      this->len += oth.len;
      return *this;
    }

private:
    Num x;
  };

  template<class T>
  concept Comparable = requires(const T &a, const T &b) {
    { a < b };
  };

  //   template<Comparable Num, NodeType Tp>
  //   struct PolyNode {
  //     int len;
  //     PolyNode(Num x) : len(1), x(x) {}
  //     PolyNode() : len(0), x(0) {}
  //     Num val() const {
  //       return x;
  //     }
  //     virtual const bool is_zero() const { return false; }
  //     template<NodeType Tp2>
  //     PolyNode<Num, Tp> &operator+=(const PolyTag<Num, Tp2> &oth) {
  //       switch (oth.tp) {
  //         case NodeType::Add:
  //           this->x += Num(this->len) * oth.val();
  //           break;
  //         default:
  //           break;
  //       }
  //       return *this;
  //     }

  //     template<NodeType Tp2>
  //     PolyNode<Num, Tp> &operator+=(const PolyNode<Num, Tp2> &oth) {
  //       switch (Tp2) {
  //         case NodeType::Add:
  //           this->x += oth.val();
  //           break;
  //         default:
  //           break;
  //       }
  //       this->len += oth.len;
  //       return *this;
  //     }

  // private:
  //     Num x;
  //   };

  template<std::size_t... Is>
  constexpr auto index_sequence_reverse(std::index_sequence<Is...> const &) -> decltype(std::index_sequence<sizeof...(Is) - 1U - Is...>{});
  template<std::size_t N>
  using make_index_sequence_reverse = decltype(index_sequence_reverse(std::make_index_sequence<N>{}));

  template<class... T, class... T2>
  void reverse_add(std::tuple<T...> &tp, const std::tuple<T2...> &tp2) {
    [&]<class TupType2, size_t... I2>(const TupType2 &_tup2, std::index_sequence<I2...>) {
      (...,
       ([&]<class E2, class TupType, size_t... I>(TupType &_tup, std::index_sequence<I...>, E2 e) {
         //  (..., (std::cout << "(" << std::get<I2>(_tup).val() << "+=" << e.val() << ")\n"));
         (..., (std::get<I>(_tup) += e));
       }(tp, std::make_index_sequence<std::min(I2 + 1, sizeof...(T))>(), std::get<I2>(_tup2))));
    }(tp2, make_index_sequence_reverse<sizeof...(T2)>());
  }

  template<class Num, NodeType... Tp>
  struct Poly {
    std::tuple<PolyTag<Num, Tp>...> x;
    Poly(PolyTag<Num, Tp>... x) : x(x...) {}

    const bool is_zero() const {
      return std::apply([](const auto &...nodes) { return (... && nodes.is_zero()); }, x);
    }

    template<NodeType... Tp2>
    void value_add(const Poly<Num, Tp2...> &oth) {
      reverse_add(x, oth.x);
    }

    template<NodeType... Tp2>
    Poly<Num, Tp...> &operator+=(const Poly<Num, Tp2...> &oth) {
      this->value_add(oth);
      return *this;
    }
  };

#define DefineNodeTp(tp)                              \
  template<typename Num, NodeType _tp = tp>           \
  struct tp##Tag : PolyTag<Num, _tp> {                \
    tp##Tag() : PolyTag<Num, NodeType::tp>() {}       \
    tp##Tag(Num x) : PolyTag<Num, NodeType::tp>(x) {} \
  };

  DefineNodeTp(Add);
  DefineNodeTp(Mul);
  DefineNodeTp(Mod);
  DefineNodeTp(Set);
  DefineNodeTp(Max);
  DefineNodeTp(Min);
}// namespace inner