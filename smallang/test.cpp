#include <iostream>

auto
add(int a)
{
  return [=](int b) { return a + b; };
}

auto
times(int a)
{
  return [=](int b) { return a * b; };
}

template<typename T>
void
print(T a)
{
  std::cout << a << "\n";
}

int
main()

{
  const auto a = (add(19))(14);
  const auto b = (times(12))(3);
  print((add(a)(b)));
}
