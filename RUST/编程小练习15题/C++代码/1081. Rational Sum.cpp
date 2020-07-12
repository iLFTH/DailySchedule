

#include <cstdio>
#include <cstring>
#include <string>
using namespace std;
 
char s[111];
 
long long gcd(long long x,long long y) {
    return y?gcd(y, x % y):x;
}
 
int main() {
long long a = 0, b = 1; //a / b
int n;
    for (scanf("%d",&n);n;--n) {
        scanf("%s",s);
        char *t = strstr(s,"/");
        if (t) {
            *t = ' ';
        }
        long long c, d;
        sscanf(s,"%lld%lld",&c,&d);
        // a / b + c / d
        long long aa = a * d + b * c;
        long long bb = b * d;
        long long g = gcd((aa < 0)?(-aa):aa, bb);
        a = aa / g;
        b = bb / g;
    }
    long long x = a / b, y = a % b;
    if (y == 0) {
        printf("%lld\n",x);
    }
    else {
        if (x) {
            printf("%lld ",x);
        }
        printf("%lld/%lld\n",y,b);
    }
    return 0;
}



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
