#include <bits/stdc++.h>
using namespace std;
using gg = long long;
int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    unordered_map<gg, gg> um;  //记录配偶ID
    gg n, a, b;
    cin >> n;
    while (n--) {
        cin >> a >> b;
        um[a] = b;
        um[b] = a;
    }
    cin >> n;
    set<gg> s;  //记录是否出现在派对上并排序
    while (n--) {
        cin >> a;
        if (not um.count(a) or not s.count(um[a])) {
            s.insert(a);
        } else {
            s.erase(um[a]);
        }
    }
    cout << s.size() << '\n' << setfill('0');
    for (auto i = s.begin(); i != s.end(); ++i) {
        cout << (i == s.begin() ? "" : " ") << setw(5) << *i;
    }
    return 0;
}