#include<bits/stdc++.h>
using namespace std;
struct T{//商品类
    int indice=0,frequency=0;//商品编号、商品购买次数
    T(int i,int f):indice(i),frequency(f){}//构造函数
    bool operator <(const T&t)const{//重载小于运算符
        return this->frequency!=t.frequency?this->frequency>t.frequency:this->indice<t.indice;
    }
};
int m[50005];//存储物品购买次数
set<T>s;
int main(){
    int N,K,a;
    scanf("%d%d%d",&N,&K,&a);
    s.insert(T(a,1));//将第一个物品压入s中
    ++m[a];//递增第一个物品购买次数
    for(int i=2;i<=N;++i){
        scanf("%d",&a);
        printf("%d: ",a);
        int k=0;
        for(auto i=s.cbegin();i!=s.cend()&&k<K;++i){//输出s中前K个物品
            printf("%s%d",i!=s.cbegin()?" ":"",i->indice);
            ++k;
        }
        printf("\n");
        auto it=s.find(T(a,m[a]));
        if(it!=s.end())//查找s中是否含当前物品，如果包含，则进行删除
            s.erase(it);
        s.insert(T(a,++m[a]));//将更新后的商品及其购买次数压入s
    }
    return 0;
}
