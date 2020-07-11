#include<bits/stdc++.h>
using namespace std;
long long gcd(long long a,long long b){//辗转相除法求最大公约数
    return b==0?a:gcd(b,a%b);
}
int main(){
    int N;
    scanf("%d",&N);
    pair<long long,long long>f,sum({0,1});//用pair来储存分数，first存储分子，second存储分母
    while(N--){
        scanf("%lld/%lld",&f.first,&f.second);//读取数据
        sum.first=sum.first*f.second+sum.second*f.first;//计算分子
        sum.second*=f.second;//计算分母
        long long k=gcd(sum.first,sum.second);//求分子分母最大公约数
        sum.first/=k;//约分
        sum.second/=k;
    }
    long long k=sum.first/sum.second;//求整数部分
    sum.first%=sum.second;//将整数部分提取出之后求分子
    if(k!=0&&sum.first!=0)
        printf("%lld %lld/%lld",k,sum.first,sum.second);
    else if(k==0&&sum.first!=0)
        printf("%lld/%lld",sum.first,sum.second);
    else
        printf("%lld",k);
    return 0;
}
