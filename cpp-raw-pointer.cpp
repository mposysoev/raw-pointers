#include <iostream>

int main()
{
    int* p = nullptr;
    int i = 5;
    p = &i;
    int j = *p;

    std::cout << "p: " << p << std::endl;
    std::cout << "i: " << i << std::endl;
    std::cout << "j: " << j << std::endl;

    return 0;
}