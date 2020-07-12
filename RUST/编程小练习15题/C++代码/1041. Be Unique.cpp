#include<bits/stdc++.h>
using namespace std;
int main(){
    unordered_map<int,int>m;
    int n;
    scanf("%d",&n);
    int A[n];
    for(int i=0;i<n;++i){
        scanf("%d",&A[i]);
        ++m[A[i]];//递增出现次数
    }
    for(int i=0;i<n;++i)
        if(m[A[i]]==1){
            printf("%d",A[i]);
            return 0;
        }
    printf("None");//如果没找到，输出None
    return 0;
}
