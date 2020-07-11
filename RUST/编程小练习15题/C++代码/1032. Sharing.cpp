#include<bits/stdc++.h>
using namespace std;
char data[1000000];//结点地址有5位数字，故这个哈希表数组至少要开到10的6次方
int Next[1000000];//结点地址有5位数字，故这个哈希表数组至少要开到10的6次方
bool hashTable[1000000];//结点地址有5位数字，故这个哈希表数组至少要开到10的6次方
int main(){
    int begin1,begin2,N;//链表1起始地址、链表2起始地址，给出的结点总数
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
