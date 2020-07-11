#include<bits/stdc++.h>
using namespace std;
char data[10000];
int Next[10000];
bool hashTable[10000];
int main(){
    int begin1,begin2,N;
    scanf("%d%d%d",&begin1,&begin2,&N);
    while(N--){
        int a;
        scanf("%d",&a);
        scanf(" %c %d",&data[a],&Next[a]);
    }
    while(begin1!=-1){
        hashTable[begin1]=true;
        begin1=Next[begin1];
    }
    while(begin2!=-1){
        if(hashTable[begin2]){
            printf("%05d",begin2);
            return 0;
        }
        begin2=Next[begin2];
    }
    printf("-1");//输出-1
    return 0;
}
