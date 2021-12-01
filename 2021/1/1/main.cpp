#include <iostream>
#include <fstream>

int main() {
	std::fstream input("1/input", std::ios::in);

	std::string line;
	std::getline(input, line);
	int prev = std::stoi(line);

	int increases = 0;
	while (std::getline(input, line)) {
		int next = std::stoi(line);

		if (next > prev) {
			increases++;
		}

		prev = next;
	}

	std::cout << "Increases: " << increases << '\n';
}
