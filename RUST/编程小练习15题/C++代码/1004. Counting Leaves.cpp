#include<iostream>
using namespace std;
struct Node {
    int father;
    int flag;
    int level;
};
int main() {
    int i, j, num, N, M, ID, child;
    struct Node node[205];
    int result[205] = {0};
    int maxlevel = 1;



    cin >> N >> M;
    for (i = 0; i <= N; i++) {
        node[i].level = 0;
        node[i].flag = 0;
        node[i].father = 0;
    }

    
    node[1].level = 1;
    for (i = 1; i <= M; i++) {
        cin >> ID >> num;
        if (num != 0)
            node[ID].flag = 1;
        for (j = 1; j <= num; j++) {
            cin >> child;
            node[child].father = ID;
        }
    }
    for (i = 1; i <= N; i++) {
        for (j = 1; j <= N; j++) {
            if (node[j].father == i)
                node[j].level = node[i].level + 1;
        }
    }
    for (i = 1; i <= N; i++) {
        if (node[i].flag == 0 && node[i].level > 0)
            result[node[i].level]++;
        if (node[i].level > maxlevel)
            maxlevel = node[i].level;
    }
    for (i = 1; i < maxlevel; i++) {
        cout << result[i] << " ";
    }
    cout << result[i];
    return 0;
}
