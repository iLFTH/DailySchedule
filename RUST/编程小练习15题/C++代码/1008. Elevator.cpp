#include<bits/stdc++.h>
using namespace std;
int main(){
    int N,current=0,sumTime=0,a;
    scanf("%d",&N);
    while(scanf("%d",&a)!=EOF){
        if(a>current)
            sumTime+=6*(a-current)+5;
        else if(a<current)
            sumTime+=4*(current-a)+5;
        else
            sumTime+=5;
        current=a;
    }
    printf("%d",sumTime);
    return 0;
}
