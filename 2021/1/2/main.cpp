#include <iostream>
#include <fstream>
#include <array>

int main() {
	std::fstream input("2/input", std::ios::in);

	std::array<int, 3> a;

	std::string line;
	int prev = 0;
	int increases = 0;
	for (int i = 0; std::getline(input, line); ++i) {
		std::array<int, 3> temp = a;
		a[0] = std::stoi(line);
		a[1] = temp[0];
		a[2] = temp[1];

		int sum = a[0] + a[1] + a[2];

		if (i >= 3 && sum > prev) {
			increases++;
		}

		prev = sum;
	}

	std::cout << "Increases: " << increases << '\n';
}
