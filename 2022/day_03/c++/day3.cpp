#include <iostream>
#include <fstream>
using namespace std;

int rankc(char c) {
  if (c >= int('a')) {
    return c - int('a') + 1;
  } else {
    return c - int('A') + 27;
  }
}

int solve1() {
  ifstream file("input.txt");

  string line;
  int out = 0;
  while (getline (file, line)) {
    int l = line.length() / 2;

    string x = line.substr(0, l);
    string y = line.substr(l, line.length());

    bool flag = false;
    for (int i=0; i<x.length(); i++) {
      for (int j=0; j<y.length(); j++) {
        if (x[i] == y[j]) {
          out += rankc(x[i]);
          flag = true;
          break;
        }
      }
      if (flag) {
        break;
      }
    }
  }
  file.close();
  return out;
}

int solve2() {
  string line;
  ifstream file("input.txt");

  int out = 0;
  string a;
  string b;
  string c;
  while (true) {
    if (!getline (file, a)) {
      break;
    }
    getline (file, b);
    getline (file, c);

    for (int i=0; i<a.length(); i++) {
      if (b.find(a[i]) != string::npos && c.find(a[i]) != string::npos) {
        out += rankc(a[i]);
        break;
      }
    }
  }
  file.close();
  return out;
}

int main() {
  cout << solve1();
  cout << "\n";
  cout << solve2();
  cout << "\n";
  return 0;
}